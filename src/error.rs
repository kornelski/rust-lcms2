use std::error::Error as StdError;
use std::fmt;
use foreign_types::ForeignType;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    ObjectCreationError,
    MissingData,
    InvalidString,
}

impl Error {
    #[inline]
    pub(crate) unsafe fn if_null<T>(handle: *mut <T as ForeignType>::CType) -> LCMSResult<T> where T: ForeignType {
        if !handle.is_null() {
            Ok(T::from_ptr(handle))
        } else {
            Err(Error::ObjectCreationError)
        }
    }
}

/// This is a regular `Result` type with LCMS-specific `Error`
pub type LCMSResult<T> = Result<T, Error>;

impl fmt::Display for Error {
    #[cold]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match *self {
            Error::ObjectCreationError => "Could not create the object.\nThe reason is not known, but it's usually caused by wrong input parameters.",
            Error::InvalidString => "String is not valid. Contains unsupported characters or is too long.",
            Error::MissingData => "Requested data is empty or does not exist.",
        })
    }
}

impl StdError for Error {
}
