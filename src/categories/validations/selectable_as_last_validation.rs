use std::collections::HashMap;

use crate::categories::category::InMemoryCategory;
use crate::categories::validations::validation::Validation;
use crate::error::{Error, ErrorKind};

/// Validates if the each end category is set as selectable as last by itself or by its parents.
///
/// To be executed only after each category has an ID assigned.
pub struct SelectableAsLastValidation {}

impl SelectableAsLastValidation {
    fn map_categories_graph(&self, categories: &[InMemoryCategory])
                            -> Result<HashMap<String, Vec<String>>, Error> {
        let mut parents: HashMap<String, Vec<String>> = HashMap::new();
        for category in categories {
            match &category.id {
                Some(id) => {
                    match &category.parent {
                        Some(parent) => {
                            match parents.get_mut(parent) {
                                Some(children) => {
                                    if children.contains(id) {
                                        return Err(Error::new(ErrorKind::DuplicatedId,
                                                              format!("parent '{}' already contains id '{}'",
                                                                      parent, id),
                                        ));
                                    }

                                    children.push(id.clone());
                                }
                                None => {
                                    let children_container: Vec<String> = vec![id.clone()];
                                    parents.insert(parent.clone(), children_container);
                                }
                            }
                        }
                        None => continue,
                    }
                }
                None => return Err(Error::new(ErrorKind::MissingId,
                                              format!("category named '{}' has no id", category.name))),
            }
        }

        Ok(parents)
    }

    fn identify_selectable_as_last(&self, categories: &[InMemoryCategory]) -> Result<Vec<String>, Error> {
        let mut selectable_as_last: Vec<String> = Vec::new();

        for category in categories {
            match &category.id {
                Some(id) => {
                    match &category.selectable_as_last {
                        Some(value) => {
                            if *value {
                                selectable_as_last.push(id.clone());
                            }
                        }
                        None => (),
                    }
                }
                None => return Err(Error::new(ErrorKind::MissingId,
                                              format!("category with name '{}' has no id", category.name))),
            }
        }

        Ok(selectable_as_last)
    }

    fn check_for_non_selectable_endings(&self, parents: HashMap<String, Vec<String>>, selectable_as_last: Vec<String>)
                                        -> Result<(), Error> {
        Ok(())
    }
}

impl Validation for SelectableAsLastValidation {
    fn validate(&self, categories: &[InMemoryCategory]) -> Result<(), Error> {
        match self.map_categories_graph(categories) {
            Ok(parents) => {
                match self.identify_selectable_as_last(categories) {
                    Ok(selectable_as_last) => {
                        self.check_for_non_selectable_endings(parents, selectable_as_last)
                    }
                    Err(error) => {
                        Err(error)
                    }
                }
            }
            Err(error) => {
                Err(error)
            }
        }
    }
}