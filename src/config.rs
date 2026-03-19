use std::fmt;

/// 应用配置（从环境变量 / .env 加载）
#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub bind_addr: String,
}

#[derive(Debug)]
pub struct ConfigError(pub String);

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ConfigError {}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").map_err(|_| {
            ConfigError("请设置 DATABASE_URL（可在项目根目录创建 .env 文件）".into())
        })?;
        Ok(Self {
            database_url,
            bind_addr: std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".into()),
        })
    }
}
