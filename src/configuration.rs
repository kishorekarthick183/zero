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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configuration_can_be_loaded() {
        let configuration = get_configuration().expect("Failed to read configuration");
        assert_eq!(configuration.application_port, 8000);
    }
}
