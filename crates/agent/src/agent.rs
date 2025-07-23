use crate::{anthropic, google, ollama, openai, openrouter};
use crate::error::Error;
use crate::client::Model;

pub struct Agent {
    anthropic: Option<anthropic::Client>,
    google: Option<google::Client>,
    ollama: Option<ollama::Client>,
    openai: Option<openai::Client>,
    openrouter: Option<openrouter::Client>,
}

impl Agent {

    /// Create a new [Agent] instance.
    pub fn new() -> Self {
        Self {
            anthropic: None,
            google: None,
            ollama: None,
            openai: None,
            openrouter: None,
        }
    }

    /// Register an Anthropic client.
    pub async fn register_anthropic(&mut self, api_key: &str) -> Result<(), Error> {
        let client = anthropic::Client::new(api_key);
        client.list_models().await?;
        self.anthropic = Some(client);
        Ok(())
    }

    /// Register a Google Gemini client.
    pub async fn register_google(&mut self, api_key: &str) -> Result<(), Error> {
        let client = google::Client::new(api_key);
        client.list_models().await?;
        self.google = Some(client);
        Ok(())
    }

    /// Register an Ollama client.
    pub async fn register_ollama(&mut self, endpoint: &str) -> Result<(), Error> {
        let client = ollama::Client::new(endpoint);
        client.list_models().await?;
        self.ollama = Some(client);
        Ok(())
    }

    /// Register an OpenAI client.
    pub async fn register_openai(&mut self, api_key: &str) -> Result<(), Error> {
        let client = openai::Client::new(api_key);
        client.list_models().await?;
        self.openai = Some(client);
        Ok(())
    }

    /// Register an OpenRouter client.
    pub async fn register_openrouter(&mut self, api_key: &str) -> Result<(), Error> {
        let client = openrouter::Client::new(api_key);
        client.list_models().await?;
        self.openrouter = Some(client);
        Ok(())
    }

    /// List available models across all registered providers.
    pub async fn list_models(&self, provider: Option<&str>) -> Result<Vec<Model>, Error> {
        let mut models: Vec<Model> = Vec::new();
        let provider = provider.unwrap_or("all").to_lowercase();
        if let Some(anthropic) = &self.anthropic {
            if provider == "all" || provider == "anthropic" {
                for model in anthropic.list_models().await? {
                    models.push(model.into());
                }
            }
        }
        if let Some(google) = &self.google {
            if provider == "all" || provider == "google" {
                for model in google.list_models().await? {
                    models.push(model.into());
                }
            }
        }
        if let Some(ollama) = &self.ollama {
            if provider == "all" || provider == "ollama" {
                for model in ollama.list_models().await? {
                    models.push(model.into());
                }
            }
        }
        if let Some(openai) = &self.openai {
            if provider == "all" || provider == "openai" {
                for model in openai.list_models().await? {
                    models.push(model.into());
                }
            }
        }
        if let Some(openrouter) = &self.openrouter {
            if provider == "all" || provider == "openrouter" {
                for model in openrouter.list_models().await? {
                    models.push(model.into());
                }
            }
        }
        Ok(models)
    }
}

pub struct ChatRequest {
}

pub struct ChatResponse {
}

impl std::iter::Iterator for crate::SSE<ChatResponse> {
    type Item = ChatResponse;
    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!();
    }
}
