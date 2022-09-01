use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ErrorKind {
    MissingId,
    IdNotFound,
    DuplicatedId,
    TypeChanged,
    IdNotTracked,
    CannotOverrideId,
    ParentNotFound,
    InvalidDataType,
    ParentNotRead,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

impl Error {
    pub fn new(kind: ErrorKind, message: String) -> Error {
        Error { kind, message }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
