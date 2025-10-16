use super::errors::Error;
use futures_util::StreamExt;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OllamaConfig {
    #[serde(rename = "libraryUrl")]
    pub library_url: String,
    #[serde(rename = "apiUrl")]
    pub api_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    name: String,
    url: String,
    description: String,
    details: Vec<String>,
    pulls: String,
    tags: String,
    updated: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelVersion {
    version: String,
    context: String,
    size: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PullProgress {
    status: String,
    digest: Option<String>,
    total: Option<u64>,
    completed: Option<u64>,
}

#[tauri::command]
pub async fn fetch_all_models(
    app: AppHandle,
    query: Option<String>,
) -> Result<Vec<Model>, Error> {
    let config = app.state::<OllamaConfig>();
    let response = reqwest::get(&config.library_url).await?.text().await?;
    let document = Html::parse_document(&response);

    let model_selector = Selector::parse("li a.group").unwrap();
    let name_selector = Selector::parse("[x-test-model-title] h2 span.truncate").unwrap();
    let description_selector = Selector::parse("p.break-words").unwrap();
    let pulls_selector = Selector::parse("[x-test-pull-count]").unwrap();
    let tags_selector = Selector::parse("[x-test-tag-count]").unwrap();
    let updated_selector = Selector::parse("[x-test-updated]").unwrap();
    let detail_selector = Selector::parse("[x-test-capability], [x-test-size]").unwrap();

    let mut models = Vec::new();

    for element in document.select(&model_selector) {
        let name = element
            .select(&name_selector)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_default()
            .trim()
            .to_string();

        if name.is_empty() {
            continue;
        }

        let url = format!(
            "https://ollama.com{}",
            element.value().attr("href").unwrap_or_default()
        );
        let description = element
            .select(&description_selector)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_default()
            .trim()
            .to_string();
        let pulls = element
            .select(&pulls_selector)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_default()
            .trim()
            .to_string();
        let tags = element
            .select(&tags_selector)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_default()
            .trim()
            .to_string();
        let updated = element
            .select(&updated_selector)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_default()
            .trim()
            .to_string();

        let details = element
            .select(&detail_selector)
            .map(|e| e.text().collect::<String>().trim().to_string())
            .collect();

        models.push(Model {
            name,
            url,
            description,
            details,
            pulls,
            tags,
            updated,
        });
    }

    if let Some(q) = query {
        let lowercase_query = q.to_lowercase();
        models = models
            .into_iter()
            .filter(|m| m.name.to_lowercase().contains(&lowercase_query))
            .collect();
    }

    Ok(models)
}

#[tauri::command]
pub async fn fetch_model_details(url: String) -> Result<Vec<ModelVersion>, Error> {
    let response = reqwest::get(&url).await?.text().await?;
    let document = Html::parse_document(&response);

    let version_selector = Selector::parse("table tbody tr").unwrap();
    let version_tag_selector = Selector::parse("td:nth-child(1) button").unwrap();
    let context_selector = Selector::parse("td:nth-child(2)").unwrap();
    let size_selector = Selector::parse("td:nth-child(3)").unwrap();

    let mut versions = Vec::new();

    for element in document.select(&version_selector) {
        let version = element
            .select(&version_tag_selector)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_default()
            .trim()
            .to_string();
        let context = element
            .select(&context_selector)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_default()
            .trim()
            .to_string();
        let size = element
            .select(&size_selector)
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap_or_default()
            .trim()
            .to_string();

        versions.push(ModelVersion {
            version,
            context,
            size,
        });
    }

    Ok(versions)
}

#[tauri::command]
pub async fn pull_model(app: AppHandle, model: String) -> Result<(), Error> {
    let config = app.state::<OllamaConfig>();
    let client = reqwest::Client::new();
    let res = client
        .post(&config.api_url)
        .json(&serde_json::json!({ "model": model, "stream": true }))
        .send()
        .await?;

    let mut stream = res.bytes_stream();

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(bytes) => {
                if let Ok(progress) = serde_json::from_slice::<PullProgress>(&bytes) {
                    app.emit("pull_progress", progress).unwrap();
                }
            }
            Err(e) => {
                // Handle the error, maybe log it or emit an error event
                log::error!("Error in stream: {}", e);
            }
        }
    }

    Ok(())
}