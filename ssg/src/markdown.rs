use std::fs;
use pulldown_cmark::{Parser, Options, html};
use serde_yaml;
use crate::model::{FrontMatter, Post};

pub fn load_post(path: &str) -> Post {
    let content = fs::read_to_string(path).expect("failed to read md");

    let parts: Vec<&str> = content.splitn(3, "---").collect();
    let front: FrontMatter = serde_yaml::from_str(parts[1]).unwrap();

    let parser = Parser::new_ext(parts[2], Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    Post {
        front,
        html: html_output,
    }
}
