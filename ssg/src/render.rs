use tera::{Tera, Context};
use std::fs;
use crate::model::Post;

pub fn render(posts: &[Post]) {
    let tera = Tera::new("templates/**/*").unwrap();

    fs::create_dir_all("dist/posts").unwrap();

    for post in posts {
        let mut ctx = Context::new();
        ctx.insert("title", &post.title);
        ctx.insert("content", &post.content_html);

        let html = tera.render("post.html", &ctx).unwrap();
        fs::write(
            format!("dist/posts/{}.html", post.slug),
            html,
        ).unwrap();
    }

    let mut ctx = Context::new();
    ctx.insert("posts", posts);
    let index_html = tera.render("index.html", &ctx).unwrap();
    fs::write("dist/index.html", index_html).unwrap();
}
