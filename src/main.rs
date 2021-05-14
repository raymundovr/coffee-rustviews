mod coffee_config;
mod gitlab_client;
mod post_message;
use coffee_config::CoffeeConfig;
use gitlab_client::MergeRequest;
use post_message::*;
use clap::{App, Arg};

#[tokio::main]
async fn main() {
    let matches = App::new("Coffee-Rustviews")
        .version("0.1.0")
        .about("Get a notification about pending Merge Requests for your (team's) projects.")
        .arg(
            Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true)
        )
        .get_matches()
    ;
    let config_file = matches.value_of("config").unwrap_or("config.json");
    let config = CoffeeConfig::load(config_file).unwrap();
    let mrs = MergeRequest::get_open(&config.gitlab).await.unwrap();
    let result = post_messages(&mrs, &config.publish).await;
    println!("Posted {} Merge Requests. Successful posts: {:?}", mrs.len(), result);
}
