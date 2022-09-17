use std::borrow::Borrow;
use std::ops::Deref;
use std::rc::Rc;

use cooplan_definitions_lib::category::Category;

#[cfg(test)]
#[test]
fn error_duplicated_id_category() {
    use crate::categories::category_id_tracker::CategoryEntry;
    use crate::categories::category_id_tracker::CategoryIdTracker;
    use crate::error::ErrorKind;
    use std::collections::HashMap;

    let mut entries: HashMap<String, CategoryEntry> = HashMap::new();
    entries.insert(
        "id".to_string(),
        CategoryEntry {
            id: "id".to_string(),
        },
    );

    let mut tracker = CategoryIdTracker::new(entries);
    let category = Category::new("id".to_string(), "id".to_string(), false, Vec::new());

    tracker
        .track_category(category.try_borrow().unwrap().id.as_str())
        .unwrap();
    assert_eq!(
        ErrorKind::DuplicatedId,
        tracker
            .track_category(category.try_borrow().unwrap().id.as_str())
            .unwrap_err()
            .kind()
    );
}

#[test]
fn error_id_category_not_found() {
    use crate::categories::category_id_tracker::CategoryIdTracker;
    use crate::error::ErrorKind;
    use std::collections::HashMap;

    let mut tracker = CategoryIdTracker::new(HashMap::new());
    let category = Category::new("id".to_string(), "id".to_string(), false, Vec::new());

    assert_eq!(
        ErrorKind::IdNotFound,
        tracker
            .track_category(category.try_borrow().unwrap().id.as_str())
            .unwrap_err()
            .kind()
    );
}

#[test]
fn track_categories_successfully() {
    use crate::categories::category_id_tracker::CategoryEntry;
    use crate::categories::category_id_tracker::CategoryIdTracker;
    use crate::error::ErrorKind;
    use std::collections::HashMap;

    let mut entries: HashMap<String, CategoryEntry> = HashMap::new();
    entries.insert(
        "id".to_string(),
        CategoryEntry {
            id: "id".to_string(),
        },
    );

    entries.insert(
        "id2".to_string(),
        CategoryEntry {
            id: "id2".to_string(),
        },
    );

    let mut tracker = CategoryIdTracker::new(entries);
    let first = Category::new("id".to_string(), "id".to_string(), false, Vec::new());

    let second = Category::new("id2".to_string(), "id2".to_string(), false, Vec::new());

    let third = Category::new("id3".to_string(), "id3".to_string(), false, Vec::new());

    tracker
        .track_category(first.try_borrow().unwrap().id.as_str())
        .unwrap();
    tracker
        .track_category(second.try_borrow().unwrap().id.as_str())
        .unwrap();
    assert_eq!(
        ErrorKind::IdNotFound,
        tracker
            .track_category(third.try_borrow().unwrap().id.as_str())
            .unwrap_err()
            .kind()
    );
}

#[test]
fn error_untracked_ids_on_close() {
    use crate::categories::category_id_tracker::CategoryEntry;
    use crate::categories::category_id_tracker::CategoryIdTracker;
    use crate::error::ErrorKind;
    use std::collections::HashMap;

    let mut entries: HashMap<String, CategoryEntry> = HashMap::new();
    entries.insert(
        "id".to_string(),
        CategoryEntry {
            id: "id".to_string(),
        },
    );

    entries.insert(
        "id2".to_string(),
        CategoryEntry {
            id: "id2".to_string(),
        },
    );

    entries.insert(
        "id3".to_string(),
        CategoryEntry {
            id: "id3".to_string(),
        },
    );

    let tracker = CategoryIdTracker::new(entries);

    assert_eq!(ErrorKind::IdNotTracked, tracker.close().unwrap_err().kind());
}

#[test]
fn track_and_close_successfully() {
    use crate::categories::category_id_tracker::CategoryEntry;
    use crate::categories::category_id_tracker::CategoryIdTracker;
    use std::collections::HashMap;

    let mut entries: HashMap<String, CategoryEntry> = HashMap::new();
    entries.insert(
        "id".to_string(),
        CategoryEntry {
            id: "id".to_string(),
        },
    );

    entries.insert(
        "id2".to_string(),
        CategoryEntry {
            id: "id2".to_string(),
        },
    );

    entries.insert(
        "id3".to_string(),
        CategoryEntry {
            id: "id3".to_string(),
        },
    );

    let mut tracker = CategoryIdTracker::new(entries);
    let first = Category::new("id".to_string(), "id".to_string(), false, Vec::new());

    let second = Category::new_into_parent(
        "id2".to_string(),
        Rc::downgrade(&first),
        "id2".to_string(),
        false,
        Vec::new(),
    )
    .unwrap();

    let third = Category::new_into_parent(
        "id3".to_string(),
        Rc::downgrade(&second),
        "id3".to_string(),
        false,
        Vec::new(),
    )
    .unwrap();

    tracker
        .track_category(first.try_borrow().unwrap().id.as_str())
        .unwrap();
    tracker
        .track_category(second.try_borrow().unwrap().id.as_str())
        .unwrap();
    tracker
        .track_category(third.try_borrow().unwrap().id.as_str())
        .unwrap();
    tracker.close().unwrap();
}
