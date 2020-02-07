use std::fmt;
use std::cmp::Ordering;
use std::marker::PhantomData;
use rmem::{zmalloc, zrealloc, zfree};
use rmem::{mem_copy, mem_move, mem_set, mem_cmp};

pub struct RString {
    len: usize,
    cap: usize,

    data: *const u8,
    _marker: PhantomData<u8>,
}

impl RString {
    #[inline]
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let (ptr, cap) = zmalloc(capacity);
        RString { len: 0, cap: cap, data: ptr as _, _marker: PhantomData }
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const u8 {
        self.data
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.data as _
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub const fn capacity(&self) -> usize {
        self.cap
    }

    #[inline]
    pub const fn avail(&self) -> usize {
        self.cap - self.len
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub const fn is_full(&self) -> bool {
        self.avail() == 0
    }
}

impl Drop for RString {
    #[inline]
    fn drop(&mut self) {
        zfree(self.as_mut_ptr());
    }
}

impl Default for RString {
    #[inline]
    fn default() -> RString {
        RString::new()
    }
}

impl RString {
    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
    }

    #[inline]
    pub fn truncate(&mut self, new_len: usize) {
        if new_len < self.len() {
            self.len = new_len;
        }
    }

    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        if min_capacity < self.capacity() {
            self.resize(min_capacity);
        }
    }

    #[inline]
    pub fn shrink_to_fit(&mut self) {
        if !self.is_full() {
            self.resize(self.len());
        }
    }

    #[inline]
    pub fn reserve(&mut self, extra: usize) {
        if self.avail() < extra {
            self.resize(self.len() + extra);
        }
    }

    fn resize(&mut self, min_capacity: usize) {
        let target_capacity = std::cmp::max(self.len(), min_capacity);
        let (ptr, cap) = zrealloc(self.as_mut_ptr(), target_capacity);

        self.data = ptr as _;
        self.cap = cap;
    }

    pub fn sub_rstr(&self, start: usize, end: usize) -> RString {
        let end = std::cmp::min(self.len(), end);
        if start < end {
            unsafe { Self::from_raw_data(self.as_ptr().add(start), end - start) }
        } else {
            Self::default()
        }
    }

    #[inline]
    pub fn lsub_rstr(&self, end: usize) -> RString {
        self.sub_rstr(0, end)
    }

    #[inline]
    pub fn rsub_rstr(&self, start: usize) -> RString {
        self.sub_rstr(start, self.len())
    }

    pub fn trim(&mut self, start: usize, end: usize) {
        let end = std::cmp::min(self.len(), end);
        if start < end {
            unsafe { mem_move(self.as_ptr().add(start), self.as_mut_ptr(), end - start); }
            self.len = end - start;
        }
    }

    #[inline]
    pub fn ltrim(&mut self, start: usize) {
        self.trim(start, self.len());
    }

    #[inline]
    pub fn rtrim(&mut self, end: usize) {
        if end < self.len() {
            self.len = end;
        }
    }
}

impl RString {
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.as_ptr(), self.len()) }
    }

    #[inline]
    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.as_mut_ptr(), self.len()) }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.len());
        unsafe { mem_copy(self.as_ptr(), bytes.as_mut_ptr(), self.len()); }
        bytes
    }

    pub unsafe fn to_string(&self) -> String {
        match std::str::from_utf8(self.as_bytes()) {
            Ok(s) => s.to_owned(),
            Err(_) => String::default(),
        }
    }

    #[inline]
    pub fn as_rstr(&self) -> &RString {
        self
    }

    #[inline]
    pub fn as_mut_rstr(&mut self) -> &mut RString {
        self
    }

    #[inline]
    pub fn to_rstr(&self) -> RString {
        self.clone()
    }

    #[inline]
    pub fn append_padding(&mut self, value: u8, count: usize) {
        self.reserve(count);

        unsafe { mem_set(self.as_mut_ptr().add(self.len()), value, count); }
        self.len += count;
    }

    unsafe fn from_raw_data(data: *const u8, len: usize) -> Self {
        let (ptr, cap) = zmalloc(len);
        mem_copy(data, ptr, len);
        RString { len: len, cap: cap, data: ptr as _, _marker: PhantomData }
    }

    unsafe fn copy_raw_data(&mut self, data: *const u8, len: usize) {
        self.clear();
        self.append_raw_data(data, len);
    }

    unsafe fn append_raw_data(&mut self, data: *const u8, len: usize) {
        self.reserve(len);

        mem_copy(data, self.as_mut_ptr().add(self.len()), len);
        self.len += len;
    }

    unsafe fn replace_raw_data(&mut self, offset: usize, data: *const u8, len: usize) {
        self.resize(offset + len);

        if self.len() < offset {
            mem_set(self.as_mut_ptr().add(self.len()), 0, offset - self.len());
        }
        mem_copy(data, self.as_mut_ptr().add(offset), len);
        self.len = std::cmp::max(self.len(), offset + len);
    }
}

macro_rules! impl_str_ops {
    ([OP_FROM] $from: ident, $stype: ty) => {
        impl RString {
            #[inline]
            pub fn $from(s: $stype) -> Self {
                unsafe { Self::from_raw_data(s.as_ptr(), s.len()) }
            }
        }
    };

    ([OP_COPY] $copy: ident, $stype: ty) => {
        impl RString {
            #[inline]
            pub fn $copy(&mut self, s: $stype) {
                unsafe { self.copy_raw_data(s.as_ptr(), s.len()); }
            }
        }
    };

    ([OP_APPEND] $append: ident, $stype: ty) => {
        impl RString {
            #[inline]
            pub fn $append(&mut self, s: $stype) {
                unsafe { self.append_raw_data(s.as_ptr(), s.len()); }
            }
        }
    };

    ([OP_REPLACE] $replace: ident, $stype: ty) => {
        impl RString {
            #[inline]
            pub fn $replace(&mut self, offset: usize, s: $stype) {
                unsafe { self.replace_raw_data(offset, s.as_ptr(), s.len()); }
            }
        }
    }
}

impl_str_ops! { [OP_FROM]    from_bytes,    &[u8]    }
impl_str_ops! { [OP_FROM]    from_str,      &str     }
impl_str_ops! { [OP_FROM]    from_rstr,     &RString }
impl_str_ops! { [OP_COPY]    copy_bytes,    &[u8]    }
impl_str_ops! { [OP_COPY]    copy_str,      &str     }
impl_str_ops! { [OP_COPY]    copy_rstr,     &RString }
impl_str_ops! { [OP_APPEND]  append_bytes,  &[u8]    }
impl_str_ops! { [OP_APPEND]  append_str,    &str     }
impl_str_ops! { [OP_APPEND]  append_rstr,   &RString }
impl_str_ops! { [OP_REPLACE] replace_bytes, &[u8]    }
impl_str_ops! { [OP_REPLACE] replace_str,   &str     }
impl_str_ops! { [OP_REPLACE] replace_rstr,  &RString }

impl Clone for RString {
    #[inline]
    fn clone(&self) -> Self {
        RString::from_rstr(self)
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        self.copy_rstr(source);
    }
}

impl PartialEq for RString {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            self.len() == other.len() && Ordering::Equal == 
                mem_cmp(self.as_ptr(), other.as_ptr(), self.len())
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !PartialEq::eq(self, other)
    }
}

impl Eq for RString { }

impl PartialOrd for RString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RString {
    fn cmp(&self, other: &Self) -> Ordering {
        let clen = std::cmp::min(self.len(), other.len());
        match unsafe { mem_cmp(self.as_ptr(), other.as_ptr(), clen) } {
            Ordering::Equal => Ord::cmp(&self.len(), &other.len()),
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl fmt::Display for RString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let printed = match std::str::from_utf8(self.as_bytes()) {
            Ok(s) => s,
            Err(_) => "<Unreadable Bytes>",
        };
        write!(f, "{}", printed)
    }
}

impl fmt::Debug for RString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let printed = match std::str::from_utf8(self.as_bytes()) {
            Ok(s) => s,
            Err(_) => "<Unreadable Bytes>",
        };

        write!(f, "{{ len: {}, cap: {}, data: <{:p}>[{}] }}", 
            self.len(), self.capacity(), self.as_ptr(), printed)
    }
}
