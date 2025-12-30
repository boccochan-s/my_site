mod model;
mod dynamo;
mod render;

use aws_config;
use aws_sdk_dynamodb::Client;

#[tokio::main]
async fn main() {
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    let posts = dynamo::fetch_posts(&client).await;
    render::render(&posts);
}
