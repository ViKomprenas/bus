use std::env::args;
use std::process::Command;
use std::fs::OpenOptions;
use std::io::{stdin, Read, Write};

fn main() {
    let cmdpath_ihatetheborrowchecker = args().skip(1).next().unwrap();
    let cmdpath = cmdpath_ihatetheborrowchecker.as_str();

    let args = args().skip(2);

    let mut pagercmd = Command::new(cmdpath);
    for arg in args {
        pagercmd.arg(arg);
    }

    let tmppath = String::from_utf8(Command::new("mktemp").output().unwrap().stdout).unwrap();
    let mut tmpfile = OpenOptions::new().write(true).open(tmppath).unwrap();
    let mut buffer: Vec<u8> = Vec::with_capacity(8192); // Arbitrary 8KiB.

    stdin().read_to_end(&mut buffer);
    tmpfile.write_all(&buffer);
}
