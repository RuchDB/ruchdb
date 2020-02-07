use std::cmp::Ordering;
use rmem::*;

#[test]
fn mem_ops_on_pkg_buf() {
    unsafe {
        // Initialize a buffer
        let (buf, cap) = malloc(size_of::<u8>() * 1024);
        mem_set(buf, 0, cap);
        let mut len = 0usize;

        // Push the first pkg into buffer
        let pkg = b"PMAGIC\x00\x00From: Tom\nData: Hello, Roy!\n\x04";
        mem_copy(pkg.as_ptr(), buf.add(len), pkg.len());
        len += pkg.len();

        // Push the second pkg into buffer
        let pkg = b"PMAGIC\x00\x00From: Tom\nData: Welcome to Rust world!\n\x04";
        mem_copy(pkg.as_ptr(), buf.add(len), pkg.len());
        len += pkg.len();

        const PKG_MAGIC: &[u8] = b"PMAGIC\x00\x00";
        const PKG_MAGIC_LEN: usize = PKG_MAGIC.len();
        const PKG_END: u8 = b'\x04';

        // Extract the first pkg data
        let rdata = b"From: Tom\nData: Hello, Roy!\n";
        assert_eq!(mem_cmp(buf, PKG_MAGIC.as_ptr(), PKG_MAGIC_LEN), Ordering::Equal);
        let plen = mem_find(buf, len, PKG_END).unwrap() + 1;
        let dlen = plen - PKG_MAGIC_LEN - 1;
        assert_eq!(dlen, rdata.len());

        let (data, dcap) = malloc(dlen);
        mem_copy(buf.add(PKG_MAGIC_LEN), data, dlen);
        assert_eq!(mem_cmp(data, rdata.as_ptr(), rdata.len()), Ordering::Equal);
        free(data, dcap);

        len -= plen;
        mem_move(buf.add(plen), buf, len);

        // Extract the second pkg data
        let rdata = b"From: Tom\nData: Welcome to Rust world!\n";
        assert_eq!(mem_cmp(buf, PKG_MAGIC.as_ptr(), PKG_MAGIC_LEN), Ordering::Equal);

        let plen = mem_find(buf, len, PKG_END).unwrap() + 1;
        let dlen = plen - PKG_MAGIC_LEN - 1;
        assert_eq!(dlen, rdata.len());

        let (data, dcap) = malloc(dlen);
        mem_copy(buf.add(PKG_MAGIC_LEN), data, dlen);
        assert_eq!(mem_cmp(data, rdata.as_ptr(), rdata.len()), Ordering::Equal);
        free(data, dcap);

        len -= plen;
        mem_move(buf.add(plen), buf, len);

        // Empty buffer
        assert_eq!(len, 0);

        free(buf, cap);
    }
}

#[test]
fn mem_ops_on_elem_queue() {
    unsafe {
        // Initialize an empty data queue
        let mut queue: Vec<u32> = vec![0; 20];
        let mut len = 0usize;

        // A batch of job data is pushed into queue
        let jdata = vec![1, 2, 3, 4, 3, 2, 1];
        mem_copy_for(jdata.as_ptr(), queue.as_mut_ptr().add(len), jdata.len());
        len += jdata.len();

        // A batch of job data is pushed into queue
        let jdata = vec![8, 7, 6, 5, 4, 3, 2, 1];
        mem_copy_for(jdata.as_ptr(), queue.as_mut_ptr().add(len), jdata.len());
        len += jdata.len();

        // Consume first 5 pieces of data from queue
        let mut cbuf = vec![0; 5];
        mem_copy_for(queue.as_ptr(), cbuf.as_mut_ptr(), cbuf.len());
        len -= cbuf.len();
        mem_move_for(queue.as_ptr().add(cbuf.len()), queue.as_mut_ptr(), len);
        assert_eq!(cbuf, vec![1, 2, 3, 4, 3]);

        // Consume next 5 pieces of data from queue
        let mut cbuf = vec![0; 5];
        mem_copy_for(queue.as_ptr(), cbuf.as_mut_ptr(), cbuf.len());
        len -= cbuf.len();
        mem_move_for(queue.as_ptr().add(cbuf.len()), queue.as_mut_ptr(), len);
        assert_eq!(cbuf, vec![2, 1, 8, 7, 6]);

        // Consume last 5 pieces of data from queue
        let mut cbuf = vec![0; 5];
        mem_copy_for(queue.as_ptr(), cbuf.as_mut_ptr(), cbuf.len());
        len -= cbuf.len();
        mem_move_for(queue.as_ptr().add(cbuf.len()), queue.as_mut_ptr(), len);
        assert_eq!(cbuf, vec![5, 4, 3, 2, 1]);

        // Empty queue
        assert_eq!(len, 0);
    }
}
