use crate::validator::is_json_valid;

#[cfg(test)]
#[test]
fn error_invalid_json() {
    assert_eq!(false, is_json_valid("{abcd"));
}

#[test]
fn empty_is_not_valid() {
    assert_eq!(false, is_json_valid(""));
}

#[test]
fn success_valid_json() {
    assert_eq!(true, is_json_valid("{\"abcd\": 2}"));
}
