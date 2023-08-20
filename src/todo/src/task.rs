use chrono::{DateTime, Local};

#[derive(Debug)]
pub struct Task{
    pub text: String,
    pub create_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let create_at: DateTime<Utc> = Utc::now();
        return Task { text, create_at };
    }
}