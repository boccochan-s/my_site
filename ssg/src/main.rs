mod markdown;
mod render;
mod model;

use walkdir::WalkDir;
use markdown::load_post;

fn main() {
    let mut posts = Vec::new();

    for entry in WalkDir::new("posts") {
        let entry = entry.unwrap();

        if entry.path().extension().and_then(|s| s.to_str()) == Some("md") {
            let post = load_post(entry.path().to_str().unwrap());

            if post.front.published {
                posts.push(post);
            }
        }
    }

    posts.sort_by(|a, b| b.front.date.cmp(&a.front.date));

    render::render_index(&posts);
    render::render_posts(&posts);
}
