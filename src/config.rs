use env_logger::Builder;
use log::LevelFilter;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub credentials: ConfigCredentials,
    pub logging: ConfigLogging,
}
#[derive(Deserialize)]
pub struct ConfigCredentials {
    pub email: String,
    pub password: String,
}
#[derive(Deserialize)]
pub struct ConfigLogging {
    pub verbose: bool,
}

const CONFIG_FILE: &str = "./Config.toml";

pub fn read_config() -> Result<Config, anyhow::Error> {
    let content = std::fs::read_to_string(CONFIG_FILE)?;

    Ok(toml::from_str::<Config>(&content)?)
}

pub fn create_logger(logging: &ConfigLogging) {
    Builder::new()
        .filter_level(if logging.verbose {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        })
        .write_style(env_logger::WriteStyle::Always)
        .format_timestamp_secs()
        .init();
}
