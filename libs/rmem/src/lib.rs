mod align;
mod alloc;
mod mem;

pub use align::{align_of, size_of, size_of_aligned, size_of_sys_aligned};
pub use align::{BYTE_ALIGN_SIZE, SYS_ALIGN_SIZE};

pub use alloc::{calloc, calloc_for, free, free_for, malloc, malloc_for, realloc};
pub use alloc::{zcalloc, zfree, zmalloc, zmem_size_of, zrealloc};

pub use mem::{mem_cmp, mem_copy, mem_find, mem_move, mem_set};
pub use mem::{mem_copy_for, mem_move_for};
