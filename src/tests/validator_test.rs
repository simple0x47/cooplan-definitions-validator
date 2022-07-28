use crate::validator::validate;

#[cfg(test)]
#[test]
fn error_invalid_json() {
    assert_eq!(false, validate("{abcd"));
}

#[test]
fn empty_is_not_valid() {
    assert_eq!(false, validate(""));
}

#[test]
fn success_valid_json() {
    assert_eq!(true, validate("{\"abcd\": 2}"));
}
