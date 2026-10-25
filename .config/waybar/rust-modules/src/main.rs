mod cava;
mod common;
mod language;
mod mpris;

fn main() {
        common::die_with_parent();

    let mut args = std::env::args().skip(1);
    let code = match args.next().as_deref() {
        Some("cava") => cava::run(),
        Some("language") => language::run(&args.next().unwrap_or_default()),
        Some("mpris") => mpris::run(&args.next().unwrap_or_else(|| "spotify".into())),
        _ => {
            eprintln!("uso: waybar-mods <cava | mpris [player] | language [filtro-do-teclado]>");
            2
        }
    };
    std::process::exit(code);
}
