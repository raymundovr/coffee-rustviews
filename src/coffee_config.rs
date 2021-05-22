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
    pub salutation: Option<String>,
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
    pub fn load(config_file: &str) -> Result<Self, config::ConfigError> {
        let mut settings = config::Config::default();
        settings
            .merge(config::File::with_name(config_file))
            .unwrap();

        settings.try_into()
    }
}
