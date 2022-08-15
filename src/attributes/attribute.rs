use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Attribute {
    pub id: Option<String>,
    pub name: String,
    pub data_type: String,
    pub unit: Option<String>,
}
