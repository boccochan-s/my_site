use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FrontMatter {
    pub title: String,
    pub slug: String,
    pub date: String,
    pub published: bool,
}

#[derive(serde::Serialize)]
pub struct Post {
    pub front: FrontMatter,
    pub html: String,
}
