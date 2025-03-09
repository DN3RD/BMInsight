use std::sync::Arc;
use crate::integrations::{LlmProvider};

use super::llm::get_llm_provider;

pub struct AppState {
    pub llm: Arc<dyn LlmProvider>,
}

impl AppState {
    pub async fn new(api_key: &str, model: &str) -> Self {
        Self {
            llm: get_llm_provider(api_key, model, None).await,
        }
    }
}
