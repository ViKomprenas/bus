use std::env::args;
use std::process::Command;

fn main() {
    let cmdpath = args().skip(1).next();
    if cmdpath.is_none() {
        println!("usage: {} <command>", args().next().unwrap());
        std::process::exit(1);
    }

    let args = args().skip(2);

    let mut cmd = Command::new(cmdpath.unwrap().as_str());
    for arg in args {
        cmd.arg(arg);
    }

    cmd.status();
}
