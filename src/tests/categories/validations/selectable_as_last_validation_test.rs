#[cfg(test)]
use std::cell::RefCell;
use std::rc::Rc;

use crate::categories::category::Category;
use crate::categories::validations::selectable_as_last_validation::SelectableAsLastValidation;
use crate::categories::validations::validation::Validation;

#[test]
fn error_if_not_selectable_as_last_and_has_no_children() {
    let invalid_category = Category::new("ABCD".to_string(), "ABCD".to_string(), false, Vec::new());

    let slice: &[Rc<RefCell<Category>>] = &[invalid_category];

    let selectable_as_last_validation = SelectableAsLastValidation::new();

    selectable_as_last_validation.validate(slice).unwrap_err();
}

#[test]
fn error_if_children_not_selectable_as_last() {
    let mut parent_category =
        Category::new("ABCD".to_string(), "ABCD".to_string(), false, Vec::new());

    match Category::new_into_parent(
        "C1".to_string(),
        Rc::downgrade(&parent_category),
        "C1".to_string(),
        false,
        Vec::new(),
    ) {
        Ok(_) => (),
        Err(error) => {
            panic!("{}", error);
        }
    }

    match Category::new_into_parent(
        "C2".to_string(),
        Rc::downgrade(&parent_category),
        "C2".to_string(),
        false,
        Vec::new(),
    ) {
        Ok(_) => (),
        Err(error) => {
            panic!("{}", error);
        }
    }

    let selectable_as_last_validation = SelectableAsLastValidation::new();

    selectable_as_last_validation
        .validate(&[parent_category])
        .unwrap_err();
}

#[test]
fn detects_correct_category_correctly() {
    let valid_category = Category::new("ABCD".to_string(), "ABCD".to_string(), true, Vec::new());

    let selectable_as_last_validation = SelectableAsLastValidation::new();

    selectable_as_last_validation
        .validate(&[valid_category])
        .unwrap();
}
