#[cfg(test)]
use crate::attributes::attribute_tracker_io::AttributeEntry;

#[test]
fn error_id_attribute_not_found() {
    use crate::attributes::attribute_tracker_io::AttributeEntry;
    use crate::attributes::{
        attribute_id_tracker::AttributeIdTracker, source_attribute::SourceAttribute,
    };
    use crate::error::ErrorKind;
    use std::collections::HashMap;

    let no_entries: HashMap<String, AttributeEntry> = HashMap::new();
    let mut id_tracker: AttributeIdTracker = AttributeIdTracker::new(&no_entries);
    let attribute: SourceAttribute = SourceAttribute {
        id: Some("ABCD".to_string()),
        name: "".to_string(),
        data_type: "".to_string(),
        unit: None,
        optional: Some(false),
    };

    assert_eq!(
        ErrorKind::IdNotFound,
        id_tracker
            .track_attribute(attribute.id.unwrap().as_str())
            .unwrap_err()
            .kind()
    );
}

#[test]
fn error_duplicated_id_attribute() {
    use crate::attributes::{
        attribute_id_tracker::AttributeIdTracker, source_attribute::SourceAttribute,
    };
    use crate::error::ErrorKind;
    use std::collections::HashMap;

    let mut entries: HashMap<String, AttributeEntry> = HashMap::new();
    entries.insert(
        "ABCD".to_string(),
        AttributeEntry {
            id: "ABCD".to_string(),
            data_type: "".to_string(),
        },
    );

    let mut id_tracker: AttributeIdTracker = AttributeIdTracker::new(&entries);
    let first_attribute: SourceAttribute = SourceAttribute {
        id: Some("ABCD".to_string()),
        name: "First".to_string(),
        data_type: "".to_string(),
        unit: None,
        optional: Some(false),
    };
    let second_attribute: SourceAttribute = SourceAttribute {
        id: Some("ABCD".to_string()),
        name: "Second".to_string(),
        data_type: "".to_string(),
        unit: None,
        optional: Some(false),
    };

    id_tracker
        .track_attribute(first_attribute.id.unwrap().as_str())
        .unwrap();
    assert_eq!(
        ErrorKind::DuplicatedId,
        id_tracker
            .track_attribute(second_attribute.id.unwrap().as_str())
            .unwrap_err()
            .kind()
    );
}

#[test]
fn error_missing_ids() {
    use crate::attributes::attribute_id_tracker::AttributeIdTracker;
    use crate::error::ErrorKind;
    use std::collections::HashMap;

    let mut entries: HashMap<String, AttributeEntry> = HashMap::new();
    entries.insert(
        "ABCD".to_string(),
        AttributeEntry {
            id: "ABCD".to_string(),
            data_type: "".to_string(),
        },
    );

    let id_tracker: AttributeIdTracker = AttributeIdTracker::new(&entries);

    assert_eq!(
        ErrorKind::IdNotTracked,
        id_tracker.close().unwrap_err().kind()
    );
}

#[test]
fn id_comparison_is_correct() {
    use crate::attributes::{
        attribute_id_tracker::AttributeIdTracker, source_attribute::SourceAttribute,
    };
    use crate::error::ErrorKind;
    use std::collections::HashMap;

    let mut entries: HashMap<String, AttributeEntry> = HashMap::new();
    entries.insert(
        "1234".to_string(),
        AttributeEntry {
            id: "1234".to_string(),
            data_type: "".to_string(),
        },
    );
    entries.insert(
        "5678".to_string(),
        AttributeEntry {
            id: "5678".to_string(),
            data_type: "".to_string(),
        },
    );

    let mut id_tracker: AttributeIdTracker = AttributeIdTracker::new(&entries);
    let first_attribute: SourceAttribute = SourceAttribute {
        id: Some("1234".to_string()),
        name: "First".to_string(),
        data_type: "".to_string(),
        unit: None,
        optional: Some(false),
    };
    let second_attribute: SourceAttribute = SourceAttribute {
        id: Some("ABCD".to_string()),
        name: "Second".to_string(),
        data_type: "".to_string(),
        unit: None,
        optional: Some(false),
    };

    id_tracker
        .track_attribute(first_attribute.id.unwrap().as_str())
        .unwrap();
    // The second attribute must not be found because the id is not in the map.
    assert_eq!(
        ErrorKind::IdNotFound,
        id_tracker
            .track_attribute(second_attribute.id.unwrap().as_str())
            .unwrap_err()
            .kind()
    );
}

#[test]
fn track_and_close_successfully() {
    use std::collections::HashMap;

    use crate::attributes::{
        attribute_id_tracker::AttributeIdTracker, source_attribute::SourceAttribute,
    };

    let mut entries: HashMap<String, AttributeEntry> = HashMap::new();
    entries.insert(
        "ABCD".to_string(),
        AttributeEntry {
            id: "ABCD".to_string(),
            data_type: "".to_string(),
        },
    );

    let mut id_tracker: AttributeIdTracker = AttributeIdTracker::new(&entries);
    let first_attribute: SourceAttribute = SourceAttribute {
        id: Some("ABCD".to_string()),
        name: "First".to_string(),
        data_type: "".to_string(),
        unit: None,
        optional: Some(false),
    };

    id_tracker
        .track_attribute(first_attribute.id.unwrap().as_str())
        .unwrap();
    id_tracker.close().unwrap();
}
