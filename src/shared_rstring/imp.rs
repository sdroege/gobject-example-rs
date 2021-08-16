use std::sync::Arc;

// No #[repr(C)] here as we export it as an opaque struct
// If it was not opaque, it must be #[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, glib::GBoxed)]
#[gboxed(type_name = "ExSharedRString")]
pub struct SharedRString(Arc<Option<String>>);

impl SharedRString {
    fn new(s: Option<String>) -> SharedRString {
        SharedRString(Arc::new(s))
    }

    // FIXME: This could borrow the &str in theory!
    fn get(&self) -> Option<String> {
        (*self.0).clone()
    }
}

pub(crate) mod ffi {
    use glib::translate::{from_glib_none, IntoGlib, ToGlibPtr};
    use libc::c_char;

    pub type ExSharedRString = super::SharedRString;

    /// # Safety
    ///
    /// Must be a valid C string, 0-terminated.
    #[no_mangle]
    pub unsafe extern "C" fn ex_shared_rstring_new(s: *const c_char) -> *mut ExSharedRString {
        let s = Box::new(super::SharedRString::new(from_glib_none(s)));
        Box::into_raw(s) as *mut _
    }

    /// # Safety
    ///
    /// Must be a valid SharedRString pointer.
    #[no_mangle]
    pub unsafe extern "C" fn ex_shared_rstring_ref(
        shared_rstring: *const ExSharedRString,
    ) -> *mut ExSharedRString {
        let shared_rstring = &*shared_rstring;
        let s = Box::new(shared_rstring.clone());

        Box::into_raw(s) as *mut _
    }

    /// # Safety
    ///
    /// Must be a valid SharedRString pointer.
    #[no_mangle]
    pub unsafe extern "C" fn ex_shared_rstring_unref(shared_rstring: *mut ExSharedRString) {
        let _ = Box::from_raw(shared_rstring);
    }

    /// # Safety
    ///
    /// Must be a valid SharedRString pointer.
    #[no_mangle]
    pub unsafe extern "C" fn ex_shared_rstring_get(
        shared_rstring: *const ExSharedRString,
    ) -> *mut c_char {
        let shared_rstring = &*shared_rstring;
        // FIXME: This could borrow the &str in theory!
        shared_rstring.get().to_glib_full()
    }

    // GObject glue
    #[no_mangle]
    pub extern "C" fn ex_shared_rstring_get_type() -> glib::ffi::GType {
        <super::SharedRString as glib::StaticType>::static_type().into_glib()
    }
}
