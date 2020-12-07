// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::translate::*;
use crate::value::FromValue;
use crate::value::FromValueOptional;
use crate::value::SetValue;
use crate::StaticType;
use crate::Type;
use bitflags::bitflags;
use std::fmt;

bitflags! {
    pub struct BindingFlags: u32 {
        const DEFAULT = 0;
        const BIDIRECTIONAL = 1;
        const SYNC_CREATE = 2;
        const INVERT_BOOLEAN = 4;
    }
}

impl fmt::Display for BindingFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl ToGlib for BindingFlags {
    type GlibType = gobject_ffi::GBindingFlags;

    fn to_glib(&self) -> gobject_ffi::GBindingFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gobject_ffi::GBindingFlags> for BindingFlags {
    unsafe fn from_glib(value: gobject_ffi::GBindingFlags) -> BindingFlags {
        BindingFlags::from_bits_truncate(value)
    }
}

impl StaticType for BindingFlags {
    fn static_type() -> Type {
        unsafe { from_glib(gobject_ffi::g_binding_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for BindingFlags {
    unsafe fn from_value_optional(value: &crate::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for BindingFlags {
    unsafe fn from_value(value: &crate::Value) -> Self {
        from_glib(crate::gobject_ffi::g_value_get_flags(
            value.to_glib_none().0,
        ))
    }
}

impl SetValue for BindingFlags {
    unsafe fn set_value(value: &mut crate::Value, this: &Self) {
        crate::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct ParamFlags: u32 {
        const READABLE = 1;
        const WRITABLE = 2;
        const READWRITE = 3;
        const CONSTRUCT = 4;
        const CONSTRUCT_ONLY = 8;
        const LAX_VALIDATION = 16;
        const STATIC_NAME = 32;
        const PRIVATE = 32;
        const STATIC_NICK = 64;
        const STATIC_BLURB = 128;
        const EXPLICIT_NOTIFY = 1073741824;
        const DEPRECATED = 2147483648;
    }
}

impl fmt::Display for ParamFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl ToGlib for ParamFlags {
    type GlibType = gobject_ffi::GParamFlags;

    fn to_glib(&self) -> gobject_ffi::GParamFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gobject_ffi::GParamFlags> for ParamFlags {
    unsafe fn from_glib(value: gobject_ffi::GParamFlags) -> ParamFlags {
        ParamFlags::from_bits_truncate(value)
    }
}

bitflags! {
    pub struct SignalFlags: u32 {
        const RUN_FIRST = 1;
        const RUN_LAST = 2;
        const RUN_CLEANUP = 4;
        const NO_RECURSE = 8;
        const DETAILED = 16;
        const ACTION = 32;
        const NO_HOOKS = 64;
        const MUST_COLLECT = 128;
        const DEPRECATED = 256;
    }
}

impl fmt::Display for SignalFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl ToGlib for SignalFlags {
    type GlibType = gobject_ffi::GSignalFlags;

    fn to_glib(&self) -> gobject_ffi::GSignalFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gobject_ffi::GSignalFlags> for SignalFlags {
    unsafe fn from_glib(value: gobject_ffi::GSignalFlags) -> SignalFlags {
        SignalFlags::from_bits_truncate(value)
    }
}
