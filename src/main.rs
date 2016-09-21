use std::env::args;
use std::process::Command;
use std::fs::OpenOptions;
use std::io::{stdin, Read, Write};

fn error(message: str) -> Box<Fn() -> !> {
    Box::new(move || {
        println!("{}", message);
        std::process::exit(1);
    })
}

fn get_tmp_file() -> String {
    let err = error("couldn't get mktemp output");

    let path = Command::new("mktemp").output().unwrap_or_else(err).stdout;
    String::from_utf8(path).unwrap_or_else(err)
}

fn main() {
    let cmdpath = args().skip(1).next().unwrap_or_else(error(
            "usage: bus <command> [args...]"));

    let args = args().skip(2);

    let mut pagercmd = Command::new(cmdpath);
    for arg in args {
        pagercmd.arg(arg);
    }

    let mut tmpfile = OpenOptions::new().write(true).open(get_tmp_file())
        .unwrap_or_else(error("couldn't open temp file"));
    let mut buffer: Vec<u8> = Vec::with_capacity(8192); // Arbitrary 8KiB.

    stdin().read_to_end(&mut buffer);
    tmpfile.write_all(&buffer);
}
