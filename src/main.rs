mod coffee_config;
use coffee_config::CoffeeConfig;

fn main() {
    let config = CoffeeConfig::load();
    println!("{:?}", config);
}
