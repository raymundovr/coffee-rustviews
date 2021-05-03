mod coffee_config;
mod gitlab_client;
mod post_message;
use coffee_config::CoffeeConfig;
use gitlab_client::MergeRequest;

#[tokio::main]
async fn main() {
    let config = CoffeeConfig::load().unwrap();
    let mrs = MergeRequest::get_open(&config.gitlab).await;
    println!("{:?}", mrs);
}
