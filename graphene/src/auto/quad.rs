// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Point;
use crate::Rect;
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug)]
    pub struct Quad(BoxedInline<ffi::graphene_quad_t>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::graphene_quad_get_type(), ptr as *mut _) as *mut ffi::graphene_quad_t,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::graphene_quad_get_type(), ptr as *mut _),
        type_ => || ffi::graphene_quad_get_type(),
    }
}

impl Quad {
    #[doc(alias = "graphene_quad_bounds")]
    pub fn bounds(&self) -> Rect {
        unsafe {
            let mut r = Rect::uninitialized();
            ffi::graphene_quad_bounds(self.to_glib_none().0, r.to_glib_none_mut().0);
            r
        }
    }

    #[doc(alias = "graphene_quad_contains")]
    pub fn contains(&self, p: &Point) -> bool {
        unsafe { ffi::graphene_quad_contains(self.to_glib_none().0, p.to_glib_none().0) }
    }
}
