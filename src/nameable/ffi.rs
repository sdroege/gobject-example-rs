use std::ffi::{c_char, c_void};

#[repr(C)]
pub struct ExNameable(c_void);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExNameableInterface {
    pub parent_iface: glib::gobject_ffi::GTypeInterface,
    pub get_name: Option<unsafe extern "C" fn(*mut ExNameable) -> *mut c_char>,
}

extern "C" {
    pub fn ex_nameable_get_type() -> glib::ffi::GType;

    pub fn ex_nameable_get_name(nameable: *mut ExNameable) -> *mut c_char;
}
