use std::ffi::c_int;

pub type ExError = c_int;

pub const EX_ERROR_INVALID_ARGUMENT: ExError = 0;
pub const EX_ERROR_FAILED: ExError = 1;

extern "C" {
    pub fn ex_error_quark() -> glib::ffi::GQuark;
}
