#[cfg(not(feature = "bindings"))]
pub mod imp;

#[cfg(feature = "bindings")]
mod ffi;
#[cfg(feature = "bindings")]
pub mod imp {
    pub use shared_rstring::ffi::*;
}

use glib::translate::*;

glib_wrapper! {
    pub struct SharedRString(Shared<imp::SharedRString>);

    match fn {
        ref => |ptr| imp::ex_shared_rstring_ref(ptr),
        unref => |ptr| imp::ex_shared_rstring_unref(ptr),
        get_type => || imp::ex_shared_rstring_get_type(),
    }
}

impl SharedRString {
    pub fn new(s: Option<&str>) -> SharedRString {
        unsafe { from_glib_full(imp::ex_shared_rstring_new(s.to_glib_none().0)) }
    }

    // FIXME: This could borrow the &str in theory!
    pub fn get(&self) -> Option<String> {
        unsafe { from_glib_full(imp::ex_shared_rstring_get(self.to_glib_none().0)) }
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
