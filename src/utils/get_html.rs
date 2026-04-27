use readabilityrs::Readability;
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::error::AppError;

pub async fn fetch_readable_content(url: &str) -> Result<String, AppError> {
    let response = reqwest::get(url).await?;
    let raw_html = response.text().await?;

    let readability = Readability::new(&raw_html, Some(url), None)
        .map_err(|e| AppError::Internal(format!("readability init failed: {e}")))?;

    let content = match readability.parse() {
        Some(article) => article.content.unwrap_or(raw_html),
        None => raw_html,
    };

    Ok(content)
}

#[derive(Debug, Deserialize)]
pub struct FetchWebpageArgs {
    pub url: String,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct FetchWebpageTool;

#[derive(Debug)]
pub enum FetchWebpageToolError {
    App(AppError),
}

impl std::fmt::Display for FetchWebpageToolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::App(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for FetchWebpageToolError {}

impl From<AppError> for FetchWebpageToolError {
    fn from(value: AppError) -> Self {
        Self::App(value)
    }
}

impl Tool for FetchWebpageTool {
    const NAME: &'static str = "fetch_webpage";
    type Error = FetchWebpageToolError;
    type Args = FetchWebpageArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "获取网页正文内容，返回可读文本（已去除大部分导航和噪音）".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "description": "要抓取的网页 URL，必须包含 http:// 或 https://"
                    }
                },
                "required": ["url"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(fetch_readable_content(&args.url).await?)
    }
}
