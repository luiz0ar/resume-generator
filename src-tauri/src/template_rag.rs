use std::fs;
use std::path::Path;
use serde_json::json;

pub fn read_local_template() -> Result<String, String> {
    let file_path = "/app/src-tauri/resume.md";
    if !Path::new(file_path).exists() {
        return Err(format!("File not found. {}", file_path));
    }
    fs::read_to_string(file_path).map_err(|e| format!("Failed to read file: {}", e))
}

pub async fn send_to_agent(prompt: String) -> Result<String, String> {
    let agent_url = "http://resume_ollama:11434/api/generate";
    let client = reqwest::Client::new();
    
    let body = json!({
        "model": "qwen2.5:7b", 
        "prompt": prompt,
        "stream": false 
    });

    let response = client.post(agent_url)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Error connecting to agent: {}", e))?;

    let json_response: serde_json::Value = response.json()
        .await
        .map_err(|e| format!("Error reading JSON response: {}", e))?;

    if let Some(generated_text) = json_response["response"].as_str() {
        Ok(generated_text.to_string())
    } else {
        Err("The agent did not return a valid 'response' field.".to_string())
    }
}