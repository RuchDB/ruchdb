use std::cmp::Ordering;

use crate::size_of;


////////////////////////////////////////////////////////////////////////////////
// Memory (Byte-Leveled) Operations
////////////////////////////////////////////////////////////////////////////////

#[inline]
pub unsafe fn mem_copy(src: *const u8, dst: *mut u8, count: usize) {
    libc::memcpy(dst as _, src as _, count);
}

#[inline]
pub unsafe fn mem_move(src: *const u8, dst: *mut u8, count: usize) {
    libc::memmove(dst as _, src as _, count);
}

#[inline]
pub unsafe fn mem_set(ptr: *mut u8, value: u8, count: usize) {
    libc::memset(ptr as _, value as _, count);
}

#[inline]
pub unsafe fn mem_cmp(ptr1: *const u8, ptr2: *const u8, count: usize) -> Ordering {
    match libc::memcmp(ptr1 as _, ptr2 as _, count) {
        v if v < 0 => Ordering::Less,
        v if v > 0 => Ordering::Greater,
        _ => Ordering::Equal,
    }
}

#[inline]
pub unsafe fn mem_find(ptr: *const u8, len: usize, value: u8) -> Option<usize> {
    let pch = libc::memchr(ptr as _, value as _, len) as *const u8;
    match pch.is_null() {
        true => None,
        false => Some(pch as usize - ptr as usize),
    }
}


////////////////////////////////////////////////////////////////////////////////
// Memory (Object-Leveled) Operations
////////////////////////////////////////////////////////////////////////////////

#[inline]
pub unsafe fn mem_copy_for<T>(src: *const T, dst: *mut T, count: usize) {
    mem_copy(src as _, dst as _, size_of::<T>() * count);
}

#[inline]
pub unsafe fn mem_move_for<T>(src: *const T, dst: *mut T, count: usize) {
    mem_move(src as _, dst as _, size_of::<T>() * count);
}


////////////////////////////////////////////////////////////////////////////////
// Unit Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod mem_ops_tests {
    use super::*;

    #[test]
    fn copy_data() {
        let (src, mut dst) = (vec![1, 2, 3, 4], vec![0; 4]);
        unsafe { mem_copy(src.as_ptr(), dst.as_mut_ptr(), size_of::<u8>() * 4); }
        assert_eq!(dst, vec![1, 2, 3, 4]);
    }

    #[test]
    fn move_data() {
        let mut elems = vec![1, 2, 3, 4, 5, 6, 7, 8];
        unsafe { mem_move(elems.as_ptr(), (&mut elems[2..]).as_mut_ptr(), size_of::<u8>() * 4); }
        assert_eq!(elems, vec![1, 2, 1, 2, 3, 4, 7, 8]);
    }

    #[test]
    fn init_data_with_zero() {
        let mut elems = vec![1, 4, 2, 5];
        unsafe { mem_set(elems.as_mut_ptr(), 0, size_of::<u8>() * 4); }
        assert_eq!(elems, vec![0; 4]);
    }

    #[test]
    fn cmp_data() {
        let (src, dst) = (vec![1, 2, 3, 4], vec![1, 2, 3, 4]);
        let ord = unsafe { mem_cmp(src.as_ptr(), dst.as_ptr(), size_of::<u8>() * 4) };
        assert_eq!(ord, Ordering::Equal);

        let (src, dst) = (vec![4, 3, 2, 1], vec![1, 2, 3, 4]);
        let ord = unsafe { mem_cmp(src.as_ptr(), dst.as_ptr(), size_of::<u8>() * 4) };
        assert_eq!(ord, Ordering::Greater);

        let (src, dst) = (vec![1, 1, 2, 2], vec![1, 2, 3, 4]);
        let ord = unsafe { mem_cmp(src.as_ptr(), dst.as_ptr(), size_of::<u8>() * 4) };
        assert_eq!(ord, Ordering::Less);   
    }

    #[test]
    fn find_byte_from_data() {
        let elems = vec![1, 2, 3, 4];
        assert_eq!(unsafe { mem_find(elems.as_ptr(), size_of::<u8>() * 4, 3) }, Some(2));
        assert_eq!(unsafe { mem_find(elems.as_ptr(), size_of::<u8>() * 4, 5) }, None);
    }

    #[test]
    fn copy_elems() {
        let (src, mut dst) = (vec![1, 2, 3, 4], vec![0; 4]);
        unsafe { mem_copy_for::<u32>(src.as_ptr(), dst.as_mut_ptr(), 4); }
        assert_eq!(dst, vec![1, 2, 3, 4]);
    }

    #[test]
    fn move_elems() {
        let mut elems = vec![1, 2, 3, 4, 5, 6, 7, 8];
        unsafe { mem_move_for::<u32>(elems.as_ptr(), (&mut elems[2..]).as_mut_ptr(), 4); }
        assert_eq!(elems, vec![1, 2, 1, 2, 3, 4, 7, 8]);
    }
}
