use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::json;


#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    pub id: String,
        pub post_id: String,
        pub user_id: String,
        pub content: String,
        pub timestamp: u64
}
impl Comment { 
    pub fn new(post_id: String, user_id: String, content: &str) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let id = format!("{}_{}", post_id, timestamp);
        Comment {
            id,
            post_id,
            user_id,
            content: content.to_string(),
                timestamp,
        }
    } 
    pub fn save_comment(comment: &Comment) -> io::Result<()> {
        let mut file =
        File::create("comments.json")?;
    let json_data = serde_json::to_string(comment)?;
    writeln!(file, "{}", json_data)?;
    Ok(())
    }
    pub fn get_post_comments(post_id: &str) -> Vec<Comment> {
    //Test implementation
    vec![
        Comment::new(post_id.to_string(),"user0".to_string(), "Great post!"),
        Comment::new(post_id.to_string(),"user1".to_string(), "OwO"),
        ]
    }
}
