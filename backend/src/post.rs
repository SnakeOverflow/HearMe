use crate::comment;
//use crate::Helpers::; // Import JString from Helpers
use serde::{Deserialize, Serialize};
use std::string::{String as JString};
use serde_json::Value::String;

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: JString,
    pub post_id: JString,
    pub post_content: JString,
    pub post_timestamp: u64,
    //pub Comment: Vec<Comment>,
}

impl Post {
    pub fn new(id: JString, content: JString) -> Self {
        Post {
            id: id.clone(),
           post_id: id.clone(),
            post_content: content.clone(),
            post_timestamp:
                get_current_timestamp(),
        }
    }
}

fn get_current_timestamp() -> u64 {
    //test value
    1234567
}
