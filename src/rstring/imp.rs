use glib::subclass::prelude::*;
use glib::translate::{from_glib_none, ToGlib, ToGlibPtr};

use libc::c_char;

// No #[repr(C)] here as we export it as an opaque struct
// If it was not opaque, it must be #[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, glib::GBoxed)]
#[gboxed(type_name = "ExRString")]
pub struct RString(Option<String>);

impl RString {
    fn new(s: Option<String>) -> RString {
        RString(s)
    }

    fn get(&self) -> Option<String> {
        self.0.clone()
    }

    fn set(&mut self, s: Option<String>) {
        self.0 = s;
    }
}

//
// Public C functions below
//
/// # Safety
///
/// Must be a valid C string, 0-terminated.
#[no_mangle]
pub unsafe extern "C" fn ex_rstring_new(s: *const c_char) -> *mut RString {
    let s = Box::new(RString::new(from_glib_none(s)));
    Box::into_raw(s)
}

/// # Safety
///
/// Must be a valid RString pointer.
#[no_mangle]
pub unsafe extern "C" fn ex_rstring_copy(rstring: *const RString) -> *mut RString {
    let rstring = &*rstring;
    let s = Box::new(rstring.clone());
    Box::into_raw(s)
}

/// # Safety
///
/// Must be a valid RString pointer.
#[no_mangle]
pub unsafe extern "C" fn ex_rstring_free(rstring: *mut RString) {
    let _ = Box::from_raw(rstring);
}

/// # Safety
///
/// Must be a valid RString pointer.
#[no_mangle]
pub unsafe extern "C" fn ex_rstring_get(rstring: *const RString) -> *mut c_char {
    let rstring = &*rstring;
    rstring.get().to_glib_full()
}

/// # Safety
///
/// Must be a valid RString pointer, and a valid C string, 0-terminated.
#[no_mangle]
pub unsafe extern "C" fn ex_rstring_set(rstring: *mut RString, s: *const c_char) {
    let rstring = &mut *rstring;
    rstring.set(from_glib_none(s));
}

// GObject glue
#[no_mangle]
pub extern "C" fn ex_rstring_get_type() -> glib::ffi::GType {
    RString::get_type().to_glib()
}
