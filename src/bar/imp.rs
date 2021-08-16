use std::cell::RefCell;

use glib::prelude::*;
use glib::subclass::prelude::*;

use crate::foo::*;

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
    type Type = super::Bar;
}

impl ObjectImpl for Bar {
    fn properties() -> &'static [glib::ParamSpec] {
        use once_cell::sync::Lazy;
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
            vec![glib::ParamSpec::new_double(
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
        match pspec.name() {
            "number" => {
                let number = value.get().unwrap();
                self.set_number(obj, number);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "number" => self.number(obj).to_value(),
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
    fn set_number(&self, this: &super::Bar, num: f64) {
        *self.number.borrow_mut() = num;
        this.notify("number");
    }

    fn number(&self, _this: &super::Bar) -> f64 {
        *self.number.borrow_mut()
    }
}

pub(crate) mod ffi {
    use glib::translate::*;
    use libc::c_char;

    pub type ExBar = <super::Bar as super::ObjectSubclass>::Instance;

    /// # Safety
    ///
    /// Must be a BarInstance object.
    #[no_mangle]
    pub unsafe extern "C" fn ex_bar_get_number(this: *mut ExBar) -> f64 {
        let imp = glib::subclass::types::InstanceStruct::impl_(&*this);
        imp.number(&from_glib_borrow(this))
    }

    /// # Safety
    ///
    /// Must be a BarInstance object.
    #[no_mangle]
    pub unsafe extern "C" fn ex_bar_set_number(this: *mut ExBar, num: f64) {
        let imp = glib::subclass::types::InstanceStruct::impl_(&*this);
        imp.set_number(&from_glib_borrow(this), num);
    }

    // GObject glue
    /// # Safety
    ///
    /// Must be a valid C string, 0-terminated.
    #[no_mangle]
    pub unsafe extern "C" fn ex_bar_new(name: *const c_char) -> *mut ExBar {
        let obj = glib::Object::new::<super::super::Bar>(&[(
            "name",
            &*glib::GString::from_glib_borrow(name),
        )])
        .unwrap();
        obj.to_glib_full()
    }

    #[no_mangle]
    pub extern "C" fn ex_bar_get_type() -> glib::ffi::GType {
        <super::super::Bar as glib::StaticType>::static_type().into_glib()
    }
}
