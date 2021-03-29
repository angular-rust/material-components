// use clutter;
// use glib::object::Cast;
// use glib::object::IsA;
// use glib::signal::connect_raw;
// use glib::signal::SignalHandlerId;
// use glib::translate::*;
// use glib::GString;
// use glib_sys;
// use ffi;
// use std::boxed::Box as Box_;
use std::fmt;
// use std::mem::transmute;
// use FloatingWidget;
// use Widget;

// glib_wrapper! {
//     pub struct Tooltip(Object<ffi::MxTooltip, ffi::MxTooltipClass, TooltipClass>) @extends FloatingWidget, Widget, clutter::Actor;

//     match fn {
//         get_type => || ffi::mx_tooltip_get_type(),
//     }
// }

pub struct Tooltip {

}

impl Tooltip {
    pub fn is_in_browse_mode() -> bool {
        // assert_initialized_main_thread!();
        // unsafe { from_glib(ffi::mx_tooltip_is_in_browse_mode()) }
        unimplemented!()
    }
}

pub const NONE_TOOLTIP: Option<&Tooltip> = None;

// pub trait TooltipExt: 'static {
//     fn get_text(&self) -> Option<GString>;

//     fn get_tip_area(&self) -> Option<clutter::Geometry>;

//     fn hide(&self);

//     fn set_text(&self, text: &str);

//     fn set_tip_area(&self, area: &clutter::Geometry);

//     fn set_tip_area_from_actor<P: IsA<clutter::Actor>>(&self, actor: &P);

//     fn show(&self);

//     fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_tip_area_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
// }

// impl<O: IsA<Tooltip>> TooltipExt for O {
//     fn get_text(&self) -> Option<GString> {
//         unsafe { from_glib_none(ffi::mx_tooltip_get_text(self.as_ref().to_glib_none().0)) }
//     }

//     fn get_tip_area(&self) -> Option<clutter::Geometry> {
//         unsafe {
//             from_glib_none(ffi::mx_tooltip_get_tip_area(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn hide(&self) {
//         unsafe {
//             ffi::mx_tooltip_hide(self.as_ref().to_glib_none().0);
//         }
//     }

//     fn set_text(&self, text: &str) {
//         unsafe {
//             ffi::mx_tooltip_set_text(self.as_ref().to_glib_none().0, text.to_glib_none().0);
//         }
//     }

//     fn set_tip_area(&self, area: &clutter::Geometry) {
//         unsafe {
//             ffi::mx_tooltip_set_tip_area(self.as_ref().to_glib_none().0, area.to_glib_none().0);
//         }
//     }

//     fn set_tip_area_from_actor<P: IsA<clutter::Actor>>(&self, actor: &P) {
//         unsafe {
//             ffi::mx_tooltip_set_tip_area_from_actor(
//                 self.as_ref().to_glib_none().0,
//                 actor.as_ref().to_glib_none().0,
//             );
//         }
//     }

//     fn show(&self) {
//         unsafe {
//             ffi::mx_tooltip_show(self.as_ref().to_glib_none().0);
//         }
//     }

//     fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_text_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxTooltip,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Tooltip>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Tooltip::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::text\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_text_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_tip_area_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_tip_area_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxTooltip,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Tooltip>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Tooltip::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::tip-area\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_tip_area_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }
// }

impl fmt::Display for Tooltip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tooltip")
    }
}
