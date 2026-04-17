use crate::error::AppError;
use crate::utils::openai_model::completion_stream;
use readabilityrs::Readability;
use rig_core::{
    providers::openai::completion::streaming::StreamingCompletionResponse as OpenAiStreamingResponse,
    streaming::StreamingCompletionResponse,
};

pub async fn get_html(
    url: &str,
) -> Result<StreamingCompletionResponse<OpenAiStreamingResponse>, AppError> {
    let response = reqwest::get(url).await?;
    let raw_html = response.text().await?;

    let readability = Readability::new(&raw_html, Some(url), None)
        .map_err(|e| AppError::Internal(format!("readability init failed: {e}")))?;

    let content = match readability.parse() {
        Some(article) => article.content.unwrap_or(raw_html),
        None => raw_html,
    };
    let question = format!(
        concat!(
            "你是网页内容摘要器。根据网页 HTML 内容，直接输出中文摘要。\n",
            "要求：只保留“这是什么”和最关键的结论、数据、结果。只输出摘要结果，不要分析过程，不要复述要求，不要输出原文。禁止补充推测、评价、背景、建议。忽略导航、广告、页脚、脚本、样式残留、示例代码和重复内容。不要出现“这篇文章”“文章提到”“对于开发者来说”“建议关注”“看起来”“用户需要”等表述。不要使用标题、列表、Markdown。如果信息不足或噪音过多，只输出：内容信息不足，无法准确总结\n",
            "网页内容：{}"
        ),
        content
    );
    Ok(completion_stream(&question).await?)
}
