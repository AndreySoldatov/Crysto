use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct EnvConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
}
