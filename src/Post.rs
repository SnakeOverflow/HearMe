use crate::Comment;
//use crate::Helpers::; // Import JString from Helpers
use serde::{Deserialize, Serialize};
use std::string::{String as JString};
use serde_json::Value::String;

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: JString,
    pub Post_id: JString,
    pub Post_content: JString,
    pub Post_timestamp: u64,
    //pub Comment: Vec<Comment>,
}

impl Post {
    pub fn new(id: JString, content: JString) -> Self {
        Post {
            id: id.clone(),
           Post_id: id.clone(),
            Post_content: content.clone(),
            Post_timestamp:
                get_current_timestamp(),
        }
    }
}

fn get_current_timestamp() -> u64 {
    //test value
    1234567
}
