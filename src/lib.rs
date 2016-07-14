#![no_std]
#![feature(const_fn)]

extern crate libc;
extern crate std;

use libc::{size_t, c_void, c_int, fputc, FILE};

struct MemChunk {
    array: [u8; 10000],
    index: size_t,
}
static mut mem: MemChunk = MemChunk {
    array: [0; 10000],
    index: 0,
};
const fn null() -> *mut c_void {
    0 as *mut libc::c_void
}

extern "C" {
    static stderr: *mut FILE;
}

#[no_mangle]
pub extern "C" fn malloc(size: size_t) -> *mut c_void {
    if size == 0 {
        return null();
    }
    let align_size = size + (size % 4);
    let cur = unsafe { (&mem.array[mem.index]) } as *const u8;
    unsafe { mem.index += align_size };
    cur as *mut libc::c_void
}


#[no_mangle]
pub extern "C" fn free(ptr: *mut c_void) {
    unsafe {
        for ch in "test test test!\n".chars() {
            fputc(ch as c_int, stderr);
        }
    }
}

#[no_mangle]
pub extern "C" fn calloc(nmemb: size_t, size: size_t) -> *mut c_void {
    if nmemb == 0 || size == 0 {
        return null();
    }
    malloc(nmemb * size)
}


#[no_mangle]
pub extern "C" fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
    let new_mem : *mut c_void = malloc(size);    
    free(ptr);
    new_mem
}
