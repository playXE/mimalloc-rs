#![feature(allocator_api)]

#![no_std]

extern crate core;

#[repr(C)]
pub struct mi_heap_t {
    __x: u8
}



#[link(name="mimalloc")]

extern "C" {
    pub fn mi_stats_print(_: *mut libc::FILE);
    pub fn mi_stats_reset();
    pub fn mi_malloc(_: usize) -> *mut u8;
    pub fn mi_free(_: *mut u8);
    pub fn mi_heap_malloc(_: *mut mi_heap_t,_: usize ) -> *mut u8;
    pub fn mi_heap_destroy(_: *mut mi_heap_t);
    pub fn mi_realloc(_: *mut u8,_: usize) -> *mut u8;
    pub fn mi_expand(_: *mut u8,_: usize) -> *mut u8;
    pub fn mi_zalloc(_: usize) -> *mut u8;
    pub fn mi_heap_get_default() -> *mut mi_heap_t;
    pub fn mi_calloc(_: usize,_: usize) -> *mut u8;
}

use core::alloc::{GlobalAlloc,Layout};

pub struct MiMalloc;

unsafe impl GlobalAlloc for MiMalloc {
    unsafe fn alloc(&self,layout: Layout) -> *mut u8 {
        let ptr = mi_malloc(layout.size());
        assert!(!ptr.is_null());
        ptr
    }

    unsafe fn dealloc(&self,ptr: *mut u8,_:Layout) {
        assert!(!ptr.is_null());
        mi_free(ptr);
    }
    unsafe fn alloc_zeroed(&self,layout: Layout) -> *mut u8 {
        mi_calloc(1,layout.size())
    }
    
    unsafe fn realloc(&self,ptr: *mut u8,layout: Layout,new_size: usize) -> *mut u8 {
        if ptr.is_null() {
            self.alloc(layout)
        } else {
            mi_realloc(ptr, new_size)
        }
    }
}
