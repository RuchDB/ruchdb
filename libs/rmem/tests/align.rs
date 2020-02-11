use rmem::{align_of, size_of, size_of_aligned, size_of_sys_aligned};

#[test]
fn calc_size_of_type() {
    assert_eq!(size_of::<bool>(), 1);

    assert_eq!(size_of::<char>(), 4);

    assert_eq!(size_of::<u8>(), 1);
    assert_eq!(size_of::<u16>(), 2);
    assert_eq!(size_of::<u32>(), 4);
    assert_eq!(size_of::<u64>(), 8);

    assert_eq!(size_of::<i8>(), 1);
    assert_eq!(size_of::<i16>(), 2);
    assert_eq!(size_of::<i32>(), 4);
    assert_eq!(size_of::<i64>(), 8);

    assert_eq!(size_of::<f32>(), 4);
    assert_eq!(size_of::<f64>(), 8);
}

#[test]
fn calc_align_of_type() {
    assert_eq!(align_of::<bool>(), 1);

    assert_eq!(align_of::<char>(), 4);

    assert_eq!(align_of::<u8>(), 1);
    assert_eq!(align_of::<u16>(), 2);
    assert_eq!(align_of::<u32>(), 4);
    assert_eq!(align_of::<u64>(), 8);

    assert_eq!(align_of::<i8>(), 1);
    assert_eq!(align_of::<i16>(), 2);
    assert_eq!(align_of::<i32>(), 4);
    assert_eq!(align_of::<i64>(), 8);

    assert_eq!(align_of::<f32>(), 4);
    assert_eq!(align_of::<f64>(), 8);
}

#[test]
fn calc_aligned_size() {
    assert_eq!(size_of_aligned(1, 1), 1);
    assert_eq!(size_of_aligned(3, 1), 3);
    assert_eq!(size_of_aligned(4, 1), 4);
    assert_eq!(size_of_aligned(5, 1), 5);
    assert_eq!(size_of_aligned(8, 1), 8);

    assert_eq!(size_of_aligned(1, 4), 4);
    assert_eq!(size_of_aligned(3, 4), 4);
    assert_eq!(size_of_aligned(4, 4), 4);
    assert_eq!(size_of_aligned(5, 4), 8);
    assert_eq!(size_of_aligned(8, 4), 8);

    assert_eq!(size_of_aligned(1, 8), 8);
    assert_eq!(size_of_aligned(3, 8), 8);
    assert_eq!(size_of_aligned(4, 8), 8);
    assert_eq!(size_of_aligned(5, 8), 8);
    assert_eq!(size_of_aligned(8, 8), 8);
}

#[test]
fn calc_sys_aligned_size() {
    if cfg!(target_pointer_width = "32") {
        assert_eq!(size_of_sys_aligned(1), 4);
        assert_eq!(size_of_sys_aligned(3), 4);
        assert_eq!(size_of_sys_aligned(4), 4);
        assert_eq!(size_of_sys_aligned(5), 8);
        assert_eq!(size_of_sys_aligned(8), 8);
    }

    if cfg!(target_pointer_width = "64") {
        assert_eq!(size_of_sys_aligned(1), 8);
        assert_eq!(size_of_sys_aligned(3), 8);
        assert_eq!(size_of_sys_aligned(4), 8);
        assert_eq!(size_of_sys_aligned(5), 8);
        assert_eq!(size_of_sys_aligned(8), 8);
    }
}
