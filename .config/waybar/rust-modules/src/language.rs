use std::io::{BufRead, BufReader};
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

use serde_json::Value;

use crate::common;

fn short_name(layout: &str) -> &'static str {
    let l = layout.to_lowercase();
    if l.contains("brazil") || l.contains("portuguese") {
        "BR"
    } else {
        "US"
    }
}

fn layout_of(input: &Value, filter: &str) -> Option<String> {
    if input["type"] != "keyboard" {
        return None;
    }
    let id = input["identifier"].as_str()?.to_lowercase();
    if !id.contains(filter) {
        return None;
    }
    input["xkb_active_layout_name"].as_str().map(str::to_owned)
}

fn current_layout(filter: &str) -> Option<String> {
    let out = Command::new("swaymsg").args(["-t", "get_inputs", "-r"]).output().ok()?;
    let inputs: Value = serde_json::from_slice(&out.stdout).ok()?;
    inputs.as_array()?.iter().find_map(|i| layout_of(i, filter))
}

fn layout_from_event(line: &str, filter: &str) -> Option<String> {
    let event: Value = serde_json::from_str(line).ok()?;
    if event["change"] == "removed" {
        return None;
    }
    layout_of(&event["input"], filter)
}

pub fn run(filter: &str) -> i32 {
    let filter = filter.to_lowercase();

    let mut cmd = Command::new("swaymsg");
    cmd.args(["-t", "subscribe", "-m", "-r", "[\"input\"]"])
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
            eprintln!("language: não consegui iniciar o swaymsg: {e}");
            return 1;
        }
    };
    let stdout = child.stdout.take().expect("stdout foi configurado como piped");

    let mut last = String::new();
    let mut publish = |layout: &str| -> bool {
        let short = short_name(layout);
        if short == last {
            return true;
        }
        last = short.to_string();
        common::emit(short).is_ok()
    };

    let mut waybar_gone = false;
    match current_layout(&filter) {
        Some(layout) => waybar_gone = !publish(&layout),
        None => eprintln!("language: nenhum teclado casa com o filtro {filter:?}"),
    }

    if !waybar_gone {
        for line in BufReader::new(stdout).lines().map_while(Result::ok) {
            if let Some(layout) = layout_from_event(&line, &filter) {
                if !publish(&layout) {
                    waybar_gone = true;
                    break;
                }
            }
        }
    }

    let _ = child.kill();
    let _ = child.wait();
    if waybar_gone { 0 } else { 1 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn layout_names_map_to_short_codes() {
        assert_eq!(short_name("Portuguese (Brazil)"), "BR");
        assert_eq!(short_name("Portuguese"), "BR");
        assert_eq!(short_name("English (US)"), "US");
    }

    #[test]
    fn only_the_filtered_keyboard_counts() {
        let kb = json!({"type": "keyboard", "identifier": "1234:5678:Hangsheng_Keyboard",
                        "xkb_active_layout_name": "Portuguese (Brazil)"});
        let power = json!({"type": "keyboard", "identifier": "0:1:Power_Button",
                           "xkb_active_layout_name": "English (US)"});
        let mouse = json!({"type": "pointer", "identifier": "hangsheng_mouse"});
        assert_eq!(layout_of(&kb, "hangsheng").as_deref(), Some("Portuguese (Brazil)"));
        assert_eq!(layout_of(&power, "hangsheng"), None);
        assert_eq!(layout_of(&mouse, "hangsheng"), None);
        assert!(layout_of(&power, "").is_some()); // sem filtro, qualquer teclado
    }

    #[test]
    fn parses_events() {
        let ev = r#"{"change":"xkb_layout","input":{"type":"keyboard","identifier":"a:b:Hangsheng","xkb_active_layout_name":"Portuguese (Brazil)"}}"#;
        assert_eq!(layout_from_event(ev, "hangsheng").as_deref(), Some("Portuguese (Brazil)"));
        assert_eq!(layout_from_event(r#"{"success": true}"#, "hangsheng"), None);
        assert_eq!(layout_from_event("lixo", "hangsheng"), None);
    }
}
