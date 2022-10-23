use cooplan_definitions_lib::attribute::Attribute;

use crate::error::{Error, ErrorKind};

use super::validation::Validation;

pub struct ReservedAttributeNamesValidation {
    reserved_keywords: Vec<String>,
}

impl ReservedAttributeNamesValidation {
    pub fn new(reserved_keywords: Vec<String>) -> ReservedAttributeNamesValidation {
        ReservedAttributeNamesValidation { reserved_keywords }
    }
}

impl Validation for ReservedAttributeNamesValidation {
    fn partially_validate(&mut self, attributes: &[Attribute]) -> Result<(), Error> {
        for attribute in attributes {
            for reserved_keyword in self.reserved_keywords.as_slice() {
                if attribute.name.to_lowercase().eq(reserved_keyword) {
                    return Err(Error::new(
                        ErrorKind::ReservedKeywordUsedAsName,
                        format!(
                            "attribute with id '{}' is using as name a reserved keyword: {}",
                            attribute.id, reserved_keyword
                        )
                        .as_str(),
                    ));
                }
            }
        }

        Ok(())
    }

    fn complete(&mut self) -> Result<(), Error> {
        // This validation does not require a completion call. Therefore, in the end we are always ok :)
        Ok(())
    }
}
