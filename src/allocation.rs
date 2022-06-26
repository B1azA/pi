use std::alloc;

pub(crate) fn allocate<T>() -> *mut u8 {
    let layout = alloc::Layout::new::<T>();
    unsafe {
        let ptr = alloc::alloc_zeroed(layout);
        ptr
    }
}

pub(crate) fn deallocate<T>(ptr: *mut u8) {
    let layout = alloc::Layout::new::<T>();
    unsafe {
        alloc::dealloc(ptr, layout);
    }
}

pub(crate) fn get_value<T: Copy>(ptr: *mut u8) -> T {
    let ptr = ptr as *mut T;
    unsafe {
        *ptr
    }
}

pub(crate) fn set_value<T>(ptr: *mut u8, value: T) {
    let ptr = ptr as *mut T;
    unsafe {
        *ptr = value;
    }
}

pub(crate) fn bytes_to_value<T: Copy>(bytes: &[u8]) -> T {
    let ptr: *const [u8] = bytes;
    let ptr = ptr as *const T;
    unsafe {
        *ptr
    }
}

pub(crate) fn allocate_size(size: usize) -> *mut u8 {
    unsafe {
        let layout = alloc::Layout::from_size_align_unchecked(1, 1);
        let ptr = alloc::alloc_zeroed(layout);
        ptr
    }  
}

pub(crate) fn deallocate_size(ptr: *mut u8, size: usize) {
    unsafe {
        let layout = alloc::Layout::from_size_align_unchecked(size, size);
        alloc::dealloc(ptr, layout);
    }
}

pub(crate) fn get_value_size<T: Copy>(ptr: *mut u8, index: isize) -> T {
    let mut ptr = ptr as *mut T;
    unsafe {
        ptr = ptr.offset(index);
        *ptr
    }  
}

pub(crate) fn set_value_size<T>(ptr: *mut u8, index: isize, value: T) {
    let mut ptr = ptr as *mut T;
    unsafe {
        ptr = ptr.offset(index);
        *ptr = value;
    }
}

pub(crate) fn offset(ptr: *mut u8, offset: isize) -> *mut u8 {
    unsafe { 
        let ptr = ptr.offset(offset);
        ptr
    }
}