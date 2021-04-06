#![allow(unused_variables)]

// use std::boxed::Box as Box_;

use crate::prelude::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Clipboard {}

impl Clipboard {
    pub fn get_default() -> Option<Clipboard> {
        // assert_initialized_main_thread!();
        // unsafe { from_glib_none(ffi::clipboard_get_default()) }
        unimplemented!()
    }
}

impl Object for Clipboard {}
impl Is<Clipboard> for Clipboard {}

impl AsRef<Clipboard> for Clipboard {
    fn as_ref(&self) -> &Clipboard {
        self
    }
}

pub const NONE_CLIPBOARD: Option<&Clipboard> = None;

pub trait ClipboardExt: 'static {
    /// get_text:
    /// @clipboard: A #Clipboard
    /// @callback: (scope async): function to be called when the text is retreived
    /// @user_data: data to be passed to the callback
    ///
    /// Request the data from the clipboard in text form. @callback is executed
    /// when the data is retreived.
    ///
    fn get_text<P: FnOnce(&Clipboard, &str) + 'static>(&self, callback: P);

    /// set_text:
    /// @clipboard: A #Clipboard
    /// @text: text to copy to the clipboard
    ///
    /// Sets text as the current contents of the clipboard.
    ///
    fn set_text(&self, text: &str);
}

impl<O: Is<Clipboard>> ClipboardExt for O {
    /// get_text:
    /// @clipboard: A #Clipboard
    /// @callback: (scope async): function to be called when the text is retreived
    /// @user_data: data to be passed to the callback
    ///
    /// Request the data from the clipboard in text form. @callback is executed
    /// when the data is retreived.
    ///
    fn get_text<P: FnOnce(&Clipboard, &str) + 'static>(&self, callback: P) {
        // let callback_data: Box_<P> = Box_::new(callback);
        // unsafe extern "C" fn callback_func<P: FnOnce(&Clipboard, &str) + 'static>(
        //     clipboard: *mut ffi::Clipboard,
        //     text: *const libc::c_char,
        //     user_data: glib_sys::gpointer,
        // ) {
        //     let clipboard = from_glib_borrow(clipboard);
        //     let text: Borrowed<String> = from_glib_borrow(text);
        //     let callback: Box_<P> = Box_::from_raw(user_data as *mut _);
        //     (*callback)(&clipboard, text.as_str());
        // }
        // let callback = Some(callback_func::<P> as _);
        // let super_callback0: Box_<P> = callback_data;
        // unsafe {
        //     ffi::clipboard_get_text(
        //         self.as_ref().to_glib_none().0,
        //         callback,
        //         Box_::into_raw(super_callback0) as *mut _,
        //     );
        // }
        
        // ClipboardClosure *closure;

        // g_return_if_fail (IS_CLIPBOARD (clipboard));
        // g_return_if_fail (callback != NULL);
      
        // closure = g_slice_new (ClipboardClosure);
        // closure->clipboard = clipboard;
        // closure->callback = callback;
        // closure->user_data = user_data;
      
        // g_object_add_weak_pointer (G_OBJECT (clipboard),
        //                            (gpointer *)&closure->clipboard);
        // g_idle_add ((GSourceFunc)clipboard_get_text_cb, closure);
    }

    /// set_text:
    /// @clipboard: A #Clipboard
    /// @text: text to copy to the clipboard
    ///
    /// Sets text as the current contents of the clipboard.
    ///
    fn set_text(&self, text: &str) {
        // g_return_if_fail (IS_CLIPBOARD (clipboard));
        // g_return_if_fail (text != NULL);

        // priv = clipboard->priv;
        // g_free (priv->text);
        // priv->text = g_strdup (text);
    }
}

impl fmt::Display for Clipboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Clipboard")
    }
}