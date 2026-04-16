use crate::error::AppError;
use crate::utils::open_ai_model::{completion, completion_stream};
use readabilityrs::Readability;

pub async fn get_html(url: &str) -> Result<String, AppError> {
    let response = reqwest::get(url).await?;
    let raw_html = response.text().await?;

    let readability = Readability::new(&raw_html, Some(url), None)
        .map_err(|e| AppError::Internal(format!("readability init failed: {e}")))?;

    let content = match readability.parse() {
        Some(article) => article.content.unwrap_or(raw_html),
        None => raw_html,
    };

    let question = format!("请从以下 HTML 文本中提取出正文内容：\n{}", content);
    // let extract_text = completion(&question).await?;
    completion_stream(&question).await?;
    // println!("content: {}", extract_text);
    Ok("".to_string())
}
