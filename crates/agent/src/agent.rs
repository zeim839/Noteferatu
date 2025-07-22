use crate::{anthropic, google, ollama, openai, openrouter};
use crate::error::Error;

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

/// Implements a generic LLM definition that captures basic attributes
/// from all clients.
pub struct Model {
    pub provider: String,
    pub name: String,
    pub supports_tools: bool,
    pub supports_search: bool,
    pub params: Vec<crate::GenerationParam>,
    pub context: u64,
}
