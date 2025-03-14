pub mod groq;
use async_trait::async_trait;
use std::error::Error;

pub enum Provider {
    Groq,
}

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn generate_response(&self, context: &str, user_message: &str) -> Result<String, Box<dyn Error>>;
}

