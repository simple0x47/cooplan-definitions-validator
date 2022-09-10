use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use crate::attributes::attribute_tracker_file_io::AttributeTrackerFileIO;
use crate::attributes::attribute_tracker_io::{AttributeEntry, AttributeTrackerIO};
use crate::attributes::source_attribute::SourceAttribute;
use crate::attributes::validations::data_type_constant_validation::DataTypeConstantValidation;
use crate::attributes::validations::data_type_validation::DataTypeValidation;
use crate::categories::category::Category;
use crate::categories::category_file_io::build_for_all_categories;
use crate::categories::category_id_generator::set_random_id;
use crate::categories::category_id_tracker::CategoryEntry;
use crate::categories::category_id_tracker_file_io::CategoryIdTrackerFileIO;
use crate::categories::category_id_tracker_io::CategoryIdTrackerIO;
use crate::categories::category_io::CategoryIO;
use crate::categories::source_category::SourceCategory;
use crate::categories::validations::id_tracking_validation::IdTrackingValidation;
use crate::categories::validations::selectable_as_last_validation::SelectableAsLastValidation;
use crate::categories::validations::validation::Validation;
use crate::config::Config;
use crate::config_file_reader::ConfigFileReader;
use crate::config_reader::ConfigReader;
use crate::error::{Error, ErrorKind};

const CONFIG_FILE_PATH: &str = "./config.json";

pub struct CI {
    config: Config,
    name_id_links: HashMap<String, String>,
    categories_io: HashMap<String, Box<dyn CategoryIO>>,
    categories_mapping: HashMap<String, Rc<RefCell<Category>>>,
    root_categories: Vec<Rc<RefCell<Category>>>,
}

impl CI {
    pub fn try_new() -> Result<CI, Error> {
        Ok(CI {
            config: {
                let config_reader = ConfigFileReader::new(CONFIG_FILE_PATH);

                match config_reader.read() {
                    Ok(config) => config,
                    Err(error) => {
                        return Err(Error::new(
                            ErrorKind::FailedToReadConfig,
                            format!("failed to read configuration file: {}", error).as_str(),
                        ))
                    }
                }
            },
            name_id_links: HashMap::new(),
            categories_io: HashMap::new(),
            categories_mapping: HashMap::new(),
            root_categories: Vec::new(),
        })
    }

    fn link_name_with_id(&mut self, name: &str, id: &str) -> Result<(), Error> {
        match self.name_id_links.get(name) {
            Some(linked_id) => {
                if !linked_id.as_str().eq(name) {
                    return Err(Error::new(
                        ErrorKind::DuplicatedName,
                        format!(
                            "category name '{}' is duplicated used for the following ids: {}, {}",
                            name, linked_id, name
                        )
                        .as_str(),
                    ));
                }
            }
            None => {
                self.name_id_links
                    .insert(String::from(name), String::from(id));
            }
        }

        Ok(())
    }

    fn read_source_categories(&mut self) -> Result<Vec<SourceCategory>, Error> {
        match build_for_all_categories() {
            Ok(categories_io) => {
                let mut source_categories = Vec::new();

                for mut category_io in categories_io {
                    match category_io.read() {
                        Ok(source_category) => {
                            self.categories_io
                                .insert(source_category.name.clone(), category_io);
                            source_categories.push(source_category);
                        }
                        Err(error) => {
                            return Err(Error::new(
                                ErrorKind::FailedToReadCategory,
                                format!("failed to read category: {}", error).as_str(),
                            ));
                        }
                    }
                }

                Ok(source_categories)
            }
            Err(error) => Err(Error::new(
                ErrorKind::FailedToReadCategory,
                format!("failed to read categories: {}", error).as_str(),
            )),
        }
    }

    fn generate_ids(&mut self, source_categories: &mut Vec<SourceCategory>) -> Result<(), Error> {
        for mut source_category in source_categories {
            match &source_category.id {
                Some(id) => {
                    match self.link_name_with_id(source_category.name.as_str(), id.as_str()) {
                        Ok(_) => (),
                        Err(error) => return Err(error),
                    }
                }
                None => match set_random_id(&mut source_category) {
                    Ok(_) => {
                        if source_category.id.is_none() {
                            return Err(Error::new(
                                ErrorKind::MissingId,
                                "category is missing id after setting a random one",
                            ));
                        }

                        match self.link_name_with_id(
                            source_category.name.as_str(),
                            source_category.id.as_ref().unwrap().as_str(),
                        ) {
                            Ok(_) => (),
                            Err(error) => return Err(error),
                        }

                        match self.update_category_id_tracker(&source_category) {
                            Ok(_) => (),
                            Err(error) => return Err(error),
                        }
                    }
                    Err(error) => return Err(error),
                },
            }

            match self.generate_ids_for_attributes(source_category.attributes.as_mut_slice()) {
                Ok(_) => (),
                Err(error) => return Err(error),
            }
        }

        Ok(())
    }

    fn update_category_id_tracker(&self, source_category: &SourceCategory) -> Result<(), Error> {
        match source_category.id.clone() {
            Some(id) => {
                let entry = CategoryEntry { id };

                let category_id_tracker_io: Box<dyn CategoryIdTrackerIO> =
                    Box::new(CategoryIdTrackerFileIO::new());

                match category_id_tracker_io.write_entry(&entry) {
                    Ok(_) => Ok(()),
                    Err(error) => Err(Error::new(
                        ErrorKind::FailedToWriteCategory,
                        format!("failed to write category's entry: {}", error).as_str(),
                    )),
                }
            }
            None => Err(Error::new(
                ErrorKind::MissingId,
                format!("category '{}' is missing id", source_category.name).as_str(),
            )),
        }
    }

    fn generate_ids_for_attributes(
        &self,
        source_attributes: &mut [SourceAttribute],
    ) -> Result<(), Error> {
        for source_attribute in source_attributes {
            match &source_attribute.id {
                Some(_) => (),
                None => {
                    match crate::attributes::attribute_id_generator::set_random_id(source_attribute)
                    {
                        Ok(_) => (),
                        Err(error) => return Err(error),
                    }

                    match self.update_attribute_tracker(source_attribute) {
                        Ok(_) => (),
                        Err(error) => return Err(error),
                    }
                }
            }
        }

        Ok(())
    }

    fn update_attribute_tracker(&self, source_attribute: &SourceAttribute) -> Result<(), Error> {
        match source_attribute.id.clone() {
            Some(id) => {
                let entry = AttributeEntry {
                    id,
                    data_type: source_attribute.data_type.clone(),
                };

                let attribute_tracker_io: Box<dyn AttributeTrackerIO> =
                    Box::new(AttributeTrackerFileIO::new());

                match attribute_tracker_io.write_entry(&entry) {
                    Ok(_) => (),
                    Err(error) => {
                        return Err(Error::new(
                            ErrorKind::FailedToWriteAttribute,
                            format!("failed to write attribute: {}", error).as_str(),
                        ))
                    }
                }
            }
            None => {
                return Err(Error::new(
                    ErrorKind::MissingId,
                    format!("attribute '{}' has no id", source_attribute.name).as_str(),
                ))
            }
        }
        Ok(())
    }

    fn map_source_categories(
        &mut self,
        source_categories: Vec<SourceCategory>,
    ) -> Result<(), Error> {
        for source_category in source_categories {
            let id = source_category.id.clone().unwrap();

            match self.create_category_from_source(source_category) {
                Ok(category) => {
                    self.categories_mapping.insert(id, category);
                }
                Err(error) => return Err(error),
            }
        }

        Ok(())
    }

    fn create_category_from_source(
        &mut self,
        source_category: SourceCategory,
    ) -> Result<Rc<RefCell<Category>>, Error> {
        match &source_category.id {
            Some(id) => match &source_category.parent_name {
                Some(_) => self.create_category_from_source_with_parent(source_category),
                None => {
                    let attributes =
                        match SourceAttribute::to_attributes(source_category.attributes.as_slice())
                        {
                            Ok(attributes) => attributes,
                            Err(error) => return Err(error),
                        };

                    let category = Category::new(
                        id.clone(),
                        source_category.name,
                        source_category.selectable_as_last.unwrap_or(false),
                        attributes,
                    );

                    self.categories_mapping
                        .insert(id.clone(), Rc::clone(&category));
                    self.root_categories.push(Rc::clone(&category));

                    Ok(category)
                }
            },
            None => Err(Error::new(
                ErrorKind::MissingId,
                "cannot create root category from source with no id",
            )),
        }
    }

    fn create_category_from_source_with_parent(
        &mut self,
        source_category: SourceCategory,
    ) -> Result<Rc<RefCell<Category>>, Error> {
        if source_category.id.is_none() {
            return Err(Error::new(
                ErrorKind::MissingId,
                format!("unexpected source category with no id").as_str(),
            ));
        }

        if source_category.parent_name.is_none() {
            return Err(Error::new(
                ErrorKind::ParentNotFound,
                format!("unexpected parentless source category").as_str(),
            ));
        }

        let parent_name = source_category.parent_name.unwrap();
        let parent_id = match self.name_id_links.get(&parent_name) {
            Some(id) => id,
            None => {
                return Err(Error::new(
                    ErrorKind::IdNotFound,
                    format!("category name '{}' is not linked with an id", parent_name).as_str(),
                ))
            }
        };

        match self.categories_mapping.get(parent_id) {
            Some(parent_category) => {
                let attributes =
                    match SourceAttribute::to_attributes(source_category.attributes.as_slice()) {
                        Ok(attributes) => attributes,
                        Err(error) => return Err(error),
                    };
                Category::new_into_parent(
                    source_category.id.unwrap(),
                    Rc::downgrade(parent_category),
                    source_category.name,
                    source_category.selectable_as_last.unwrap_or(false),
                    attributes,
                )
            }
            None => Err(Error::new(
                ErrorKind::ParentNotAvailable,
                format!("parent '{}' has not been read yet", parent_id).as_str(),
            )),
        }
    }

    fn run_validations(&mut self) -> Result<(), Error> {
        let id_tracking_validation = IdTrackingValidation::new();

        match id_tracking_validation.validate(self.root_categories.as_slice()) {
            Ok(_) => (),
            Err(error) => return Err(error),
        }

        let selectable = SelectableAsLastValidation::new();

        match selectable.validate(self.root_categories.as_slice()) {
            Ok(_) => (),
            Err(error) => return Err(error),
        }

        match self.run_attributes_validations() {
            Ok(_) => (),
            Err(error) => return Err(error),
        }

        Ok(())
    }

    fn run_attributes_validations(&mut self) -> Result<(), Error> {
        match self.initialize_attributes_validations() {
            Ok(attribute_validations) => {
                for category_pointer in self.root_categories.as_slice() {
                    match category_pointer.try_borrow() {
                        Ok(category) => {
                            match self.run_attributes_validations_for_category(
                                category,
                                attribute_validations.as_slice(),
                            ) {
                                Ok(_) => (),
                                Err(error) => return Err(error),
                            }
                        }
                        Err(error) => {
                            return Err(Error::new(
                                ErrorKind::FailedToBorrowCategory,
                                format!("failed to borrow category: {}", error).as_str(),
                            ))
                        }
                    }
                }

                for attribute_validation_pointer in attribute_validations {
                    match attribute_validation_pointer.try_borrow_mut() {
                        Ok(mut attribute_validation) => match attribute_validation.complete() {
                            Ok(_) => (),
                            Err(error) => return Err(error),
                        },
                        Err(error) => {
                            return Err(Error::new(
                                ErrorKind::FailedToBorrowAttributeValidation,
                                format!("failed to borrow attribute validation: {}", error)
                                    .as_str(),
                            ))
                        }
                    }
                }

                Ok(())
            }
            Err(error) => Err(error),
        }
    }

    fn initialize_attributes_validations(
        &self,
    ) -> Result<Vec<Rc<RefCell<dyn crate::attributes::validations::validation::Validation>>>, Error>
    {
        let mut validations: Vec<
            Rc<RefCell<dyn crate::attributes::validations::validation::Validation>>,
        > = Vec::new();

        let attribute_entries = AttributeTrackerFileIO::new();

        match attribute_entries.read_entries() {
            Ok(entries) => {
                validations.push(Rc::new(RefCell::new(crate::attributes::validations::id_tracking_validation::IdTrackingValidation::new(&entries))));
                validations.push(Rc::new(RefCell::new(DataTypeConstantValidation::new(
                    &entries,
                ))));
            }
            Err(error) => {
                return Err(Error::new(
                    ErrorKind::FailedToReadAttribute,
                    format!("failed to read attributes' entries: {}", error).as_str(),
                ))
            }
        }

        validations.push(Rc::new(RefCell::new(DataTypeValidation::new(
            self.config.valid_data_types(),
        ))));

        Ok(validations)
    }

    fn run_attributes_validations_for_category(
        &self,
        category: Ref<Category>,
        attribute_validations: &[Rc<
            RefCell<dyn crate::attributes::validations::validation::Validation>,
        >],
    ) -> Result<(), Error> {
        for attribute_validation_pointer in attribute_validations {
            match attribute_validation_pointer.try_borrow_mut() {
                Ok(mut attribute_validation) => {
                    match attribute_validation.partially_validate(category.attributes.as_slice()) {
                        Ok(_) => (),
                        Err(error) => return Err(error),
                    }
                }
                Err(error) => {
                    return Err(Error::new(
                        ErrorKind::FailedToBorrowAttributeValidation,
                        format!("failed to borrow attribute validation: {}", error).as_str(),
                    ))
                }
            }
        }

        for child_category_pointer in category.children.as_slice() {
            match child_category_pointer.try_borrow() {
                Ok(child_category) => {
                    match self.run_attributes_validations_for_category(
                        child_category,
                        attribute_validations,
                    ) {
                        Ok(_) => (),
                        Err(error) => return Err(error),
                    }
                }
                Err(error) => {
                    return Err(Error::new(
                        ErrorKind::FailedToBorrowCategory,
                        format!("failed to borrow category: {}", error).as_str(),
                    ));
                }
            }
        }

        Ok(())
    }

    fn apply_changes(&mut self) -> Result<(), Error> {
        for root_category in self.root_categories.as_slice() {
            match self.apply_changes_for_category(root_category) {
                Ok(_) => (),
                Err(error) => return Err(error),
            }
        }

        Ok(())
    }

    fn apply_changes_for_category(&self, category: &Rc<RefCell<Category>>) -> Result<(), Error> {
        match category.try_borrow() {
            Ok(borrowed_category) => {
                match self.categories_io.get(&borrowed_category.name.clone()) {
                    Some(category_io) => match category_io.write(category) {
                        Ok(_) => (),
                        Err(error) => {
                            return Err(Error::new(
                                ErrorKind::FailedToWriteCategory,
                                format!(
                                    "failed to write category '{}' with id '{}': {}",
                                    borrowed_category.name, borrowed_category.id, error
                                )
                                .as_str(),
                            ))
                        }
                    },
                    None => {
                        return Err(Error::new(
                            ErrorKind::MissingCategoryIO,
                            format!(
                                "category '{}' with id '{}' has no category io mapped",
                                borrowed_category.name, borrowed_category.id
                            )
                            .as_str(),
                        ))
                    }
                }

                for child_category in borrowed_category.children.as_slice() {
                    match self.apply_changes_for_category(child_category) {
                        Ok(_) => (),
                        Err(error) => return Err(error),
                    }
                }

                Ok(())
            }
            Err(error) => Err(Error::new(
                ErrorKind::FailedToBorrowCategory,
                format!("failed to borrow category: {}", error).as_str(),
            )),
        }
    }

    pub fn run_ci_logic(&mut self) -> Result<(), Error> {
        match self.read_source_categories() {
            Ok(mut source_categories) => match self.generate_ids(&mut source_categories) {
                Ok(_) => match self.map_source_categories(source_categories) {
                    Ok(_) => match self.apply_changes() {
                        Ok(_) => match self.run_validations() {
                            Ok(_) => Ok(()),
                            Err(error) => Err(error),
                        },
                        Err(error) => Err(error),
                    },
                    Err(error) => Err(error),
                },
                Err(error) => Err(error),
            },
            Err(error) => Err(error),
        }
    }
}
