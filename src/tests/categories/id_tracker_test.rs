#[cfg(test)]
#[test]
fn error_no_id_category() {
    use crate::categories::{category::Category, id_tracker::IdTracker};
    use crate::error::ErrorKind;
    use std::collections::HashMap;

    let mut tracker = IdTracker::new(HashMap::new());
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
    use crate::categories::id_tracker::Entry;
    use crate::categories::{category::Category, id_tracker::IdTracker};
    use crate::error::ErrorKind;
    use std::collections::HashMap;

    let mut entries: HashMap<String, Entry> = HashMap::new();
    entries.insert(
        "id".to_string(),
        Entry {
            id: "id".to_string(),
        },
    );

    let mut tracker = IdTracker::new(entries);
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
    use crate::categories::{category::Category, id_tracker::IdTracker};
    use crate::error::ErrorKind;
    use std::collections::HashMap;

    let mut tracker = IdTracker::new(HashMap::new());
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
    use crate::categories::id_tracker::Entry;
    use crate::categories::{category::Category, id_tracker::IdTracker};
    use crate::error::ErrorKind;
    use std::collections::HashMap;

    let mut entries: HashMap<String, Entry> = HashMap::new();
    entries.insert(
        "id".to_string(),
        Entry {
            id: "id".to_string(),
        },
    );

    entries.insert(
        "id2".to_string(),
        Entry {
            id: "id2".to_string(),
        },
    );

    let mut tracker = IdTracker::new(entries);
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
    use crate::categories::id_tracker::Entry;
    use crate::categories::{category::Category, id_tracker::IdTracker};
    use crate::error::ErrorKind;
    use std::collections::HashMap;

    let mut entries: HashMap<String, Entry> = HashMap::new();
    entries.insert(
        "id".to_string(),
        Entry {
            id: "id".to_string(),
        },
    );

    entries.insert(
        "id2".to_string(),
        Entry {
            id: "id2".to_string(),
        },
    );

    let mut tracker = IdTracker::new(entries);
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
    use crate::categories::id_tracker::Entry;
    use crate::categories::id_tracker::IdTracker;
    use crate::error::ErrorKind;
    use std::collections::HashMap;

    let mut entries: HashMap<String, Entry> = HashMap::new();
    entries.insert(
        "id".to_string(),
        Entry {
            id: "id".to_string(),
        },
    );

    entries.insert(
        "id2".to_string(),
        Entry {
            id: "id2".to_string(),
        },
    );

    entries.insert(
        "id3".to_string(),
        Entry {
            id: "id3".to_string(),
        },
    );

    let tracker = IdTracker::new(entries);

    assert_eq!(ErrorKind::IdNotTracked, tracker.close().unwrap_err().kind());
}

#[test]
fn track_and_close_successfully() {
    use crate::categories::id_tracker::Entry;
    use crate::categories::{category::Category, id_tracker::IdTracker};
    use std::collections::HashMap;

    let mut entries: HashMap<String, Entry> = HashMap::new();
    entries.insert(
        "id".to_string(),
        Entry {
            id: "id".to_string(),
        },
    );

    entries.insert(
        "id2".to_string(),
        Entry {
            id: "id2".to_string(),
        },
    );

    entries.insert(
        "id3".to_string(),
        Entry {
            id: "id3".to_string(),
        },
    );

    let mut tracker = IdTracker::new(entries);
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
