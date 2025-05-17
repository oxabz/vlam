use core::pin::Pin;
use crate::VLACtx;

/// Type describing an array stored in the stack
pub struct VLArray<'ctx, T>{
    pub(crate) ptr: *mut T,
    pub(crate) len: usize,
    pub(crate) _ctx: &'ctx Pin<&'ctx mut VLACtx>,
}

impl<T> core::ops::Deref for  VLArray<'_, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<T> core::ops::DerefMut for VLArray<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl<T> Drop for VLArray<'_, T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.len {
                core::ptr::drop_in_place(self.ptr.add(i));
            }
        };
    }
}

impl <'ctx, 'a, T:'a> core::iter::IntoIterator for &'a VLArray<'ctx, T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl <'ctx, 'a, T:'a> core::iter::IntoIterator for &'a mut VLArray<'ctx, T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
