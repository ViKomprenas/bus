use std::env::args;
use std::process::Command;

fn main() {
    let cmdpath = args().skip(1).next();
    let args = args().skip(2);

    let cmd = Command::new(cmdpath);
    for arg in args {
        cmd.arg(arg);
    }

    cmd.status();
}
