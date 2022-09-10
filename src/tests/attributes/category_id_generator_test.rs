use crate::attributes::attribute_tracker_io::AttributeEntry;

#[cfg(test)]
#[test]
fn error_on_setting_random_id_to_attribute_with_id() {
    use crate::error::ErrorKind;

    use crate::attributes::{
        attribute_id_generator::set_random_id, source_attribute::SourceAttribute,
    };

    let mut first_attribute: SourceAttribute = SourceAttribute {
        id: Some("ABCD".to_string()),
        name: "First".to_string(),
        data_type: "float".to_string(),
        unit: None,
        optional: Some(false),
    };

    assert_eq!(
        ErrorKind::CannotOverrideId,
        set_random_id(&mut first_attribute).unwrap_err().kind()
    );
}

#[test]
fn unique_random_id_constraint() {
    use std::collections::HashMap;

    use crate::attributes::{
        attribute_id_generator::set_random_id, attribute_id_tracker::AttributeIdTracker,
        source_attribute::SourceAttribute,
    };

    let mut entries: HashMap<String, AttributeEntry> = HashMap::new();
    let mut attributes: Vec<SourceAttribute> = Vec::new();

    for i in 0..1000 {
        let mut attribute: SourceAttribute = SourceAttribute {
            id: None,
            name: format!("{}", i),
            data_type: "float".to_string(),
            unit: None,
            optional: Some(false),
        };

        set_random_id(&mut attribute).unwrap();
        let attribute_id = attribute.id.clone().unwrap();

        let entry: AttributeEntry = AttributeEntry {
            id: attribute_id.clone(),
            data_type: "float".to_string(),
        };

        entries.insert(attribute_id.clone(), entry);
        attributes.push(attribute);
    }

    let mut id_tracker: AttributeIdTracker = AttributeIdTracker::new(&entries);

    for attribute in attributes {
        id_tracker
            .track_attribute(attribute.id.unwrap().as_str())
            .unwrap();
    }

    id_tracker.close().unwrap();
}
