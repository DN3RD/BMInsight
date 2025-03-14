use async_trait::async_trait;
use groq_api_rust::{
    AsyncGroqClient,
    ChatCompletionMessage,
    ChatCompletionRequest,
    ChatCompletionRoles
};
use std::error::Error;

use super::LlmProvider;

pub struct Groq {
    client: AsyncGroqClient,
    model: String,
}

impl Groq {
    pub async fn new(api_key: String, model: String) -> Self {
        let client = AsyncGroqClient::new(api_key, None).await;
        Self { 
            client,
            model 
        }
    }
}

#[async_trait]
impl LlmProvider for Groq {
    async fn generate_response(&self, 
                               context: &str,
                               user_message: &str) -> Result<String, Box<dyn Error>> {
        let messages = vec![
            ChatCompletionMessage {
                role: ChatCompletionRoles::System,
                content: context.to_string(),
                name: None, 
            },
            ChatCompletionMessage {
                role: ChatCompletionRoles::User,
                content: user_message.to_string(),
                name: None, 
            }
        ];
        let request = ChatCompletionRequest::new(&self.model, messages);

        let response = self.client.chat_completion(request).await?;
        Ok(response.choices[0].message.content.clone())
    }
}
