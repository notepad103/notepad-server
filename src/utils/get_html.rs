use crate::error::AppError;
use readabilityrs::Readability;

pub async fn get_html(url: &str) -> Result<String, AppError> {
    let response = reqwest::get(url).await?;
    let raw_html = response.text().await?;

    let readability = Readability::new(&raw_html, Some(url), None)
        .map_err(|e| AppError::Internal(format!("readability init failed: {e}")))?;

    match readability.parse() {
        Some(article) => Ok(article.content.unwrap_or(raw_html)),
        None => Ok(raw_html),
    }
}
