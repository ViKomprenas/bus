use std::env::args;
use std::process::Command;
use std::fs::OpenOptions;
use std::io::{stdin, Read, Write};

fn error(message: &str) -> ! {
    write!(std::io::stderr(), "{}\n", message);
    std::process::exit(1);
}

fn get_tmp_file() -> String {
    let err = "couldn't get mktemp output";

    let path = Command::new("mktemp").output().unwrap_or_else(|_|error(err)).stdout;
    String::from_utf8(path).unwrap_or_else(|_|error(err))
}

fn main() {
    let cmdpath = args().skip(1).next()
            .unwrap_or_else(||error("usage: bus <command> [args...]"));

    let args = args().skip(2);

    let mut pagercmd = Command::new(cmdpath);
    for arg in args {
        pagercmd.arg(arg);
    }

    let tmpfilepath = get_tmp_file();
    let mut tmpfile = OpenOptions::new().write(true).create(true).open(&tmpfilepath)
        .unwrap_or_else(|e|error(&format!("couldn't open temp file: {} ({})", e, &tmpfilepath)));
    let mut buffer: Vec<u8> = Vec::with_capacity(8192); // Arbitrary 8KiB.

    stdin().read_to_end(&mut buffer);
    tmpfile.write_all(&buffer);

    pagercmd.arg(tmpfilepath);
    pagercmd.status();
}
