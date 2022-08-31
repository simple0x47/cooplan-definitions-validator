#[cfg(test)]
use std::cell::RefCell;
use std::rc::Rc;

use crate::categories::category::Category;
use crate::categories::validations::selectable_as_last_validation::SelectableAsLastValidation;
use crate::categories::validations::validation::Validation;
use crate::error::ErrorKind;

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

#[test]
fn detects_correct_expanded_category_tree_correctly() {
    use std::rc::Rc;

    use crate::categories::category::Category;

    let root_category = Category::new("root".to_string(), "root".to_string(), false, Vec::new());

    let first_inner_category = Category::new_into_parent(
        "inner 1".to_string(),
        Rc::downgrade(&root_category),
        "inner 1".to_string(),
        false,
        Vec::new(),
    );

    let second_inner_category = Category::new_into_parent(
        "inner 2".to_string(),
        Rc::downgrade(&(first_inner_category.unwrap())),
        "inner 2".to_string(),
        false,
        Vec::new(),
    )
    .unwrap();

    let third_inner_category = Category::new_into_parent(
        "inner 3".to_string(),
        Rc::downgrade(&second_inner_category),
        "inner 3".to_string(),
        true,
        Vec::new(),
    );

    let forth_inner_category = Category::new_into_parent(
        "inner 4".to_string(),
        Rc::downgrade(&(third_inner_category.unwrap())),
        "inner 4".to_string(),
        false,
        Vec::new(),
    );

    let third_alt_inner_category = Category::new_into_parent(
        "inner 3 alt".to_string(),
        Rc::downgrade(&second_inner_category),
        "inner 3 alt".to_string(),
        true,
        Vec::new(),
    );

    let second_alt_inner_category = Category::new_into_parent(
        "inner 2 alt".to_string(),
        Rc::downgrade(&root_category),
        "inner 2 alt".to_string(),
        true,
        Vec::new(),
    );

    let selectable_as_last_validation = SelectableAsLastValidation::new();

    selectable_as_last_validation
        .validate(&[root_category])
        .unwrap();
}

#[test]
fn error_if_there_is_a_non_selectable_last_category() {
    use std::rc::Rc;

    use crate::categories::category::Category;

    let root_category = Category::new("root".to_string(), "root".to_string(), false, Vec::new());

    let first_inner_category = Category::new_into_parent(
        "inner 1".to_string(),
        Rc::downgrade(&root_category),
        "inner 1".to_string(),
        false,
        Vec::new(),
    );

    let second_inner_category = Category::new_into_parent(
        "inner 2".to_string(),
        Rc::downgrade(&(first_inner_category.unwrap())),
        "inner 2".to_string(),
        false,
        Vec::new(),
    )
    .unwrap();

    let third_inner_category = Category::new_into_parent(
        "inner 3".to_string(),
        Rc::downgrade(&second_inner_category),
        "inner 3".to_string(),
        true,
        Vec::new(),
    );

    let forth_inner_category = Category::new_into_parent(
        "inner 4".to_string(),
        Rc::downgrade(&(third_inner_category.unwrap())),
        "inner 4".to_string(),
        false,
        Vec::new(),
    );

    let third_alt_inner_category = Category::new_into_parent(
        "inner 3 alt".to_string(),
        Rc::downgrade(&second_inner_category),
        "inner 3 alt".to_string(),
        true,
        Vec::new(),
    );

    let second_alt_inner_category = Category::new_into_parent(
        "inner 2 alt".to_string(),
        Rc::downgrade(&root_category),
        "inner 2 alt".to_string(),
        false,
        Vec::new(),
    );

    let selectable_as_last_validation = SelectableAsLastValidation::new();

    assert_eq!(
        ErrorKind::LastCategoryNotSelectable,
        selectable_as_last_validation
            .validate(&[root_category])
            .unwrap_err()
            .kind()
    );
}
