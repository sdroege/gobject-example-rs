use libc::{c_char, c_void};

// Opaque struct
#[repr(C)]
pub struct ExSharedRString(c_void);

extern "C" {
    pub fn ex_shared_rstring_get_type() -> glib::ffi::GType;

    pub fn ex_shared_rstring_new(s: *const c_char) -> *mut ExSharedRString;
    pub fn ex_shared_rstring_ref(shared_rstring: *const ExSharedRString) -> *mut ExSharedRString;
    pub fn ex_shared_rstring_unref(shared_rstring: *mut ExSharedRString);

    pub fn ex_shared_rstring_get(shared_rstring: *const ExSharedRString) -> *mut c_char;
}
