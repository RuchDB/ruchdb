////////////////////////////////////////////////////////////////////////////////
// Type/Data Size & Alignment
////////////////////////////////////////////////////////////////////////////////

/// `Byte Alignment` is always the same (1 byte) on ALL types of OS.
///
/// `System-Dependent Alignment` is defined as the alignment of `pointer/isize/usize` type,
/// that it values 4 bytes on 32-bit OS while 8 bytes on 64-bit OS.

pub const BYTE_ALIGN_SIZE: usize = 1usize;
pub const SYS_ALIGN_SIZE: usize = align_of::<usize>();

/// Calculate size (in bytes) of a certain type.
///
/// It has great dependencies on target OS & compiler.
/// Normally, `isize/usize` has the same size as pointer type that
/// it has size of 4 bytes on 32-bit OS while 8 bytes on 64-bit OS.
///
/// # Examples
///
/// ```
/// # use rmem::size_of;
///
/// assert_eq!(size_of::<u8>(),  1);
/// assert_eq!(size_of::<u16>(), 2);
/// assert_eq!(size_of::<u32>(), 4);
/// assert_eq!(size_of::<u64>(), 8);
///
/// #[cfg(target_pointer_width = "32")]
/// assert_eq!(size_of::<*const u8>(), 4);
/// #[cfg(target_pointer_width = "64")]
/// assert_eq!(size_of::<*const u8>(), 8);
///
/// assert_eq!(size_of::<usize>(), size_of::<*const u8>());
/// ```
#[inline]
pub const fn size_of<T>() -> usize {
    std::mem::size_of::<T>()
}

/// Calculate MIN alignment size (in bytes) of a certain type.
///
/// It has great dependencies on target OS & compiler.
/// Normally, `isize/usize` has the same alignment size as pointer type that
/// it has alignment size of 4 bytes on 32-bit OS while 8 bytes on 64-bit OS.
///
/// # Examples
///
/// ```
/// # use rmem::align_of;
///
/// assert_eq!(align_of::<u8>(),  1);
/// assert_eq!(align_of::<u16>(), 2);
/// assert_eq!(align_of::<u32>(), 4);
/// assert_eq!(align_of::<u64>(), 8);
///
/// #[cfg(target_pointer_width = "32")]
/// assert_eq!(align_of::<*const u8>(), 4);
/// #[cfg(target_pointer_width = "64")]
/// assert_eq!(align_of::<*const u8>(), 8);
///
/// assert_eq!(align_of::<usize>(), align_of::<*const u8>());
/// ```
#[inline]
pub const fn align_of<T>() -> usize {
    std::mem::align_of::<T>()
}

/// Calculate aligned size for target size based on provided alignment.
///
/// # Notes
///
/// The provided `align` MUST be the power of 2, and normally values [1, 2, 4, 8].
///
/// # Examples
///
/// ```
/// # use rmem::size_of_aligned;
///
/// assert_eq!(size_of_aligned(3, 1), 3);
/// assert_eq!(size_of_aligned(4, 1), 4);
/// assert_eq!(size_of_aligned(5, 1), 5);
///
/// assert_eq!(size_of_aligned(3, 4), 4);
/// assert_eq!(size_of_aligned(4, 4), 4);
/// assert_eq!(size_of_aligned(5, 4), 8);
/// ```
#[inline]
pub const fn size_of_aligned(size: usize, align: usize) -> usize {
    (size + (align - 1)) & !(align - 1)
}

/// Calculate aligned size for target size based on system-dependent alignment.
///
/// `System-Dependent Alignment` is defined as the alignment of `pointer/isize/usize` type.
///
/// # Examples
///
/// ```
/// # use rmem::size_of_sys_aligned;
///
/// if cfg!(target_pointer_width = "32") {
///     assert_eq!(size_of_sys_aligned(2), 4);
///     assert_eq!(size_of_sys_aligned(4), 4);
///     assert_eq!(size_of_sys_aligned(6), 8);
///     assert_eq!(size_of_sys_aligned(8), 8);
/// }
///
/// if cfg!(target_pointer_width = "64") {
///     assert_eq!(size_of_sys_aligned(2), 8);
///     assert_eq!(size_of_sys_aligned(4), 8);
///     assert_eq!(size_of_sys_aligned(6), 8);
///     assert_eq!(size_of_sys_aligned(8), 8);
/// }
/// ```
#[inline]
pub const fn size_of_sys_aligned(size: usize) -> usize {
    size_of_aligned(size, SYS_ALIGN_SIZE)
}
