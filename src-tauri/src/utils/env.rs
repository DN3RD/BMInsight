use std::env;
use dotenvy_macro::dotenv;

#[derive(Debug)]
pub struct Env {
    pub llm_api_key: String,
    pub llm_model: String,
}

pub fn load_env() -> Result<Env, env::VarError> {


    let llm_api_key = dotenv!("LLM_API_KEY").to_string();
    let llm_model = dotenv!("LLM_MODEL").to_string();

    Ok(Env {
        llm_api_key,
        llm_model,
    })
}