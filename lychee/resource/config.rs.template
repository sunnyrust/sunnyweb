use serde::Deserialize;
#[derive(Deserialize)]
pub struct WebConfig {
    pub addr: String,
    pub version: String,
    pub runmode: String,
}
#[derive(Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub with_secure: bool,
}
#[derive(Deserialize)]
pub struct DbConfig {
    pub pg:String,
    pub connections:u32,
}
#[derive(Deserialize)]
pub struct Config {
    pub web: WebConfig,
    pub db: DbConfig,
    pub redis: RedisConfig,
}
impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize()
    }
    pub fn from_file(path: &'static str) -> Result<Self, config::ConfigError> {
        // let config = fs::read_to_string(path).unwrap();
        // serde_json::from_str(&config).unwrap()
        config::Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name(path))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        // .add_source(config::Environment::with_prefix("APP"))
        .add_source(config::Environment::default())
        .build()?
        .try_deserialize()
    }
}
#[derive(Deserialize,Clone,Debug)]
pub struct WebHotelInfo{
    pub web_addr:String,
    pub web_version:String,
}