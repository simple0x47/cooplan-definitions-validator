use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::attributes::attribute::Attribute;
use crate::error::{Error, ErrorKind};

/// Full, in-memory category.
#[derive(Debug, Clone)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub parent: Option<Weak<RefCell<Category>>>,
    pub selectable_as_last: bool,
    pub children: Vec<Rc<RefCell<Category>>>,
    pub attributes: Vec<Attribute>,
}

impl Category {
    pub fn new_into_parent(
        id: String,
        parent: Weak<RefCell<Category>>,
        name: String,
        selectable_as_last: bool,
        attributes: Vec<Attribute>,
    ) -> Result<Rc<RefCell<Category>>, Error> {
        let category = Category {
            id,
            name,
            parent: Some(parent.clone()),
            selectable_as_last,
            children: Vec::new(),
            attributes,
        };

        let category_strong = Rc::new(RefCell::new(category));

        match parent.upgrade() {
            Some(p) => match p.try_borrow_mut() {
                Ok(mut p) => {
                    p.children.push(Rc::clone(&category_strong));

                    Ok(category_strong)
                }
                Err(error) => Err(Error::new(
                    ErrorKind::FailedToBorrowCategory,
                    format!("parent cannot be borrowed: {}", error).as_str(),
                )),
            },
            None => Err(Error::new(
                ErrorKind::ParentNotAvailable,
                "parent is no longer available",
            )),
        }
    }

    pub fn new(
        id: String,
        name: String,
        selectable_as_last: bool,
        attributes: Vec<Attribute>,
    ) -> Rc<RefCell<Category>> {
        Rc::new(RefCell::new(Category {
            id,
            name,
            parent: None,
            selectable_as_last,
            children: Vec::new(),
            attributes,
        }))
    }
}
