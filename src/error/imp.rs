#[derive(Clone, Copy, Debug, PartialEq, Eq, glib::ErrorDomain)]
#[error_domain(name = "ex-error")]
pub enum Error {
    InvalidArgument,
    Failed,
}

pub(crate) mod ffi {
    use glib::translate::*;

    pub type ExError = i32;

    pub const EX_ERROR_INVALID_ARGUMENT: ExError = super::Error::InvalidArgument as i32;
    pub const EX_ERROR_FAILED: ExError = super::Error::Failed as i32;

    #[no_mangle]
    pub unsafe extern "C" fn ex_error_quark() -> glib::ffi::GQuark {
        <super::Error as glib::error::ErrorDomain>::domain().into_glib()
    }
}
