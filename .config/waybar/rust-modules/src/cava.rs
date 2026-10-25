use std::io::{BufRead, BufReader};
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::common;

const BLOCKS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

const CAVA_CONFIG: &str = "\
[general]
bars = 8

[output]
method = raw
raw_target = /dev/stdout
data_format = ascii
ascii_max_range = 7
";

fn config_path() -> PathBuf {
    let dir = std::env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/tmp"));
    dir.join("waybar-mods-cava.conf")
}

/// "0;3;7;1;" -> "▁▄█▂"
fn render(raw: &str) -> String {
    raw.chars()
        .filter_map(|c| c.to_digit(10))
        .filter_map(|d| BLOCKS.get(d as usize).copied())
        .collect()
}

pub fn run() -> i32 {
    let path = config_path();
    if let Err(e) = std::fs::write(&path, CAVA_CONFIG) {
        eprintln!("cava: não consegui escrever {}: {e}", path.display());
        return 1;
    }

    let mut cmd = Command::new("cava");
    cmd.arg("-p")
        .arg(&path)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());

    unsafe {
        cmd.pre_exec(|| {
            libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGTERM as libc::c_ulong);
            Ok(())
        });
    }

    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("cava: não consegui iniciar o cava: {e}");
            return 1;
        }
    };
    let stdout = child.stdout.take().expect("stdout foi configurado como piped");

    let mut last = String::new();
    for line in BufReader::new(stdout).lines().map_while(Result::ok) {
        let frame = render(&line);
        if frame == last {
            continue;
        }
        if common::emit(&frame).is_err() {
            break; // waybar fechou o pipe
        }
        last = frame;
    }

    let _ = child.kill();
    let _ = child.wait();
    0
}

#[cfg(test)]
mod tests {
    use super::render;

    #[test]
    fn digits_become_blocks() {
        assert_eq!(render("0;3;7;1;"), "▁▄█▂");
        assert_eq!(render(""), "");
    }
}
