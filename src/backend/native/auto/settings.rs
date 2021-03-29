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

// glib_wrapper! {
//     pub struct Settings(Object<ffi::MxSettings, ffi::MxSettingsClass, SettingsClass>);

//     match fn {
//         get_type => || ffi::mx_settings_get_type(),
//     }
// }

pub struct Settings {

}

impl Settings {
    pub fn get_default() -> Option<Settings> {
        // assert_initialized_main_thread!();
        // unsafe { from_glib_none(ffi::mx_settings_get_default()) }
        unimplemented!()
    }
}

pub const NONE_SETTINGS: Option<&Settings> = None;

// pub trait SettingsExt: 'static {
//     fn get_property_drag_threshold(&self) -> u32;

//     fn set_property_drag_threshold(&self, drag_threshold: u32);

//     fn get_property_font_name(&self) -> Option<GString>;

//     fn set_property_font_name(&self, font_name: Option<&str>);

//     fn get_property_icon_theme(&self) -> Option<GString>;

//     fn set_property_icon_theme(&self, icon_theme: Option<&str>);

//     fn get_property_long_press_timeout(&self) -> u32;

//     fn set_property_long_press_timeout(&self, long_press_timeout: u32);

//     fn get_property_small_screen(&self) -> bool;

//     fn set_property_small_screen(&self, small_screen: bool);

//     fn get_property_touch_mode(&self) -> bool;

//     fn set_property_touch_mode(&self, touch_mode: bool);

//     fn connect_property_drag_threshold_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId;

//     fn connect_property_font_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_icon_theme_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_long_press_timeout_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId;

//     fn connect_property_small_screen_notify<F: Fn(&Self) + 'static>(&self, f: F)
//         -> SignalHandlerId;

//     fn connect_property_touch_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
// }

// impl<O: IsA<Settings>> SettingsExt for O {
//     fn get_property_drag_threshold(&self) -> u32 {
//         unsafe {
//             let mut value = Value::from_type(<u32 as StaticType>::static_type());
//             gobject_sys::g_object_get_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"drag-threshold\0".as_ptr() as *const _,
//                 value.to_glib_none_mut().0,
//             );
//             value
//                 .get()
//                 .expect("Return Value for property `drag-threshold` getter")
//                 .unwrap()
//         }
//     }

//     fn set_property_drag_threshold(&self, drag_threshold: u32) {
//         unsafe {
//             gobject_sys::g_object_set_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"drag-threshold\0".as_ptr() as *const _,
//                 Value::from(&drag_threshold).to_glib_none().0,
//             );
//         }
//     }

//     fn get_property_font_name(&self) -> Option<GString> {
//         unsafe {
//             let mut value = Value::from_type(<GString as StaticType>::static_type());
//             gobject_sys::g_object_get_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"font-name\0".as_ptr() as *const _,
//                 value.to_glib_none_mut().0,
//             );
//             value
//                 .get()
//                 .expect("Return Value for property `font-name` getter")
//         }
//     }

//     fn set_property_font_name(&self, font_name: Option<&str>) {
//         unsafe {
//             gobject_sys::g_object_set_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"font-name\0".as_ptr() as *const _,
//                 Value::from(font_name).to_glib_none().0,
//             );
//         }
//     }

//     fn get_property_icon_theme(&self) -> Option<GString> {
//         unsafe {
//             let mut value = Value::from_type(<GString as StaticType>::static_type());
//             gobject_sys::g_object_get_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"icon-theme\0".as_ptr() as *const _,
//                 value.to_glib_none_mut().0,
//             );
//             value
//                 .get()
//                 .expect("Return Value for property `icon-theme` getter")
//         }
//     }

//     fn set_property_icon_theme(&self, icon_theme: Option<&str>) {
//         unsafe {
//             gobject_sys::g_object_set_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"icon-theme\0".as_ptr() as *const _,
//                 Value::from(icon_theme).to_glib_none().0,
//             );
//         }
//     }

//     fn get_property_long_press_timeout(&self) -> u32 {
//         unsafe {
//             let mut value = Value::from_type(<u32 as StaticType>::static_type());
//             gobject_sys::g_object_get_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"long-press-timeout\0".as_ptr() as *const _,
//                 value.to_glib_none_mut().0,
//             );
//             value
//                 .get()
//                 .expect("Return Value for property `long-press-timeout` getter")
//                 .unwrap()
//         }
//     }

//     fn set_property_long_press_timeout(&self, long_press_timeout: u32) {
//         unsafe {
//             gobject_sys::g_object_set_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"long-press-timeout\0".as_ptr() as *const _,
//                 Value::from(&long_press_timeout).to_glib_none().0,
//             );
//         }
//     }

//     fn get_property_small_screen(&self) -> bool {
//         unsafe {
//             let mut value = Value::from_type(<bool as StaticType>::static_type());
//             gobject_sys::g_object_get_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"small-screen\0".as_ptr() as *const _,
//                 value.to_glib_none_mut().0,
//             );
//             value
//                 .get()
//                 .expect("Return Value for property `small-screen` getter")
//                 .unwrap()
//         }
//     }

//     fn set_property_small_screen(&self, small_screen: bool) {
//         unsafe {
//             gobject_sys::g_object_set_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"small-screen\0".as_ptr() as *const _,
//                 Value::from(&small_screen).to_glib_none().0,
//             );
//         }
//     }

//     fn get_property_touch_mode(&self) -> bool {
//         unsafe {
//             let mut value = Value::from_type(<bool as StaticType>::static_type());
//             gobject_sys::g_object_get_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"touch-mode\0".as_ptr() as *const _,
//                 value.to_glib_none_mut().0,
//             );
//             value
//                 .get()
//                 .expect("Return Value for property `touch-mode` getter")
//                 .unwrap()
//         }
//     }

//     fn set_property_touch_mode(&self, touch_mode: bool) {
//         unsafe {
//             gobject_sys::g_object_set_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"touch-mode\0".as_ptr() as *const _,
//                 Value::from(&touch_mode).to_glib_none().0,
//             );
//         }
//     }

//     fn connect_property_drag_threshold_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_drag_threshold_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxSettings,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Settings>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Settings::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::drag-threshold\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_drag_threshold_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_font_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_font_name_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxSettings,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Settings>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Settings::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::font-name\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_font_name_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_icon_theme_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_icon_theme_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxSettings,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Settings>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Settings::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::icon-theme\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_icon_theme_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_long_press_timeout_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_long_press_timeout_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxSettings,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Settings>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Settings::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::long-press-timeout\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_long_press_timeout_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_small_screen_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_small_screen_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxSettings,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Settings>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Settings::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::small-screen\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_small_screen_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_touch_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_touch_mode_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::MxSettings,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: IsA<Settings>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Settings::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::touch-mode\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_touch_mode_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }
// }

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Settings")
    }
}