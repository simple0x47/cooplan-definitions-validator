#[cfg(test)]
#[test]
fn detects_reserved_keywords_within_attributes_names() {
    use cooplan_definitions_lib::attribute::Attribute;

    use crate::attributes::validations::{
        reserved_attribute_names_validation::ReservedAttributeNamesValidation,
        validation::Validation,
    };

    let mut attributes: Vec<Attribute> = Vec::new();

    let example_attribute: Attribute = Attribute {
        id: "1".to_string(),
        name: "1".to_string(),
        data_type: "string".to_string(),
        unit: None,
        optional: false,
    };
    attributes.push(example_attribute);

    let incorrect_attribute: Attribute = Attribute {
        id: "2".to_string(),
        name: "type".to_string(),
        data_type: "string".to_string(),
        unit: None,
        optional: false,
    };
    attributes.push(incorrect_attribute);

    let reserved_keywords: Vec<String> = vec!["type".to_string(), "version".to_string()];
    let mut validation = ReservedAttributeNamesValidation::new(reserved_keywords);

    assert!(validation
        .partially_validate(attributes.as_slice())
        .is_err());
}

#[test]
fn no_false_positives() {
    use cooplan_definitions_lib::attribute::Attribute;

    use crate::attributes::validations::{
        reserved_attribute_names_validation::ReservedAttributeNamesValidation,
        validation::Validation,
    };

    let mut attributes: Vec<Attribute> = Vec::new();

    let example_attribute: Attribute = Attribute {
        id: "1".to_string(),
        name: "1".to_string(),
        data_type: "string".to_string(),
        unit: None,
        optional: false,
    };
    attributes.push(example_attribute);

    let incorrect_attribute: Attribute = Attribute {
        id: "2".to_string(),
        name: "tipe".to_string(),
        data_type: "string".to_string(),
        unit: None,
        optional: false,
    };
    attributes.push(incorrect_attribute);

    let reserved_keywords: Vec<String> = vec!["type".to_string(), "version".to_string()];
    let mut validation = ReservedAttributeNamesValidation::new(reserved_keywords);

    assert!(validation.partially_validate(attributes.as_slice()).is_ok());
}
