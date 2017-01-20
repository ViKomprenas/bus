use std::env::args;
use std::process::{Command, exit};
use std::fs::OpenOptions;
use std::io::{stdin, Read, Write};

mod err;
use self::err::{TmpFileError, error};

fn get_tmp_file() -> Result<String, TmpFileError> {
    let cmd = try!(Command::new("mktemp").output());
    if cmd.status.success() {
        Ok(try!(String::from_utf8(cmd.stdout)).replace("\n", ""))
    } else {
        Err(TmpFileError::MkTempFailed(cmd.status.code()))
    }
}

fn pipe<R: Read, W: Write>(reader: &mut R, writer: &mut W) -> std::io::Result<()> {
    // This is safe because the memory will always be written to before it is read. (Assuming
    // correct implementations of reader and writer.)
    unsafe {
        let mut buffer: [u8; 8192] = std::mem::uninitialized();
        // I'd swear there is a nicer way of doing this...
        while let Some(len) = try!(reader.read(&mut buffer).map(|n| if n > 0 { Some(n) } else { None })) {
            try!(writer.write_all(&mut buffer[0..len]));
        }
    }
    Ok(())
}

fn main() {
    let cmdpath = args().skip(1).next().unwrap_or_else(|| {
                            let estr = &format!("usage: {} command [args...]\ncouldn't default to $PAGER",
                                                args().next().unwrap_or("bus".to_string()));
                            std::env::var("PAGER").unwrap_or_else(|_|error(estr))
                      });

    let args = args().skip(2);

    let tmpfilepath = get_tmp_file().unwrap_or_else(|e|error(&format!("{}", e)));
    let mut tmpfile = OpenOptions::new().write(true).create(true).open(&tmpfilepath)
        .unwrap_or_else(|e|error(&format!("couldn't open temp file: {} ({})", e, &tmpfilepath)));

    pipe(&mut stdin(), &mut tmpfile).unwrap_or_else(|e|error(&format!("{}", e)));

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
    exit(c);
}
