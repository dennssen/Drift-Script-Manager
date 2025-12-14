use std::io::{Error, ErrorKind};
use opener::OpenError;
use serde_json::error::Category;
use zip::result::ZipError;

pub fn zip_error_to_io(zip_error: &ZipError) -> Error {
    match zip_error {
        ZipError::Io(e) => {
            Error::new(e.kind(), format!("{}", e))
        }
        ZipError::InvalidArchive(_) => {
            Error::new(ErrorKind::Other, "Invalid Zip Archive")
        }
        ZipError::UnsupportedArchive(_) => {
            Error::new(ErrorKind::Other, "Unsupported Zip Archive")
        }
        ZipError::FileNotFound => {
            Error::new(ErrorKind::Other, "Specified file not found in archive")
        }
        ZipError::InvalidPassword => {
            Error::new(ErrorKind::Other, "Provided password is incorrect")
        }
        _ => {
            Error::new(ErrorKind::Other, "Panic!")
        }
    }
}

pub fn open_error_to_io(open_error: &OpenError) -> Error {
    match open_error {
        OpenError::Io(e) => {
            Error::new(e.kind(), format!("{}", e))
        }
        OpenError::Spawn {cmds, source } => {
            Error::new(source.kind(), format!("cmds: {}, Err: {}", cmds, source))
        }
        OpenError::ExitStatus {cmd, status, stderr} => {
            Error::new(ErrorKind::Other, format!("cmd: {}, status: {}, stderr: {}", cmd, status, stderr))
        }
        _ => {
            Error::new(ErrorKind::Other, "panic!")
        }
    }
}

pub fn json_error_to_io(json_error: &serde_json::Error) -> Error {
    match json_error.classify() {
        Category::Io => {
            Error::new(json_error.io_error_kind().unwrap(), "Failure to read or write bytes on an I/O stream")
        }
        Category::Syntax => {
            Error::new(ErrorKind::Other, "File data is syntactically invalid JSON")
        }
        Category::Data => {
            Error::new(ErrorKind::Other, "File data is semantically incorrect")
        }
        Category::Eof => {
            Error::new(ErrorKind::Other, "Unexpected end of the input data")
        }
    }
}