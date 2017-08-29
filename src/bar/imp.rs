use glib_ffi;
use gobject_ffi;

use std::ffi::CString;
use std::sync::{Once, ONCE_INIT};
use std::mem;
use std::ptr;

use std::cell::RefCell;

use glib;
use glib::translate::{from_glib_none, ToGlibPtr};

use libc::c_char;

use bar::Bar as BarWrapper;
use foo;

// Instance struct
#[repr(C)]
pub struct Bar {
    pub parent: foo::imp::Foo,
}

// Class struct aka "vtable"
//
// Here we would store virtual methods and similar
#[repr(C)]
pub struct BarClass {
    pub parent_class: foo::imp::FooClass,
}

#[repr(u32)]
enum Properties {
    Number = 1,
}

// We could put our data into the Bar struct above but that's discouraged nowadays so let's just
// keep it all in BarPrivate
//
// We use RefCells here for each field as GObject conceptually uses interior mutability everywhere.
// If this was to be used from multiple threads, these would have to be mutexes or otherwise
// Sync+Send
struct BarPrivate {
    number: RefCell<f64>,
}

// static mut is unsafe, but we only ever initialize it from class_init() which is guaranteed to be
// called from a single place ever and then only read it
struct BarClassPrivate {
    parent_class: *const foo::imp::FooClass,
    properties: *const Vec<*const gobject_ffi::GParamSpec>,
}
static mut PRIV: BarClassPrivate = BarClassPrivate {
    parent_class: 0 as *const _,
    properties: 0 as *const _,
};

impl Bar {
    // Helper functions
    fn get_class(&self) -> &BarClass {
        unsafe {
            let klass = (*(self as *const _ as *const gobject_ffi::GTypeInstance)).g_class;
            &*(klass as *const BarClass)
        }
    }

    fn get_priv(&self) -> &BarPrivate {
        unsafe {
            let private = gobject_ffi::g_type_instance_get_private(
                self as *const _ as *mut gobject_ffi::GTypeInstance,
                ex_bar_get_type(),
            ) as *const Option<BarPrivate>;

            (&*private).as_ref().unwrap()
        }
    }

    // Instance struct and private data initialization, called from GObject
    unsafe extern "C" fn init(obj: *mut gobject_ffi::GTypeInstance, _klass: glib_ffi::gpointer) {
        let private = gobject_ffi::g_type_instance_get_private(
            obj as *mut gobject_ffi::GTypeInstance,
            ex_bar_get_type(),
        ) as *mut Option<BarPrivate>;

        ptr::write(
            private,
            Some(BarPrivate {
                number: RefCell::new(0.0),
            }),
        );
    }

    //
    // Virtual method implementations / trampolines to safe implementations
    //
    unsafe extern "C" fn finalize(obj: *mut gobject_ffi::GObject) {
        // Free private data by replacing it with None
        let private = gobject_ffi::g_type_instance_get_private(
            obj as *mut gobject_ffi::GTypeInstance,
            ex_bar_get_type(),
        ) as *mut Option<BarPrivate>;
        let _ = (*private).take();

        (*(PRIV.parent_class as *const gobject_ffi::GObjectClass))
            .finalize
            .map(|f| f(obj));
    }

    unsafe extern "C" fn set_property(
        obj: *mut gobject_ffi::GObject,
        id: u32,
        value: *mut gobject_ffi::GValue,
        _pspec: *mut gobject_ffi::GParamSpec,
    ) {
        let this = &*(obj as *mut Bar);
        let private = (*this).get_priv();

        match mem::transmute::<u32, Properties>(id) {
            Properties::Number => {
                // FIXME: Need impl FromGlibPtrBorrow for Value
                let num = gobject_ffi::g_value_get_double(value);
                Bar::set_number(&from_glib_none(obj as *mut Bar), private, num);
            }
            _ => unreachable!(),
        }
    }

    unsafe extern "C" fn get_property(
        obj: *mut gobject_ffi::GObject,
        id: u32,
        value: *mut gobject_ffi::GValue,
        _pspec: *mut gobject_ffi::GParamSpec,
    ) {
        let private = (*(obj as *mut Bar)).get_priv();

        match mem::transmute::<u32, Properties>(id) {
            Properties::Number => {
                let num = Bar::get_number(&from_glib_none(obj as *mut Bar), private);
                // FIXME: Need impl FromGlibPtrBorrow for Value
                gobject_ffi::g_value_set_double(value, num);
            }
            _ => unreachable!(),
        }
    }

    unsafe extern "C" fn increment_trampoline(this: *mut foo::imp::Foo, inc: i32) -> i32 {
        let this = this as *mut Bar;
        let private = (*this).get_priv();

        Bar::increment(&from_glib_none(this), private, inc)
    }

    //
    // Safe implementations. These take the wrapper type, and not &Self, as first argument
    //
    fn increment(this: &BarWrapper, private: &BarPrivate, inc: i32) -> i32 {
        // We could do our own stuff here but instead we just chain
        // up with twice the inc
        //
        // TODO: Ideally we would have safe wrappers around the virtual methods
        unsafe {
            ((*PRIV.parent_class).increment.as_ref().unwrap())(this.to_glib_none().0, 2 * inc)
        }
    }

    fn set_number(this: &BarWrapper, private: &BarPrivate, num: f64) {
        *private.number.borrow_mut() = num;

        unsafe {
            gobject_ffi::g_object_notify_by_pspec(
                this.to_glib_none().0,
                (*PRIV.properties)[Properties::Number as usize] as *mut _,
            );
        }
    }

    fn get_number(_this: &BarWrapper, private: &BarPrivate) -> f64 {
        *private.number.borrow_mut()
    }
}

impl BarClass {
    // Class struct initialization, called from GObject
    unsafe extern "C" fn init(klass: glib_ffi::gpointer, _klass_data: glib_ffi::gpointer) {
        // This is an Option<_> so that we can replace its value with None on finalize() to
        // release all memory it holds
        gobject_ffi::g_type_class_add_private(klass, mem::size_of::<Option<BarPrivate>>() as usize);

        {
            let gobject_klass = &mut *(klass as *mut gobject_ffi::GObjectClass);
            gobject_klass.finalize = Some(Bar::finalize);
            gobject_klass.set_property = Some(Bar::set_property);
            gobject_klass.get_property = Some(Bar::get_property);

            let mut properties = Vec::new();

            let name_cstr = CString::new("number").unwrap();
            let nick_cstr = CString::new("Number").unwrap();
            let blurb_cstr = CString::new("Some number").unwrap();

            properties.push(ptr::null());
            properties.push(gobject_ffi::g_param_spec_double(
                name_cstr.as_ptr(),
                nick_cstr.as_ptr(),
                blurb_cstr.as_ptr(),
                0.0,
                100.0,
                0.0,
                gobject_ffi::G_PARAM_READWRITE,
            ));
            gobject_ffi::g_object_class_install_properties(
                gobject_klass,
                properties.len() as u32,
                properties.as_mut_ptr() as *mut *mut _,
            );

            PRIV.properties = Box::into_raw(Box::new(properties));
        }

        {
            let foo_klass = &mut *(klass as *mut foo::imp::FooClass);
            foo_klass.increment = Some(Bar::increment_trampoline);
        }

        PRIV.parent_class =
            gobject_ffi::g_type_class_peek_parent(klass) as *const foo::imp::FooClass;
    }
}

//
// Public C functions below
//

// Trampolines to safe Rust implementations
#[no_mangle]
pub unsafe extern "C" fn ex_bar_get_number(this: *mut Bar) -> f64 {
    let private = (*this).get_priv();

    Bar::get_number(&from_glib_none(this), private)
}

#[no_mangle]
pub unsafe extern "C" fn ex_bar_set_number(this: *mut Bar, num: f64) {
    let private = (*this).get_priv();

    Bar::set_number(&from_glib_none(this), private, num);
}

// GObject glue
#[no_mangle]
pub unsafe extern "C" fn ex_bar_new(name: *const c_char) -> *mut Bar {
    // FIXME: need to prevent the string copies (property name and the value) here
    let prop_name_name = "name".to_glib_none();
    let prop_name_str: Option<String> = from_glib_none(name);
    let prop_name_value = glib::Value::from(prop_name_str.as_ref());

    let mut properties = [
        gobject_ffi::GParameter {
            name: prop_name_name.0,
            value: prop_name_value.into_raw(),
        },
    ];
    let this = gobject_ffi::g_object_newv(
        ex_bar_get_type(),
        properties.len() as u32,
        properties.as_mut_ptr(),
    );

    // FIXME: Ugly
    gobject_ffi::g_value_unset(&mut properties[0].value);

    this as *mut Bar
}

#[no_mangle]
pub unsafe extern "C" fn ex_bar_get_type() -> glib_ffi::GType {
    static mut TYPE: glib_ffi::GType = gobject_ffi::G_TYPE_INVALID;
    static ONCE: Once = ONCE_INIT;

    ONCE.call_once(|| {
        let type_info = gobject_ffi::GTypeInfo {
            class_size: mem::size_of::<BarClass>() as u16,
            base_init: None,
            base_finalize: None,
            class_init: Some(BarClass::init),
            class_finalize: None,
            class_data: ptr::null(),
            instance_size: mem::size_of::<Bar>() as u16,
            n_preallocs: 0,
            instance_init: Some(Bar::init),
            value_table: ptr::null(),
        };

        let type_name = CString::new("ExBar").unwrap();

        TYPE = gobject_ffi::g_type_register_static(
            foo::imp::ex_foo_get_type(),
            type_name.as_ptr(),
            &type_info,
            gobject_ffi::GTypeFlags::empty(),
        );
    });

    TYPE
}
