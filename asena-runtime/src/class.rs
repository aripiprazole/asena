use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

use crate::{
    array_list::{Arguments, ArrayList},
    Object, VTable,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct Class {
    pub name: *const c_char,
    pub superclasses: ArrayList<Class>,
}

impl Class {
    #[inline]
    #[export_name = "_ZClass::new_instance"]
    pub fn new_instance(&self) -> *const Object {
        Box::leak(Box::new(Object {
            ptr: std::ptr::null_mut(),
            strong_count: Box::leak(Box::new(1)),
            class: self,
            vtable: Box::leak(Box::new(VTable {
                apply: def_class_apply,
                to_string: def_class_to_string,
            })),
        }))
    }

    #[inline]
    pub fn name(&self) -> String {
        CString::from(unsafe { CStr::from_ptr(self.name) })
            .to_str()
            .unwrap()
            .to_string()
    }
}

extern "C" fn def_class_apply(this: Object, _arguments: Arguments) -> Object {
    let name = this.class().name();

    panic!("apply is not implemented to type {name}")
}

extern "C" fn def_class_to_string(this: Object, _arguments: Arguments) -> *const c_char {
    let class = this.class();

    class.name
}
