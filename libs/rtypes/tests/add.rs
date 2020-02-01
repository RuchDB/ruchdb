use rtypes::add;

#[test]
fn add_test() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(3, 3), 6);
    assert_eq!(add(4, 3), 7);
    assert_eq!(add(5, 3), 8);
    assert_eq!(add(6, 3), 9);
}
