/// This module heavily on the current 'categories' folder.
/// These tests are expecting only 3 categories: food, fruit and pear.
#[cfg(test)]
use std::rc::Rc;

use cooplan_definitions_lib::category::Category;

use crate::categories::validations::id_tracking_validation::IdTrackingValidation;
use crate::categories::validations::validation::Validation;
use crate::error::ErrorKind;

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
