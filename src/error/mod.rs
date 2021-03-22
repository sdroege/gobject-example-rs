#[cfg(not(feature = "bindings"))]
pub mod imp;
#[cfg(not(feature = "bindings"))]
use imp::ffi;

#[cfg(feature = "bindings")]
mod ffi;

use glib::translate::*;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum ErrorDomain {
    InvalidArgument,
    Failed,
    __Unknown(i32),
}

impl ToGlib for ErrorDomain {
    type GlibType = ffi::ExError;

    fn to_glib(&self) -> ffi::ExError {
        match *self {
            ErrorDomain::InvalidArgument => ffi::EX_ERROR_INVALID_ARGUMENT,
            ErrorDomain::Failed => ffi::EX_ERROR_FAILED,
            ErrorDomain::__Unknown(value) => value,
        }
    }
}

impl FromGlib<ffi::ExError> for ErrorDomain {
    unsafe fn from_glib(value: ffi::ExError) -> Self {
        match value {
            0 => ErrorDomain::InvalidArgument,
            1 => ErrorDomain::Failed,
            value => ErrorDomain::__Unknown(value),
        }
    }
}

impl glib::error::ErrorDomain for ErrorDomain {
    fn domain() -> glib::Quark {
        unsafe { from_glib(ffi::ex_error_quark()) }
    }

    fn code(self) -> i32 {
        self.to_glib()
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
        let err = glib::Error::new(ErrorDomain::Failed, "We are all mad here.");
        assert!(err.is::<ErrorDomain>());
        assert!(matches!(err.kind::<ErrorDomain>(), Some(ErrorDomain::Failed)));
    }
}
