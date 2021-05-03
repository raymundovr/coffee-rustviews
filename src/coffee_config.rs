use config;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Gitlab {
    pub base_url: String,
    pub include_wip: Option<bool>,
    pub projects: Option<Vec<String>>,
    pub token: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Publish {
    pub slack: Option<PublishChannel>,
    pub teams: Option<PublishChannel>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct PublishChannel {
    pub webhook_url: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct CoffeeConfig {
    pub gitlab: Gitlab,
    pub publish: Publish,
}

impl CoffeeConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        let mut settings = config::Config::default();
        settings
            // Load from config.json file
            .merge(config::File::with_name("config.json"))
            .unwrap()
            // Load from ENV_VARS prefixed with APP
            .merge(config::Environment::with_prefix("APP"))
            .unwrap();

        settings.try_into()
    }
}

impl Gitlab {
    pub fn new(base_url: String, token: String) -> Self {
        Gitlab {
            base_url,
            token,
            include_wip: None,
            projects: None,
        }
    }
}
