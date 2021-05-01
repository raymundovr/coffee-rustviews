mod coffee_config;
mod gitlab_client;
use coffee_config::CoffeeConfig;

fn main() {
    let config = CoffeeConfig::load().unwrap();

}
