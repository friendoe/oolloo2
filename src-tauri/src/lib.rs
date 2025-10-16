mod commands;
use commands::{default::{read, write}, ollama::{fetch_all_models, fetch_model_details, pull_model, OllamaConfig}};
use tauri::Manager;

#[allow(clippy::missing_panics_doc)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context = tauri::generate_context!();
    let config = if let Some(ollama_config) = context.config().plugins.0.get("ollama") {
        serde_json::from_value(ollama_config.clone()).unwrap()
    } else {
        OllamaConfig {
            library_url: "https://ollama.com/library".to_string(),
            api_url: "http://localhost:11434/api/pull".to_string(),
        }
    };

    tauri::Builder::default()
        .setup(move |app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            app.manage(config);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![read, write, fetch_all_models, fetch_model_details, pull_model])
        .run(context)
        .expect("error while running tauri application");
}
