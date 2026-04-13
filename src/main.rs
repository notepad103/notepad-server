//! 二进制入口：加载配置并启动服务

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    notepad::Config::init_global()?;
    notepad::init_tracing();
    notepad::run().await
}
