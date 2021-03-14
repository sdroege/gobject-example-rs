use std::cell::RefCell;

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;

use libc::c_char;

use super::Bar as BarWrapper;
use crate::foo::*;

pub mod ffi {
    pub type Bar = <super::Bar as super::ObjectSubclass>::Instance;
}

// We could put our data into the Bar struct above but that's discouraged nowadays so let's just
// keep it all in Bar
//
// We use RefCells here for each field as GObject conceptually uses interior mutability everywhere.
// If this was to be used from multiple threads, these would have to be mutexes or otherwise
// Sync+Send
#[derive(Debug, Default)]
pub struct Bar {
    number: RefCell<f64>,
}

#[glib::object_subclass]
impl ObjectSubclass for Bar {
    const NAME: &'static str = "ExBar";
    type ParentType = Foo;
    type Type = BarWrapper;
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

impl FooImpl for Bar {
    fn increment(&self, obj: &Foo, inc: i32) -> i32 {
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
//
/// # Safety
///
/// Must be a BarInstance object.
#[no_mangle]
pub unsafe extern "C" fn ex_bar_get_number(this: *mut ffi::Bar) -> f64 {
    let imp = (*this).get_impl();
    imp.get_number(&from_glib_borrow(this))
}

/// # Safety
///
/// Must be a BarInstance object.
#[no_mangle]
pub unsafe extern "C" fn ex_bar_set_number(this: *mut ffi::Bar, num: f64) {
    let imp = (*this).get_impl();
    imp.set_number(&from_glib_borrow(this), num);
}

// GObject glue
/// # Safety
///
/// Must be a valid C string, 0-terminated.
#[no_mangle]
pub unsafe extern "C" fn ex_bar_new(name: *const c_char) -> *mut ffi::Bar {
    let obj = glib::Object::new::<BarWrapper>(&[("name", &*glib::GString::from_glib_borrow(name))])
        .unwrap();
    obj.to_glib_full()
}

#[no_mangle]
pub extern "C" fn ex_bar_get_type() -> glib::ffi::GType {
    Bar::get_type().to_glib()
}
