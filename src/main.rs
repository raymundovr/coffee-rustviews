mod coffee_config;
mod gitlab_client;
mod post_message;
use coffee_config::CoffeeConfig;
use gitlab_client::MergeRequest;
use post_message::*;

#[tokio::main]
async fn main() {
    let config = CoffeeConfig::load().unwrap();
    let mrs = MergeRequest::get_open(&config.gitlab).await.unwrap();
    let result = post_messages(&mrs, &config.publish).await;
    println!("Posted {} Merge Requests. Successful posts: {:?}", mrs.len(), result);
}
