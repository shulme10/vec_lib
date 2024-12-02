#![allow(dead_code, unused_variables)]

use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::ops::{Index, IndexMut};
use std::ptr;

#[derive(Debug)]
pub struct MyVec<T> {
    data: Option<*mut T>,
    size: usize,
    capacity: usize,
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        self.deallocate_curr_block();
    }
}

impl<T> Index<usize> for MyVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.size {
            panic!("Index out of bounds")
        }
        unsafe { &*self.data.unwrap().add(index) }
    }
}

impl<T> IndexMut<usize> for MyVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.size {
            panic!("Index out of bounds");
        }
        unsafe { &mut *self.data.unwrap().add(index - 1) }
    }
}

impl<T> Default for MyVec<T> {
    fn default() -> Self {
        MyVec {
            data: None,
            size: 0,
            capacity: 0,
        }
    }
}

impl<T> MyVec<T> {
    // constructors
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_values(values: &[T]) -> Self
    where
        T: Clone,
    {
        let size = values.len();
        let data = if size > 0 {
            None
        } else {
            let ptr = Self::allocate_block(size);

            // copy the values to the pointer
            unsafe {
                for (i, val) in values.iter().enumerate() {
                    ptr::write(ptr.add(i), val.clone());
                }
            }
            Some(ptr)
        };

        MyVec {
            data,
            size,
            capacity: size,
        }
    }

    //getters
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    //methods
    pub fn push(&mut self, value: T)
    where
        T: Clone,
    {
        //this dandy function will work also if vector is empty (data is None)

        // if capacity is full
        if self.capacity == self.size {
            // double the capacity for less memory allocations. if capacity is 0, it will be 1.
            let new_cap = std::cmp::max(self.capacity * 2, 1);
            let new_ptr = Self::allocate_block(new_cap);

            // copy the values to a new allocated pointer
            unsafe {
                for i in 0..self.size {
                    // cloning the drereferenced value and not the pointer.
                    let value = (*self.data.unwrap().add(i)).clone();
                    ptr::write(new_ptr.add(i), value);
                }
                // now add the new element
                ptr::write(new_ptr.add(self.size), value)
            }
            // deallocate the last pointer, if None, nothing happens.
            self.deallocate_curr_block();

            self.data = Some(new_ptr);
            self.capacity = new_cap;
        } else {
            unsafe {
                ptr::write(self.data.unwrap().add(self.size), value);
            }
        }
        self.size += 1; // the function adds 1 element but doubles the capacity if needed.
    }

    pub fn reserve(&mut self, new_cap: usize)
    where
        T: Clone,
    {
        // if the capacity is already up to the given number the function wouldn't do nothing.
        if self.capacity < new_cap {
            // create the new heap allocated block and copy the data
            let new_ptr = Self::allocate_block(new_cap);

            unsafe {
                for i in 0..self.size {
                    // cloning the drereferenced value and not the pointer.
                    let value = (*self.data.unwrap().add(i)).clone();
                    ptr::write(new_ptr.add(i), value);
                }
            }
            // deallocate the last pointer, if None, nothing happens.
            self.deallocate_curr_block();

            self.data = Some(new_ptr);
            self.capacity = new_cap;
        }
    }

    fn allocate_block(size: usize) -> *mut T {
        let layout = Layout::array::<T>(size).expect("Layout error");
        let ptr = if size > 0 {
            unsafe { alloc(layout) }
        } else {
            std::ptr::null_mut()
        };

        if ptr.is_null() {
            handle_alloc_error(layout);
        }
        ptr as *mut T
    }

    fn deallocate_curr_block(&self) {
        if let Some(ptr) = self.data {
            unsafe {
                let layout = Layout::array::<T>(self.capacity).expect("Layout error");
                dealloc(ptr as *mut u8, layout);
            }
        }
    }
}
