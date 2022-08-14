#[cfg(test)]
#[test]
fn error_no_id_category() {
    use std::{collections::HashMap, io::ErrorKind};

    use crate::categories::{category::Category, id_tracker::IdTracker};

    let mut tracker = IdTracker::new(HashMap::new());
    let category = Category {
        id: None,
        parent: None,
        name: "".to_string(),
        attributes: Vec::new(),
    };

    assert_eq!(
        ErrorKind::InvalidInput,
        tracker.track_category(&category).unwrap_err().kind()
    );
}

#[test]
fn error_duplicated_id_category() {
    use std::{collections::HashMap, io::ErrorKind};

    use crate::categories::id_tracker::Entry;
    use crate::categories::{category::Category, id_tracker::IdTracker};

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
        ErrorKind::InvalidData,
        tracker.track_category(&category).unwrap_err().kind()
    );
}

#[test]
fn error_id_category_not_found() {
    use std::{collections::HashMap, io::ErrorKind};

    use crate::categories::{category::Category, id_tracker::IdTracker};

    let mut tracker = IdTracker::new(HashMap::new());
    let category = Category {
        id: Some("id".to_string()),
        parent: None,
        name: "".to_string(),
        attributes: Vec::new(),
    };

    assert_eq!(
        ErrorKind::NotFound,
        tracker.track_category(&category).unwrap_err().kind()
    );
}

#[test]
fn track_categories_successfully() {
    use std::{collections::HashMap, io::ErrorKind};

    use crate::categories::id_tracker::Entry;
    use crate::categories::{category::Category, id_tracker::IdTracker};

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
        ErrorKind::NotFound,
        tracker.track_category(&third).unwrap_err().kind()
    );
}

#[test]
fn error_category_parent_not_found() {
    use std::{collections::HashMap, io::ErrorKind};

    use crate::categories::id_tracker::Entry;
    use crate::categories::{category::Category, id_tracker::IdTracker};

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
        ErrorKind::InvalidInput,
        tracker.track_category(&second).unwrap_err().kind()
    );
}

#[test]
fn track_and_close_successfully() {
    use std::{collections::HashMap, io::ErrorKind};

    use crate::categories::id_tracker::Entry;
    use crate::categories::{category::Category, id_tracker::IdTracker};

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
