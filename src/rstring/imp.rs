use glib_ffi;
use gobject_ffi;

use std::ffi::CString;
use std::sync::{Once, ONCE_INIT};
use std::mem;

use glib::translate::{from_glib_none, ToGlibPtr};

use libc::{c_char, c_void};

// No #[repr(C)] here as we export it as an opaque struct
// If it was not opaque, it must be #[repr(C)]
#[derive(Clone)]
pub struct RString(Option<String>);

impl RString {
    fn new(s: Option<String>) -> RString {
        RString(s)
    }

    fn get(&self) -> Option<String> {
        self.0.as_ref().map(|s| s.clone())
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
    callback_guard!();

    let s = Box::new(RString::new(from_glib_none(s)));
    Box::into_raw(s)
}

#[no_mangle]
pub unsafe extern "C" fn ex_rstring_copy(rstring: *const RString) -> *mut RString {
    callback_guard!();

    let rstring = &*rstring;
    let s = Box::new(rstring.clone());
    Box::into_raw(s)
}

#[no_mangle]
pub unsafe extern "C" fn ex_rstring_free(rstring: *mut RString) {
    callback_guard!();

    let _ = Box::from_raw(rstring);
}

#[no_mangle]
pub unsafe extern "C" fn ex_rstring_get(rstring: *const RString) -> *mut c_char {
    callback_guard!();

    let rstring = &*rstring;
    rstring.get().to_glib_full()
}

#[no_mangle]
pub unsafe extern "C" fn ex_rstring_set(rstring: *mut RString, s: *const c_char) {
    callback_guard!();

    let rstring = &mut *rstring;
    rstring.set(from_glib_none(s));
}

// GObject glue
#[no_mangle]
pub unsafe extern "C" fn ex_rstring_get_type() -> glib_ffi::GType {
    callback_guard!();

    static mut TYPE: glib_ffi::GType = gobject_ffi::G_TYPE_INVALID;
    static ONCE: Once = ONCE_INIT;

    ONCE.call_once(|| {
        let type_name = CString::new("ExRString").unwrap();

        TYPE = gobject_ffi::g_boxed_type_register_static(
            type_name.as_ptr(),
            Some(mem::transmute(ex_rstring_copy as *const c_void)),
            Some(mem::transmute(ex_rstring_free as *const c_void)),
        );

    });

    TYPE
}
