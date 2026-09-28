use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub credentials: ConfigCredentials,
}
#[derive(Deserialize)]
pub struct ConfigCredentials {
    pub email: String,
    pub password: String,
}

const CONFIG_FILE: &str = "./Config.toml";

pub fn read_config() -> Config {
    let content =
        std::fs::read_to_string(CONFIG_FILE).expect("negali nuskaityti konfiguracijos failo");

    toml::from_str::<Config>(&content).expect("negali analizuoti konfiguracijos")
}
