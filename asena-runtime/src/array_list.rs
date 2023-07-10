use crate::Object;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct ArrayList<T> {
    pub length: usize,
    pub arguments: *const *const T,
}

impl<T> ArrayList<T> {
    pub const fn empty() -> Self {
        Self {
            length: 0,
            arguments: std::ptr::null(),
        }
    }

    pub const fn get(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            return None;
        }

        unsafe { Some(&*(*self.arguments.add(index))) }
    }
}

impl<T> Default for ArrayList<T> {
    fn default() -> Self {
        Self {
            length: Default::default(),
            arguments: std::ptr::null(),
        }
    }
}

pub type Arguments = ArrayList<Object>;
