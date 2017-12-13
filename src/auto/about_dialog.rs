// This file was generated by gir (13f739b) from gir-files (db49619)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use Dialog;
use License;
use Widget;
use Window;
use ffi;
use gdk_pixbuf;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use signal::Inhibit;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct AboutDialog(Object<ffi::GtkAboutDialog, ffi::GtkAboutDialogClass>): Dialog, Window, Bin, Container, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_about_dialog_get_type(),
    }
}

impl AboutDialog {
    pub fn new() -> AboutDialog {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_about_dialog_new()).downcast_unchecked()
        }
    }
}

impl Default for AboutDialog {
    fn default() -> Self {
        Self::new()
    }
}

pub trait AboutDialogExt {
    fn add_credit_section(&self, section_name: &str, people: &[&str]);

    fn get_artists(&self) -> Vec<String>;

    fn get_authors(&self) -> Vec<String>;

    fn get_comments(&self) -> Option<String>;

    fn get_copyright(&self) -> Option<String>;

    fn get_documenters(&self) -> Vec<String>;

    fn get_license(&self) -> Option<String>;

    fn get_license_type(&self) -> License;

    fn get_logo(&self) -> Option<gdk_pixbuf::Pixbuf>;

    fn get_logo_icon_name(&self) -> Option<String>;

    fn get_program_name(&self) -> Option<String>;

    fn get_translator_credits(&self) -> Option<String>;

    fn get_version(&self) -> Option<String>;

    fn get_website(&self) -> Option<String>;

    fn get_website_label(&self) -> Option<String>;

    fn get_wrap_license(&self) -> bool;

    fn set_artists(&self, artists: &[&str]);

    fn set_authors(&self, authors: &[&str]);

    fn set_comments<'a, P: Into<Option<&'a str>>>(&self, comments: P);

    fn set_copyright<'a, P: Into<Option<&'a str>>>(&self, copyright: P);

    fn set_documenters(&self, documenters: &[&str]);

    fn set_license<'a, P: Into<Option<&'a str>>>(&self, license: P);

    fn set_license_type(&self, license_type: License);

    fn set_logo<'a, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>>(&self, logo: P);

    fn set_logo_icon_name<'a, P: Into<Option<&'a str>>>(&self, icon_name: P);

    fn set_program_name(&self, name: &str);

    fn set_translator_credits<'a, P: Into<Option<&'a str>>>(&self, translator_credits: P);

    fn set_version<'a, P: Into<Option<&'a str>>>(&self, version: P);

    fn set_website<'a, P: Into<Option<&'a str>>>(&self, website: P);

    fn set_website_label<'a, P: Into<Option<&'a str>>>(&self, website_label: P);

    fn set_wrap_license(&self, wrap_license: bool);

    fn connect_activate_link<F: Fn(&Self, &str) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_artists_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_authors_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_comments_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_copyright_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_documenters_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_license_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_license_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_logo_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_logo_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_program_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_translator_credits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_version_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_website_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_website_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wrap_license_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<AboutDialog> + IsA<glib::object::Object>> AboutDialogExt for O {
    fn add_credit_section(&self, section_name: &str, people: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_add_credit_section(self.to_glib_none().0, section_name.to_glib_none().0, people.to_glib_none().0);
        }
    }

    fn get_artists(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_about_dialog_get_artists(self.to_glib_none().0))
        }
    }

    fn get_authors(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_about_dialog_get_authors(self.to_glib_none().0))
        }
    }

    fn get_comments(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_comments(self.to_glib_none().0))
        }
    }

    fn get_copyright(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_copyright(self.to_glib_none().0))
        }
    }

    fn get_documenters(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_about_dialog_get_documenters(self.to_glib_none().0))
        }
    }

    fn get_license(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_license(self.to_glib_none().0))
        }
    }

    fn get_license_type(&self) -> License {
        unsafe {
            from_glib(ffi::gtk_about_dialog_get_license_type(self.to_glib_none().0))
        }
    }

    fn get_logo(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_logo(self.to_glib_none().0))
        }
    }

    fn get_logo_icon_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_logo_icon_name(self.to_glib_none().0))
        }
    }

    fn get_program_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_program_name(self.to_glib_none().0))
        }
    }

    fn get_translator_credits(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_translator_credits(self.to_glib_none().0))
        }
    }

    fn get_version(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_version(self.to_glib_none().0))
        }
    }

    fn get_website(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_website(self.to_glib_none().0))
        }
    }

    fn get_website_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_website_label(self.to_glib_none().0))
        }
    }

    fn get_wrap_license(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_about_dialog_get_wrap_license(self.to_glib_none().0))
        }
    }

    fn set_artists(&self, artists: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_set_artists(self.to_glib_none().0, artists.to_glib_none().0);
        }
    }

    fn set_authors(&self, authors: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_set_authors(self.to_glib_none().0, authors.to_glib_none().0);
        }
    }

    fn set_comments<'a, P: Into<Option<&'a str>>>(&self, comments: P) {
        let comments = comments.into();
        let comments = comments.to_glib_none();
        unsafe {
            ffi::gtk_about_dialog_set_comments(self.to_glib_none().0, comments.0);
        }
    }

    fn set_copyright<'a, P: Into<Option<&'a str>>>(&self, copyright: P) {
        let copyright = copyright.into();
        let copyright = copyright.to_glib_none();
        unsafe {
            ffi::gtk_about_dialog_set_copyright(self.to_glib_none().0, copyright.0);
        }
    }

    fn set_documenters(&self, documenters: &[&str]) {
        unsafe {
            ffi::gtk_about_dialog_set_documenters(self.to_glib_none().0, documenters.to_glib_none().0);
        }
    }

    fn set_license<'a, P: Into<Option<&'a str>>>(&self, license: P) {
        let license = license.into();
        let license = license.to_glib_none();
        unsafe {
            ffi::gtk_about_dialog_set_license(self.to_glib_none().0, license.0);
        }
    }

    fn set_license_type(&self, license_type: License) {
        unsafe {
            ffi::gtk_about_dialog_set_license_type(self.to_glib_none().0, license_type.to_glib());
        }
    }

    fn set_logo<'a, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>>(&self, logo: P) {
        let logo = logo.into();
        let logo = logo.to_glib_none();
        unsafe {
            ffi::gtk_about_dialog_set_logo(self.to_glib_none().0, logo.0);
        }
    }

    fn set_logo_icon_name<'a, P: Into<Option<&'a str>>>(&self, icon_name: P) {
        let icon_name = icon_name.into();
        let icon_name = icon_name.to_glib_none();
        unsafe {
            ffi::gtk_about_dialog_set_logo_icon_name(self.to_glib_none().0, icon_name.0);
        }
    }

    fn set_program_name(&self, name: &str) {
        unsafe {
            ffi::gtk_about_dialog_set_program_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    fn set_translator_credits<'a, P: Into<Option<&'a str>>>(&self, translator_credits: P) {
        let translator_credits = translator_credits.into();
        let translator_credits = translator_credits.to_glib_none();
        unsafe {
            ffi::gtk_about_dialog_set_translator_credits(self.to_glib_none().0, translator_credits.0);
        }
    }

    fn set_version<'a, P: Into<Option<&'a str>>>(&self, version: P) {
        let version = version.into();
        let version = version.to_glib_none();
        unsafe {
            ffi::gtk_about_dialog_set_version(self.to_glib_none().0, version.0);
        }
    }

    fn set_website<'a, P: Into<Option<&'a str>>>(&self, website: P) {
        let website = website.into();
        let website = website.to_glib_none();
        unsafe {
            ffi::gtk_about_dialog_set_website(self.to_glib_none().0, website.0);
        }
    }

    fn set_website_label<'a, P: Into<Option<&'a str>>>(&self, website_label: P) {
        let website_label = website_label.into();
        let website_label = website_label.to_glib_none();
        unsafe {
            ffi::gtk_about_dialog_set_website_label(self.to_glib_none().0, website_label.0);
        }
    }

    fn set_wrap_license(&self, wrap_license: bool) {
        unsafe {
            ffi::gtk_about_dialog_set_wrap_license(self.to_glib_none().0, wrap_license.to_glib());
        }
    }

    fn connect_activate_link<F: Fn(&Self, &str) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str) -> Inhibit + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate-link",
                transmute(activate_link_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_artists_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::artists",
                transmute(notify_artists_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_authors_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::authors",
                transmute(notify_authors_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_comments_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::comments",
                transmute(notify_comments_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_copyright_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::copyright",
                transmute(notify_copyright_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_documenters_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::documenters",
                transmute(notify_documenters_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_license_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::license",
                transmute(notify_license_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_license_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::license-type",
                transmute(notify_license_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_logo_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::logo",
                transmute(notify_logo_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_logo_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::logo-icon-name",
                transmute(notify_logo_icon_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_program_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::program-name",
                transmute(notify_program_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_translator_credits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::translator-credits",
                transmute(notify_translator_credits_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_version_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::version",
                transmute(notify_version_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_website_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::website",
                transmute(notify_website_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_website_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::website-label",
                transmute(notify_website_label_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_wrap_license_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::wrap-license",
                transmute(notify_wrap_license_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_link_trampoline<P>(this: *mut ffi::GtkAboutDialog, uri: *mut libc::c_char, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<AboutDialog> {
    callback_guard!();
    let f: &&(Fn(&P, &str) -> Inhibit + 'static) = transmute(f);
    f(&AboutDialog::from_glib_borrow(this).downcast_unchecked(), &String::from_glib_none(uri)).to_glib()
}

unsafe extern "C" fn notify_artists_trampoline<P>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AboutDialog::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_authors_trampoline<P>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AboutDialog::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_comments_trampoline<P>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AboutDialog::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_copyright_trampoline<P>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AboutDialog::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_documenters_trampoline<P>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AboutDialog::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_license_trampoline<P>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AboutDialog::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_license_type_trampoline<P>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AboutDialog::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_logo_trampoline<P>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AboutDialog::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_logo_icon_name_trampoline<P>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AboutDialog::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_program_name_trampoline<P>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AboutDialog::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_translator_credits_trampoline<P>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AboutDialog::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_version_trampoline<P>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AboutDialog::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_website_trampoline<P>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AboutDialog::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_website_label_trampoline<P>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AboutDialog::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_wrap_license_trampoline<P>(this: *mut ffi::GtkAboutDialog, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AboutDialog> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AboutDialog::from_glib_borrow(this).downcast_unchecked())
}
