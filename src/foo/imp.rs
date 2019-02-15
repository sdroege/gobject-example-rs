use glib_ffi;
use gobject_ffi;

use std::ops;

use std::cell::RefCell;

use glib;
use glib::prelude::*;
use glib::subclass;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::ToValue;

use libc::c_char;

use foo::Foo as FooWrapper;

// Instance struct
#[repr(C)]
pub struct Foo {
    pub parent: gobject_ffi::GObject,
}

unsafe impl InstanceStruct for Foo {
    type Type = FooPrivate;
}

// Class struct aka "vtable"
//
// Here we would store virtual methods and similar
#[repr(C)]
pub struct FooClass {
    pub parent_class: gobject_ffi::GObjectClass,
    pub increment: Option<unsafe extern "C" fn(*mut Foo, inc: i32) -> i32>,
    pub incremented: Option<unsafe extern "C" fn(*mut Foo, val: i32, inc: i32)>,
}

unsafe impl ClassStruct for FooClass {
    type Type = FooPrivate;
}

impl ops::Deref for FooClass {
    type Target = glib::ObjectClass;

    fn deref(&self) -> &glib::ObjectClass {
        unsafe { &*(self as *const _ as *const glib::ObjectClass) }
    }
}

impl ops::DerefMut for FooClass {
    fn deref_mut(&mut self) -> &mut glib::ObjectClass {
        unsafe { &mut *(self as *mut _ as *mut glib::ObjectClass) }
    }
}

static PROPERTIES: [subclass::Property; 1] = [subclass::Property("name", |name| {
    glib::ParamSpec::string(
        name,
        "Name",
        "Name of this object",
        None,
        glib::ParamFlags::READWRITE,
    )
})];

// Our private state for the class
//
// We use RefCells here for each field as GObject conceptually uses interior mutability everywhere.
// If this was to be used from multiple threads, these would have to be mutexes or otherwise
// Sync+Send
pub struct FooPrivate {
    name: RefCell<Option<String>>,
    counter: RefCell<i32>,
}

impl ObjectSubclass for FooPrivate {
    const NAME: &'static str = "ExFoo";
    type ParentType = glib::Object;
    type Instance = Foo;
    type Class = FooClass;

    glib_object_subclass!();

    fn type_init(type_: &mut subclass::InitializingType<Self>) {
        type_.add_interface::<super::nameable::Nameable>();
    }

    fn class_init(klass: &mut FooClass) {
        klass.increment = Some(increment_default_trampoline);
        klass.incremented = Some(incremented_default_trampoline);

        klass.install_properties(&PROPERTIES);

        klass.add_signal_with_class_handler(
            "incremented",
            glib::SignalFlags::empty(),
            &[i32::static_type(), i32::static_type()],
            glib::Type::Unit,
            |_, args| {
                let obj = args[0].get::<glib::Object>().unwrap();
                let val = args[1].get::<i32>().unwrap();
                let inc = args[2].get::<i32>().unwrap();

                unsafe {
                    let klass = &*(obj.get_object_class() as *const _ as *const FooClass);
                    if let Some(ref func) = klass.incremented {
                        func(obj.as_ptr() as *mut Foo, val, inc);
                    }
                }

                None
            },
        );
    }

    fn new() -> Self {
        Self {
            name: RefCell::new(None),
            counter: RefCell::new(0),
        }
    }
}

impl ObjectImpl for FooPrivate {
    glib_object_impl!();

    fn constructed(&self, obj: &glib::Object) {
        self.parent_constructed(obj);
    }

    fn set_property(&self, obj: &glib::Object, id: usize, value: &glib::Value) {
        let prop = &PROPERTIES[id];

        match *prop {
            subclass::Property("name", ..) => {
                let name = value.get();
                self.set_name(obj.downcast_ref().unwrap(), name);
            }
            _ => unimplemented!(),
        }
    }

    fn get_property(&self, obj: &glib::Object, id: usize) -> Result<glib::Value, ()> {
        let prop = &PROPERTIES[id];

        match *prop {
            subclass::Property("name", ..) => {
                Ok(self.get_name(obj.downcast_ref().unwrap()).to_value())
            }
            _ => unimplemented!(),
        }
    }
}

impl super::nameable::NameableImpl for FooPrivate {
    fn get_name(&self, this: &super::nameable::Nameable) -> Option<String> {
        self.get_name(this.dynamic_cast_ref().unwrap())
    }
}

impl FooPrivate {
    //
    // Safe implementations. These take the wrapper type, and not &Self, as first argument
    //
    fn increment(&self, this: &FooWrapper, inc: i32) -> i32 {
        let mut val = self.counter.borrow_mut();

        *val += inc;

        this.emit("incremented", &[&*val, &inc]).unwrap();

        *val
    }

    fn incremented(&self, _this: &FooWrapper, _val: i32, _inc: i32) {
        // Could do something here. Default/class handler of the "incremented"
        // signal that could be overriden by subclasses
    }

    fn get_counter(&self, _this: &FooWrapper) -> i32 {
        *self.counter.borrow()
    }

    fn get_name(&self, _this: &FooWrapper) -> Option<String> {
        self.name.borrow().clone()
    }

    fn set_name(&self, _this: &FooWrapper, name: Option<String>) {
        *self.name.borrow_mut() = name;
    }
}

//
// Public C functions below
//

// Virtual method callers
#[no_mangle]
pub unsafe extern "C" fn ex_foo_increment(this: *mut Foo, inc: i32) -> i32 {
    let klass = (*this).get_class();

    (klass.increment.as_ref().unwrap())(this, inc)
}

// Trampolines to safe Rust implementations
#[no_mangle]
pub unsafe extern "C" fn ex_foo_get_counter(this: *mut Foo) -> i32 {
    let imp = (*this).get_impl();
    imp.get_counter(&from_glib_borrow(this))
}

#[no_mangle]
pub unsafe extern "C" fn ex_foo_get_name(this: *mut Foo) -> *mut c_char {
    let imp = (*this).get_impl();
    imp.get_name(&from_glib_borrow(this)).to_glib_full()
}

// GObject glue
#[no_mangle]
pub unsafe extern "C" fn ex_foo_new(name: *const c_char) -> *mut Foo {
    let obj = glib::Object::new(
        FooPrivate::get_type(),
        &[("name", &glib::GString::from_glib_borrow(name))],
    )
    .unwrap()
    .downcast::<FooWrapper>()
    .unwrap();
    obj.to_glib_full()
}

#[no_mangle]
pub unsafe extern "C" fn ex_foo_get_type() -> glib_ffi::GType {
    FooPrivate::get_type().to_glib()
}

// Virtual method default implementation trampolines
unsafe extern "C" fn increment_default_trampoline(this: *mut Foo, inc: i32) -> i32 {
    let imp = (*this).get_impl();
    imp.increment(&from_glib_borrow(this), inc)
}

unsafe extern "C" fn incremented_default_trampoline(this: *mut Foo, val: i32, inc: i32) {
    let imp = (*this).get_impl();
    imp.incremented(&from_glib_borrow(this), val, inc);
}
