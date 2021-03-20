use crate::foo;
use libc::c_char;

#[repr(C)]
pub struct ExBar {
    pub parent: foo::ffi::ExFoo,
}

#[repr(C)]
pub struct ExBarClass {
    pub parent_class: foo::ffi::ExFooClass,
}

extern "C" {
    pub fn ex_bar_new(name: *const c_char) -> *mut ExBar;
    pub fn ex_bar_get_type() -> glib::ffi::GType;

    pub fn ex_bar_set_number(this: *mut ExBar, num: f64);
    pub fn ex_bar_get_number(this: *mut ExBar) -> f64;
}
