use std::os::raw::c_char;

use array_list::{Arguments, ArrayList};
use class::Class;

macro_rules! cstring {
    ($e:expr) => {
        $e as *const u8 as *const std::os::raw::c_char
    };
}

pub mod array_list;
pub mod class;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct Object {
    pub ptr: *mut std::ffi::c_void,
    pub strong_count: *mut usize,
    pub class: *const Class,
    pub vtable: *const VTable,
}

impl Object {
    pub fn class(&self) -> &Class {
        if self.class.is_null() {
            return &ANY_CLASS;
        }

        unsafe { &*self.class }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct VTable {
    pub apply: unsafe extern "C" fn(Object, Arguments) -> Object,
    pub to_string: unsafe extern "C" fn(Object, Arguments) -> *const c_char,
}

pub const ANY_CLASS: Class = Class {
    name: cstring!(b"Any"),
    superclasses: ArrayList::empty(),
};
