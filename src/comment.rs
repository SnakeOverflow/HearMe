#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    pub id: String,
        pub post_id String,
        pub user_id: String,
        pub content: String,
        pub timestamp: u64
}
use std::fs::File;
use std::io::{self, Write};
use serde_json::json;

impl Comment { 
    pub post_id: String,
    pub fn new:(post_id: String, user_id: String, content: &str) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let id = format("{}_{}", post_id, timestamp);
        Comment {
            id,
            post_id,
            user_id,
            content:
                content.to_string(),
                timestamp,
        }
    } 
    fn save_comment(comment: &Comment) -> io::Result<()> {
    let file =
        File::create("comments.json")?;
    let json_data = json!(comment)?;
    serde_json::to_writer(file,&json_data)?;
    Ok(())
    }
    fn get_comments_for_post(postt_id: &str) -> Vec<Comment> {
    //Test implementation
    vec![
        Comment::new(post_id.to_string(),"user0".to_string(), "Great post!")
        Comment::new(post_id.to_string(),"user1".to_string(), "OwO"),
    ]
}

