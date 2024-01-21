// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    error::Error,
    fmt::{self, Display},
};

macro_rules! from_err {
    ($($err:path => $variant:ident),*) => {
        $(
            impl From<$err> for BlkidErr {
                fn from(v: $err) -> Self {
                    BlkidErr::$variant(v)
                }
            }
        )*
    }
}

from_err!(
    std::ffi::NulError => Null,
    std::ffi::FromBytesWithNulError => BytesWithNull,
    std::io::Error => IO,
    std::str::Utf8Error => UTF8,
    std::string::FromUtf8Error => FromUTF8,
    std::num::TryFromIntError => FromInt
);

/// Re-export of `Result` with an error type of `BlkidErr`
pub type Result<T> = std::result::Result<T, BlkidErr>;

/// Error representing all errors returned by binding methods
#[derive(Debug)]
pub enum BlkidErr {
    /// Wraps `std::ffi::NulError`
    Null(std::ffi::NulError),
    /// Wraps `std::ffi::FromBytesWithNulError`
    BytesWithNull(std::ffi::FromBytesWithNulError),
    /// Wraps `std::ffi::IntoStringError`
    IntoString(std::ffi::IntoStringError),
    /// A libblkid method returned a positive error code which means nothing
    PositiveReturnCode,
    /// A conversion failed
    InvalidConv,
    /// UTF8 error
    UTF8(std::str::Utf8Error),
    /// UTF8 error
    FromUTF8(std::string::FromUtf8Error),
    /// Int conversion error
    FromInt(std::num::TryFromIntError),
    /// IO error
    IO(std::io::Error),
    /// UUID error
    Uuid(uuid::Error),
    /// An unspecified error type and an error message providing more information
    Other(String),
    /// An error code was returned by libblkid
    LibErr,
}

impl Display for BlkidErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            BlkidErr::Null(ref e) => write!(f, "Null error during string conversion: {e}"),
            BlkidErr::BytesWithNull(ref e) => write!(f, "Null error when converting from slice: {e}"),
            BlkidErr::IntoString(ref e) => write!(f, "Could not convert C string to string: {e}"),
            BlkidErr::PositiveReturnCode => {
                write!(f, "Positive return code found when <= 0 was expected")
            }
            BlkidErr::InvalidConv => write!(f, "The requested conversion was unsuccessful"),
            BlkidErr::UTF8(ref e) => write!(f, "UTF8 error: {e}"),
            BlkidErr::FromUTF8(ref e) => write!(f, "UTF8 conversion error: {e}"),
            BlkidErr::FromInt(ref e) => write!(f, "Int conversion error: {e}"),
            BlkidErr::IO(ref e) => write!(f, "An IO error occurred: {e}"),
            BlkidErr::Uuid(ref e) => write!(f, "A UUID error occurred: {e}"),
            BlkidErr::Other(ref s) => write!(f, "{s}"),
            BlkidErr::LibErr => write!(f, "libblkid returned an error code indicating an operation could not be completed successfully"),
        }
    }
}

impl Error for BlkidErr {}
