#[cfg(test)]
#[test]
fn error_on_setting_random_id_to_attribute_with_id() {
    use crate::error::ErrorKind;

    use crate::attributes::{attribute::Attribute, id_generator::set_random_id};

    let first_attribute: Attribute = Attribute {
        id: Some("ABCD".to_string()),
        name: "First".to_string(),
        data_type: "float".to_string(),
        unit: None,
    };

    assert_eq!(
        ErrorKind::CannotOverrideId,
        set_random_id(first_attribute).unwrap_err().kind()
    );
}

#[test]
fn unique_random_id_constraint() {
    use std::collections::HashMap;

    use crate::attributes::{
        attribute::Attribute, id_generator::set_random_id, id_tracker::Entry, id_tracker::IdTracker,
    };

    let mut entries: HashMap<String, Entry> = HashMap::new();
    let mut attributes: Vec<Attribute> = Vec::new();

    for i in 0..1000 {
        let attribute: Attribute = Attribute {
            id: None,
            name: format!("{}", i),
            data_type: "float".to_string(),
            unit: None,
        };

        let attribute_with_id = set_random_id(attribute).unwrap();
        let attribute_id = attribute_with_id.id.clone().unwrap();

        let entry: Entry = Entry {
            id: attribute_id.clone(),
            attribute_type: "float".to_string(),
        };

        entries.insert(attribute_id.clone(), entry);
        attributes.push(attribute_with_id);
    }

    let mut id_tracker: IdTracker = IdTracker::new(entries);

    for attribute in attributes {
        id_tracker.track_attribute(&attribute).unwrap();
    }

    id_tracker.close().unwrap();
}
