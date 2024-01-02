use std::{
    marker::PhantomData,
    ptr::NonNull,
    cell::Cell,
    sync::Mutex,
    ops::{ Deref, DerefMut },
    fmt::Debug,
};

use crate::{ Result, RuntimeError };

use libimmixcons::{
    object::{ HeapObject, GCRTTI, Gc, RawGc },
    GCObject,
    immix_alloc,
    threading::immix_register_thread,
    immix_alloc_safe,
};

#[derive(Clone, Copy)]
pub struct Guard<'a>(pub(crate) PhantomData<&'a ()>);

pub struct GuardedHeap<'a>(Guard<'a>);

impl<'a> GuardedHeap<'a> {
    pub(crate) fn new(guard: Guard<'a>) -> Self {
        Self(guard)
    }

    pub fn allocate(&self, size: usize, rtti: &GCRTTI) -> Result<NonNull<GCObject>> {
        let ptr = immix_alloc(size, rtti);

        NonNull::new(ptr).ok_or(RuntimeError::AllocationFailure)
    }

    pub fn allocate_single<T: HeapObject>(&self, value: T) -> Result<GuardedCell<T>> {
        let ptr = self.allocate(value.heap_size(), &T::RTTI)?;

        unsafe {
            ptr.cast::<RawGc>().as_ref().data().cast::<T>().write(value);
            Ok(
                GuardedCell::new(Gc {
                    ptr: ptr.cast(),
                    marker: PhantomData,
                })
            )
        }
    }
}

pub struct GuardedCell<T: HeapObject>(Gc<T>);

impl<T: HeapObject> std::hash::Hash for GuardedCell<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.ptr.hash(state)
    }
}

impl<T: HeapObject> Debug for GuardedCell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0.ptr))
    }
}

impl<T: HeapObject> PartialEq<GuardedCell<T>> for GuardedCell<T> {
    fn eq(&self, other: &GuardedCell<T>) -> bool {
        self.0.ptr == other.0.ptr
    }
}

impl<T: HeapObject> GuardedCell<T> {
    pub fn new(inner: Gc<T>) -> Self {
        Self(inner)
    }

    pub fn get<'a>(&self, guard: Guard<'a>) -> GuardedPtr<'a, T> {
        GuardedPtr(self.0, guard)
    }
}
impl<T: HeapObject> Clone for GuardedCell<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: HeapObject> Copy for GuardedCell<T> {}

#[derive(Clone, Copy)]
pub struct GuardedPtr<'a, T: HeapObject>(Gc<T>, Guard<'a>);

impl<'a, T: HeapObject> Deref for GuardedPtr<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<'a, T: HeapObject> DerefMut for GuardedPtr<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}
