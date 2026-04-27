use rig::{
    agent::StreamingResult,
    client::{ProviderClient, completion::CompletionClient},
    completion::{CompletionModel, CompletionRequest},
    providers::openai::{
        self,
        completion::streaming::StreamingCompletionResponse as OpenAiCompletionStreamingResponse,
    },
    streaming::{StreamingCompletionResponse, StreamingPrompt},
};
use std::sync::LazyLock;

use crate::AppError;
use crate::utils::get_html::FetchWebpageTool;

static OPENAI_CLIENT: LazyLock<openai::Client> = LazyLock::new(openai::Client::from_env);

static DEEPSEEK_REASONER: LazyLock<openai::CompletionModel> = LazyLock::new(|| {
    let client = OPENAI_CLIENT.clone().completions_api();
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

pub async fn create_agent(
    prompt: &str,
) -> Result<StreamingResult<OpenAiCompletionStreamingResponse>, AppError> {
    let client = OPENAI_CLIENT.clone().completions_api();
    let agent = client
        .agent("deepseek-chat")
        .preamble(
            "你是一个网页信息助手。需要网页内容时，优先调用 fetch_webpage 工具获取网页正文，再基于工具结果回答。",
        )
        .tool(FetchWebpageTool)
        .temperature(0.7)
        .build();
    let response = agent.stream_prompt(prompt).await;
    
    Ok(response)
}
