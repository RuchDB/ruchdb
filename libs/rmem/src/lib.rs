
mod align;
mod alloc;
mod mem;

pub use align::{BYTE_ALIGN_SIZE, SYS_ALIGN_SIZE};
pub use align::{size_of, align_of, size_of_aligned, size_of_sys_aligned};

pub use alloc::{malloc, free, calloc, realloc, malloc_for, free_for, calloc_for};
pub use alloc::{zmalloc, zfree, zcalloc, zrealloc, zmem_size_of};

pub use mem::{mem_copy, mem_move, mem_set, mem_cmp, mem_find};
pub use mem::{mem_copy_for, mem_move_for};
