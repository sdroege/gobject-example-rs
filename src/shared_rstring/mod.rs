#[cfg(not(feature = "bindings"))]
pub mod imp;
#[cfg(not(feature = "bindings"))]
use imp::ffi;

/// cbindgen:ignore
#[cfg(feature = "bindings")]
mod ffi;

use glib::translate::*;

// We use a Boxed with copy/free since imp::ref() returns a new Box* to hold an
// Arc clone and handle refcounting.
//
// TODO: turn into a Shared and do the refcounting ourself.
glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SharedRString(Boxed<ffi::ExSharedRString>);

    match fn {
        copy => |ptr| ffi::ex_shared_rstring_ref(ptr),
        free => |ptr| ffi::ex_shared_rstring_unref(ptr),
        type_ => || ffi::ex_shared_rstring_get_type(),
    }
}

impl SharedRString {
    pub fn new(s: Option<&str>) -> SharedRString {
        unsafe { from_glib_full(ffi::ex_shared_rstring_new(s.to_glib_none().0)) }
    }

    // FIXME: This could borrow the &str in theory!
    pub fn get(&self) -> Option<String> {
        unsafe { from_glib_full(ffi::ex_shared_rstring_get(self.to_glib_none().0)) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let s = SharedRString::new(Some("bla"));
        assert_eq!(s.get(), Some("bla".into()));

        let s2 = s.clone();
        assert_eq!(s.get(), Some("bla".into()));
        assert_eq!(s2.get(), Some("bla".into()));
    }
}
