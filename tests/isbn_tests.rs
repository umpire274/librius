use librius::isbn::normalize_isbn;

#[test]
fn test_plain_output() {
    let r = normalize_isbn("978-88-203-8269-8", true).unwrap();
    assert_eq!(r, "9788820382698");
}

#[test]
fn test_hyphenated_output() {
    let r = normalize_isbn("9788820382698", false).unwrap();
    assert_eq!(r, "978-88-203-8269-8");
}

#[test]
fn test_invalid_length() {
    assert!(normalize_isbn("12345", true).is_err());
}

#[test]
fn test_invalid_characters() {
    assert!(normalize_isbn("97A88203826B8", false).is_err());
}
