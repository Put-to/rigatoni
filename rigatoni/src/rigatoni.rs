use serde::{Deserialize, Serialize};
use thiserror::Error;
use anyhow::{Result, Error};
use reqwest::Client as HttpClient;
use serde_json::json;
use std::{fs, path::{Path, PathBuf}, result};



#[derive(Clone)]
pub struct OllamaClient {
    base_url: String,
    model: String,
    http_client: HttpClient,
}

impl OllamaClient {
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

    pub async fn chat(&mut self,  preamble: Vec::<Message>) -> Result<OllamaResponse, CompletionError> {
        let url = format!("{}api/chat", self.base_url);

        let request_body = json!({
            "model": self.model,
            "messages": preamble,
            "stream": false
        });

        let response = self
            .http_client
            .post(&url)
            .json(&request_body)
            .send()
            .await?
            .json::<OllamaResponse>()
            .await?;

        Ok(response)
    }

    pub async fn create_model(
        &self,
        model: &str,
        path: Option<&str>,
        modelfile: Option<&str>,
        quantize: Option<&str>,
        stream: bool,
    ) -> Result<CreateResponse> {
        let parsed_modelfile = if let Some(file_path) = path {
            let real_path = PathBuf::from(file_path);
            if real_path.exists(){
                let content = fs::read_to_string(&real_path);
                self.parse_modelfile(&content, Some(real_path.parent().unwrap()))?
            }else{
                return Err(Error::msg("Path does not exist"));
            }
        }else if let Some(modelfile_content) = modelfile{
            self.parse_modelfile(modelfile_content, None)?
        } else{
            return Err(Error::msg("Must provide Either Path or ModelFile"));
        };

        let request_body = CreateRequest{
            model: model.to_string(),
            modelfile: parsed_modelfile,
            quantize: quantize.map(|q| q.to_string()),
            stream
        };

        let url = format!{"{}api/create", self.base_url};
        let response = self.http_client.post(&url).json(&request_body).send().await?;

        let result: CreateResponse = response.json().await?;
        Ok(result)
            
        }

        
    fn parse_modelfile(&self, modelfile:&str, base: Option<&Path>)->Result<String>{

    }
    }





#[derive(Serialize, Deserialize, Debug)]
pub struct CreateRequest {
    model: String,
    modelfile: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantize: Option<String>,
    stream: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateResponse {
    pub status: String,
    pub details: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OllamaResponse {
    pub message: Option<Message>, 
    pub done_reason: Option<String>, 
    pub done: bool,          
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
