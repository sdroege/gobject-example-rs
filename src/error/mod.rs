#[cfg(not(feature = "bindings"))]
pub mod imp;
#[cfg(not(feature = "bindings"))]
use imp::ffi;

/// cbindgen:ignore
#[cfg(feature = "bindings")]
mod ffi;

use glib::translate::*;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum Error {
    InvalidArgument,
    Failed,
    __Unknown(i32),
}

impl IntoGlib for Error {
    type GlibType = ffi::ExError;

    fn into_glib(self) -> ffi::ExError {
        match self {
            Error::InvalidArgument => ffi::EX_ERROR_INVALID_ARGUMENT,
            Error::Failed => ffi::EX_ERROR_FAILED,
            Error::__Unknown(value) => value,
        }
    }
}

impl FromGlib<ffi::ExError> for Error {
    unsafe fn from_glib(value: ffi::ExError) -> Self {
        match value {
            0 => Error::InvalidArgument,
            1 => Error::Failed,
            value => Error::__Unknown(value),
        }
    }
}

impl glib::error::ErrorDomain for Error {
    fn domain() -> glib::Quark {
        unsafe { from_glib(ffi::ex_error_quark()) }
    }

    fn code(self) -> i32 {
        self.into_glib()
    }

    fn from(code: i32) -> Option<Self> {
        unsafe { Some(Self::from_glib(code)) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        let err = glib::Error::new(Error::Failed, "We are all mad here.");
        assert!(err.is::<Error>());
        assert!(matches!(err.kind::<Error>(), Some(Error::Failed)));
    }
}
