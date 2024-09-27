use reqwest::Client;
use serde_json::{from_str, json, Value};

use crate::system::system_prompt;

pub async fn model(prompt: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key=AIzaSyC2bvEKYQJ_ubfZ2gW5evT7tl5fSfGv61I";

    let system = system_prompt();

    let full_prompt = format!("{} {}", system, prompt);

    let request_body = json!({
       "contents":[
           {
               "parts": [
                   {
                       "text": full_prompt
                   }
               ]
           }
       ]
    });

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    // Get the response body as text
    let response_text = response.text().await?;

    // Deserialize the JSON string
    let response_json: Value = from_str(&response_text)?;

    if let Some(text) = response_json
        .get("candidates")
        .and_then(|candidates| candidates.as_array())
        .and_then(|candidates_array| candidates_array.get(0))
        .and_then(|candidate| candidate.get("content"))
        .and_then(|content| content.get("parts"))
        .and_then(|parts| parts.as_array())
        .and_then(|parts_array| parts_array.get(0))
        .and_then(|part| part.get("text"))
        .and_then(|text| text.as_str())
    {
        println!("{}", text);
        Ok(text.to_string())
    } else {
        println!("Campo 'text' não encontrado ou não é uma string.");
        Ok("Campo 'text' não encontrado ou não é uma string.".to_string())
    }
}
