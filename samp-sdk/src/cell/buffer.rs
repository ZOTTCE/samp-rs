use std::ops::{Deref, DerefMut};

use super::{Ref, Cell};
use crate::amx::Amx;
use crate::error::AmxResult;

pub struct Buffer<'amx> {
    inner: Ref<'amx, i32>,
    len: usize,
}

impl<'amx> Buffer<'amx> {
    pub fn new(reference: Ref<'amx, i32>, len: usize) -> Buffer<'amx> {
        Buffer {
            inner: reference,
            len,
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[i32] {
        unsafe {
            std::slice::from_raw_parts(self.inner.as_ptr(), self.len)
        }
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [i32] {
        unsafe {
            std::slice::from_raw_parts_mut(self.inner.as_mut_ptr(), self.len)
        }
    }
}

// Buffer cannot be parsed
impl<'amx> Cell<'amx> for Buffer<'amx> {
    #[inline]
    fn as_cell(&self) -> i32 {
        self.inner.as_cell()
    }
}

impl Deref for Buffer<'_> {
    type Target = [i32];
    
    fn deref(&self) -> &[i32] {
        self.as_slice()
    }
}

impl DerefMut for Buffer<'_> {
    fn deref_mut(&mut self) -> &mut [i32] {
        self.as_mut_slice()
    }
}

/// It's more like a temorary buffer that comes from AMX when a native is calling.
/// 
/// #Example
/// ```
/// fn null_my_array(amx: &Amx, array: UnsizedBuffer, length: usize) -> AmxResult<u32> {
///     let array = array.into_sized_buffer(length);
/// 
///     unsafe {
///         let slice = array.as_mut_slice();
///         std::ptr::write_bytes(slice.as_mut_ptr(), 0, length);
///     }
/// 
///     return Ok(1)
/// }
/// ```
pub struct UnsizedBuffer<'amx> {
    inner: Ref<'amx, i32>,
}

impl<'amx> UnsizedBuffer<'amx> {
    /// Convert `UnsizedBuffer` into `Buffer` with given length.
    ///
    /// #Example
    /// fn push_ones(amx: &Amx, unsized: UnsizedBuffer, length: usize) {
    ///     let mut buffer = unsized.into_sized_buffer(length);
    ///     let slice = buffer.as_mut_slice();
    ///     
    ///     for item in slice.iter_mut() {
    ///         *item = 1;
    ///     }
    /// }
    pub fn into_sized_buffer(self, len: usize) -> Buffer<'amx> {
        Buffer::new(self.inner, len)
    }

    #[inline]
    pub fn as_ptr(&self) -> *const i32 {
        self.inner.as_ptr()
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut i32 {
        self.inner.as_mut_ptr()
    }
}

impl<'amx> Cell<'amx> for UnsizedBuffer<'amx> {
    fn from_raw(amx: &'amx Amx, cell: i32) -> AmxResult<UnsizedBuffer<'amx>> {
        Ok(UnsizedBuffer {
            inner: amx.get_ref(cell)?
        })
    }

    #[inline]
    fn as_cell(&self) -> i32 {
        self.inner.as_cell()
    }
}