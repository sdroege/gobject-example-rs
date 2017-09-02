use glib_ffi;
use gobject_ffi;
use libc::{c_char, c_void};

// Opaque struct
#[repr(C)]
pub struct SharedRString(c_void);

extern "C" {
    pub fn ex_shared_rstring_get_type() -> glib_ffi::GType;

    pub fn ex_shared_rstring_new(s: *const c_char) -> *const SharedRString;
    pub fn ex_shared_rstring_ref(shared_rstring: *mut SharedRString) -> *mut RString;
    pub fn ex_shared_rstring_unref(shared_rstring: *mut SharedRString);

    pub fn ex_shared_rstring_get(shared_rstring: *mut SharedRString) -> *mut c_char;
}
