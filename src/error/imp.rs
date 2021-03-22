#[derive(Clone, Copy, Debug, PartialEq, Eq, glib::GErrorDomain)]
#[gerror_domain(name = "ExError")]
pub enum ErrorDomain {
    InvalidArgument,
    Failed,
}

pub(crate) mod ffi {
    use glib::translate::*;

    pub type ExError = i32;

    pub const EX_ERROR_INVALID_ARGUMENT: ExError = super::ErrorDomain::InvalidArgument as i32;
    pub const EX_ERROR_FAILED: ExError = super::ErrorDomain::Failed as i32;

    #[no_mangle]
    pub unsafe extern "C" fn ex_error_quark() -> glib::ffi::GQuark {
        <super::ErrorDomain as glib::error::ErrorDomain>::domain().to_glib()
    }
}
