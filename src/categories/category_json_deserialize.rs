use crate::categories::category::Category;

pub fn deserialize_categories_from_json_strings(
    categories_json: &Vec<String>,
) -> Result<Vec<Category>, serde_json::Error> {
    let mut categories: Vec<Category> = Vec::new();

    for category_json in categories_json {
        match deserialize_category_from_json(category_json.as_str()) {
            Ok(category) => categories.push(category),
            Err(error) => return Err(error),
        }
    }

    Ok(categories)
}

fn deserialize_category_from_json(json: &str) -> Result<Category, serde_json::Error> {
    serde_json::de::from_str(json)
}
