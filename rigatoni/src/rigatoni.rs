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
}

impl OllamaClient {
    /// Creates a new client for the Ollama API.
    pub fn new() -> Self {
        Self {
            http_client: HttpClient::new(),
            base_url: "http://localhost:11434/".to_string(),
            model: "llama3.2".to_string(),
        }
    }

    pub fn set_model(&mut self, model: &str){
        self.model = model.to_string();
    }

    /// Sends a completion request to the Ollama model.
    pub async fn completion(&mut self,  preamble: Vec::<Message>) -> Result<OllamaResponse, CompletionError> {
        let url = format!("{}api/chat", self.base_url);

    

        let request_body = json!({
            "model": self.model,
            "messages": preamble,
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
