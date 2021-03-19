#[cfg(not(feature = "bindings"))]
pub mod imp;

/// cbindgen:ignore
#[cfg(feature = "bindings")]
mod ffi;
#[cfg(feature = "bindings")]
pub mod imp {
    pub use super::ffi::*;
}

use glib::translate::*;

glib::wrapper! {
    pub struct RString(Boxed<imp::RString>);

    match fn {
        copy => |ptr| imp::ex_rstring_copy(ptr),
        free => |ptr| imp::ex_rstring_free(ptr),
        get_type => || imp::ex_rstring_get_type(),
    }
}

impl RString {
    pub fn new(s: Option<&str>) -> RString {
        unsafe { from_glib_full(imp::ex_rstring_new(s.to_glib_none().0)) }
    }

    pub fn get(&self) -> Option<String> {
        unsafe { from_glib_full(imp::ex_rstring_get(self.to_glib_none().0)) }
    }

    pub fn set(&mut self, s: Option<&str>) {
        unsafe { imp::ex_rstring_set(self.to_glib_none_mut().0, s.to_glib_none().0) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let s = RString::new(Some("bla"));
        assert_eq!(s.get(), Some("bla".into()));

        let mut s2 = s.clone();
        s2.set(Some("blabla"));
        assert_eq!(s.get(), Some("bla".into()));
        assert_eq!(s2.get(), Some("blabla".into()));
    }
}
