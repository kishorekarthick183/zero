use config::Config;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub application_port: u16,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = Config::builder()
        .add_source(config::File::with_name("configuration"))
        .build()?;
    settings.try_deserialize()
}
