use crate::attributes::attribute::Attribute;

pub struct AttributeValidator {
    valid_data_types: Vec<String>,
}

impl AttributeValidator {
    pub fn new(valid_data_types: Vec<String>) -> AttributeValidator {
        AttributeValidator {
            valid_data_types: valid_data_types,
        }
    }

    fn is_data_type_valid(&self, attribute: &Attribute) -> bool {
        for valid_data_type in &self.valid_data_types {
            if valid_data_type.eq(&attribute.data_type) {
                return true;
            }
        }

        false
    }

    pub fn is_attribute_valid(&self, attribute: &Attribute) -> bool {
        self.is_data_type_valid(attribute)
    }
}
