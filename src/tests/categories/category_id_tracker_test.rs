#[cfg(test)]
#[test]
fn error_no_id_category() {
    use crate::categories::{category::Category, category_id_tracker::CategoryIdTracker};
    use crate::error::ErrorKind;
    use std::collections::HashMap;

    let mut tracker = CategoryIdTracker::new(HashMap::new());
    let category = Category {
        id: None,
        parent: None,
        name: "".to_string(),
        attributes: Vec::new(),
    };

    assert_eq!(
        ErrorKind::MissingId,
        tracker.track_category(&category).unwrap_err().kind()
    );
}

#[test]
fn error_duplicated_id_category() {
    use crate::categories::category_id_tracker::CategoryEntry;
    use crate::categories::{category::Category, category_id_tracker::CategoryIdTracker};
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
    let category = Category {
        id: Some("id".to_string()),
        parent: None,
        name: "".to_string(),
        attributes: Vec::new(),
    };

    tracker.track_category(&category).unwrap();
    assert_eq!(
        ErrorKind::DuplicatedId,
        tracker.track_category(&category).unwrap_err().kind()
    );
}

#[test]
fn error_id_category_not_found() {
    use crate::categories::{category::Category, category_id_tracker::CategoryIdTracker};
    use crate::error::ErrorKind;
    use std::collections::HashMap;

    let mut tracker = CategoryIdTracker::new(HashMap::new());
    let category = Category {
        id: Some("id".to_string()),
        parent: None,
        name: "".to_string(),
        attributes: Vec::new(),
    };

    assert_eq!(
        ErrorKind::IdNotFound,
        tracker.track_category(&category).unwrap_err().kind()
    );
}

#[test]
fn track_categories_successfully() {
    use crate::categories::category_id_tracker::CategoryEntry;
    use crate::categories::{category::Category, category_id_tracker::CategoryIdTracker};
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
    let first = Category {
        id: Some("id".to_string()),
        parent: None,
        name: "".to_string(),
        attributes: Vec::new(),
    };

    let second = Category {
        id: Some("id2".to_string()),
        parent: None,
        name: "".to_string(),
        attributes: Vec::new(),
    };

    let third = Category {
        id: Some("id3".to_string()),
        parent: None,
        name: "".to_string(),
        attributes: Vec::new(),
    };

    tracker.track_category(&first).unwrap();
    tracker.track_category(&second).unwrap();
    assert_eq!(
        ErrorKind::IdNotFound,
        tracker.track_category(&third).unwrap_err().kind()
    );
}

#[test]
fn error_category_parent_not_found() {
    use crate::categories::category_id_tracker::CategoryEntry;
    use crate::categories::{category::Category, category_id_tracker::CategoryIdTracker};
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
    let first = Category {
        id: Some("id".to_string()),
        parent: None,
        name: "".to_string(),
        attributes: Vec::new(),
    };

    let second = Category {
        id: Some("id2".to_string()),
        parent: Some("id3".to_string()),
        name: "".to_string(),
        attributes: Vec::new(),
    };
    tracker.track_category(&first).unwrap();
    assert_eq!(
        ErrorKind::ParentNotFound,
        tracker.track_category(&second).unwrap_err().kind()
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
    use crate::categories::{category::Category, category_id_tracker::CategoryIdTracker};
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
    let first = Category {
        id: Some("id".to_string()),
        parent: None,
        name: "".to_string(),
        attributes: Vec::new(),
    };

    let second = Category {
        id: Some("id2".to_string()),
        parent: Some("id".to_string()),
        name: "".to_string(),
        attributes: Vec::new(),
    };

    let third = Category {
        id: Some("id3".to_string()),
        parent: Some("id2".to_string()),
        name: "".to_string(),
        attributes: Vec::new(),
    };

    tracker.track_category(&first).unwrap();
    tracker.track_category(&second).unwrap();
    tracker.track_category(&third).unwrap();
    tracker.close().unwrap();
}
