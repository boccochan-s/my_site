// dynamo.rs
use aws_sdk_dynamodb::{Client, types::AttributeValue};
use crate::model::Post;

pub async fn fetch_posts(client: &Client) -> Vec<Post> {
    let resp = client
        .query()
        .table_name("blog_posts")
        .key_condition_expression("pk = :pk")
        .expression_attribute_values(
            ":pk",
            AttributeValue::S("POST".into()),
        )
        .send()
        .await
        .unwrap();

    resp.items.unwrap_or_default().into_iter().map(|item| {
        Post {
            title: item["title"].as_s().unwrap().to_string(),
            slug: item["slug"].as_s().unwrap().to_string(),
            category: item["category"].as_n().unwrap().parse::<u8>().unwrap(),
            content_html: item["content_html"].as_s().unwrap().to_string(),
            created_at: item["created_at"].as_s().unwrap().to_string(),
            updated_at: item["updated_at"].as_s().unwrap().to_string(),
        }
    }).collect()
}
