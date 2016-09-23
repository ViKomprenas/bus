use std;
use std::io::Write;
use std::process::exit;

#[derive(Debug)]
pub enum TmpFileError {
    IOErr(std::io::Error),
    MkTempFailed(Option<i32>),
    Utf8Err(std::string::FromUtf8Error),
}

impl std::error::Error for TmpFileError {
    fn description(&self) -> &str {
        use self::TmpFileError::*;
        match *self {
            IOErr(_) => "error creating temporary file: IO error",
            MkTempFailed(_) => "command \"mktemp\" failed",
            Utf8Err(_) => "error creating temporary file: input wasn't UTF8",
        }
    }

     fn cause(&self) -> Option<&std::error::Error> {
        use self::TmpFileError::*;
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
        use self::TmpFileError::*;
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

pub fn error(message: &str) -> ! {
    // There is probably no other way than to ignore the error (since we're exiting anyway).
    let _ = write!(std::io::stderr(), "{}\n", message);
    exit(1);
}
