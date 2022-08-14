use crate::attributes::attribute::Attribute;

pub struct Category {
    pub id: Option<String>,
    pub parent: Option<String>,
    pub name: String,
    pub attributes: Vec<Attribute>,
}
