use serde::{Deserialize, Serialize};
use anyhow::Result;
use thiserror::Error;
use reqwest::Client as HttpClient;
use serde_json::json;


#[derive(Clone)]
pub struct OllamaClient {
    base_url: String,
    model: String,
    http_client: HttpClient,
    preamble: Vec::<Message>,
}

impl OllamaClient {
    /// Creates a new client for the Ollama API.
    pub fn new() -> Self {
        Self {
            http_client: HttpClient::new(),
            base_url: "http://localhost:11434/".to_string(),
            model: "gemma2".to_string(),
            preamble: Vec::<Message>::new(),
        }
    }

    /// Sends a completion request to the Ollama model.
    pub async fn completion(&mut self,  prompt: &str) -> Result<OllamaResponse, CompletionError> {
        let url = format!("{}api/chat", self.base_url);

        self.preamble.push(Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        });

        let request_body = json!({
            "model": self.model,
            "messages": self.preamble,
            "stream": false
        });
    //    println!("Request body: {}", serde_json::to_string_pretty(&request_body).unwrap());

        let response = self
            .http_client
            .post(&url)
            .json(&request_body)
            .send()
            .await?
            .json::<OllamaResponse>()
            .await?;

     //   print!("response: {:?}", response);
        // Extract the assistant's message
        if let Some(message) = response.message.clone() {
            self.preamble.push(Message {
                role: "assistant".to_string(),
                content: message.content.to_string(),
            });
        }

       
     //   print!("preamble: {:?}", self.preamble);

        Ok(response)
    }
}

/// Represents the response payload from Ollama's API.
#[derive(Serialize, Deserialize, Debug)]
pub struct OllamaResponse {
    pub message: Option<Message>, // Assistant's message
    pub done_reason: Option<String>, // Reason the generation completed
    pub done: bool,                // Indicates if the response is finished
}

#[derive(Debug, Error)]
pub enum CompletionError {
    /// Http error (e.g., connection error, timeout, etc.).
    #[error("HttpError: {0}")]
    HttpError(#[from] reqwest::Error),

  
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}