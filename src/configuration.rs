use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub application_port: u16,
    pub database: DatabaseSettings,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

pub fn get_configuration() -> Result<Settings, ConfigError> {
    let settings = Config::builder()
        .add_source(config::File::with_name("configuration"))
        .add_source(Environment::with_prefix("APP").separator("__"))
        .build()?;
    settings.try_deserialize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configuration_can_be_loaded() {
        let configuration = get_configuration().expect("Failed to read configuration");
        assert_eq!(configuration.application_port, 8000);
    }
}
