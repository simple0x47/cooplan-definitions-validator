#[cfg(test)]
#[test]
fn error_invalid_json() {
    use crate::validator::is_json_valid;

    assert_eq!(false, is_json_valid("{abcd"));
}

#[test]
fn empty_is_not_valid() {
    use crate::validator::is_json_valid;

    assert_eq!(false, is_json_valid(""));
}

#[test]
fn success_valid_json() {
    use crate::validator::is_json_valid;

    assert_eq!(true, is_json_valid("{\"abcd\": 2}"));
}
