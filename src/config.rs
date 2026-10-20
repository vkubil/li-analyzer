use std::sync::LazyLock;

use env_logger::Builder;
use log::LevelFilter;
use serde::Deserialize;

pub static CONFIG: LazyLock<Config> =
    LazyLock::new(|| read_config().expect("Negalėjo nuskaityti konfiguracijos failo"));

#[derive(Deserialize)]
pub struct Config {
    pub credentials: ConfigCredentials,
    pub logging: ConfigLogging,
    pub database: ConfigDatabase,
    pub fetch: ConfigFetch,
}
#[derive(Deserialize)]
pub struct ConfigCredentials {
    pub email: String,
    pub password: String,
}
#[derive(Deserialize)]
pub struct ConfigFetch {
    pub university_id: u16,
}
#[derive(Deserialize)]
pub struct ConfigLogging {
    pub verbose: bool,
}
#[derive(Deserialize)]
pub struct ConfigDatabase {
    pub url: String,
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
