use crate::attributes::attribute::Attribute;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    pub id: Option<String>,
    pub parent: Option<String>,
    pub name: String,
    pub selectable_as_last: Option<bool>,
    pub attributes: Vec<Attribute>,
}
