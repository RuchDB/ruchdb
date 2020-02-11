use std::alloc::{handle_alloc_error, Layout};

use crate::{align_of, size_of, size_of_aligned, BYTE_ALIGN_SIZE};

////////////////////////////////////////////////////////////////////////////////
// Memory Layout
////////////////////////////////////////////////////////////////////////////////

/// Create a layout (for `memory allocation/deallocation`) for a certain type/value.
#[inline]
const fn layout_of<T>() -> Layout {
    unsafe { layout_of_aligned(size_of::<T>(), align_of::<T>()) }
}

/// Create a layout (for `memory allocation/deallocation`) for amount of bytes/buffer.
///
/// # Notes
///
/// `size` can be an arbitrary value, however it's highly RECOMMENDED to be aligned
/// with the `System-Dependent Alignment` for Memory-Saving & Performance purpose.
#[inline]
const fn layout_of_bytes(size: usize) -> Layout {
    unsafe { layout_of_aligned(size, BYTE_ALIGN_SIZE) }
}

/// Create a layout (for `memory allocation/deallocation`)
/// based on a perfect-aligned size with a valid provided alignment.
///
/// # Notes
///
/// `align` MUST be the power of 2, and normally values [1, 2, 4, 8].
///
/// `size` MUST be aligned, in other words `size` can be exactly divisible by `align`.
#[inline]
const unsafe fn layout_of_aligned(size: usize, align: usize) -> Layout {
    Layout::from_size_align_unchecked(size, align)
}

////////////////////////////////////////////////////////////////////////////////
// Memory Allocation/Deallocation
////////////////////////////////////////////////////////////////////////////////

/// Allocate memory based on a valid layout.
///
/// `malloc_with_layout` & `free_with_layout` SHOULD work as pairs
/// for memory allocation & deallocation separately.
///
/// # Panics
///
/// ZERO-sized layout is NOT supported/permitted.
///
/// # Aborts
///
/// It will abort while memory allocation errors/failures occur (such as OOM).
///
/// # Notes
///
/// While memory allocation fails, it will direct to `handle_alloc_error` to do some
/// resource saving/clearing & logging/reporting jobs before ABORTING the process.
///
/// The DEFAULT behavior of `handle_alloc_error` is just to print error message to `stderr`.
/// And it can be replaced with HOOKs -- `set_alloc_error_hook` & `take_alloc_error_hook`.
fn malloc_with_layout(layout: Layout) -> (*mut u8, usize) {
    unsafe {
        let ptr = std::alloc::alloc(layout);
        if ptr.is_null() {
            handle_alloc_error(layout);
        }

        (ptr, layout.size())
    }
}

/// Deallocate memory (previously allocated) with the same layout (previously provided).
///
/// `free_with_layout` SHOULD work as pairs with `malloc_with_layout`, `realloc_with_layout`
/// or `calloc_with_layout` for memory allocation/reallocation & deallocation works.
///
/// # Notes
///
/// `dealloc` MAY only marks the memory block as free (in memory pool managed by allocator),
/// WITHOUT modifying `pointer` to be NULL and/or returning memory back to the system,
/// thus the `pointer` remains VALID and the memory stays ACCESSIBLE in some time.
///
/// It's highly RECOMMENDED to reassign NULL (with `ptr::null()`) to `pointer` after
/// its memory deallocated, as well as to check if `pointer` is NULL (with `is_null()`)
/// before taking use of it each time.
fn free_with_layout(ptr: *mut u8, layout: Layout) {
    if !ptr.is_null() {
        unsafe {
            std::alloc::dealloc(ptr, layout);
        }
    }
}

/// Allocate memory with zero-initialized based on a valid layout.
///
/// `calloc_with_layout` acts similarly with `malloc_with_layout`,
/// except that it will initialize the memory with zero.
///
/// `calloc_with_layout` & `free_with_layout` SHOULD work as pairs
/// for memory allocation & deallocation separately.
///
/// # Panics
///
/// ZERO-sized layout is NOT supported/permitted.
///
/// # Aborts
///
/// It will abort while memory allocation errors/failures occur (such as OOM).
fn calloc_with_layout(layout: Layout) -> (*mut u8, usize) {
    unsafe {
        let ptr = std::alloc::alloc_zeroed(layout);
        if ptr.is_null() {
            handle_alloc_error(layout);
        }

        (ptr, layout.size())
    }
}

/// Reallocate memory with another layout for memory scaling purpose.
///
/// It will allocate new memory block with `new_layout` if original NULL `pointer` is provided,
/// otherwise will reallocate enough memory based on the original one.
///
/// `realloc_with_layout` & `free_with_layout` SHOULD work as pairs
/// for memory reallocation & deallocation separately.
///
/// # Panics
///
/// The `new_layout` with ZERO size is NOT supported/permitted.
///
/// # Notes
///
/// `old_layout` SHOULD be the same as previously provided one for allocation/reallocation.
///
/// `old_layout` & `new_layout` SHOULD has the same valid alignment.
///
/// The newer `pointer` MAYBE the same as the original one, and MAYBE NOT.
/// DO NOT take any assumptions on them.
///
/// # Aborts
///
/// It will abort while memory reallocation errors/failures occur (such as OOM).
fn realloc_with_layout(ptr: *mut u8, old_layout: Layout, new_layout: Layout) -> (*mut u8, usize) {
    if new_layout.size() == old_layout.size() {
        return (ptr, new_layout.size());
    }

    unsafe {
        let ptr = match ptr.is_null() {
            true => std::alloc::alloc(new_layout),
            false => std::alloc::realloc(ptr, old_layout, new_layout.size()),
        };
        if ptr.is_null() {
            handle_alloc_error(new_layout);
        }

        (ptr, new_layout.size())
    }
}

/// Allocate memory/buffer with a certain size.
///
/// A valid `pointer` as well as the `size` of the allocated memory will be returned.
/// The `size` of the allocated memory SHOULD be the same as the provided one.
///
/// `malloc` & `free` SHOULD work as pairs for memory allocation & deallocation separately.
///
/// # Notes
///
/// `size` can be an arbitrary value, however it's highly RECOMMENDED to be aligned
/// with the `System-Dependent Alignment` for Memory-Saving & Performance purpose.
///
/// # Panics
///
/// ZERO size is NOT supported/permitted.
///
/// # Aborts
///
/// It will abort while memory allocation errors/failures occur (such as OOM).
///
/// # Examples
///
/// ```
/// # #[allow(unused_assignments)]
/// # use rmem::{size_of_sys_aligned, malloc, free};
///
/// let size = size_of_sys_aligned(6);
/// let (mut ptr, msize) = malloc(size);
/// assert!(!ptr.is_null());
/// assert_eq!(msize, size);
///
/// // Do works with ptr...
///
/// free(ptr, size);
/// ptr = std::ptr::null_mut();
/// ```
#[inline]
pub fn malloc(size: usize) -> (*mut u8, usize) {
    malloc_with_layout(layout_of_bytes(size))
}

/// Deallocate memory with the same size previously provided.
///
/// `free` SHOULD work as pairs with `malloc`, `realloc` or `calloc`
/// for memory allocation/reallocation & deallocation works.
#[inline]
pub fn free(ptr: *mut u8, size: usize) {
    free_with_layout(ptr, layout_of_bytes(size));
}

/// Allocate memory/buffer with zero-initialized with a certain size.
///
/// `calloc` acts similarly with `malloc`, except that it will initialize the memory with zero.
///
/// `calloc` & `free` SHOULD work as pairs for memory allocation & deallocation separately.
///
/// # Panics
///
/// ZERO size is NOT supported/permitted.
///
/// # Aborts
///
/// It will abort while memory allocation errors/failures occur (such as OOM).
///
/// # Examples
///
/// ```
/// # #[allow(unused_assignments)]
/// # use rmem::{size_of_sys_aligned, calloc, free};
///
/// let (mut ptr, msize) = calloc(size_of_sys_aligned(6));
/// assert!(!ptr.is_null());
/// assert_eq!(msize, 8);
/// assert_eq!(unsafe { *(ptr as *const u64) }, 0);
///
/// // Do works with ptr...
///
/// free(ptr, msize);
/// ptr = std::ptr::null_mut();
/// ```
#[inline]
pub fn calloc(size: usize) -> (*mut u8, usize) {
    calloc_with_layout(layout_of_bytes(size))
}

/// Reallocate memory/buffer with another size for memory scaling purpose.
///
/// It will allocate new memory block with `new_size` if original NULL `pointer` is provided,
/// otherwise will reallocate enough memory based on the original one.
///
/// `realloc` & `free` SHOULD work as pairs for memory reallocation & deallocation separately.
///
/// # Panics
///
/// The `new_size` with ZERO size is NOT supported/permitted.
///
/// # Aborts
///
/// It will abort while memory reallocation errors/failures occur (such as OOM).
///
/// # Examples
///
/// ```
/// # #[allow(unused_assignments)]
/// # use rmem::{size_of_sys_aligned, malloc, realloc, free};
///
/// let (ptr, size) = malloc(size_of_sys_aligned(8));
/// assert!(!ptr.is_null());
/// assert_eq!(size, 8);
///
/// // Do works with ptr...
///
/// // Reverse more memory for further works.
/// let (mut ptr, size) = realloc(ptr, size, size_of_sys_aligned(16));
/// assert!(!ptr.is_null());
/// assert_eq!(size, 16);
///
/// // Do further works with ptr...
///
/// free(ptr, size);
/// ptr = std::ptr::null_mut();
/// ```
#[inline]
pub fn realloc(ptr: *mut u8, old_size: usize, new_size: usize) -> (*mut u8, usize) {
    realloc_with_layout(ptr, layout_of_bytes(old_size), layout_of_bytes(new_size))
}

/// Allocate memory/element with a certain type.
///
/// A valid element `pointer` with its `size` will be returned.
///
/// `malloc_for` & `free_for` SHOULD work as pairs for memory allocation & deallocation separately.
///
/// # Aborts
///
/// It will abort while memory allocation errors/failures occur (such as OOM).
///
/// # Examples
///
/// ```
/// # #[allow(unused_assignments)]
/// # use rmem::{size_of, malloc_for, free_for};
///
/// let (mut ptr, size) = malloc_for::<u32>();
/// assert!(!ptr.is_null());
/// assert_eq!(size, size_of::<u32>());
///
/// unsafe {
///     *ptr = 10;
///     println!("{}", *ptr);
/// }
///
/// free_for::<u32>(ptr);
/// ptr = std::ptr::null_mut();
/// ```
#[inline]
pub fn malloc_for<T>() -> (*mut T, usize) {
    let (ptr, msize) = malloc_with_layout(layout_of::<T>());
    (ptr as _, msize)
}

/// Deallocate memory/element with its type provided.
///
/// `free_for` SHOULD work as pairs with `malloc_for` or `calloc_for`
/// for memory allocation & deallocation works.
#[inline]
pub fn free_for<T>(ptr: *mut T) {
    free_with_layout(ptr as _, layout_of::<T>());
}

/// Allocate memory/element with zero-initialized with a certain type.
///
/// `calloc_for` acts similarly with `malloc_for`, except that
/// it will initialize the memory/element with zero.
///
/// `calloc_for` & `free_for` SHOULD work as pairs
/// for memory allocation & deallocation separately.
///
/// # Aborts
///
/// It will abort while memory allocation errors/failures occur (such as OOM).
///
/// # Examples
///
/// ```
/// # #[allow(unused_assignments)]
/// # use rmem::{size_of, calloc_for, free_for};
///
/// let (mut ptr, size) = calloc_for::<u32>();
/// assert!(!ptr.is_null());
/// assert_eq!(size, size_of::<u32>());
/// assert_eq!(unsafe { *ptr }, 0);
///
/// unsafe {
///     *ptr = 10;
///     println!("{}", *ptr);
/// }
///
/// free_for::<u32>(ptr);
/// ptr = std::ptr::null_mut();
/// ```
#[inline]
pub fn calloc_for<T>() -> (*mut T, usize) {
    let (ptr, msize) = calloc_with_layout(layout_of::<T>());
    (ptr as _, msize)
}

////////////////////////////////////////////////////////////////////////////////
// ZMEM-Style Memory Allocation/Deallocation
////////////////////////////////////////////////////////////////////////////////

/// ZMEM is a size-aware memory allocation/deallocation style (introduced from Redis).
///
/// ZMEM-style memory (aligned with type `usize`) contains two parts:
///   1) Header Part (`usize`): The size of the allocated body part.
///   2) Body Part (`*mut u8`): The real allocated memory required.
///
/// Once required memory is allocated, the `pointer` (of body part) and `size` will be returned
/// with header part invisible to user, which acts just like what `malloc` does.
///
/// However, the `size` can be extracted with only `pointer`, as the size (header part) is
/// binded with the pointer (body part).
///
/// Moreover, the `size` (of body part) MAYBE larger than the provided/required one
/// because of memory alignment (based on the alignment of `usize`).
///
/// # Notes
///
/// Allocated memory with zero size in ZMEM-style, also contains valid memory with
/// `size_of::<usize>()` bytes as its header part.
///
/// In other words, allocating memory in ZMEM-style will SURELY result in valid pointer,
/// except for allocation failures (such as OOM) which will cause process aborting.

const ZMEM_HEADER_SIZE: usize = size_of::<usize>();
const ZMEM_ALIGN_SIZE: usize = align_of::<usize>();

/// Allocate ZMEM-style memory/buffer with required size.
///
/// A valid memory/buffer `pointer` with its `size` will be returned.
/// The `size` of allocated memory MAYBE larger than the provided one because of memory alignment.
///
/// `zmalloc` & `zfree` SHOULD work as pairs for memory allocation & deallocation separately.
///
/// # Aborts
///
/// It will abort while memory allocation errors/failures occur (such as OOM).
///
/// # Examples
///
/// ```
/// # #[allow(unused_assignments)]
/// # use rmem::{zmalloc, zfree, zmem_size_of};
///
/// let (mut ptr, size) = zmalloc(6);
/// assert!(!ptr.is_null());
/// assert_eq!(size, 8);
/// assert_eq!(zmem_size_of(ptr), 8);
///
/// // Do works with ptr...
///
/// zfree(ptr);
/// // Reassign NULL to ptr after memory deallocation. (Can be ignored if it unused anymore)
/// ptr = std::ptr::null_mut();
/// ```
pub fn zmalloc(size: usize) -> (*mut u8, usize) {
    let bsize = size_of_aligned(size, ZMEM_ALIGN_SIZE);
    let (ptr, _) = malloc(ZMEM_HEADER_SIZE + bsize);

    unsafe {
        *(ptr as *mut usize) = bsize;
        (ptr.offset(ZMEM_HEADER_SIZE as _), bsize)
    }
}

/// Deallocate ZMEM-style memory/buffer previously allocated.
///
/// `zmalloc` & `zfree` SHOULD work as pairs for memory allocation & deallocation separately.
pub fn zfree(ptr: *mut u8) {
    if !ptr.is_null() {
        unsafe {
            let ptr = (ptr as *const usize).offset(-1);
            let bsize = *ptr;

            free(ptr as _, ZMEM_HEADER_SIZE + bsize);
        }
    }
}

/// Allocate ZMEM-style memory/buffer with zero-initialized with required size.
///
/// `zcalloc` acts similarly with `zmalloc`, except that it will initialize the memory with zero.
///
/// `zcalloc` & `zfree` SHOULD work as pairs for memory allocation & deallocation separately.
///
/// # Aborts
///
/// It will abort while memory allocation errors/failures occur (such as OOM).
///
/// # Examples
///
/// ```
/// # #[allow(unused_assignments)]
/// # use rmem::{zcalloc, zfree};
///
/// let (mut ptr, size) = zcalloc(8);
/// assert!(!ptr.is_null());
/// assert_eq!(size, 8);
/// assert_eq!(unsafe { *(ptr as *const u64) }, 0);
///
/// // Do works with ptr...
///
/// zfree(ptr);
/// ptr = std::ptr::null_mut();
/// ```
pub fn zcalloc(size: usize) -> (*mut u8, usize) {
    let bsize = size_of_aligned(size, ZMEM_ALIGN_SIZE);
    let (ptr, _) = calloc(ZMEM_HEADER_SIZE + bsize);

    unsafe {
        *(ptr as *mut usize) = bsize;
        (ptr.offset(ZMEM_HEADER_SIZE as _), bsize)
    }
}

/// Reallocate ZMEM-style memory/buffer with another size for memory scaling purpose.
///
/// It will allocate new memory block with `size` if original NULL `pointer` is provided,
/// otherwise will reallocate enough memory based on the original one.
///
/// `zrealloc` & `zfree` SHOULD work as pairs for memory reallocation & deallocation separately.
///
/// # Aborts
///
/// It will abort while memory reallocation errors/failures occur (such as OOM).
///
/// # Examples
///
/// ```
/// # #[allow(unused_assignments)]
/// # use rmem::{zmalloc, zrealloc, zfree};
///
/// let (ptr, size) = zmalloc(8);
/// assert!(!ptr.is_null());
/// assert_eq!(size, 8);
///
/// // Do works with ptr...
///
/// let (mut ptr, size) = zrealloc(ptr, 16);
/// assert!(!ptr.is_null());
/// assert_eq!(size, 16);
///
/// // Do further works with ptr...
///
/// zfree(ptr);
/// ptr = std::ptr::null_mut();
/// ```
pub fn zrealloc(ptr: *mut u8, new_size: usize) -> (*mut u8, usize) {
    let (old_ptr, old_msize) = if ptr.is_null() {
        (std::ptr::null_mut::<u8>(), 0usize)
    } else {
        unsafe {
            let ptr = (ptr as *const usize).offset(-1);
            (ptr as _, *ptr)
        }
    };

    let new_bsize = size_of_aligned(new_size, ZMEM_ALIGN_SIZE);
    let (new_ptr, _) = realloc(old_ptr, old_msize, ZMEM_HEADER_SIZE + new_bsize);

    unsafe {
        *(new_ptr as *mut usize) = new_bsize;
        (new_ptr.offset(ZMEM_HEADER_SIZE as _), new_bsize)
    }
}

/// Extract size (of body part) of ZMEM-style memory.
#[inline]
pub fn zmem_size_of(ptr: *mut u8) -> usize {
    match ptr.is_null() {
        true => 0usize,
        false => unsafe { *(ptr as *const usize).offset(-1) },
    }
}

////////////////////////////////////////////////////////////////////////////////
// Unit Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod layout_tests {
    use super::*;

    use crate::size_of_sys_aligned;

    #[test]
    fn calc_layout_of_type() {
        assert_eq!(Ok(layout_of::<u8>()), Layout::from_size_align(1, 1));
        assert_eq!(Ok(layout_of::<u16>()), Layout::from_size_align(2, 2));
        assert_eq!(Ok(layout_of::<u32>()), Layout::from_size_align(4, 4));
        assert_eq!(Ok(layout_of::<u64>()), Layout::from_size_align(8, 8));
    }

    #[test]
    fn calc_layout_of_required_bytes() {
        // An arbitrary size can be provided.
        assert_eq!(Ok(layout_of_bytes(1)), Layout::from_size_align(1, 1));
        assert_eq!(Ok(layout_of_bytes(3)), Layout::from_size_align(3, 1));
        assert_eq!(Ok(layout_of_bytes(4)), Layout::from_size_align(4, 1));
        assert_eq!(Ok(layout_of_bytes(5)), Layout::from_size_align(5, 1));

        // It's RECOMMENDED to provide an system-aligned size.
        if cfg!(target_pointer_width = "32") {
            assert_eq!(
                Ok(layout_of_bytes(size_of_sys_aligned(1))),
                Layout::from_size_align(4, 1)
            );
            assert_eq!(
                Ok(layout_of_bytes(size_of_sys_aligned(3))),
                Layout::from_size_align(4, 1)
            );
            assert_eq!(
                Ok(layout_of_bytes(size_of_sys_aligned(4))),
                Layout::from_size_align(4, 1)
            );
            assert_eq!(
                Ok(layout_of_bytes(size_of_sys_aligned(5))),
                Layout::from_size_align(8, 1)
            );
        }
        if cfg!(target_pointer_width = "64") {
            assert_eq!(
                Ok(layout_of_bytes(size_of_sys_aligned(1))),
                Layout::from_size_align(8, 1)
            );
            assert_eq!(
                Ok(layout_of_bytes(size_of_sys_aligned(3))),
                Layout::from_size_align(8, 1)
            );
            assert_eq!(
                Ok(layout_of_bytes(size_of_sys_aligned(4))),
                Layout::from_size_align(8, 1)
            );
            assert_eq!(
                Ok(layout_of_bytes(size_of_sys_aligned(5))),
                Layout::from_size_align(8, 1)
            );
        }
    }

    #[test]
    fn calc_layout_of_aligned_size() {
        unsafe {
            assert_eq!(Ok(layout_of_aligned(1, 1)), Layout::from_size_align(1, 1));
            assert_eq!(Ok(layout_of_aligned(3, 1)), Layout::from_size_align(3, 1));
            assert_eq!(Ok(layout_of_aligned(4, 1)), Layout::from_size_align(4, 1));
            assert_eq!(Ok(layout_of_aligned(5, 1)), Layout::from_size_align(5, 1));

            assert_eq!(Ok(layout_of_aligned(4, 4)), Layout::from_size_align(4, 4));
            assert_eq!(Ok(layout_of_aligned(8, 4)), Layout::from_size_align(8, 4));
        }
    }
}

#[cfg(test)]
#[allow(unused_assignments)]
mod mem_alloc_tests {
    use super::*;

    use crate::size_of_sys_aligned;

    #[test]
    fn mem_alloc_with_layout() {
        let layout = layout_of_bytes(size_of_sys_aligned(6));
        let (mut ptr, msize) = malloc_with_layout(layout);
        assert!(!ptr.is_null());
        assert_eq!(msize, 8);

        free_with_layout(ptr, layout);
        ptr = std::ptr::null_mut();
    }

    #[test]
    fn mem_calloc_with_layout() {
        let layout = layout_of_bytes(8);
        let (mut ptr, size) = calloc_with_layout(layout);
        assert!(!ptr.is_null());
        assert_eq!(size, 8);
        assert_eq!(unsafe { *(ptr as *const u64) }, 0);

        free_with_layout(ptr, layout);
        ptr = std::ptr::null_mut();
    }

    #[test]
    fn mem_realloc_with_layout() {
        let layout = layout_of_bytes(8);
        let (ptr, size) = malloc_with_layout(layout);
        assert!(!ptr.is_null());
        assert_eq!(size, 8);

        let new_layout = layout_of_bytes(16);
        let (mut ptr, size) = realloc_with_layout(ptr, layout, new_layout);
        assert!(!ptr.is_null());
        assert_eq!(size, 16);

        free_with_layout(ptr, new_layout);
        ptr = std::ptr::null_mut();
    }

    #[test]
    fn mem_alloc_with_size() {
        let size = size_of_sys_aligned(6);
        let (mut ptr, msize) = malloc(size);
        assert!(!ptr.is_null());
        assert_eq!(msize, size);

        free(ptr, size);
        ptr = std::ptr::null_mut();
    }

    #[test]
    fn mem_calloc_with_size() {
        let (mut ptr, size) = calloc(size_of_sys_aligned(8));
        assert!(!ptr.is_null());
        assert_eq!(size, 8);
        assert_eq!(unsafe { *(ptr as *const u64) }, 0);

        free(ptr, size);
        ptr = std::ptr::null_mut();
    }

    #[test]
    fn mem_realloc_with_size() {
        let (ptr, size) = malloc(size_of_sys_aligned(8));
        assert!(!ptr.is_null());
        assert_eq!(size, 8);

        let (mut ptr, size) = realloc(ptr, size, size_of_sys_aligned(16));
        assert!(!ptr.is_null());
        assert_eq!(size, 16);

        free(ptr, size);
        ptr = std::ptr::null_mut();
    }

    #[test]
    fn mem_realloc_for_null_pointer() {
        let (mut ptr, size) = realloc(std::ptr::null_mut(), 0, 8);
        assert!(!ptr.is_null());
        assert_eq!(size, 8);

        free(ptr, size);
        ptr = std::ptr::null_mut();
    }

    #[test]
    fn mem_alloc_with_type() {
        let (mut ptr, size) = malloc_for::<u32>();
        assert!(!ptr.is_null());
        assert_eq!(size, size_of::<u32>());

        free_for::<u32>(ptr);
        ptr = std::ptr::null_mut();
    }

    #[test]
    fn mem_calloc_with_type() {
        let (mut ptr, size) = calloc_for::<u32>();
        assert!(!ptr.is_null());
        assert_eq!(size, size_of::<u32>());
        assert_eq!(unsafe { *ptr }, 0);

        free_for::<u32>(ptr);
        ptr = std::ptr::null_mut();
    }
}

#[cfg(test)]
#[allow(unused_assignments)]
mod zmem_alloc_tests {
    use super::*;

    #[test]
    fn zmem_alloc_with_size() {
        let (mut ptr, size) = zmalloc(6);
        assert!(!ptr.is_null());
        assert_eq!(size, 8);
        assert_eq!(zmem_size_of(ptr), 8);

        zfree(ptr);
        ptr = std::ptr::null_mut();
    }

    #[test]
    fn zmem_calloc_with_size() {
        let (mut ptr, size) = zcalloc(8);
        assert!(!ptr.is_null());
        assert_eq!(size, 8);
        assert_eq!(zmem_size_of(ptr), 8);
        assert_eq!(unsafe { *(ptr as *const u64) }, 0);

        zfree(ptr);
        ptr = std::ptr::null_mut();
    }

    #[test]
    fn zmem_realloc_with_size() {
        let (ptr, size) = zmalloc(8);
        assert!(!ptr.is_null());
        assert_eq!(size, 8);
        assert_eq!(zmem_size_of(ptr), 8);

        let (mut ptr, size) = zrealloc(ptr, 16);
        assert!(!ptr.is_null());
        assert_eq!(size, 16);
        assert_eq!(zmem_size_of(ptr), 16);

        zfree(ptr);
        ptr = std::ptr::null_mut();
    }

    #[test]
    fn zmem_realloc_for_null_pointer() {
        let (mut ptr, size) = zrealloc(std::ptr::null_mut(), 8);
        assert!(!ptr.is_null());
        assert_eq!(size, 8);
        assert_eq!(zmem_size_of(ptr), 8);

        zfree(ptr);
        ptr = std::ptr::null_mut();
    }
}
