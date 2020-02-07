use rmem::*;

#[test]
fn mem_alloc_memory() {
    let (ptr, size) = malloc(size_of_sys_aligned(6));
    assert!(!ptr.is_null());
    assert_eq!(size, 8);

    let (ptr, size) = realloc(ptr, size, size_of_sys_aligned(15));
    assert!(!ptr.is_null());
    assert_eq!(size, 16);

    free(ptr, size);

    let (ptr, size) = calloc(size_of::<u32>());
    assert!(!ptr.is_null());
    assert_eq!(size, 4);
    assert_eq!(unsafe { *(ptr as *const u32) }, 0);

    free(ptr, size);
}

#[test]
fn mem_alloc_element() {
    let (ptr, size) = malloc_for::<u32>();
    assert!(!ptr.is_null());
    assert_eq!(size, 4);

    free_for(ptr);

    let (ptr, size) = calloc_for::<u32>();
    assert!(!ptr.is_null());
    assert_eq!(size, 4);
    assert_eq!(unsafe { *ptr }, 0);

    free_for(ptr);
}

#[test]
fn zmem_alloc_memory() {
    let (ptr, size) = zmalloc(6);
    assert!(!ptr.is_null());
    assert_eq!(size, 8);
    assert_eq!(zmem_size_of(ptr), 8);

    let (ptr, size) = zrealloc(ptr, 15);
    assert!(!ptr.is_null());
    assert_eq!(size, 16);
    assert_eq!(zmem_size_of(ptr), 16);

    zfree(ptr);

    let (ptr, size) = zcalloc(6);
    assert!(!ptr.is_null());
    assert_eq!(size, 8);
    assert_eq!(zmem_size_of(ptr), 8);
    assert_eq!(unsafe { *(ptr as *const u64) }, 0);

    zfree(ptr);
}
