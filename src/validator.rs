use serde_json::Error;

use serde_json::Value;

pub fn is_json_valid(json: &str) -> bool {
    let value: Result<Value, Error> = serde_json::from_str(json);

    match value {
        Ok(_) => true,
        Err(error) => {
            println!("{:?}", error);
            false
        }
    }
}
