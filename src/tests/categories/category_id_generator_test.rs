use std::borrow::Borrow;

use crate::categories::in_memory_category::InMemoryCategory;

#[cfg(test)]
#[test]
fn error_on_setting_random_id_to_category_with_id() {
    use crate::categories::category_id_generator::set_random_id;
    use crate::error::ErrorKind;

    use crate::categories::category::Category;

    let first_category: Category = Category {
        id: Some("ABCD".to_string()),
        parent: None,
        name: "First".to_string(),
        selectable_as_last: Some(false),
        attributes: Vec::new(),
    };

    assert_eq!(
        ErrorKind::CannotOverrideId,
        set_random_id(first_category).unwrap_err().kind()
    );
}

#[test]
fn unique_random_id_constraint() {
    use std::collections::HashMap;

    use crate::categories::{
        category::Category, category_id_generator::set_random_id,
        category_id_tracker::CategoryEntry, category_id_tracker::CategoryIdTracker,
    };

    let mut entries: HashMap<String, CategoryEntry> = HashMap::new();
    let mut categories: Vec<Category> = Vec::new();

    for i in 0..1000 {
        let category: Category = Category {
            id: None,
            parent: None,
            name: format!("{}", i),
            selectable_as_last: Some(false),
            attributes: Vec::new(),
        };

        let category_with_id = set_random_id(category).unwrap();
        let category_id = category_with_id.id.clone().unwrap();

        let entry: CategoryEntry = CategoryEntry {
            id: category_id.clone(),
        };

        entries.insert(category_id.clone(), entry);
        categories.push(category_with_id);
    }

    let mut id_tracker: CategoryIdTracker = CategoryIdTracker::new(entries);

    for category in categories {
        let category_pointer = InMemoryCategory::new(category.id.unwrap(), category.name,
                                                     category.selectable_as_last.unwrap());
        id_tracker.track_category(category_pointer.borrow()).unwrap();
    }

    id_tracker.close().unwrap();
}
