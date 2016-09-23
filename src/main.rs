use std::env::args;
use std::process::{Command, exit};
use std::fs::{OpenOptions, remove_file};
use std::io::{stdin, Read, Write};

fn error(message: &str) -> ! {
    write!(std::io::stderr(), "{}\n", message);
    exit(1);
}

fn get_tmp_file() -> String {
    let err = "couldn't get mktemp output";

    let path = Command::new("mktemp").output().unwrap_or_else(|_|error(err)).stdout;
    String::from_utf8(path).unwrap_or_else(|_|error(err))
}

fn main() {
    let cmdpath = args().skip(1).next().unwrap_or_else(|| {
                            let estr = &format!("usage: {} command [args...]\ncouldn't default to $PAGER",
                                                args().next().unwrap());
                            std::env::var("PAGER").unwrap_or_else(|e|error(estr))
                      });

    let args = args().skip(2);

    let tmpfilepath = get_tmp_file();
    let mut tmpfile = OpenOptions::new().write(true).create(true).open(&tmpfilepath)
        .unwrap_or_else(|e|error(&format!("couldn't open temp file: {} ({})", e, &tmpfilepath)));
    let mut buffer: Vec<u8> = Vec::with_capacity(8192); // Arbitrary 8KiB.

    stdin().read_to_end(&mut buffer);
    tmpfile.write_all(&buffer);

    let mut did_brace = false;
    let mut pagercmd = Command::new(cmdpath);
    for arg in args {
        did_brace = did_brace || arg.contains("{}");
        pagercmd.arg(arg.replace("{}", &tmpfilepath));
    }
    if !did_brace {
        pagercmd.arg(&tmpfilepath);
    }

    let c = pagercmd.status().map(|x|x.code())
            .unwrap_or_else(|e|error(&format!("couldn't open pagercmd: {}", e)))
            .unwrap_or_else(||error(&format!("couldn't open pagercmd")));

    // Sleep before deleting the file because sometimes it's important, e.g. my (viko) 's' script.
    std::thread::sleep(std::time::Duration::from_millis(10));
    if let Err(e) = remove_file(tmpfilepath) {
        error(&format!("couldn't delete file: {}", e));
    } else {
        exit(c);
    }
}
