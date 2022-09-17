use std::collections::HashMap;

use crate::{
    attributes::attribute_tracker_io::AttributeEntry,
    error::{Error, ErrorKind},
};

use super::validation::Validation;

/// Validates whether an attribute's data type has changed since it was created.
pub struct DataTypeConstantValidation {
    expected_data_types: HashMap<String, String>,
    failed_validations: Vec<Error>,
}

impl DataTypeConstantValidation {
    pub fn new(entries: &HashMap<String, AttributeEntry>) -> DataTypeConstantValidation {
        let mut expected_data_type: HashMap<String, String> = HashMap::new();

        for entry in entries {
            expected_data_type.insert(entry.0.clone(), entry.1.data_type.clone());
        }

        DataTypeConstantValidation {
            expected_data_types: expected_data_type,
            failed_validations: Vec::new(),
        }
    }
}

impl Validation for DataTypeConstantValidation {
    fn partially_validate(
        &mut self,
        attributes: &[cooplan_definitions_lib::attribute::Attribute],
    ) -> Result<(), Error> {
        for attribute in attributes {
            match self.expected_data_types.remove(&attribute.id) {
                Some(expected_data_type) => {
                    if expected_data_type.ne(&attribute.data_type) {
                        self.failed_validations.push(Error::new(
                            ErrorKind::FailedDataTypeConstantAttributeValidation,
                            format!(
                                "attribute '{}' with id '{}' changed its data type from '{}' to '{}'",
                                attribute.name,
                                attribute.id,
                                expected_data_type,
                                attribute.data_type
                            )
                            .as_str(),
                        ));
                    }
                }
                None => {
                    return Err(Error::new(
                        ErrorKind::DuplicatedId,
                        format!("attribute's id '{}' has been validated twice", attribute.id)
                            .as_str(),
                    ))
                }
            }
        }

        Ok(())
    }

    fn complete(&mut self) -> Result<(), Error> {
        if self.failed_validations.is_empty() {
            return Ok(());
        }

        let mut error_accumulation: String = String::new();

        for error in self.failed_validations.as_slice() {
            error_accumulation.push('\n');
            error_accumulation.push_str(error.message.as_str());
        }

        Err(Error::new(
            ErrorKind::FailedDataTypeConstantAttributeValidation,
            &error_accumulation,
        ))
    }
}
