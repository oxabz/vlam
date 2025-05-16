use core::pin::Pin;
use crate::VLACtx;

/// Type describing an array stored in the stack
pub struct VLArray<'ctx, T>{
    pub(crate) ptr: *mut T,
    pub(crate) len: usize,
    pub(crate) _ctx: &'ctx Pin<&'ctx mut VLACtx>,
}

impl<'ctx, T> core::ops::Deref for  VLArray<'ctx, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<'ctx, T> core::ops::DerefMut for VLArray<'ctx, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl<'ctx, T> Drop for VLArray<'ctx, T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.len {
                core::ptr::drop_in_place(self.ptr.offset(i as isize));
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
