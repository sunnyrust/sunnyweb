use serde::Deserialize;
#[derive(Deserialize)]
pub struct DbConfig {
    pub pg: String,
    pub connections: u32,
}
#[derive(Deserialize)]
pub struct AppConfig {
    pub addr:String,
}
#[derive(Deserialize)]
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
