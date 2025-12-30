// model.rs
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Post {
    pub title: String,
    pub slug: String,
    pub category: u8,
    pub content_html: String,
    pub created_at: String,
    pub updated_at: String,
}
