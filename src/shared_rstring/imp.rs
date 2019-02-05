use glib_ffi;

use std::sync::Arc;

use glib::subclass::prelude::*;
use glib::translate::{from_glib_none, ToGlib, ToGlibPtr};

use libc::c_char;

// No #[repr(C)] here as we export it as an opaque struct
// If it was not opaque, it must be #[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SharedRString(Arc<Option<String>>);

impl BoxedType for SharedRString {
    // This type name must be unique per process.
    const NAME: &'static str = "ExSharedRString";

    // This macro defines a
    //   fn get_type() -> glib::Type
    // function
    glib_boxed_type!();
}

// This macro derives some traits on the struct
glib_boxed_derive_traits!(SharedRString);

impl SharedRString {
    fn new(s: Option<String>) -> SharedRString {
        SharedRString(Arc::new(s))
    }

    // FIXME: This could borrow the &str in theory!
    fn get(&self) -> Option<String> {
        (*self.0).clone()
    }
}

//
// Public C functions below
//
#[no_mangle]
pub unsafe extern "C" fn ex_shared_rstring_new(s: *const c_char) -> *mut SharedRString {
    let s = Box::new(SharedRString::new(from_glib_none(s)));
    Box::into_raw(s) as *mut _
}

#[no_mangle]
pub unsafe extern "C" fn ex_shared_rstring_ref(
    shared_rstring: *mut SharedRString,
) -> *mut SharedRString {
    let shared_rstring = &*shared_rstring;
    let s = Box::new(shared_rstring.clone());

    Box::into_raw(s) as *mut _
}

#[no_mangle]
pub unsafe extern "C" fn ex_shared_rstring_unref(shared_rstring: *mut SharedRString) {
    let _ = Box::from_raw(shared_rstring);
}

#[no_mangle]
pub unsafe extern "C" fn ex_shared_rstring_get(shared_rstring: *mut SharedRString) -> *mut c_char {
    let shared_rstring = &*shared_rstring;
    // FIXME: This could borrow the &str in theory!
    shared_rstring.get().to_glib_full()
}

// GObject glue
#[no_mangle]
pub unsafe extern "C" fn ex_shared_rstring_get_type() -> glib_ffi::GType {
    SharedRString::get_type().to_glib()
}
