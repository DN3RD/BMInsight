pub mod core;
pub mod integrations;
pub mod utils;

use utils::env::*;
use core::{state::AppState, cmd::*};

#[cfg_attr(mobile, taurpub(crate), i::mobile_entry_point)]
pub async fn run() {

    #[cfg(debug_assertions)]
    let builder = tauri::Builder::default().plugin(tauri_plugin_devtools::init());
    #[cfg(not(debug_assertions))]
    let builder = tauri::Builder::default();
    
    //Load Env vars
    let env_vars = load_env().expect("Failed to load environment variables");
    
    let state = AppState::new(
        &env_vars.llm_api_key,
        &env_vars.llm_model,
    ).await;
    

    builder
        .plugin(tauri_plugin_opener::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            compute_bmi
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
