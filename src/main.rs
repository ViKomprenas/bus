use std::env::args;
use std::process::{Command, exit};
use std::fs::{OpenOptions, remove_file};
use std::io::{stdin, Read, Write};

#[derive(Debug)]
enum TmpFileError {
    IOErr(std::io::Error),
    MkTempFailed(Option<i32>),
    Utf8Err(std::string::FromUtf8Error),
}

impl std::error::Error for TmpFileError {
    fn description(&self) -> &str {
        use TmpFileError::*;
        match *self {
            IOErr(_) => "error creating temporary file: IO error",
            MkTempFailed(_) => "command \"mktemp\" failed",
            Utf8Err(_) => "error creating temporary file: input wasn't UTF8",
        }
    }

     fn cause(&self) -> Option<&std::error::Error> {
        use TmpFileError::*;
         match *self {
             IOErr(ref err) => Some(err),
             // TODO: replace with something meaningful
             MkTempFailed(_) => None,
             Utf8Err(ref err) => Some(err),
         }
     }
}

impl std::fmt::Display for TmpFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        use TmpFileError::*;
        match *self {
            IOErr(ref err) => write!(f, "error creating temporary file: {}", err),
            MkTempFailed(Some(err)) => write!(f, "command \"mktemp\" failed with exit code: {}", err),
            // TODO: conditional compilation should be used with ExitStatusExt on Unix
            MkTempFailed(None) => write!(f, "command \"mktemp\" was terminated by a signal"),
            Utf8Err(ref err) => write!(f, "error creating temporary file: {}", err),
        }
    }
}

impl From<std::io::Error> for TmpFileError {
    fn from(err: std::io::Error) -> Self {
        TmpFileError::IOErr(err)
    }
}

impl From<std::string::FromUtf8Error> for TmpFileError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        TmpFileError::Utf8Err(err)
    }
}

fn error(message: &str) -> ! {
    // There is probably no other way than to ignore the error (since we're exiting anyway).
    let _ = write!(std::io::stderr(), "{}\n", message);
    exit(1);
}

fn get_tmp_file() -> Result<String, TmpFileError> {
    let cmd = try!(Command::new("mktemp").output());
    if cmd.status.success() {
        Ok(try!(String::from_utf8(cmd.stdout)))
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

    // Sleep before deleting the file because sometimes it's important, e.g. my (viko) 's' script.
    std::thread::sleep(std::time::Duration::from_millis(10));
    if let Err(e) = remove_file(tmpfilepath) {
        error(&format!("couldn't delete file: {}", e));
    } else {
        exit(c);
    }
}
