use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::categories::category::Category;
use crate::categories::category_file_io::build_for_all_categories;
use crate::categories::category_id_generator::set_random_id;
use crate::categories::category_io::CategoryIO;
use crate::categories::source_category::SourceCategory;
use crate::categories::validations::id_tracking_validation::IdTrackingValidation;
use crate::categories::validations::selectable_as_last_validation::SelectableAsLastValidation;
use crate::categories::validations::validation::Validation;
use crate::error::{Error, ErrorKind};

pub struct Validator {
    name_id_links: HashMap<String, String>,
    categories_io: HashMap<String, Box<dyn CategoryIO>>,
    categories_mapping: HashMap<String, Rc<RefCell<Category>>>,
    root_categories: Vec<Rc<RefCell<Category>>>,
}

impl Validator {
    pub fn new() -> Validator {
        Validator {
            name_id_links: HashMap::new(),
            categories_io: HashMap::new(),
            categories_mapping: HashMap::new(),
            root_categories: Vec::new(),
        }
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

    fn track_existing_ids(&mut self) -> Result<Vec<SourceCategory>, Error> {
        match build_for_all_categories() {
            Ok(categories_io) => {
                let mut category_pointers: Vec<Rc<RefCell<Category>>> = Vec::new();

                let mut source_categories = Vec::new();

                for mut category_io in categories_io {
                    match category_io.read() {
                        Ok(source_category) => {
                            match source_category.id.clone() {
                                Some(id) => {
                                    category_pointers.push(Category::new(
                                        id.clone(),
                                        source_category.name.clone(),
                                        false,
                                        source_category.attributes.clone(),
                                    ));

                                    match self.link_name_with_id(
                                        source_category.name.as_str(),
                                        id.as_str(),
                                    ) {
                                        Ok(_) => (),
                                        Err(error) => return Err(error),
                                    }
                                }
                                None => (),
                            }

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

                let id_tracking_validation = IdTrackingValidation::new();

                match id_tracking_validation.validate(category_pointers.as_slice()) {
                    Ok(_) => Ok(source_categories),
                    Err(error) => Err(error),
                }
            }
            Err(error) => Err(Error::new(
                ErrorKind::FailedToReadCategory,
                format!("failed to read categories: {}", error).as_str(),
            )),
        }
    }

    fn generate_ids_for_new_categories(
        &mut self,
        source_categories: &mut Vec<SourceCategory>,
    ) -> Result<(), Error> {
        for mut source_category in source_categories {
            match &source_category.id {
                Some(_) => (),
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
                    }
                    Err(error) => return Err(error),
                },
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
            Some(id) => match &source_category.parent {
                Some(_) => self.create_category_from_source_with_parent(source_category),
                None => {
                    let category = Category::new(
                        id.clone(),
                        source_category.name,
                        source_category.selectable_as_last.unwrap_or(false),
                        source_category.attributes,
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

        if source_category.parent.is_none() {
            return Err(Error::new(
                ErrorKind::ParentNotFound,
                format!("unexpected parentless source category").as_str(),
            ));
        }

        let parent_id = source_category.parent.unwrap();

        match self.categories_mapping.get(&parent_id) {
            Some(parent_category) => Category::new_into_parent(
                source_category.id.unwrap(),
                Rc::downgrade(parent_category),
                source_category.name,
                source_category.selectable_as_last.unwrap_or(false),
                source_category.attributes,
            ),
            None => Err(Error::new(
                ErrorKind::ParentNotAvailable,
                format!("parent '{}' has not been read yet", parent_id).as_str(),
            )),
        }
    }

    fn selectable_as_last_validation(&mut self) -> Result<(), Error> {
        let selectable = SelectableAsLastValidation::new();

        selectable.validate(self.root_categories.as_slice())
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

    pub fn validate(&mut self) -> Result<(), Error> {
        match self.track_existing_ids() {
            Ok(mut source_categories) => {
                match self.generate_ids_for_new_categories(&mut source_categories) {
                    Ok(_) => match self.map_source_categories(source_categories) {
                        Ok(_) => match self.selectable_as_last_validation() {
                            Ok(_) => self.apply_changes(),
                            Err(error) => Err(error),
                        },
                        Err(error) => Err(error),
                    },
                    Err(error) => Err(error),
                }
            }
            Err(error) => Err(error),
        }
    }
}
