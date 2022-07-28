#[cfg(test)]
use crate::attributes::id_tracker::Entry;

#[test]
fn error_no_id_attribute() {
    use std::{collections::HashMap, io::ErrorKind};

    use crate::attributes::{attribute::Attribute, id_tracker::IdTracker};

    let mut id_tracker: IdTracker = IdTracker::new(HashMap::new());
    let no_id_attribute: Attribute = Attribute {
        id: None,
        name: "".to_string(),
        data_type: "".to_string(),
        unit: None,
    };

    assert_eq!(
        ErrorKind::InvalidInput,
        id_tracker
            .track_attribute(&no_id_attribute)
            .unwrap_err()
            .kind()
    );
}

#[test]
fn error_id_attribute_not_found() {
    use std::{collections::HashMap, io::ErrorKind};

    use crate::attributes::{attribute::Attribute, id_tracker::IdTracker};

    let mut id_tracker: IdTracker = IdTracker::new(HashMap::new());
    let attribute: Attribute = Attribute {
        id: Some("ABCD".to_string()),
        name: "".to_string(),
        data_type: "".to_string(),
        unit: None,
    };

    assert_eq!(
        ErrorKind::NotFound,
        id_tracker.track_attribute(&attribute).unwrap_err().kind()
    );
}

#[test]
fn error_duplicated_id_attribute() {
    use std::{collections::HashMap, io::ErrorKind};

    use crate::attributes::{attribute::Attribute, id_tracker::IdTracker};

    let mut entries: HashMap<String, Entry> = HashMap::new();
    entries.insert(
        "ABCD".to_string(),
        Entry {
            id: "ABCD".to_string(),
            attribute_type: "".to_string(),
        },
    );

    let mut id_tracker: IdTracker = IdTracker::new(entries);
    let first_attribute: Attribute = Attribute {
        id: Some("ABCD".to_string()),
        name: "First".to_string(),
        data_type: "".to_string(),
        unit: None,
    };
    let second_attribute: Attribute = Attribute {
        id: Some("ABCD".to_string()),
        name: "Second".to_string(),
        data_type: "".to_string(),
        unit: None,
    };

    id_tracker.track_attribute(&first_attribute).unwrap();
    assert_eq!(
        ErrorKind::InvalidData,
        id_tracker
            .track_attribute(&second_attribute)
            .unwrap_err()
            .kind()
    );
}

#[test]
fn error_missing_ids() {
    use std::{collections::HashMap, io::ErrorKind};

    use crate::attributes::id_tracker::IdTracker;

    let mut entries: HashMap<String, Entry> = HashMap::new();
    entries.insert(
        "ABCD".to_string(),
        Entry {
            id: "ABCD".to_string(),
            attribute_type: "".to_string(),
        },
    );

    let mut id_tracker: IdTracker = IdTracker::new(entries);

    assert_eq!(
        ErrorKind::InvalidData,
        id_tracker.close().unwrap_err().kind()
    );
}

#[test]
fn id_comparison_is_correct() {
    use std::{collections::HashMap, io::ErrorKind};

    use crate::attributes::{attribute::Attribute, id_tracker::IdTracker};

    let mut entries: HashMap<String, Entry> = HashMap::new();
    entries.insert(
        "1234".to_string(),
        Entry {
            id: "1234".to_string(),
            attribute_type: "".to_string(),
        },
    );
    entries.insert(
        "5678".to_string(),
        Entry {
            id: "5678".to_string(),
            attribute_type: "".to_string(),
        },
    );

    let mut id_tracker: IdTracker = IdTracker::new(entries);
    let first_attribute: Attribute = Attribute {
        id: Some("1234".to_string()),
        name: "First".to_string(),
        data_type: "".to_string(),
        unit: None,
    };
    let second_attribute: Attribute = Attribute {
        id: Some("ABCD".to_string()),
        name: "Second".to_string(),
        data_type: "".to_string(),
        unit: None,
    };

    id_tracker.track_attribute(&first_attribute).unwrap();
    // The second attribute must not be found because the id is not in the map.
    assert_eq!(
        ErrorKind::NotFound,
        id_tracker
            .track_attribute(&second_attribute)
            .unwrap_err()
            .kind()
    );
}

#[test]
fn track_and_close_successfully() {
    use std::collections::HashMap;

    use crate::attributes::{attribute::Attribute, id_tracker::IdTracker};

    let mut entries: HashMap<String, Entry> = HashMap::new();
    entries.insert(
        "ABCD".to_string(),
        Entry {
            id: "ABCD".to_string(),
            attribute_type: "".to_string(),
        },
    );

    let mut id_tracker: IdTracker = IdTracker::new(entries);
    let first_attribute: Attribute = Attribute {
        id: Some("ABCD".to_string()),
        name: "First".to_string(),
        data_type: "".to_string(),
        unit: None,
    };

    id_tracker.track_attribute(&first_attribute).unwrap();
    id_tracker.close().unwrap();
}
