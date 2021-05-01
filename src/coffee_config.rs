
use config;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Gitlab {
    base_url: String,
    include_wip: Option<bool>,
    projects: Option<Vec<String>>,
    token: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Publish {
    slack: Option<PublishChannel>,
    teams: Option<PublishChannel>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct PublishChannel {
    webhook_url: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct CoffeeConfig {
    gitlab: Gitlab,
    publish: Publish,
}

impl CoffeeConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        let mut settings = config::Config::default();
        settings
            // Load from Settings.toml file
            .merge(config::File::with_name("config"))
            .unwrap()
            // Load from ENV_VARS prefixed with APP
            .merge(config::Environment::with_prefix("APP"))
            .unwrap();

        settings.try_into()
    }
}
