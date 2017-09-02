use glib_ffi;
use gobject_ffi;
use libc::{c_char, c_void};

#[repr(C)]
pub struct Nameable(c_void);

#[repr(C)]
pub struct NameableInterface {
    pub parent_iface: gobject_ffi::GTypeInterface,
    pub get_name: Option<unsafe extern "C" fn(*mut Nameable) -> *mut c_char>,
}

extern "C" {
    pub fn ex_nameable_get_type() -> glib_ffi::GType;

    pub fn ex_nameable_get_name(nameable: *mut Nameable) -> *mut c_char;
}
