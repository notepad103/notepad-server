use futures::TryStreamExt;
use rig_core::{
    client::ProviderClient,
    client::completion::CompletionClient,
    completion::{CompletionModel, CompletionRequest},
    providers::openai,
    streaming::StreamedAssistantContent,
};
use std::sync::LazyLock;

use crate::AppError;

static DEEPSEEK_REASONER: LazyLock<openai::CompletionModel> = LazyLock::new(|| {
    let client = openai::Client::from_env().completions_api();
    client.completion_model("deepseek-reasoner")
});

fn get_request(question: &str) -> CompletionRequest {
    DEEPSEEK_REASONER
        .completion_request(question)
        .temperature(0.3)
        .build()
}

pub async fn completion(question: &str) -> Result<String, AppError> {
    let request = get_request(question);

    let answer = DEEPSEEK_REASONER
        .completion(request)
        .await
        .map_err(|e| AppError::Internal(format!("llm completion failed: {e}")))?;

    let text = match answer.choice.first() {
        rig_core::completion::message::AssistantContent::Text(text) => text.text,
        other => {
            return Err(AppError::Internal(format!(
                "llm returned non-text content: {other:?}"
            )));
        }
    };

    Ok(text)
}

pub async fn completion_stream(question: &str) -> Result<String, AppError> {
    let request = get_request(question);
    let mut answer = DEEPSEEK_REASONER
        .stream(request)
        .await
        .map_err(|e| AppError::Internal(format!("llm stream failed: {e}")))?;
    while let Some(chunk) = answer
        .try_next()
        .await
        .map_err(|e| AppError::Internal(format!("llm stream chunk failed: {e}")))?
    {
        match chunk {
            StreamedAssistantContent::Text(text) => print!("{text}"),
            StreamedAssistantContent::ToolCallDelta { .. } => { /* handle tool call deltas */ }
            StreamedAssistantContent::Final(_res) => { /* handle final response */ }
            _ => {}
        }
    }
    Ok("".to_string())
}
