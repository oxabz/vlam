/*!
Module containing the context struct for the VLAs.

**Safety**
The context is in charge of restoring the stack frame before the end of the function.
This mean it only make sense in the function it is created in.
To insure it is not moved to another scope it should be pinned to the stack at all time. And every method operate on a Pin
*/

use core::marker::PhantomPinned;
use core::pin::Pin;
use core::ptr::{null, null_mut, write, write_bytes};
use crate::array::VLArray;

/// The minimum alignment of the arrays (due to stack pointer alignment)
const MIN_ALIGN : usize = crate::utils::max(align_of::<u128>(), crate::arch::STACK_ALIGNMENT);

/**
Context within which the variable length array are valid.
It save the initial stack pointer and restore it when dropped so that the array can push it to allocate themselves.

**Safety**
As it is related to the stack pointer of the function, it should not be moved therefore all of its function are gated behind a stack pin
Do not construct it through the init function unless you understand what you are doing.
*/
pub struct VLACtx{
    saved_sp: *const u8,
    // Should be pinned because of the raw pointer but double it just to be sure
    _phantom_pinned: PhantomPinned,
}


impl VLACtx{
    /**
     Create the context for the VLA.

     **Note** Unless you know what you are doing use the `vlam::ctx!` macro

     # Safety
     The function is unsafe because it produces a VLACtx that is not protected by a pin
     which means it can be moved outside the scope that makes it valid. Moving VLA will lead to UB.
     */
    #[inline(always)]
    pub unsafe fn init() -> Self{
        #[allow(unused_assignments)]
        let mut saved_sp = null();

        // SAFETY : This only save a frame pointer
        unsafe {
            crate::arch::save_stack_pointer!(saved_sp);
        };

        VLACtx{saved_sp, _phantom_pinned: Default::default() }
    }

    /**
    Allocates the space for len bytes (not necessarily exactly len)

    ** Safety :** This should only be within inside a function that already has a context initialized.
        The pointer produced should not be used outside the function this function was called in
     */
    #[inline(always)]
    unsafe fn allocate_buffer(size: usize) -> *mut u8 {
        #[allow(unused_assignments)]
        let mut origin: *mut u8 = null_mut();
        let blocks = size.div_ceil(MIN_ALIGN);
        let size = blocks * MIN_ALIGN;
        // Push the stack down to allow space for the
        // SAFETY : This is only ok because we save the initial state of the stack pointer before changing it
        // And we will release it
        unsafe{
            crate::arch::allocate_on_stack!(origin, size);
            origin.sub(size)
        }
    }

    /**
     * Create a buffer of `len` bytes initialized at 0
     */
    #[inline(always)]
    pub fn zeroed_buffer<'ctx>(self: &'ctx Pin<&'ctx mut Self>, len:usize) -> VLArray<'ctx, u8> {
        let size = len * size_of::<u8>();
        let origin = unsafe {
            Self::allocate_buffer(size)
        };

        unsafe { write_bytes(origin, 0x00, size) }

        VLArray{
            ptr:origin,
            len,
            _ctx: self,
        }
    }

    /**
     * Create a buffer of `len` bytes initialized by cloning a value
     */
    #[inline(always)]
    pub fn array_from_clone<'ctx, T: Clone>(self: &'ctx Pin<&'ctx mut Self>, value: T, len:usize) -> VLArray<'ctx, T> {
        let size = len * size_of::<T>();
        let origin = unsafe {
            Self::allocate_buffer(size) as *mut T
        };

        for i in 0..len {
            unsafe {
                write(origin.add(i), value.clone());
            }
        }

        VLArray{
            ptr:origin,
            len,
            _ctx: self,
        }
    }

    /**
     * Create a buffer of `len` bytes initialized by cloning values from a slice
     */
    #[inline(always)]
    pub fn array_from_cloneable_slice<'ctx, T: Clone>(self: &'ctx Pin<&'ctx mut Self>, values: &[T]) -> VLArray<'ctx, T> {
        let len = values.len();
        let size = size_of_val(values);
        let origin = unsafe {
            Self::allocate_buffer(size) as *mut T
        };

        for (i,v) in values.iter().enumerate() {
            unsafe {
                write(origin.add(i), v.clone());
            }
        }

        VLArray{
            ptr:origin,
            len,
            _ctx: self,
        }
    }

    /**
     * Create a buffer of `len` bytes initialized by cloning a value
     */
    #[inline(always)]
    pub fn array_from_exact_size_iterator<'ctx, T, I, IT>(self: &'ctx Pin<&'ctx mut Self>, it: I) -> VLArray<'ctx, T>
    where
        I: IntoIterator<Item = T, IntoIter = IT>,
        IT: ExactSizeIterator<Item = T>
    {
        let it  = it.into_iter();
        let len = it.len();
        let size = len * size_of::<T>();
        let origin = unsafe {
            Self::allocate_buffer(size) as *mut T
        };

        for (i,v) in it.enumerate() {
            unsafe {
                write(origin.add(i), v);
            }
        }

        VLArray{
            ptr:origin,
            len,
            _ctx: self,
        }
    }



}

impl Drop for VLACtx{
    #[inline(always)]
    fn drop(&mut self) {
        let saved_sp  = self.saved_sp;
        // SAFETY: Restores the stack pointer.
        // This is only safe because the type is pinned to the stack at all time.
        unsafe {
            crate::arch::restore_stack_pointer!(saved_sp);
        };
    }
}
