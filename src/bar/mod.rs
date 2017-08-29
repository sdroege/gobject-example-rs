#[cfg(not(feature = "bindings"))]
pub mod imp;

#[cfg(feature = "bindings")]
mod ffi;
#[cfg(feature = "bindings")]
pub mod imp {
    pub use bar::ffi::*;
}

use glib_ffi;
use gobject_ffi;

use foo;

use glib::translate::*;

use std::ptr;
use std::mem;

glib_wrapper! {
    pub struct Bar(Object<imp::Bar>): foo::Foo;

    match fn {
        get_type => || imp::ex_bar_get_type(),
    }
}

impl Bar {
    pub fn new(name: Option<&str>) -> Bar {
        unsafe { from_glib_full(imp::ex_bar_new(name.to_glib_none().0)) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use foo::FooExt;

    #[test]
    fn test_new() {
        let bar = Bar::new(Some("bar's name"));

        drop(bar);
    }

    #[test]
    fn test_counter() {
        let bar = Bar::new(Some("bar's name"));

        assert_eq!(bar.get_counter(), 0);
        assert_eq!(bar.increment(1), 2);
        assert_eq!(bar.get_counter(), 2);
        assert_eq!(bar.increment(10), 22);
        assert_eq!(bar.get_counter(), 22);
    }

    #[test]
    fn test_name() {
        let bar = Bar::new(Some("bar's name"));

        assert_eq!(bar.get_name(), Some("bar's name".into()));
        assert_eq!(bar.get_property_name(), Some("bar's name".into()));
    }
}
