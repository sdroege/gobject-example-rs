use glib_ffi;

use std::cell::RefCell;

use glib;
use glib::prelude::*;
use glib::subclass;
use glib::subclass::prelude::*;
use glib::translate::*;

use libc::c_char;

use bar::Bar as BarWrapper;
use foo;

pub type Bar = <BarPrivate as ObjectSubclass>::Instance;
pub type BarClass = <BarPrivate as ObjectSubclass>::Class;

static PROPERTIES: [subclass::Property; 1] = [subclass::Property("number", |name| {
    glib::ParamSpec::double(
        name,
        "Number`",
        "Some Number",
        0.0,
        100.0,
        0.0,
        glib::ParamFlags::READWRITE,
    )
})];

// We could put our data into the Bar struct above but that's discouraged nowadays so let's just
// keep it all in BarPrivate
//
// We use RefCells here for each field as GObject conceptually uses interior mutability everywhere.
// If this was to be used from multiple threads, these would have to be mutexes or otherwise
// Sync+Send
pub struct BarPrivate {
    number: RefCell<f64>,
}

impl ObjectSubclass for BarPrivate {
    const NAME: &'static str = "ExBar";
    type ParentType = foo::Foo;
    type Instance = subclass::simple::InstanceStruct<Self>;
    type Class = subclass::simple::ClassStruct<Self>;

    glib_object_subclass!();

    fn class_init(klass: &mut Self::Class) {
        klass.install_properties(&PROPERTIES);
    }

    fn new() -> Self {
        Self {
            number: RefCell::new(0.0),
        }
    }
}

impl ObjectImpl for BarPrivate {
    glib_object_impl!();

    fn set_property(&self, obj: &glib::Object, id: usize, value: &glib::Value) {
        let prop = &PROPERTIES[id];

        match *prop {
            subclass::Property("number", ..) => {
                let number = value.get().unwrap();
                self.set_number(obj.downcast_ref().unwrap(), number);
            }
            _ => unimplemented!(),
        }
    }

    fn get_property(&self, obj: &glib::Object, id: usize) -> Result<glib::Value, ()> {
        let prop = &PROPERTIES[id];

        match *prop {
            subclass::Property("number", ..) => {
                Ok(self.get_number(obj.downcast_ref().unwrap()).to_value())
            }
            _ => unimplemented!(),
        }
    }
}

impl foo::FooImpl for BarPrivate {
    fn increment(&self, obj: &foo::Foo, inc: i32) -> i32 {
        self.parent_increment(obj, 2 * inc)
    }
}

impl BarPrivate {
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
pub unsafe extern "C" fn ex_bar_get_number(this: *mut Bar) -> f64 {
    let imp = (*this).get_impl();
    imp.get_number(&from_glib_borrow(this))
}

#[no_mangle]
pub unsafe extern "C" fn ex_bar_set_number(this: *mut Bar, num: f64) {
    let imp = (*this).get_impl();
    imp.set_number(&from_glib_borrow(this), num);
}

// GObject glue
#[no_mangle]
pub unsafe extern "C" fn ex_bar_new(name: *const c_char) -> *mut Bar {
    let obj = glib::Object::new(
        BarPrivate::get_type(),
        &[("name", &glib::GString::from_glib_borrow(name))],
    )
    .unwrap()
    .downcast::<BarWrapper>()
    .unwrap();
    obj.to_glib_full()
}

#[no_mangle]
pub unsafe extern "C" fn ex_bar_get_type() -> glib_ffi::GType {
    BarPrivate::get_type().to_glib()
}
