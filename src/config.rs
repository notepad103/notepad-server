use std::fmt;
use std::sync::OnceLock;

/// 全局配置单例，启动时通过 `init_global()` 初始化
static GLOBAL_CONFIG: OnceLock<Config> = OnceLock::new();

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

    /// 从环境变量加载并注册到全局，仅应在程序入口调用一次。
    pub fn init_global() -> Result<&'static Config, ConfigError> {
        let config = Self::from_env()?;
        Ok(GLOBAL_CONFIG.get_or_init(|| config))
    }

    /// 获取已初始化的全局配置，未初始化时 panic。
    pub fn global() -> &'static Config {
        GLOBAL_CONFIG.get().expect("Config 未初始化，请先调用 Config::init_global()")
    }
}
