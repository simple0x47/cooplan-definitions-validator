use std::collections::HashMap;
use std::io::{Error, ErrorKind};

use crate::attributes::attribute_id_tracker::{AttributeEntry, AttributeIdTracker};
use crate::attributes::attribute_id_tracker_file_io::AttributeIdTrackerFileIO;

use crate::attributes::attribute_id_tracker_io::AttributeIdTrackerIO;
use crate::attributes::attribute_validator::AttributeValidator;
use crate::attributes::attribute_validator_builder::build_attribute_validator;
use crate::categories::category_id_tracker_file_io::CategoryIdTrackerFileIO;

use super::category_file_io::build_for_all_categories;
use super::category_id_tracker::{CategoryEntry, CategoryIdTracker};
use super::category_id_tracker_io::CategoryIdTrackerIO;
use super::category_io::CategoryIO;

pub struct CategoryValidator {
    parent: HashMap<String, Vec<String>>,
    selectable_as_last: Vec<String>,
}

impl CategoryValidator {
    fn new() -> CategoryValidator {
        CategoryValidator {
            parent: HashMap::new(),
            selectable_as_last: Vec::new(),
        }
    }

    fn write_new_ids(&mut self) -> Result<(), Error> {
        let category_id_tracker_io = CategoryIdTrackerFileIO::new();
        let attribute_id_tracker_io = AttributeIdTrackerFileIO::new();

        match build_for_all_categories() {
            Ok(categories_files_io) => {
                for mut category_file_io in categories_files_io {
                    match category_file_io.read() {
                        Ok(mut category) => {
                            let mut was_category_updated: bool = false;

                            match &category.parent {
                                Some(parent_id) => {
                                    if self.selectable_as_last.contains(&parent_id) {
                                        category.selectable_as_last = Some(true);
                                    }
                                }
                                None => (),
                            }

                            match &category.id {
                                Some(id) => match &category.selectable_as_last {
                                    Some(value) => {
                                        if *value {
                                            self.selectable_as_last.push((*id).clone());
                                        } else {
                                            match self.parent.get(id) {
                                                Some(children) => {
                                                    if children.is_empty() {
                                                        return Err(Error::new(
                                                            ErrorKind::InvalidInput,
                                                             format!(
                                                                "[{}] category must be selectable as last or have children",
                                                                category_file_io.path()
                                                            )
                                                        ));
                                                    }
                                                }
                                                None => return Err(Error::new(
                                                    ErrorKind::Unsupported,
                                                     format!(
                                                        "[{}] category's children container has not been initialized previously",
                                                        category_file_io.path()
                                                    )
                                                )),
                                            }
                                        }
                                    }
                                    None => {
                                        match self.parent.get(id) {
                                            Some(children) => {
                                                if children.is_empty() {
                                                    return Err(Error::new(
                                                        ErrorKind::InvalidInput,
                                                         format!(
                                                            "[{}] category must be selectable as last or have children",
                                                            category_file_io.path()
                                                        )
                                                    ));
                                                }
                                            }
                                            None => return Err(Error::new(
                                                ErrorKind::Unsupported,
                                                 format!(
                                                    "[{}] category's children container has not been initialized previously",
                                                    category_file_io.path()
                                                )
                                            )),
                                        }
                                    }
                                },
                                None => {
                                    match crate::categories::category_id_generator::set_random_id(
                                        category,
                                    ) {
                                        Ok(cat) => {
                                            category = cat;
                                            match &category.id {
                                                Some(id) => {
                                                    was_category_updated = true;

                                                    match category_id_tracker_io.write_entry(
                                                        &CategoryEntry { id: id.clone() },
                                                    ) {
                                                        Ok(_) => (),
                                                        Err(error) => {
                                                            return Err(Error::new(
                                                                error.kind(),
                                                                format!(
                                                                    "[{}] {}",
                                                                    category_file_io.path(),
                                                                    error
                                                                ),
                                                            ));
                                                        }
                                                    }
                                                }
                                                None => {
                                                    return Err(Error::new(
                                                        ErrorKind::InvalidData,
                                                        format!(
                                                            "[{}] unexpected: category id is none",
                                                            category_file_io.path()
                                                        ),
                                                    ));
                                                }
                                            }
                                        }
                                        Err(error) => {
                                            return Err(Error::new(
                                                ErrorKind::Other,
                                                format!("[{}] {}", category_file_io.path(), error),
                                            ));
                                        }
                                    }
                                }
                            }

                            let attributes = category.attributes;
                            category.attributes = Vec::new();

                            for mut attribute in attributes {
                                if attribute.id.is_none() {
                                    match crate::attributes::attribute_id_generator::set_random_id(
                                        attribute,
                                    ) {
                                        Ok(attr) => {
                                            attribute = attr;

                                            match &attribute.id {
                                                Some(id) => {
                                                    was_category_updated = true;

                                                    match attribute_id_tracker_io.write_entry(
                                                        &AttributeEntry {
                                                            id: id.clone(),
                                                            attribute_type: attribute
                                                                .data_type
                                                                .clone(),
                                                        },
                                                    ) {
                                                        Ok(_) => (),
                                                        Err(error) => {
                                                            return Err(Error::new(
                                                                error.kind(),
                                                                format!(
                                                                    "[{}] {}",
                                                                    category_file_io.path(),
                                                                    error
                                                                ),
                                                            ));
                                                        }
                                                    }
                                                }
                                                None => {
                                                    return Err(Error::new(
                                                        ErrorKind::InvalidData,
                                                        format!(
                                                            "[{}] unexpected: attribute id is none",
                                                            category_file_io.path()
                                                        ),
                                                    ));
                                                }
                                            }
                                        }
                                        Err(error) => {
                                            return Err(Error::new(
                                                ErrorKind::Other,
                                                format!("[{}] {}", category_file_io.path(), error),
                                            ));
                                        }
                                    }
                                }

                                category.attributes.push(attribute);
                            }

                            if was_category_updated {
                                match category_file_io.write(&category) {
                                    Ok(_) => (),
                                    Err(error) => {
                                        return Err(Error::new(
                                            ErrorKind::Other,
                                            format!("[{}] {}", category_file_io.path(), error),
                                        ));
                                    }
                                }
                            }
                        }
                        Err(error) => {
                            return Err(Error::new(
                                ErrorKind::Other,
                                format!("[{}] {}", category_file_io.path(), error),
                            ));
                        }
                    }
                }

                return Ok(());
            }
            Err(error) => return Err(error),
        }
    }

    fn validate_categories(
        &mut self,
        attributes_validator: AttributeValidator,
    ) -> Result<(), Error> {
        let category_id_tracker_io = CategoryIdTrackerFileIO::new();
        let attribute_id_tracker_io = AttributeIdTrackerFileIO::new();

        match category_id_tracker_io.read_entries() {
            Ok(category_entries) => {
                let mut category_id_tracker = CategoryIdTracker::new(category_entries);

                match attribute_id_tracker_io.read_entries() {
                    Ok(attribute_entries) => {
                        let mut attribute_id_tracker = AttributeIdTracker::new(attribute_entries);

                        match build_for_all_categories() {
                            Ok(categories_files_io) => {
                                for mut category_file_io in categories_files_io {
                                    match category_file_io.read() {
                                        Ok(mut category) => {
                                            match &category.id {
                                                Some(id) => {
                                                    if self.parent.contains_key(id) {
                                                        return Err(Error::new(
                                                            ErrorKind::InvalidData,
                                                            format!(
                                                                "category {} is duplicated",
                                                                id
                                                            ),
                                                        ));
                                                    }

                                                    self.parent.insert((*id).clone(), Vec::new());

                                                    match &category.parent {
                                                        Some(parent_id) => {
                                                            match self.parent.get_mut(parent_id) {
                                                                Some(children) => {
                                                                    children.push((*id).clone());
                                                                }
                                                                None => return Err(Error::new(
                                                                    ErrorKind::InvalidInput,
                                                                    format!("parent category has not been read first")
                                                                    )),
                                                            }
                                                        }
                                                        None => (),
                                                    }
                                                }
                                                None => continue,
                                            }

                                            match category_id_tracker.track_category(&category) {
                                                Ok(_) => {
                                                    let attributes = category.attributes;
                                                    category.attributes = Vec::new();

                                                    for attribute in attributes {
                                                        match attributes_validator
                                                            .check_attribute_validity(&attribute)
                                                        {
                                                            Ok(_) => (),
                                                            Err(error) => {
                                                                return Err(Error::new(
                                                                    ErrorKind::InvalidData,
                                                                    format!(
                                                                        "[{}] {}",
                                                                        category_file_io.path(),
                                                                        error
                                                                    ),
                                                                ));
                                                            }
                                                        }

                                                        if attribute.id.is_none() {
                                                            continue;
                                                        }

                                                        match attribute_id_tracker
                                                            .track_attribute(&attribute)
                                                        {
                                                            Ok(_) => {
                                                                category.attributes.push(attribute);
                                                            }
                                                            Err(error) => {
                                                                return Err(Error::new(
                                                                    ErrorKind::InvalidData,
                                                                    format!(
                                                                        "[{}] {}",
                                                                        category_file_io.path(),
                                                                        error
                                                                    ),
                                                                ));
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(error) => {
                                                    return Err(Error::new(
                                                        ErrorKind::InvalidData,
                                                        format!(
                                                            "[{}] {}",
                                                            category_file_io.path(),
                                                            error
                                                        ),
                                                    ));
                                                }
                                            }
                                        }
                                        Err(error) => {
                                            return Err(Error::new(
                                                ErrorKind::InvalidData,
                                                format!("[{}] {}", category_file_io.path(), error),
                                            ));
                                        }
                                    }
                                }
                            }
                            Err(error) => {
                                return Err(error);
                            }
                        }

                        match category_id_tracker.close() {
                            Ok(_) => match attribute_id_tracker.close() {
                                Ok(_) => return Ok(()),
                                Err(error) => {
                                    return Err(Error::new(ErrorKind::Other, format!("{}", error)));
                                }
                            },
                            Err(error) => {
                                return Err(Error::new(ErrorKind::Other, format!("{}", error)));
                            }
                        }
                    }
                    Err(error) => {
                        return Err(error);
                    }
                }
            }
            Err(error) => {
                return Err(error);
            }
        }
    }
}

pub fn validate_categories() -> Result<(), Error> {
    let mut category_validator = CategoryValidator::new();
    match build_attribute_validator() {
        Ok(attribute_validator) => {
            match category_validator.validate_categories(attribute_validator) {
                Ok(_) => match category_validator.write_new_ids() {
                    Ok(_) => Ok(()),
                    Err(error) => Err(error),
                },
                Err(error) => Err(error),
            }
        }
        Err(error) => Err(error),
    }
}
