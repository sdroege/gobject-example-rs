use std::cell::RefCell;
use std::ops;

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::ToValue;

use crate::nameable::*;

// Class struct aka "vtable"
//
// Here we would store virtual methods and similar
#[repr(C)]
pub struct FooClass {
    pub parent_class: glib::gobject_ffi::GObjectClass,
    pub increment: Option<unsafe extern "C" fn(*mut ffi::ExFoo, inc: i32) -> i32>,
    pub incremented: Option<unsafe extern "C" fn(*mut ffi::ExFoo, val: i32, inc: i32)>,
}

unsafe impl ClassStruct for FooClass {
    type Type = Foo;
}

impl ops::Deref for FooClass {
    type Target = glib::Class<glib::Object>;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const _ as *const Self::Target) }
    }
}

impl ops::DerefMut for FooClass {
    fn deref_mut(&mut self) -> &mut glib::Class<glib::Object> {
        unsafe { &mut *(self as *mut _ as *mut glib::Class<glib::Object>) }
    }
}

// Our private state for the class
//
// We use RefCells here for each field as GObject conceptually uses interior mutability everywhere.
// If this was to be used from multiple threads, these would have to be mutexes or otherwise
// Sync+Send
#[derive(Debug, Default)]
pub struct Foo {
    name: RefCell<Option<String>>,
    counter: RefCell<i32>,
}

#[glib::object_subclass]
impl ObjectSubclass for Foo {
    const NAME: &'static str = "ExFoo";
    type ParentType = glib::Object;
    type Type = super::Foo;
    type Class = FooClass;
    type Interfaces = (Nameable,);

    fn class_init(klass: &mut Self::Class) {
        klass.increment = Some(increment_default_trampoline);
        klass.incremented = Some(incremented_default_trampoline);
    }
}

impl ObjectImpl for Foo {
    fn signals() -> &'static [glib::subclass::Signal] {
        use once_cell::sync::Lazy;
        static SIGNALS: Lazy<Vec<glib::subclass::Signal>> = Lazy::new(|| {
            vec![glib::subclass::Signal::builder(
                "incremented",
                &[i32::static_type().into(), i32::static_type().into()],
                glib::Type::UNIT.into(),
            )
            .class_handler(|_, args| {
                let obj = args[0].get::<glib::Object>().unwrap();
                let val = args[1].get::<i32>().unwrap();
                let inc = args[2].get::<i32>().unwrap();

                unsafe {
                    let klass = &*(obj.object_class() as *const _ as *const FooClass);
                    if let Some(ref func) = klass.incremented {
                        func(obj.as_ptr() as *mut ffi::ExFoo, val, inc);
                    }
                }

                None
            })
            .build()]
        });

        SIGNALS.as_ref()
    }

    fn properties() -> &'static [glib::ParamSpec] {
        use once_cell::sync::Lazy;
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
            vec![glib::ParamSpecString::new(
                "name",
                "Name",
                "Name of this object",
                None,
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
            "name" => {
                let name = value.get().unwrap();
                self.set_name(obj, name);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "name" => self.name(obj).to_value(),
            _ => unimplemented!(),
        }
    }
}

impl NameableImpl for Foo {
    fn name(&self, nameable: &Self::Type) -> Option<String> {
        self.name(nameable.dynamic_cast_ref().unwrap())
    }
}

impl Foo {
    //
    // Safe implementations. These take the wrapper type, and not &Self, as first argument
    //
    fn increment(&self, this: &super::Foo, inc: i32) -> i32 {
        let mut val = self.counter.borrow_mut();

        *val += inc;

        this.emit_by_name::<()>("incremented", &[&*val, &inc]);

        *val
    }

    fn incremented(&self, _this: &super::Foo, _val: i32, _inc: i32) {
        // Could do something here. Default/class handler of the "incremented"
        // signal that could be overriden by subclasses
    }

    fn counter(&self, _this: &super::Foo) -> i32 {
        *self.counter.borrow()
    }

    fn name(&self, _this: &super::Foo) -> Option<String> {
        self.name.borrow().clone()
    }

    fn set_name(&self, _this: &super::Foo, name: Option<String>) {
        *self.name.borrow_mut() = name;
    }
}

pub(crate) mod ffi {
    use glib::translate::*;
    use libc::c_char;

    pub type ExFoo = <super::Foo as super::ObjectSubclass>::Instance;
    pub type ExFooClass = super::FooClass;

    /// # Safety
    ///
    /// Must be a ExFoo object.
    #[no_mangle]
    pub unsafe extern "C" fn ex_foo_increment(this: *mut ExFoo, inc: i32) -> i32 {
        let klass = glib::subclass::types::InstanceStruct::class(&*this);

        (klass.increment.unwrap())(this, inc)
    }

    // Trampolines to safe Rust implementations
    /// # Safety
    ///
    /// Must be a FooInstance object.
    #[no_mangle]
    pub unsafe extern "C" fn ex_foo_get_counter(this: *mut ExFoo) -> i32 {
        let imp = glib::subclass::types::InstanceStruct::imp(&*this);
        imp.counter(&from_glib_borrow(this))
    }

    /// # Safety
    ///
    /// Must be a FooInstance object.
    #[no_mangle]
    pub unsafe extern "C" fn ex_foo_get_name(this: *mut ExFoo) -> *mut c_char {
        let imp = glib::subclass::types::InstanceStruct::imp(&*this);
        imp.name(&from_glib_borrow(this)).to_glib_full()
    }

    // GObject glue
    /// # Safety
    ///
    /// Must be a valid C string, 0-terminated.
    #[no_mangle]
    pub unsafe extern "C" fn ex_foo_new(name: *const c_char) -> *mut ExFoo {
        let obj = glib::Object::new::<super::super::Foo>(&[(
            "name",
            &*glib::GString::from_glib_borrow(name),
        )])
        .unwrap();
        obj.to_glib_full()
    }

    #[no_mangle]
    pub extern "C" fn ex_foo_get_type() -> glib::ffi::GType {
        <super::super::Foo as glib::StaticType>::static_type().into_glib()
    }
}

// Virtual method default implementation trampolines
unsafe extern "C" fn increment_default_trampoline(this: *mut ffi::ExFoo, inc: i32) -> i32 {
    let imp = (*this).imp();
    imp.increment(&from_glib_borrow(this), inc)
}

unsafe extern "C" fn incremented_default_trampoline(this: *mut ffi::ExFoo, val: i32, inc: i32) {
    let imp = (*this).imp();
    imp.incremented(&from_glib_borrow(this), val, inc);
}
