use glib_ffi;
use gobject_ffi;

use std::ffi::CString;
use std::sync::{Once, ONCE_INIT};
use std::mem;
use std::sync::Arc;

use glib::translate::{from_glib_none, ToGlibPtr};

use libc::{c_char, c_void};

// No #[repr(C)] here as we export it as an opaque struct
// If it was not opaque, it must be #[repr(C)]
pub struct SharedRString(Option<String>);

impl SharedRString {
    fn new(s: Option<String>) -> Arc<SharedRString> {
        Arc::new(SharedRString(s))
    }

    // FIXME: This could borrow the &str in theory!
    fn get(&self) -> Option<String> {
        self.0.clone()
    }
}

//
// Public C functions below
//
#[no_mangle]
pub unsafe extern "C" fn ex_shared_rstring_new(s: *const c_char) -> *mut SharedRString {
    callback_guard!();

    let s = SharedRString::new(from_glib_none(s));
    Arc::into_raw(s) as *mut _
}

#[no_mangle]
pub unsafe extern "C" fn ex_shared_rstring_ref(
    shared_rstring: *mut SharedRString,
) -> *mut SharedRString {
    callback_guard!();

    let shared_rstring = Arc::from_raw(shared_rstring);
    let s = shared_rstring.clone();

    // Forget it and keep it alive, we will still need it later
    let _ = Arc::into_raw(shared_rstring);

    Arc::into_raw(s) as *mut _
}

#[no_mangle]
pub unsafe extern "C" fn ex_shared_rstring_unref(shared_rstring: *mut SharedRString) {
    callback_guard!();

    let _ = Arc::from_raw(shared_rstring);
}

#[no_mangle]
pub unsafe extern "C" fn ex_shared_rstring_get(shared_rstring: *mut SharedRString) -> *mut c_char {
    callback_guard!();

    let shared_rstring = &*shared_rstring;
    // FIXME: This could borrow the &str in theory!
    shared_rstring.get().to_glib_full()
}

// GObject glue
#[no_mangle]
pub unsafe extern "C" fn ex_shared_rstring_get_type() -> glib_ffi::GType {
    callback_guard!();

    static mut TYPE: glib_ffi::GType = gobject_ffi::G_TYPE_INVALID;
    static ONCE: Once = ONCE_INIT;

    ONCE.call_once(|| {
        let type_name = CString::new("ExSharedRString").unwrap();

        TYPE = gobject_ffi::g_boxed_type_register_static(
            type_name.as_ptr(),
            Some(mem::transmute(ex_shared_rstring_ref as *const c_void)),
            Some(mem::transmute(ex_shared_rstring_ref as *const c_void)),
        );

    });

    TYPE
}
