use glib_ffi;

use glib::subclass::prelude::*;
use glib::translate::{from_glib_none, ToGlib, ToGlibPtr};

use libc::c_char;

// No #[repr(C)] here as we export it as an opaque struct
// If it was not opaque, it must be #[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RString(Option<String>);

impl BoxedType for RString {
    // This type name must be unique per process.
    const NAME: &'static str = "ExRString";

    // This macro defines a
    //   fn get_type() -> glib::Type
    // function
    glib_boxed_type!();
}

// This macro derives some traits on the struct
glib_boxed_derive_traits!(RString);

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
#[no_mangle]
pub unsafe extern "C" fn ex_rstring_new(s: *const c_char) -> *mut RString {
    let s = Box::new(RString::new(from_glib_none(s)));
    Box::into_raw(s)
}

#[no_mangle]
pub unsafe extern "C" fn ex_rstring_copy(rstring: *const RString) -> *mut RString {
    let rstring = &*rstring;
    let s = Box::new(rstring.clone());
    Box::into_raw(s)
}

#[no_mangle]
pub unsafe extern "C" fn ex_rstring_free(rstring: *mut RString) {
    let _ = Box::from_raw(rstring);
}

#[no_mangle]
pub unsafe extern "C" fn ex_rstring_get(rstring: *const RString) -> *mut c_char {
    let rstring = &*rstring;
    rstring.get().to_glib_full()
}

#[no_mangle]
pub unsafe extern "C" fn ex_rstring_set(rstring: *mut RString, s: *const c_char) {
    let rstring = &mut *rstring;
    rstring.set(from_glib_none(s));
}

// GObject glue
#[no_mangle]
pub unsafe extern "C" fn ex_rstring_get_type() -> glib_ffi::GType {
    RString::get_type().to_glib()
}
