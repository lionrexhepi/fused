use core::panic;
use std::{ mem::size_of, ptr::{ NonNull, drop_in_place }, fmt::Debug };
use crate::{ Result, RuntimeError };
use libimmixcons::{ object::{ HeapObject, GCRTTI, RawGc, Gc }, immix_alloc, immix_noop_visit };

pub type ArrayCapacity = u32;

#[derive(Debug)]
#[repr(C)]
struct GCedArray<T: Debug> {
    capacity: ArrayCapacity,
    used: ArrayCapacity,

    items_ptr: Option<NonNull<T>>,
}

impl<T: Debug> Clone for GCedArray<T> {
    fn clone(&self) -> Self {
        Self {
            items_ptr: self.items_ptr.clone(),
            capacity: self.capacity,
            used: self.used,
        }
    }
}

impl<T: Debug> Copy for GCedArray<T> {}

impl<T: Debug> HeapObject for GCedArray<T> {
    const RTTI: GCRTTI = GCRTTI {
        heap_size: |data| unsafe {
            let this = &*data.add(0).cast::<Self>();
            println!("{:?}", this);

            size_of::<Self>() + (this.capacity as usize) * size_of::<T>()
        },
        visit_references: immix_noop_visit,
        needs_finalization: true,
        finalizer: Some(|data| unsafe {
            let this = &*(*(data as *mut RawGc)).data().cast::<Self>();
            println!("{:x}", data as usize);
            if let Some(ptr) = this.items_ptr {
                for i in 0..this.used {
                    println!("Dropping item {i}");
                    drop_in_place(ptr.offset(i as isize).as_ptr());
                }
            } else {
                panic!("Why no ptr");
            }
            drop_in_place(this as *const _ as *mut Self)
        }),
    };

    fn heap_size(&self) -> usize {
        core::mem::size_of_val(self) + size_of::<T>() * (self.capacity as usize)
    }

    fn needs_finalization(&self) -> bool {
        true
    }
}

impl<T: Debug> GCedArray<T> {
    pub fn with_capacity(capacity: ArrayCapacity) -> Result<Gc<Self>> {
        let item_size = size_of::<T>();
        let capacity_bytes = (capacity as usize)
            .checked_mul(item_size)
            .ok_or(RuntimeError::InvalidArrayCapacity(capacity))?;
        let ptr = immix_alloc(capacity_bytes + size_of::<Self>(), &Self::RTTI as *const GCRTTI);

        unsafe {
            let ptr = (*(ptr as *mut RawGc)).data().cast::<Self>();
            let items_ptr = ptr.add(size_of::<Self>()).cast();
            println!("{:x}", items_ptr as usize);
            let result = Self {
                capacity,
                used: 0,
                items_ptr: Some(NonNull::new_unchecked(items_ptr)),
            };
            ptr.cast::<Self>().write(result);
            Ok(Gc::from_raw(ptr))
        }
    }

    fn resize(&mut self, new_capacity: ArrayCapacity) -> Result<()> {
        if new_capacity == 0 {
            self.items_ptr = None;
            self.capacity = 0;
            self.used = 0;
            return Ok(());
        }
        let item_size = size_of::<T>();
        let old_capacity_bytes = (self.capacity as usize)
            .checked_mul(item_size)
            .ok_or(RuntimeError::InvalidArrayCapacity(self.capacity))?;
        let new_capacity_bytes = (new_capacity as usize)
            .checked_mul(item_size)
            .ok_or(RuntimeError::InvalidArrayCapacity(new_capacity))?;
        if let Some(old_ptr) = self.items_ptr {
            let new_ptr = immix_alloc(new_capacity_bytes, &Self::RTTI as *const GCRTTI);
            unsafe {
                let new_ptr = (*(new_ptr as *mut RawGc)).data();
                std::ptr::copy_nonoverlapping(old_ptr.as_ptr(), new_ptr.cast(), old_capacity_bytes);
            }
        } else {
            *self = *Self::with_capacity(new_capacity)?;
        }

        Ok(())
    }

    pub fn push(&mut self, item: T) {
        let ptr = if let Some(ptr) = self.items_ptr {
            ptr
        } else {
            println!("Resizing");
            self.resize(growth(self.capacity)).expect("Resize to work");
            self.items_ptr.unwrap()
        };

        unsafe {
            ptr.offset(self.used as isize).write(item);
        }

        println!("pointer of push: {:x}", self as *mut _ as usize);
        self.used += 1;
    }

    pub fn inner(&self) -> Option<*const T> {
        self.items_ptr.map(|ptr| unsafe { ptr.as_ref() as *const T })
    }

    pub fn inner_mut(&mut self) -> Option<*mut T> {
        self.items_ptr.map(|ptr| unsafe { ptr.as_ref() as *const T as *mut T })
    }

    pub const fn new() -> Self {
        Self {
            capacity: 0,
            used: 0,
            items_ptr: None,
        }
    }
}

const DEFAULT_ARRAY_SIZE: ArrayCapacity = 8;

///Given an old size len, returns the new size to grow to. Default is a factor of 1.5
pub const fn growth(len: ArrayCapacity) -> ArrayCapacity {
    if len < DEFAULT_ARRAY_SIZE { DEFAULT_ARRAY_SIZE } else { len + (len >> 1) }
}

#[cfg(test)]
mod test {
    use libimmixcons::{
        immix_init,
        immix_noop_callback,
        threading::{ immix_register_thread, immix_unregister_thread },
        immix_init_logger,
        immix_collect,
    };

    #[derive(Debug)]
    struct CustomValue;

    impl Drop for CustomValue {
        fn drop(&mut self) {
            println!("Dropped :(")
        }
    }

    #[test]
    fn test_finalize() {
        immix_init(0, 0, immix_noop_callback, 0 as *mut _);
        immix_init_logger();
        immix_register_thread();
        immix_collect(true);
        {
            let mut arr = super::GCedArray::<CustomValue>::with_capacity(1).unwrap();
            arr.push(CustomValue);
            arr.push(CustomValue);
        }
        immix_unregister_thread();
    }
}
