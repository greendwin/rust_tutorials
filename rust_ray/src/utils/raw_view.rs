use std::mem;
use std::slice;

pub trait RawView {
    fn raw_view(&self) -> &[u8];
}

pub unsafe trait RawStruct {}

impl<T> RawView for T
where
    T: RawStruct,
{
    fn raw_view(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self as *const Self as *const u8, mem::size_of::<Self>()) }
    }
}

impl<T> RawView for [T]
where
    T: RawView,
{
    fn raw_view(&self) -> &[u8] {
        let item_size = mem::size_of::<T>();
        unsafe { slice::from_raw_parts(self.as_ptr() as *const u8, item_size * self.len()) }
    }
}

unsafe impl RawStruct for u8 {}
