use rtypes::RString;

#[test]
fn create_rstr() {
    assert_eq!(RString::new().len(), 0);
    assert_eq!(RString::default().len(), 0);
    assert_eq!(RString::with_capacity(10).len(), 0);

    const STR: &str = "RString Type";

    let s = RString::from_str(STR);
    assert_eq!(s.len(), STR.len());
    assert_eq!(s.as_bytes(), STR.as_bytes());
    assert_eq!(RString::from_rstr(&s).as_bytes(), STR.as_bytes());
    assert_eq!(RString::from_bytes(STR.as_bytes()).as_bytes(), STR.as_bytes());

    assert_eq!(s.clone().as_bytes(), STR.as_bytes());
}

#[test]
fn cmp_rstrs() {
    assert_eq!(RString::from_str("RString"), RString::from_bytes(b"RString"));
    assert_ne!(RString::from_str("RString"), RString::from_bytes(b"Rust"));

    assert!(RString::from_str("RString") == RString::from_bytes(b"RString"));
    assert!(RString::from_str("RString") <  RString::from_bytes(b"Rust"));
    assert!(RString::from_str("RString") >  RString::from_bytes(b"R"));
}

#[test]
fn basic_ops_on_rstr() {
    let mut s = RString::from_bytes(b"Hello");
    s.append_bytes(b" Rust");
    assert_eq!(s, RString::from_str("Hello Rust"));
    s.replace_rstr(6, &RString::from_str("RString"));
    assert_eq!(s, RString::from_str("Hello RString"));
    s.append_padding(b'!', 1);
    assert_eq!(s, RString::from_str("Hello RString!"));

    assert_eq!(s.lsub_rstr(5), RString::from_str("Hello"));
    assert_eq!(s.rsub_rstr(6), RString::from_str("RString!"));
    assert_eq!(s.sub_rstr(6, s.len() - 1), RString::from_str("RString"));

    s.copy_bytes(b"Hello, Rust!");
    assert_eq!(s, RString::from_str("Hello, Rust!"));
    s.truncate(s.len() - 1);
    assert_eq!(s, RString::from_str("Hello, Rust"));
    s.ltrim(7);
    assert_eq!(s, RString::from_str("Rust"));
    s.clear();
    assert_eq!(s, RString::new());
}
