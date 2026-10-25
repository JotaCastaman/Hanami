use std::io::{BufRead, BufReader};
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use std::sync::mpsc::{self, RecvTimeoutError, Sender};
use std::thread;
use std::time::Duration;

use serde_json::json;
use unicode_width::UnicodeWidthChar;

use crate::common::{self, escape_markup};

/// width
const WIDTH: usize = 25;
/// scroll interval
const TICK: Duration = Duration::from_millis(200);
/// ticks
const HOLD_TICKS: u32 = 8;
/// separator
const SEPARATOR: &str = "   •   ";

const ICON_PLAYING: &str = "\u{f0387} \u{f04c}";
const ICON_PAUSED: &str = "\u{f0387} \u{f04b}";
const ICON_STOPPED: &str = "\u{f0387} \u{f04b}";

#[derive(Clone, Copy, PartialEq, Debug)]
enum Status {
    Playing,
    Paused,
    Stopped,
}

#[derive(Debug, PartialEq)]
enum Event {
    Track { status: Status, text: String },
    Gone,
}

/// "Playing\tArtist\tTitle"
fn parse(line: &str) -> Event {
    let mut parts = line.splitn(3, '\t');
    let status = match parts.next() {
        Some("Playing") => Status::Playing,
        Some("Paused") => Status::Paused,
        _ => return Event::Gone,
    };
    let artist = parts.next().unwrap_or("").trim();
    let title = parts.next().unwrap_or("").trim();
    let text = match (artist.is_empty(), title.is_empty()) {
        (_, true) if artist.is_empty() => return Event::Gone,
        (true, _) => title.to_string(),
        (_, true) => artist.to_string(),
        _ => format!("{artist} - {title}"),
    };
    Event::Track { status, text }
}

fn text_width(s: &str) -> usize {
    s.chars().map(|c| c.width().unwrap_or(0)).sum()
}

fn window(text: &str, offset: usize, width: usize) -> String {
    let looped: Vec<char> = text.chars().chain(SEPARATOR.chars()).collect();
    let mut out = String::new();
    let mut used = 0;
    for &c in looped.iter().cycle().skip(offset).take(looped.len()) {
        let w = c.width().unwrap_or(0);
        if used + w > width {
            break;
        }
        out.push(c);
        used += w;
    }
    out.push_str(&" ".repeat(width - used));
    out
}

struct State {
    status: Status,
    text: String,
    offset: usize,
    hold: u32,
}

impl State {
    fn new() -> Self {
        State { status: Status::Stopped, text: String::new(), offset: 0, hold: 0 }
    }

    fn apply(&mut self, event: Event) {
        match event {
            Event::Track { status, text } => {
                if text != self.text {
                    self.text = text;
                    self.offset = 0;
                    self.hold = HOLD_TICKS;
                }
                self.status = status;
            }
            Event::Gone => *self = State::new(),
        }
    }

       fn needs_scroll(&self) -> bool {
        self.status == Status::Playing && text_width(&self.text) > WIDTH
    }

    fn tick(&mut self) {
        if self.hold > 0 {
            self.hold -= 1;
            return;
        }
        let cycle = self.text.chars().count() + SEPARATOR.chars().count();
        self.offset = (self.offset + 1) % cycle;
        if self.offset == 0 {
            self.hold = HOLD_TICKS;
        }
    }

    fn render(&self) -> String {
        let (icon, class) = match self.status {
            Status::Stopped => {
                return json!({ "text": format!("{ICON_STOPPED} N/A"), "class": "stopped" })
                    .to_string();
            }
            Status::Playing => (ICON_PLAYING, "playing"),
            Status::Paused => (ICON_PAUSED, "paused"),
        };
        let shown = if text_width(&self.text) <= WIDTH {
            self.text.clone()
        } else {
            window(&self.text, self.offset, WIDTH)
        };
        json!({
            "text": format!("{icon} {}", escape_markup(&shown)),
            "class": class,
            "tooltip": escape_markup(&self.text),
        })
        .to_string()
    }
}

fn follow_player(player: String, tx: Sender<Event>) {
    loop {
        let mut cmd = Command::new("playerctl");
        cmd.arg(format!("--player={player}"))
            .args(["--follow", "metadata", "--format", "{{status}}\t{{artist}}\t{{title}}"])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());
        // SAFETY
        unsafe {
            cmd.pre_exec(|| {
                libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGTERM as libc::c_ulong);
                Ok(())
            });
        }

        match cmd.spawn() {
            Ok(mut child) => {
                let stdout = child.stdout.take().expect("stdout foi configurado como piped");
                for line in BufReader::new(stdout).lines().map_while(Result::ok) {
                    if tx.send(parse(&line)).is_err() {
                        let _ = child.kill();
                        return;
                    }
                }
                let _ = child.wait();
            }
            Err(e) => eprintln!("mpris: não consegui iniciar o playerctl: {e}"),
        }

        if tx.send(Event::Gone).is_err() {
            return;
        }
        thread::sleep(Duration::from_secs(2));
    }
}

pub fn run(player: &str) -> i32 {
    let (tx, rx) = mpsc::channel();
    let player = player.to_owned();
    thread::spawn(move || follow_player(player, tx));

    let mut state = State::new();
    let mut last = state.render();
    if common::emit(&last).is_err() {
        return 0;
    }

    loop {
        let event = if state.needs_scroll() {
            rx.recv_timeout(TICK)
        } else {
            rx.recv().map_err(|_| RecvTimeoutError::Disconnected)
        };
        match event {
            Ok(ev) => state.apply(ev),
            Err(RecvTimeoutError::Timeout) => state.tick(),
            Err(RecvTimeoutError::Disconnected) => return 1,
        }

        let out = state.render();
        if out != last {
            if common::emit(&out).is_err() {
                return 0; // waybar fechou o pipe: sai
            }
            last = out;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_has_constant_width_and_wraps() {
        let text = "abcdefghij";
        assert_eq!(window(text, 0, 5), "abcde");
        assert_eq!(window(text, 15, 5), "  abc"); // emendou com o começo
        let cycle = text.chars().count() + SEPARATOR.chars().count();
        for off in 0..cycle {
            assert_eq!(text_width(&window(text, off, 5)), 5);
        }
    }

    #[test]
    fn window_handles_wide_chars() {
        let text = "日本語のタイトル";
        let cycle = text.chars().count() + SEPARATOR.chars().count();
        for off in 0..cycle {
            assert_eq!(text_width(&window(text, off, 7)), 7);
        }
    }

    #[test]
    fn parse_lines() {
        assert_eq!(
            parse("Playing\tArtist\tTitle"),
            Event::Track { status: Status::Playing, text: "Artist - Title".into() }
        );
        assert_eq!(
            parse("Paused\t\tOnly Title"),
            Event::Track { status: Status::Paused, text: "Only Title".into() }
        );
        assert_eq!(parse(""), Event::Gone);
        assert_eq!(parse("Stopped\t\t"), Event::Gone);
    }

    #[test]
    fn output_is_valid_json_and_escaped() {
        let mut s = State::new();
        s.apply(Event::Track { status: Status::Playing, text: "Simon & \"Garfunkel\"".into() });
        let v: serde_json::Value = serde_json::from_str(&s.render()).unwrap();
        assert!(v["text"].as_str().unwrap().contains("&amp;"));
        assert_eq!(v["class"], "playing");
    }
}
