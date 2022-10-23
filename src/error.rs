use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ErrorKind {
    MissingId,
    IdNotFound,
    DuplicatedId,
    DuplicatedName,
    TypeChanged,
    IdNotTracked,
    CannotOverrideId,
    ParentNotFound,
    InvalidDataType,
    ParentNotRead,
    LastCategoryNotSelectable,
    ParentNotAvailable,
    FailedToBorrowCategory,
    FailedToReadCategory,
    FailedToWriteCategory,
    FailedToTrackCategory,
    MissingCategoryIO,
    FailedToReadAttribute,
    FailedToWriteAttribute,
    FailedToBorrowAttribute,
    FailedToBorrowAttributeTracker,
    FailedToBorrowAttributeValidation,
    FailedDataTypeAttributeValidation,
    FailedDataTypeConstantAttributeValidation,
    FailedToReadConfig,
    ReservedKeywordUsedAsName,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

impl Error {
    pub fn new(kind: ErrorKind, message: &str) -> Error {
        Error {
            kind,
            message: message.to_string(),
        }
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

impl From<cooplan_definitions_lib::error::Error> for Error {
    fn from(error: cooplan_definitions_lib::error::Error) -> Self {
        let kind: ErrorKind = match error.kind() {
            cooplan_definitions_lib::error::ErrorKind::FailedToBorrowCategory => {
                ErrorKind::FailedToBorrowCategory
            }
            cooplan_definitions_lib::error::ErrorKind::MissingId => ErrorKind::MissingId,
            cooplan_definitions_lib::error::ErrorKind::ParentNotAvailable => {
                ErrorKind::ParentNotAvailable
            }
            cooplan_definitions_lib::error::ErrorKind::FailedToValidateCategory => {
                ErrorKind::InvalidDataType
            }
            cooplan_definitions_lib::error::ErrorKind::FailedToValidateSourceAttribute => {
                ErrorKind::InvalidDataType
            }
        };

        Error {
            kind,
            message: error.message.clone(),
        }
    }
}
