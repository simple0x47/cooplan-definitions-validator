#[cfg(test)]
use std::cell::RefCell;
use std::rc::Rc;

use crate::attributes::source_attribute::SourceAttribute;
use crate::categories::category::Category;

#[test]
fn error_on_setting_random_id_to_category_with_id() {
    use crate::categories::category_id_generator::set_random_id;
    use crate::error::ErrorKind;

    use crate::categories::source_category::SourceCategory;

    let mut first_category: SourceCategory = SourceCategory {
        id: Some("ABCD".to_string()),
        parent: None,
        parent_name: None,
        name: "First".to_string(),
        selectable_as_last: Some(false),
        attributes: Vec::new(),
    };

    assert_eq!(
        ErrorKind::CannotOverrideId,
        set_random_id(&mut first_category).unwrap_err().kind()
    );
}

#[test]
fn unique_random_id_constraint() {
    use std::collections::HashMap;

    use crate::categories::{
        category_id_generator::set_random_id, category_id_tracker::CategoryEntry,
        category_id_tracker::CategoryIdTracker, source_category::SourceCategory,
    };

    let mut entries: HashMap<String, CategoryEntry> = HashMap::new();
    let mut categories: Vec<Rc<RefCell<Category>>> = Vec::new();

    for i in 0..1000 {
        let mut source_category: SourceCategory = SourceCategory {
            id: None,
            parent: None,
            parent_name: None,
            name: format!("{}", i),
            selectable_as_last: Some(false),
            attributes: Vec::new(),
        };

        set_random_id(&mut source_category).unwrap();

        let id = source_category.id.clone().unwrap();
        let entry: CategoryEntry = CategoryEntry {
            id: source_category.id.clone().unwrap(),
        };

        entries.insert(id.clone(), entry);
        categories.push(Category::new(
            id.clone(),
            source_category.name,
            source_category.selectable_as_last.unwrap_or(false),
            SourceAttribute::to_attributes(source_category.attributes.as_slice()).unwrap(),
        ));
    }

    let mut id_tracker: CategoryIdTracker = CategoryIdTracker::new(entries);

    for category in categories {
        id_tracker
            .track_category(category.try_borrow().unwrap().id.as_str())
            .unwrap();
    }

    id_tracker.close().unwrap();
}
