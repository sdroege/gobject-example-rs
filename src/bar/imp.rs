use glib_ffi;

use std::cell::RefCell;

use glib;
use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

use libc::c_char;

use bar::Bar as BarWrapper;
use foo;

pub mod ffi {
    pub type Bar = <super::Bar as super::ObjectSubclass>::Instance;
}

// We could put our data into the Bar struct above but that's discouraged nowadays so let's just
// keep it all in Bar
//
// We use RefCells here for each field as GObject conceptually uses interior mutability everywhere.
// If this was to be used from multiple threads, these would have to be mutexes or otherwise
// Sync+Send
pub struct Bar {
    number: RefCell<f64>,
}

#[glib::object_subclass]
impl ObjectSubclass for Bar {
    const NAME: &'static str = "ExBar";
    type ParentType = foo::Foo;
    type Type = BarWrapper;

    fn new() -> Self {
        Self {
            number: RefCell::new(0.0),
        }
    }
}

impl ObjectImpl for Bar {
    fn properties() -> &'static [glib::ParamSpec] {
        use once_cell::sync::Lazy;
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
            vec![glib::ParamSpec::double(
                "number",
                "Number",
                "Some number",
                0.0,
                100.0,
                0.0,
                glib::ParamFlags::READWRITE,
            )]
        });

        PROPERTIES.as_ref()
    }

    fn set_property(
        &self,
        obj: &Self::Type,
        _id: usize,
        value: &glib::Value,
        pspec: &glib::ParamSpec,
    ) {
        match pspec.get_name() {
            "number" => {
                let number = value.get().unwrap().unwrap();
                self.set_number(obj.downcast_ref().unwrap(), number);
            }
            _ => unimplemented!(),
        }
    }

    fn get_property(&self, obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.get_name() {
            "number" => self.get_number(obj.downcast_ref().unwrap()).to_value(),
            _ => unimplemented!(),
        }
    }
}

impl foo::FooImpl for Bar {
    fn increment(&self, obj: &foo::Foo, inc: i32) -> i32 {
        self.parent_increment(obj, 2 * inc)
    }
}

impl Bar {
    fn set_number(&self, this: &BarWrapper, num: f64) {
        *self.number.borrow_mut() = num;
        this.notify("number");
    }

    fn get_number(&self, _this: &BarWrapper) -> f64 {
        *self.number.borrow_mut()
    }
}

//
// Public C functions below
//

// Trampolines to safe Rust implementations
#[no_mangle]
pub unsafe extern "C" fn ex_bar_get_number(this: *mut ffi::Bar) -> f64 {
    let imp = (*this).get_impl();
    imp.get_number(&from_glib_borrow(this))
}

#[no_mangle]
pub unsafe extern "C" fn ex_bar_set_number(this: *mut ffi::Bar, num: f64) {
    let imp = (*this).get_impl();
    imp.set_number(&from_glib_borrow(this), num);
}

// GObject glue
#[no_mangle]
pub unsafe extern "C" fn ex_bar_new(name: *const c_char) -> *mut ffi::Bar {
    let obj = glib::Object::new::<BarWrapper>(&[("name", &*glib::GString::from_glib_borrow(name))])
        .unwrap();
    obj.to_glib_full()
}

#[no_mangle]
pub unsafe extern "C" fn ex_bar_get_type() -> glib_ffi::GType {
    Bar::get_type().to_glib()
}
