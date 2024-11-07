use serde_json::Value;

pub type JString = String;
pub fn value_to_string(value: &Value) -> Option<JString> {
    match value {
        Value::String(s) =>
            Some(s.clone()),
            _ => None, // Return None if it's not a Value::String
    }
}



