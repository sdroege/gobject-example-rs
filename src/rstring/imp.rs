// No #[repr(C)] here as we export it as an opaque struct
// If it was not opaque, it must be #[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, glib::Boxed)]
#[boxed_type(name = "ExRString")]
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

pub(crate) mod ffi {
    use glib::translate::{from_glib_none, IntoGlib, ToGlibPtr};
    use libc::c_char;

    pub type ExRString = super::RString;

    /// # Safety
    ///
    /// Must be a valid C string, 0-terminated.
    #[no_mangle]
    pub unsafe extern "C" fn ex_rstring_new(s: *const c_char) -> *mut ExRString {
        let s = Box::new(super::RString::new(from_glib_none(s)));
        Box::into_raw(s)
    }

    /// # Safety
    ///
    /// Must be a valid RString pointer.
    #[no_mangle]
    pub unsafe extern "C" fn ex_rstring_copy(rstring: *const ExRString) -> *mut ExRString {
        let rstring = &*rstring;
        let s = Box::new(rstring.clone());
        Box::into_raw(s)
    }

    /// # Safety
    ///
    /// Must be a valid RString pointer.
    #[no_mangle]
    pub unsafe extern "C" fn ex_rstring_free(rstring: *mut ExRString) {
        let _ = Box::from_raw(rstring);
    }

    /// # Safety
    ///
    /// Must be a valid RString pointer.
    #[no_mangle]
    pub unsafe extern "C" fn ex_rstring_get(rstring: *const ExRString) -> *mut c_char {
        let rstring = &*rstring;
        rstring.get().to_glib_full()
    }

    /// # Safety
    ///
    /// Must be a valid RString pointer, and a valid C string, 0-terminated.
    #[no_mangle]
    pub unsafe extern "C" fn ex_rstring_set(rstring: *mut ExRString, s: *const c_char) {
        let rstring = &mut *rstring;
        rstring.set(from_glib_none(s));
    }

    // GObject glue
    #[no_mangle]
    pub extern "C" fn ex_rstring_get_type() -> glib::ffi::GType {
        <super::RString as glib::StaticType>::static_type().into_glib()
    }
}
