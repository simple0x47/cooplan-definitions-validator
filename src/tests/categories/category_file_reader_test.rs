#[cfg(test)]
#[test]
fn reads_only_json_files() {
    use crate::categories::category_file_reader::CategoryFileReader;
    use crate::categories::category_reader::CategoryReader;

    let mut category_file_reader = CategoryFileReader::new();
    let categories = category_file_reader.read_json_category().unwrap();

    for category in categories {
        if category.contains("NOOO! GOD, NOOO GOD!, PLEASE!, NO!, NO!, NO!, NOOOOOOOOOO!") {
            panic!("Read .txt file.");
        }
    }
}

#[test]
fn reads_expected_categories() {
    use crate::categories::category_file_reader::CategoryFileReader;
    use crate::categories::category_reader::CategoryReader;

    let mut category_file_reader = CategoryFileReader::new();
    let categories = category_file_reader.read_json_category().unwrap();

    assert_eq!(categories.len(), 3);

    for category in categories {
        if !(category.contains("food") || category.contains("fruit") || category.contains("pear")) {
            panic!("Unexpected category detected.")
        }
    }
}
