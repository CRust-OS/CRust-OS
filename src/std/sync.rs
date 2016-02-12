use core::cell::UnsafeCell;

pub struct RwLock<T: ?Sized> {
    data: UnsafeCell<T>,
}

unsafe impl<T: ?Sized + Send + Sync> Send for RwLock<T> {}
unsafe impl<T: ?Sized + Send + Sync> Sync for RwLock<T> {}

impl<T> RwLock<T> {
    pub const fn new(t: T) -> RwLock<T> {
        RwLock { data: UnsafeCell::new(t) }
    }
}

impl<T: ?Sized> RwLock<T> {
    pub fn read(&self) -> &T {
        unsafe {
            &*self.data.get()
        }
    }

    pub fn write(&self) -> &mut T {
        unsafe {
            &mut *self.data.get()
        }
    }
}
