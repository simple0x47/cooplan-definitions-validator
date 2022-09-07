#[cfg(test)]
#[test]
fn detects_incorrect_data_type() {
    use crate::{
        attributes::{attribute::Attribute, attribute_validator::AttributeValidator},
        error::ErrorKind,
    };

    let valid_data_types: Vec<String> = vec![
        "numeric".to_string(),
        "string".to_string(),
        "date".to_string(),
        "enum".to_string(),
    ];

    let attribute_validator: AttributeValidator = AttributeValidator::new(valid_data_types);

    let incorrect_attribute: Attribute = Attribute {
        id: Some("ABCD".to_string()),
        data_type: "lol".to_string(),
        name: "Invalid Attribute".to_string(),
        unit: None,
        optional: None,
    };

    assert_eq!(
        ErrorKind::InvalidDataType,
        attribute_validator
            .check_attribute_validity(&incorrect_attribute)
            .unwrap_err()
            .kind()
    );
}

#[test]
fn validates_correctly_a_valid_attribute() {
    use crate::attributes::{attribute::Attribute, attribute_validator::AttributeValidator};

    let valid_data_types: Vec<String> = vec![
        "numeric".to_string(),
        "string".to_string(),
        "date".to_string(),
        "enum".to_string(),
    ];

    let attribute_validator: AttributeValidator = AttributeValidator::new(valid_data_types);

    let correct_attribute: Attribute = Attribute {
        id: Some("AAAA".to_string()),
        data_type: "numeric".to_string(),
        name: "Valid Attribute".to_string(),
        unit: None,
        optional: Some(false),
    };

    attribute_validator
        .check_attribute_validity(&correct_attribute)
        .unwrap();
}
