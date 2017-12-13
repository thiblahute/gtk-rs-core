// This file was generated by gir (13f739b) from gir-files (db49619)
// DO NOT EDIT

use ApplicationInhibitFlags;
use Window;
use ffi;
use gio;
use gio_ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Application(Object<ffi::GtkApplication, ffi::GtkApplicationClass>): [
        gio::Application => gio_ffi::GApplication,
        gio::ActionGroup => gio_ffi::GActionGroup,
        gio::ActionMap => gio_ffi::GActionMap,
    ];

    match fn {
        get_type => || ffi::gtk_application_get_type(),
    }
}

pub trait GtkApplicationExt {
    fn add_accelerator<'a, P: Into<Option<&'a glib::Variant>>>(&self, accelerator: &str, action_name: &str, parameter: P);

    fn add_window<P: IsA<Window>>(&self, window: &P);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_accels_for_action(&self, detailed_action_name: &str) -> Vec<String>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_actions_for_accel(&self, accel: &str) -> Vec<String>;

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_active_window(&self) -> Option<Window>;

    fn get_app_menu(&self) -> Option<gio::MenuModel>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_menu_by_id(&self, id: &str) -> Option<gio::Menu>;

    fn get_menubar(&self) -> Option<gio::MenuModel>;

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_window_by_id(&self, id: u32) -> Option<Window>;

    fn get_windows(&self) -> Vec<Window>;

    fn inhibit<'a, 'b, P: IsA<Window> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b str>>>(&self, window: Q, flags: ApplicationInhibitFlags, reason: R) -> u32;

    fn is_inhibited(&self, flags: ApplicationInhibitFlags) -> bool;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn list_action_descriptions(&self) -> Vec<String>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn prefers_app_menu(&self) -> bool;

    fn remove_accelerator<'a, P: Into<Option<&'a glib::Variant>>>(&self, action_name: &str, parameter: P);

    fn remove_window<P: IsA<Window>>(&self, window: &P);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_accels_for_action(&self, detailed_action_name: &str, accels: &[&str]);

    fn set_app_menu<'a, P: IsA<gio::MenuModel> + 'a, Q: Into<Option<&'a P>>>(&self, app_menu: Q);

    fn set_menubar<'a, P: IsA<gio::MenuModel> + 'a, Q: Into<Option<&'a P>>>(&self, menubar: Q);

    fn uninhibit(&self, cookie: u32);

    fn get_property_active_window(&self) -> Option<Window>;

    fn get_property_register_session(&self) -> bool;

    fn set_property_register_session(&self, register_session: bool);

    fn connect_window_added<F: Fn(&Self, &Window) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_window_removed<F: Fn(&Self, &Window) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_active_window_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_app_menu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_menubar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_register_session_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Application> + IsA<glib::object::Object>> GtkApplicationExt for O {
    fn add_accelerator<'a, P: Into<Option<&'a glib::Variant>>>(&self, accelerator: &str, action_name: &str, parameter: P) {
        let parameter = parameter.into();
        let parameter = parameter.to_glib_none();
        unsafe {
            ffi::gtk_application_add_accelerator(self.to_glib_none().0, accelerator.to_glib_none().0, action_name.to_glib_none().0, parameter.0);
        }
    }

    fn add_window<P: IsA<Window>>(&self, window: &P) {
        unsafe {
            ffi::gtk_application_add_window(self.to_glib_none().0, window.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_accels_for_action(&self, detailed_action_name: &str) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_application_get_accels_for_action(self.to_glib_none().0, detailed_action_name.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_actions_for_accel(&self, accel: &str) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_application_get_actions_for_accel(self.to_glib_none().0, accel.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_active_window(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_active_window(self.to_glib_none().0))
        }
    }

    fn get_app_menu(&self) -> Option<gio::MenuModel> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_app_menu(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_menu_by_id(&self, id: &str) -> Option<gio::Menu> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_menu_by_id(self.to_glib_none().0, id.to_glib_none().0))
        }
    }

    fn get_menubar(&self) -> Option<gio::MenuModel> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_menubar(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_6", feature = "dox"))]
    fn get_window_by_id(&self, id: u32) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_window_by_id(self.to_glib_none().0, id))
        }
    }

    fn get_windows(&self) -> Vec<Window> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_application_get_windows(self.to_glib_none().0))
        }
    }

    fn inhibit<'a, 'b, P: IsA<Window> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b str>>>(&self, window: Q, flags: ApplicationInhibitFlags, reason: R) -> u32 {
        let window = window.into();
        let window = window.to_glib_none();
        let reason = reason.into();
        let reason = reason.to_glib_none();
        unsafe {
            ffi::gtk_application_inhibit(self.to_glib_none().0, window.0, flags.to_glib(), reason.0)
        }
    }

    fn is_inhibited(&self, flags: ApplicationInhibitFlags) -> bool {
        unsafe {
            from_glib(ffi::gtk_application_is_inhibited(self.to_glib_none().0, flags.to_glib()))
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn list_action_descriptions(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_application_list_action_descriptions(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn prefers_app_menu(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_application_prefers_app_menu(self.to_glib_none().0))
        }
    }

    fn remove_accelerator<'a, P: Into<Option<&'a glib::Variant>>>(&self, action_name: &str, parameter: P) {
        let parameter = parameter.into();
        let parameter = parameter.to_glib_none();
        unsafe {
            ffi::gtk_application_remove_accelerator(self.to_glib_none().0, action_name.to_glib_none().0, parameter.0);
        }
    }

    fn remove_window<P: IsA<Window>>(&self, window: &P) {
        unsafe {
            ffi::gtk_application_remove_window(self.to_glib_none().0, window.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_accels_for_action(&self, detailed_action_name: &str, accels: &[&str]) {
        unsafe {
            ffi::gtk_application_set_accels_for_action(self.to_glib_none().0, detailed_action_name.to_glib_none().0, accels.to_glib_none().0);
        }
    }

    fn set_app_menu<'a, P: IsA<gio::MenuModel> + 'a, Q: Into<Option<&'a P>>>(&self, app_menu: Q) {
        let app_menu = app_menu.into();
        let app_menu = app_menu.to_glib_none();
        unsafe {
            ffi::gtk_application_set_app_menu(self.to_glib_none().0, app_menu.0);
        }
    }

    fn set_menubar<'a, P: IsA<gio::MenuModel> + 'a, Q: Into<Option<&'a P>>>(&self, menubar: Q) {
        let menubar = menubar.into();
        let menubar = menubar.to_glib_none();
        unsafe {
            ffi::gtk_application_set_menubar(self.to_glib_none().0, menubar.0);
        }
    }

    fn uninhibit(&self, cookie: u32) {
        unsafe {
            ffi::gtk_application_uninhibit(self.to_glib_none().0, cookie);
        }
    }

    fn get_property_active_window(&self) -> Option<Window> {
        unsafe {
            let mut value = Value::from_type(<Window as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "active-window".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_register_session(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "register-session".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_register_session(&self, register_session: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "register-session".to_glib_none().0, Value::from(&register_session).to_glib_none().0);
        }
    }

    fn connect_window_added<F: Fn(&Self, &Window) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Window) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "window-added",
                transmute(window_added_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_window_removed<F: Fn(&Self, &Window) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Window) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "window-removed",
                transmute(window_removed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_active_window_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::active-window",
                transmute(notify_active_window_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_app_menu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::app-menu",
                transmute(notify_app_menu_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_menubar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::menubar",
                transmute(notify_menubar_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_register_session_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::register-session",
                transmute(notify_register_session_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn window_added_trampoline<P>(this: *mut ffi::GtkApplication, window: *mut ffi::GtkWindow, f: glib_ffi::gpointer)
where P: IsA<Application> {
    callback_guard!();
    let f: &&(Fn(&P, &Window) + 'static) = transmute(f);
    f(&Application::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(window))
}

unsafe extern "C" fn window_removed_trampoline<P>(this: *mut ffi::GtkApplication, window: *mut ffi::GtkWindow, f: glib_ffi::gpointer)
where P: IsA<Application> {
    callback_guard!();
    let f: &&(Fn(&P, &Window) + 'static) = transmute(f);
    f(&Application::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(window))
}

unsafe extern "C" fn notify_active_window_trampoline<P>(this: *mut ffi::GtkApplication, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Application> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Application::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_app_menu_trampoline<P>(this: *mut ffi::GtkApplication, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Application> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Application::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_menubar_trampoline<P>(this: *mut ffi::GtkApplication, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Application> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Application::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_register_session_trampoline<P>(this: *mut ffi::GtkApplication, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Application> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Application::from_glib_borrow(this).downcast_unchecked())
}
