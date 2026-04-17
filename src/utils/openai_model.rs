use rig_core::{
    client::{ProviderClient, completion::CompletionClient},
    completion::{CompletionModel, CompletionRequest},
    providers::openai,
    providers::openai::completion::streaming::StreamingCompletionResponse as OpenAiStreamingResponse,
    streaming::StreamingCompletionResponse,
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

pub async fn completion_stream(
    question: &str,
) -> Result<StreamingCompletionResponse<OpenAiStreamingResponse>, AppError> {
    let request = get_request(question);
    let answer = DEEPSEEK_REASONER
        .stream(request)
        .await
        .map_err(|e| AppError::Internal(format!("llm stream failed: {e}")))?;
    Ok(answer)
}
