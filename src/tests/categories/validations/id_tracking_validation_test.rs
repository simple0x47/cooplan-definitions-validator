use std::rc::Rc;

use crate::categories::category::Category;
use crate::categories::validations::id_tracking_validation::IdTrackingValidation;
use crate::categories::validations::validation::Validation;
use crate::error::ErrorKind;

/// This module heavily on the current 'categories' folder.
/// These tests are expecting only 3 categories: food, fruit and pear.
#[cfg(test)]
#[test]
fn error_if_missing_categories() {
    let food = Category::new(
        "9961ed43-919d-4d4a-86d9-92688dc2e12d".to_string(),
        "food".to_string(),
        false,
        Vec::new(),
    );

    let id_tracking_validation = IdTrackingValidation::new();

    assert_eq!(
        ErrorKind::IdNotTracked,
        id_tracking_validation.validate(&[food]).unwrap_err().kind()
    );
}

fn success_if_all_categories_have_been_passed() {
    let food = Category::new(
        "9961ed43-919d-4d4a-86d9-92688dc2e12d".to_string(),
        "food".to_string(),
        false,
        Vec::new(),
    );
    let fruit = Category::new_into_parent(
        "c34976de-3973-474c-a3e7-727b523be171".to_string(),
        Rc::downgrade(&food),
        "fruit".to_string(),
        false,
        Vec::new(),
    )
    .unwrap();
    let pear = Category::new_into_parent(
        "fb8ac0bf-dd1c-4129-bd31-5d8c7528a03b".to_string(),
        Rc::downgrade(&fruit),
        "pear".to_string(),
        true,
        Vec::new(),
    )
    .unwrap();

    let id_tracking_validation = IdTrackingValidation::new();

    id_tracking_validation.validate(&[food]).unwrap();
}
