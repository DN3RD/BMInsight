use std::env;
use dotenv::dotenv;

#[derive(Debug)]
pub struct Env {
    pub llm_api_key: String,
    pub llm_model: String,
}

pub fn load_env() -> Result<Env, env::VarError> {
    dotenv().map_err(|e| eprintln!("Failed to load .env file: {}", e)).ok();

    let llm_api_key = env::var("LLM_API_KEY")?;
    let llm_model = env::var("LLM_MODEL")?;

    Ok(Env {
        llm_api_key,
        llm_model,
    })
}