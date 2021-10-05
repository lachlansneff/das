use std::{cmp, marker::{PhantomData, Unsize}, ops::{CoerceUnsized, Deref, DispatchFromDyn}, ptr::NonNull, sync::atomic::{self, AtomicUsize, Ordering}, fmt::Debug};

use crate::basic::Basic;



struct Inner<T: ?Sized> {
    count: AtomicUsize,
    data: T,
}

pub struct Expr<T: ?Sized + Basic = dyn Basic> {
    ptr: NonNull<Inner<T>>,
    _marker: PhantomData<Inner<T>>,
}

impl<T: Basic> Expr<T> {
    pub fn new(data: T) -> Self {
        Self {
            ptr: NonNull::from(Box::leak(Box::new(Inner {
                count: AtomicUsize::new(1),
                data,
            }))),
            _marker: PhantomData,
        }
    }
}

impl<T: ?Sized + Basic> Expr<T> {
    #[inline]
    fn inner(&self) -> &Inner<T> {
        unsafe {
            self.ptr.as_ref()
        }
    }

    unsafe fn get_mut_unchecked(&mut self) -> &mut T {
        unsafe {
            &mut self.ptr.as_mut().data
        }
    }

    #[inline(never)]
    unsafe fn drop_slow(&mut self) {
        unsafe {
            drop(Box::from_raw(self.ptr.as_ptr()));
        }
    }
}

unsafe impl<T: ?Sized + Basic + Send + Sync> Send for Expr<T> {}
unsafe impl<T: ?Sized + Basic + Send + Sync> Sync for Expr<T> {}

impl<T: ?Sized + Basic + Unsize<U>, U: ?Sized + Basic> CoerceUnsized<Expr<U>> for Expr<T> {}
impl<T: ?Sized + Basic + Unsize<U>, U: ?Sized + Basic> DispatchFromDyn<Expr<U>> for Expr<T> {}


impl<T: Basic + Clone> Expr<T> {
    #[inline]
    pub fn make_mut(&mut self) -> &mut T {
        if self.inner().count.load(Ordering::Acquire) != 1 {
            // Another pointer exists, so we must clone.
            let data = self.inner().data.clone();
            *self = Self::new(data);
        }

        unsafe {
            self.get_mut_unchecked()
        }
    }
}

impl<T: ?Sized + Basic> Clone for Expr<T> {
    fn clone(&self) -> Self {
        self.inner().count.fetch_add(1, Ordering::Relaxed);

        Self { ..*self }
    }
}

impl<T: ?Sized + Basic> Deref for Expr<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner().data
    }
}

impl<T: ?Sized + Basic> Drop for Expr<T> {
    fn drop(&mut self) {
        if self.inner().count.fetch_sub(1, Ordering::Release) != 1 {
            return;
        }

        atomic::fence(Ordering::Acquire);

        unsafe { self.drop_slow() }
    }
}

impl<T: ?Sized + Basic + PartialEq> PartialEq for Expr<T> {
    fn eq(&self, other: &Self) -> bool {
        <T as PartialEq>::eq(&*self, &*other)
    }

    fn ne(&self, other: &Self) -> bool {
        T::ne(&*self, &*other)
    }
}

impl<T: ?Sized + Basic + Eq> Eq for Expr<T> {}

impl<T: ?Sized + Basic + PartialOrd> PartialOrd for Expr<T> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        T::partial_cmp(&*self, &*other)
    }

    fn lt(&self, other: &Self) -> bool {
        T::lt(&*self, &*other)
    }

    fn le(&self, other: &Self) -> bool {
        T::le(&*self, &*other)
    }

    fn gt(&self, other: &Self) -> bool {
        T::gt(&*self, &*other)
    }

    fn ge(&self, other: &Self) -> bool {
        T::ge(&*self, &*other)
    }
}

impl<T: ?Sized + Basic + Ord> Ord for Expr<T> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        <T as Ord>::cmp(&*self, &*other)
    }
}

impl<T: ?Sized + Basic + Debug> Debug for Expr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <T as Debug>::fmt(&*self, f)
    }
}
