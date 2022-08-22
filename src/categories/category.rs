use serde::{Deserialize, Serialize};

use crate::attributes::attribute::Attribute;

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    pub id: Option<String>,
    pub parent: Option<String>,
    pub name: String,
    pub selectable_as_last: Option<bool>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone)]
pub struct InMemoryCategory {
    pub id: String,
    pub parent: Box<InMemoryCategory>,
    pub name: String,
    pub selectable_as_last: bool,
    pub children: Vec<InMemoryCategory>,
}
