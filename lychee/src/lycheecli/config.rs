use serde::Deserialize;
#[derive(Deserialize,Debug)]
pub struct DbConfig {
    pub pg: String,
    pub connections: u32,
}

#[derive(Deserialize,Debug)]
pub struct AppConfig {
    pub edition: Vec<String>,
}
#[derive(Deserialize,Debug)]
pub struct Config {
    pub db: DbConfig,
    pub app: AppConfig,
}
impl Config {
    /// read config from env
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize()
    }
    /// read from file
    pub fn from_file(path: &'static str) -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::File::with_name(path))
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize()
    }
}
