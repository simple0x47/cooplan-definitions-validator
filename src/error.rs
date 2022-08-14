#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ErrorKind {
    MissingId,
    IdNotFound,
    DuplicatedId,
    TypeChanged,
    IdNotTracked,
    CannotOverrideId,
    ParentNotFound,
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
