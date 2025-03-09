use std::sync::Arc;
use crate::integrations::{groq::Groq, LlmProvider, Provider};


pub async fn get_llm_provider(
    api_key: &str, 
    model: &str,
    provider: Option<Provider>) -> Arc<dyn LlmProvider> {
    match provider.unwrap_or(Provider::Groq) {
        Provider::Groq => {
            Arc::new(Groq::new(api_key.to_owned(), model.to_owned()).await)
        }
    }
}
