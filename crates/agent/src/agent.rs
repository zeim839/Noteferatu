use crate::{anthropic, google, ollama, openai, openrouter};

type AgentError = crate::Error<String>;

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
    pub fn register_anthropic(&mut self, api_key: &str) {
        self.anthropic = Some(anthropic::Client::new(api_key));
    }

    /// Register a Google Gemini client.
    pub fn register_google(&mut self, api_key: &str) {
        self.google = Some(google::Client::new(api_key));
    }

    /// Register an Ollama client.
    pub fn register_ollama(&mut self) {
        self.ollama = Some(ollama::Client::new());
    }

    /// Register an OpenAI client.
    pub fn register_openai(&mut self, api_key: &str) {
        self.openai = Some(openai::Client::new(api_key));
    }

    /// Register an OpenRouter client.
    pub fn register_openrouter(&mut self, api_key: &str) {
        self.openrouter = Some(openrouter::Client::new(api_key));
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
