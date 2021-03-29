// use clutter;
// use glib::object::Cast;
// use glib::object::IsA;
// use glib::signal::connect_raw;
// use glib::signal::SignalHandlerId;
// use glib::translate::*;
// use glib::GString;
// use glib::StaticType;
// use glib::Value;
// use glib_sys;
// use gobject_sys;
// use ffi;
// use std::boxed::Box as Box_;
use std::fmt;
// use std::mem::transmute;
// use Widget;

// glib_wrapper! {
//     pub struct Expander(Object<ffi::MxExpander, ffi::MxExpanderClass, ExpanderClass>) @extends Widget, clutter::Actor;

//     match fn {
//         get_type => || ffi::mx_expander_get_type(),
//     }
// }

pub struct Expander {

}

impl Expander {
    pub fn new() -> Expander {
        // assert_initialized_main_thread!();
        // unsafe { clutter::Actor::from_glib_none(ffi::mx_expander_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for Expander {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_EXPANDER: Option<&Expander> = None;

// pub trait ExpanderExt: 'static {
//     fn get_expanded(&self) -> bool;

//     fn set_expanded(&self, expanded: bool);

//     fn set_label(&self, label: &str);

//     fn get_property_label(&self) -> Option<GString>;

//     fn connect_expand_complete<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_expanded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
// }

// impl<O: IsA<Expander>> ExpanderExt for O {
//     fn get_expanded(&self) -> bool {
//         unsafe {
//             from_glib(ffi::mx_expander_get_expanded(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn set_expanded(&self, expanded: bool) {
//         unsafe {
//             ffi::mx_expander_set_expanded(self.as_ref().to_glib_none().0, expanded.to_glib());
//         }
//     }

//     fn set_label(&self, label: &str) {
//         unsafe {
//             ffi::mx_expander_set_label(self.as_ref().to_glib_none().0, label.to_glib_none().0);
//         }
//     }

//     fn get_property_label(&self) -> Option<GString> {
//         unsafe {
//             let mut value = Value::from_type(<GString as StaticType>::static_type());
//             gobject_sys::g_object_get_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"label\0".as_ptr() as *const _,
//                 value.to_glib_none_mut().0,
//             );
//             value
//                 .get()
//                 .expect("Return Value for property `label` getter")
//         }
//     }

//     fn connect_expand_complete<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn expand_complete_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxExpander,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Expander>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Expander::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"expand-complete\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     expand_complete_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_expanded_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_expanded_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxExpander,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Expander>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Expander::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::expanded\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_expanded_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_label_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxExpander,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Expander>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Expander::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::label\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_label_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }
// }

impl fmt::Display for Expander {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Expander")
    }
}