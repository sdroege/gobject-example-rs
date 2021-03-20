use libc::{c_char, c_void};

// Opaque struct
#[repr(C)]
pub struct ExRString(c_void);

extern "C" {
    pub fn ex_rstring_get_type() -> glib::ffi::GType;

    pub fn ex_rstring_new(s: *const c_char) -> *mut ExRString;
    pub fn ex_rstring_copy(rstring: *const ExRString) -> *mut ExRString;
    pub fn ex_rstring_free(rstring: *mut ExRString);

    pub fn ex_rstring_get(rstring: *const ExRString) -> *mut c_char;
    pub fn ex_rstring_set(rstring: *mut ExRString, s: *const c_char);
}
