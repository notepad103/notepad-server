use rig_core::{
    client::ProviderClient, client::completion::CompletionClient, completion::CompletionModel,
    providers::openai,
};
use std::sync::LazyLock;

use crate::AppError;

static DEEPSEEK_REASONER: LazyLock<openai::CompletionModel> = LazyLock::new(|| {
    let client = openai::Client::from_env().completions_api();
    client.completion_model("deepseek-reasoner")
});

pub async fn completion(question: &str) -> Result<String, AppError> {
    let request = DEEPSEEK_REASONER
        .completion_request(question)
        .temperature(0.3)
        .build();
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
