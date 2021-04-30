
use config;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct CoffeeConfig {
    env: String,
}

impl CoffeeConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        let mut settings = config::Config::default();
        settings
            // Load from Settings.toml file
            .merge(config::File::with_name("JSettings"))
            .unwrap()
            // Load from ENV_VARS prefixed with APP
            .merge(config::Environment::with_prefix("APP"))
            .unwrap();

        settings.try_into()
    }
}
