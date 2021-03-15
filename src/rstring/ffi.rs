use libc::{c_char, c_void};

// Opaque struct
#[repr(C)]
pub struct RString(c_void);

extern "C" {
    pub fn ex_rstring_get_type() -> glib::ffi::GType;

    pub fn ex_rstring_new(s: *const c_char) -> *mut RString;
    pub fn ex_rstring_copy(rstring: *const RString) -> *mut RString;
    pub fn ex_rstring_free(rstring: *mut RString);

    pub fn ex_rstring_get(rstring: *const RString) -> *mut c_char;
    pub fn ex_rstring_set(rstring: *mut RString, s: *const c_char);
}
