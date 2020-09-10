use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ChartsLibError {
    kind: ErrorKind,
    description: String,
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    IncompatibleOrientationAndDataAxis,
    DatasetToAddIsNotTheSameLengthAsExistingDatasets,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ChartsLibError {
    pub(crate) fn new(kind: ErrorKind, description: String) -> ChartsLibError {
        ChartsLibError { kind, description }
    }
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl fmt::Display for ChartsLibError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.description)
    }
}

impl Error for ChartsLibError {}
