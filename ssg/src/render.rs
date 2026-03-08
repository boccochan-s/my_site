use tera::{Tera, Context};
use std::fs;
use std::path::Path;
use crate::model::Post;

pub fn render_posts(posts: &[Post]) {
    let tera = Tera::new("templates/**/*").unwrap();

    // CSS ファイルをコピー
    fs::create_dir_all("dist/css").unwrap();
    copy_css().unwrap_or_else(|e| eprintln!("CSS copy failed: {}", e));

    // img フォルダを dist/img にコピー
    copy_img().unwrap_or_else(|e| eprintln!("img copy failed: {}", e));

    fs::create_dir_all("dist/posts").unwrap();

    for post in posts {
        let mut ctx = Context::new();
        ctx.insert("post", &post);
        ctx.insert("is_index", &false);

        let html = tera.render("post.html", &ctx).unwrap();
        fs::write(
            format!("dist/posts/{}.html", post.front.slug),
            html,
        ).unwrap();
    }
}

pub fn render_index(posts: &[Post]) {
    let tera = Tera::new("templates/**/*").unwrap();

    let mut ctx = Context::new();
    ctx.insert("posts", &posts);
    ctx.insert("is_index", &true);
    let index_html = tera.render("index.html", &ctx).unwrap();
    fs::create_dir_all("dist").unwrap();
    fs::write("dist/index.html", index_html).unwrap();
}

pub fn render_privacy() {
    let tera = Tera::new("templates/**/*").unwrap();
    let ctx = Context::new();
    let html = tera.render("privacy.html", &ctx).unwrap();
    fs::create_dir_all("dist").unwrap();
    fs::write("dist/privacy.html", html).unwrap();
}

fn copy_css() -> std::io::Result<()> {
    if let Ok(entries) = fs::read_dir("templates/css") {
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap();
                fs::copy(&path, format!("dist/css/{}", file_name.to_string_lossy()))?;
            }
        }
    }
    Ok(())
}

fn copy_img() -> std::io::Result<()> {
    let img_dir = Path::new("img");
    if !img_dir.exists() || !img_dir.is_dir() {
        return Ok(());
    }
    fs::create_dir_all("dist/img")?;
    for entry in fs::read_dir(img_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap();
            fs::copy(&path, format!("dist/img/{}", file_name.to_string_lossy()))?;
        }
    }
    Ok(())
}
