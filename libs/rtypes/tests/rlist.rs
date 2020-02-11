use rtypes::RList;

#[test]
fn basic() {
    let mut list = RList::new();

    // Check empty list behaves right
    assert_eq!(list.pop_front(), None);

    // Populate list
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    // Check normal removal
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), Some(2));

    // Push some more just to make sure nothing's corrupted
    list.push_front(4);
    list.push_front(5);

    // Check normal removal
    assert_eq!(list.pop_front(), Some(5));
    assert_eq!(list.pop_front(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), None);

    // ---- back -----

    // Check empty list behaves right
    assert_eq!(list.pop_back(), None);

    // Populate list
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    // Check normal removal
    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_back(), Some(2));

    // Push some more just to make sure nothing's corrupted
    list.push_back(4);
    list.push_back(5);

    // Check normal removal
    assert_eq!(list.pop_back(), Some(5));
    assert_eq!(list.pop_back(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), None);
}

#[test]
fn insert() {
    let mut list = RList::new();
    for i in 0..9 {
        list.push_back(i);
    }
    list.insert_after(4, 9);
    assert_eq!(list.get(5), Some(9));
    list.insert_before(1, 9);
    assert_eq!(list.get(1), Some(9));
    assert_eq!(list.range(1..4), vec![9, 1, 2]);
    list.trim(1..5);
    assert_eq!(list.to_vec(), vec![9, 1, 2, 3]);
    assert_eq!(list.remove(2), Some(2));
    assert_eq!(list.to_vec(), vec![9, 1, 3]);
}
