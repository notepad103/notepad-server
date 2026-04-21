use rig::{
    agent::StreamingResult,
    client::{ProviderClient, completion::CompletionClient},
    completion::{CompletionModel, CompletionRequest},
    providers::openai::{
        self,
        completion::streaming::StreamingCompletionResponse as OpenAiCompletionStreamingResponse,
        responses_api::streaming::StreamingCompletionResponse as OpenAiResponsesStreamingResponse,
    },
    streaming::{StreamingCompletionResponse, StreamingPrompt},
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
        rig::completion::message::AssistantContent::Text(text) => text.text,
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
) -> Result<StreamingCompletionResponse<OpenAiCompletionStreamingResponse>, AppError> {
    let request = get_request(question);
    let answer = DEEPSEEK_REASONER
        .stream(request)
        .await
        .map_err(|e| AppError::Internal(format!("llm stream failed: {e}")))?;
    Ok(answer)
}

pub async fn create_agent() -> Result<StreamingResult<OpenAiResponsesStreamingResponse>, AppError> {
    let client = openai::Client::from_env();
    let agent = client
        .agent("deepseek-reasoner")
        .preamble("你是一名旅游达人,根据用户的问题,给出旅游建议")
        .temperature(0.7)
        .build();
    let response = agent.stream_prompt("给我推荐一个上海明天的游玩计划").await;
    Ok(response)
}
