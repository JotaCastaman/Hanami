use std::io::{self, Write};

/pub fn die_with_parent() {
    // SAFETY: 
    unsafe {
        let parent = libc::getppid();
        libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGTERM as libc::c_ulong);
        // Se o pai morreu entre as duas linhas acima, ninguém vai nos avisar.
        if libc::getppid() != parent {
            std::process::exit(0);
        }
    }
}

pub fn emit(line: &str) -> io::Result<()> {
    let mut out = io::stdout().lock();
    writeln!(out, "{line}")?;
    out.flush()
}

pub fn escape_markup(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}
