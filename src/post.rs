pub struct Post {
    pub id: String,
    pub content: String,
    pub timestamp: u64,
}

impl Post {
    pub fn new(id: String, content: String) -> Self {
        Post {
            id,
            content,
            timestamp:
                get_current_timestamp(),
        }
    }
}

fn get_current_timestamp() -> u64 {
    //test value
    1234567
}


