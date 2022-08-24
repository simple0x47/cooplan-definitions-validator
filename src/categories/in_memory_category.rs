use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::error::{Error, ErrorKind};

#[derive(Debug, Clone)]
pub struct InMemoryCategory {
    pub id: String,
    pub name: String,
    pub parent: Option<Weak<RefCell<InMemoryCategory>>>,
    pub selectable_as_last: bool,
    pub children: Vec<Rc<RefCell<InMemoryCategory>>>,
}

impl InMemoryCategory {
    pub fn new_into_parent(id: String, parent: Weak<RefCell<InMemoryCategory>>, name: String, selectable_as_last: bool)
                           -> Result<Rc<RefCell<InMemoryCategory>>, Error> {
        let mut category = InMemoryCategory {
            id,
            name,
            parent: Some(parent.clone()),
            selectable_as_last,
            children: Vec::new(),
        };

        let category_strong = Rc::new(RefCell::new(category));

        match parent.upgrade() {
            Some(p) => {
                match p.try_borrow_mut() {
                    Ok(mut p) => {
                        p.children.push(Rc::clone(&category_strong));

                        Ok(category_strong)
                    }
                    Err(error) => Err(Error::new(ErrorKind::FailedToBorrowCategory,
                                                 format!("parent cannot be borrowed: {}", error).as_str())),
                }
            }
            None => Err(Error::new(ErrorKind::ParentNotAvailable, "parent is no longer available")),
        }
    }

    pub fn new(id: String, name: String, selectable_as_last: bool) -> Rc<RefCell<InMemoryCategory>> {
        Rc::new(RefCell::new(InMemoryCategory {
            id,
            name,
            parent: None,
            selectable_as_last,
            children: Vec::new(),
        }))
    }
}