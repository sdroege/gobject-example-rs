use std::cell::Cell;
use std::sync::OnceLock;

use glib::prelude::*;
use glib::subclass::prelude::*;

use crate::foo::*;

// We could put our data into the Bar struct above but that's discouraged nowadays so let's just
// keep it all in Bar
//
// We use Cell/RefCell here for each field as GObject conceptually uses interior mutability everywhere.
// If this was to be used from multiple threads, these would have to be mutexes or otherwise
// Sync+Send
#[derive(Debug, Default)]
pub struct Bar {
    number: Cell<f64>,
}

#[glib::object_subclass]
impl ObjectSubclass for Bar {
    const NAME: &'static str = "ExBar";
    type ParentType = Foo;
    type Type = super::Bar;
}

impl ObjectImpl for Bar {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: OnceLock<Vec<glib::ParamSpec>> = OnceLock::new();
        PROPERTIES.get_or_init(|| {
            vec![glib::ParamSpecDouble::builder("number")
                .nick("Number")
                .blurb("Some number")
                .default_value(0.0)
                .maximum(100.0)
                .minimum(0.0)
                .build()]
        })
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "number" => {
                let number = value.get().unwrap();
                self.set_number(number);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "number" => self.number().to_value(),
            _ => unimplemented!(),
        }
    }
}

impl FooImpl for Bar {
    fn increment(&self, obj: &Foo, inc: i32) -> i32 {
        self.parent_increment(obj, 2 * inc)
    }
}

impl Bar {
    fn set_number(&self, num: f64) {
        self.number.set(num);
        self.obj().notify("number");
    }

    fn number(&self) -> f64 {
        self.number.get()
    }
}

pub(crate) mod ffi {
    use super::*;
    use glib::translate::*;
    use std::ffi::c_char;

    pub type ExBar = <super::Bar as super::ObjectSubclass>::Instance;

    /// # Safety
    ///
    /// Must be a BarInstance object.
    #[no_mangle]
    pub unsafe extern "C" fn ex_bar_get_number(this: *mut ExBar) -> f64 {
        let imp = (*this).imp();
        imp.number()
    }

    /// # Safety
    ///
    /// Must be a BarInstance object.
    #[no_mangle]
    pub unsafe extern "C" fn ex_bar_set_number(this: *mut ExBar, num: f64) {
        let imp = (*this).imp();
        imp.set_number(num);
    }

    // GObject glue
    /// # Safety
    ///
    /// Must be a valid C string, 0-terminated.
    #[no_mangle]
    pub unsafe extern "C" fn ex_bar_new(name: *const c_char) -> *mut ExBar {
        let obj = glib::Object::builder::<super::super::Bar>()
            .property("name", &*glib::GString::from_glib_borrow(name))
            .build();
        obj.to_glib_full()
    }

    #[no_mangle]
    pub extern "C" fn ex_bar_get_type() -> glib::ffi::GType {
        <super::super::Bar as StaticType>::static_type().into_glib()
    }
}
