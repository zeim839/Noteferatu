use crate::core::{Client, Error, Model, Request, Response, Result};

use crate::providers::anthropic::Client as Anthropic;
use crate::providers::google::Client as Google;
use crate::providers::ollama::Client as Ollama;
use crate::providers::openai::Client as OpenAI;
use crate::providers::openrouter::Client as OpenRouter;

/// Agent combines many LLM clients into one.
#[derive(Default)]
pub struct Agent {
    anthropic: Option<Anthropic>,
    google: Option<Google>,
    ollama: Option<Ollama>,
    openai: Option<OpenAI>,
    openrouter: Option<OpenRouter>,
}

impl Agent {

    /// Create a new [Agent].
    pub fn new() -> Self {
        Self::default()
    }

    /// Set up an [Anthropic](crate::providers::anthropic) client.
    pub async fn connect_anthropic(&mut self, api_key: &str) -> Result<()> {
        let client = Anthropic::new(api_key);
        client.check().await?;
        self.anthropic = Some(client);
        Ok(())
    }

    /// Set up a [Google](crate::providers::google) client.
    pub async fn connect_google(&mut self, api_key: &str) -> Result<()> {
        let client = Google::new(api_key);
        client.check().await?;
        self.google = Some(client);
        Ok(())
    }

    /// Set up an [Ollama](crate::providers::ollama) client.
    pub async fn connect_ollama(&mut self, api_key: &str) -> Result<()> {
        let client = Ollama::new(api_key);
        client.check().await?;
        self.ollama = Some(client);
        Ok(())
    }

    /// Set up an [OpenAI](crate::providers::openai) client.
    pub async fn connect_openai(&mut self, api_key: &str) -> Result<()> {
        let client = OpenAI::new(api_key);
        client.check().await?;
        self.openai = Some(client);
        Ok(())
    }

    /// Set up an [OpenRouter](crate::providers::openrouter) client.
    pub async fn connect_openrouter(&mut self, api_key: &str) -> Result<()> {
        let client = OpenRouter::new(api_key);
        client.check().await?;
        self.openrouter = Some(client);
        Ok(())
    }

    /// Splits a model identifier string into provider and model ID.
    fn split_model_id(model_id: &str) -> Result<(&str, &str)> {
        model_id.split_once(':')
            .ok_or_else(|| Error::InvalidModelId(model_id.to_string()))
    }
}

impl Client for Agent {
    type Error = Error;
    type Request = Request;
    type StreamResponse = Response;
    type Response = Response;

    /// Generate a non-streaming chat completion.
    async fn completion(&self, mut req: Request) -> Result<Response> {
        let (provider, model_id) = Agent::split_model_id(&req.model)?;
        let provider_owned = provider.to_string();
        req.model = model_id.to_string();
        match provider_owned.to_lowercase().as_str() {
            "anthropic" => {
                let client = self.anthropic.as_ref().ok_or_else(|| {
                    Error::ProviderNotConfigured("anthropic".to_string())
                })?;
                let res = client.completion(req.into()).await?;
                Ok(res.into())
            }
            "google" => {
                let client = self.google.as_ref()
                    .ok_or_else(|| Error::ProviderNotConfigured("google".to_string()))?;
                let res = client.completion(req.into()).await?;
                Ok(res.into())
            }
            "ollama" => {
                let client = self.ollama.as_ref()
                    .ok_or_else(|| Error::ProviderNotConfigured("ollama".to_string()))?;
                let res = client.completion(req.into()).await?;
                Ok(res.into())
            }
            "openai" => {
                let client = self.openai.as_ref()
                    .ok_or_else(|| Error::ProviderNotConfigured("openai".to_string()))?;
                let res = client.completion(req.into()).await?;
                Ok(res.into())
            }
            "openrouter" => {
                let client = self.openrouter.as_ref().ok_or_else(|| {
                    Error::ProviderNotConfigured("openrouter".to_string())
                })?;
                let res = client.completion(req.into()).await?;
                Ok(res.into())
            }
            provider => Err(Error::ProviderNotConfigured(provider.to_string())),
        }
    }

    /// Generate a streaming chat completion.
    async fn stream_completion<T>(&self, mut req: Request, mut cb: T) -> Result<()>
    where T: FnMut(Self::StreamResponse) {
        let (provider, model_id) = Agent::split_model_id(&req.model)?;
        let provider_owned = provider.to_string();
        req.model = model_id.to_string();
        match provider_owned.to_lowercase().as_str() {
            "anthropic" => {
                let client = self.anthropic.as_ref().ok_or_else(|| {
                    Error::ProviderNotConfigured("anthropic".to_string())
                })?;

                use crate::core::{Message, MessageContent, Role, ToolCall, Usage};
                use crate::providers::anthropic::DeltaKind;
                use crate::providers::anthropic::StreamEventType;
                use std::collections::HashMap;

                enum ContentBlock {
                    Text,
                    ToolUse {
                        id: String,
                        name: String,
                        args: String,
                    },
                }

                let mut role = Role::Assistant;
                let mut usage = Usage::default();
                let mut blocks: HashMap<usize, ContentBlock> = HashMap::new();

                client.stream_completion(req.into(), |event| match event.kind {
                    StreamEventType::MessageStart => {
                        if let Some(msg) = event.message {
                            role = msg.role;
                            usage.prompt_tokens = msg.usage.input_tokens.unwrap_or(0);
                            usage.completion_tokens = msg.usage.output_tokens.unwrap_or(0);
                            usage.total_tokens = usage.prompt_tokens + usage.completion_tokens;
                        }
                    }
                    StreamEventType::ContentBlockStart => {
                        if let (Some(index), Some(block)) = (event.index, event.content_block) {
                            use crate::providers::anthropic::ContentKind;
                            match block.kind {
                                ContentKind::Text => {
                                    blocks.insert(index, ContentBlock::Text);
                                }
                                ContentKind::ToolUse => {
                                    blocks.insert(
                                        index,
                                        ContentBlock::ToolUse {
                                            id: block.tool_use_id.unwrap_or_default(),
                                            name: block.name.unwrap_or_default(),
                                            args: String::new(),
                                        },
                                    );
                                }
                                _ => {}
                            }
                        }
                    }
                    StreamEventType::ContentBlockDelta => {
                        if let (Some(index), Some(delta)) = (event.index, event.delta) {
                            if let Some(block) = blocks.get_mut(&index) {
                                match (block, delta.kind) {
                                    (ContentBlock::Text, Some(DeltaKind::TextDelta)) => {
                                        if let Some(text) = delta.text {
                                            let msg = Message {
                                                role: role.clone(),
                                                content: MessageContent::Text(text),
                                            };
                                            cb(Response {
                                                messages: vec![msg],
                                                usage: usage.clone(),
                                                error: None,
                                            });
                                        }
                                    }
                                    (
                                        ContentBlock::ToolUse { args, .. },
                                        Some(DeltaKind::InputJsonDelta),
                                    ) => {
                                        if let Some(json) = delta.partial_json {
                                            args.push_str(&json);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    StreamEventType::ContentBlockStop => {
                        if let Some(index) = event.index {
                            if let Some(ContentBlock::ToolUse { id, name, args }) =
                                blocks.get(&index)
                            {
                                let arguments = match serde_json::from_str(args) {
                                    Ok(args) => args,
                                    Err(_) => serde_json::Value::Null,
                                };

                                let msg = Message {
                                    role: role.clone(),
                                    content: MessageContent::ToolCall(ToolCall {
                                        id: id.clone(),
                                        name: name.clone(),
                                        arguments,
                                    }),
                                };
                                cb(Response {
                                    messages: vec![msg],
                                    usage: usage.clone(),
                                    error: None,
                                });
                            }
                        }
                    }
                    StreamEventType::MessageDelta => {
                        if let Some(u) = event.usage {
                            if let Some(tkns) = u.output_tokens {
                                usage.completion_tokens = tkns;
                            }

                            usage.total_tokens =
                                usage.prompt_tokens + usage.completion_tokens;

                            cb(Response {
                                messages: Vec::new(),
                                usage: usage.clone(),
                                error:None,
                            });
                        }
                    }
                    StreamEventType::Error => {
                        cb(Response {
                            messages: Vec::new(),
                            usage: usage.clone(),
                            error: event.error.map(|err| Error::Anthropic(err)),
                        });
                    }
                    _ => {}
                }).await
            }
            "google" => {
                let client = self.google.as_ref()
                    .ok_or_else(|| Error::ProviderNotConfigured("google".to_string()))?;
                client.stream_completion(req.into(), |res| cb(res.into())).await
            }
            "ollama" => {
                let client = self.ollama.as_ref()
                    .ok_or_else(|| Error::ProviderNotConfigured("ollama".to_string()))?;
                client.stream_completion(req.into(), |res| cb(res.into())).await
            }
            "openai" => {
                let client = self.openai.as_ref()
                    .ok_or_else(|| Error::ProviderNotConfigured("openai".to_string()))?;
                client.stream_completion(req.into(), |res| cb(res.into())).await
            }
            "openrouter" => {
                let client = self.openrouter.as_ref().ok_or_else(|| {
                    Error::ProviderNotConfigured("openrouter".to_string())
                })?;
                client.stream_completion(req.into(), |res| cb(res.into())).await
            }
            provider => Err(Error::ProviderNotConfigured(provider.to_string())),
        }
    }

    /// List the models available on the client.
    async fn list_models(&self) -> Result<Vec<Model>> {
        let mut models: Vec<Model> = Vec::new();
        if let Some(client) = &self.anthropic {
            models.extend(client.list_models().await?);
        }
        if let Some(client) = &self.google {
            models.extend(client.list_models().await?);
        }
        if let Some(client) = &self.ollama {
            models.extend(client.list_models().await?);
        }
        if let Some(client) = &self.openai {
            models.extend(client.list_models().await?);
        }
        if let Some(client) = &self.openrouter {
            models.extend(client.list_models().await?);
        }
        Ok(models)
    }

    /// Check whether the client is connected.
    async fn check(&self) -> Result<()> {
        // The `connect_*` methods already perform a check upon
        // connection. This check confirms the agent object is usable.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Message, MessageContent, Request, Role, ToolDefinition};
    use dotenv::dotenv;
    use std::env;

    use crate::providers::ollama::Client as Ollama;

    // A helper to create an agent with clients that have valid API keys.
    async fn create_test_agent() -> Agent {
        dotenv().ok();
        let mut agent = Agent::new();
        if let Ok(key) = env::var("ANTHROPIC_API_KEY") {
            if !key.is_empty() {
                agent.connect_anthropic(&key).await.unwrap();
            }
        }
        if let Ok(key) = env::var("GOOGLE_API_KEY") {
            if !key.is_empty() {
                agent.connect_google(&key).await.unwrap();
            }
        }
        let ollama_client = Ollama::new("http://localhost:11434");
        if ollama_client.check().await.is_ok() {
            agent.connect_ollama("http://localhost:11434").await.unwrap();
        }
        if let Ok(key) = env::var("OPENAI_API_KEY") {
            if !key.is_empty() {
                agent.connect_openai(&key).await.unwrap();
            }
        }
        if let Ok(key) = env::var("OPENROUTER_API_KEY") {
            if !key.is_empty() {
                agent.connect_openrouter(&key).await.unwrap();
            }
        }
        agent
    }

    #[tokio::test]
    async fn test_completion() {
        let agent = create_test_agent().await;

        if agent.anthropic.is_some() {
            let req = Request {
                model: "anthropic:claude-3-haiku-20240307".to_string(),
                messages: vec![Message {
                    role: Role::User,
                    content: MessageContent::Text("Hello".to_string()),
                }],
                max_tokens: Some(10),
                system: None,
                tools: vec![],
            };
            let res = agent.completion(req).await.unwrap();
            assert!(!res.messages.is_empty());
        }

        if agent.google.is_some() {
            let req = Request {
                model: "google:gemini-2.5-pro".to_string(),
                messages: vec![Message {
                    role: Role::User,
                    content: MessageContent::Text("Hello".to_string()),
                }],
                max_tokens: Some(10),
                system: None,
                tools: vec![],
            };
            let res = agent.completion(req).await.unwrap();
            assert!(!res.messages.is_empty());
        }

        if agent.ollama.is_some() {
            let req = Request {
                model: "ollama:gemma3n:e4b".to_string(),
                messages: vec![Message {
                    role: Role::User,
                    content: MessageContent::Text("Hello".to_string()),
                }],
                max_tokens: Some(10),
                system: None,
                tools: vec![],
            };
            let res = agent.completion(req).await.unwrap();
            assert!(!res.messages.is_empty());
        }

        if agent.openai.is_some() {
            let req = Request {
                model: "openai:gpt-4o-mini".to_string(),
                messages: vec![Message {
                    role: Role::User,
                    content: MessageContent::Text("Hello".to_string()),
                }],
                max_tokens: Some(10),
                system: None,
                tools: vec![],
            };
            let res = agent.completion(req).await.unwrap();
            assert!(!res.messages.is_empty());
        }

        if agent.openrouter.is_some() {
            let req = Request {
                model: "openrouter:mistralai/mistral-7b-instruct:free".to_string(),
                messages: vec![Message {
                    role: Role::User,
                    content: MessageContent::Text("Hello".to_string()),
                }],
                max_tokens: Some(10),
                system: None,
                tools: vec![],
            };
            let res = agent.completion(req).await.unwrap();
            assert!(!res.messages.is_empty());
        }
    }

    const TEST_TOOL: &str = r#"{"type": "object","properties": {}}"#;

    #[tokio::test]
    async fn test_stream_completion() {
        let agent = create_test_agent().await;

        if agent.anthropic.is_some() {
            let req = Request {
                model: "anthropic:claude-3-haiku-20240307".to_string(),
                messages: vec![Message {
                    role: Role::User,
                    content: MessageContent::Text("Introduce yourself, then fetch the latest weather".to_string()),
                }],
                max_tokens: Some(1024),
                system: None,
                tools: vec![ToolDefinition {
                    name: "get_weather".to_string(),
                    description: "Get the current weather".to_string(),
                    parameters: Some(serde_json::from_str(TEST_TOOL).unwrap()),
                }],
            };

            let mut has_text = false;
            let mut has_tool_call = false;
            agent.stream_completion(req, |res| {
                for msg in res.messages {
                    match msg.content {
                        MessageContent::Text(_) => has_text = true,
                        MessageContent::ToolCall(_) => has_tool_call = true,
                        _ => {}
                    }
                }
            }).await.unwrap();

            assert!(has_text);
            assert!(has_tool_call);
        }

        if agent.google.is_some() {
            let req = Request {
                model: "google:gemini-2.5-pro".to_string(),
                messages: vec![Message {
                    role: Role::User,
                    content: MessageContent::Text("Hello".to_string()),
                }],
                max_tokens: Some(10),
                system: None,
                tools: vec![],
            };

            let mut has_response = false;
            agent.stream_completion(req, |_| { has_response = true;})
                .await.unwrap();

            assert!(has_response);
        }

        if agent.ollama.is_some() {
            let req = Request {
                model: "ollama:gemma3n:e4b".to_string(),
                messages: vec![Message {
                    role: Role::User,
                    content: MessageContent::Text("Hello".to_string()),
                }],
                max_tokens: None,
                system: None,
                tools: vec![],
            };

            let mut has_response = false;
            agent.stream_completion(req, |_| { has_response = true; })
                .await.unwrap();

            assert!(has_response);
        }

        if agent.openai.is_some() {
            let req = Request {
                model: "openai:gpt-4o-mini".to_string(),
                messages: vec![Message {
                    role: Role::User,
                    content: MessageContent::Text("Hello".to_string()),
                }],
                max_tokens: Some(10),
                system: None,
                tools: vec![],
            };

            let mut has_response = false;
            agent.stream_completion(req, |_| { has_response = true; })
                .await.unwrap();

            assert!(has_response);
        }

        if agent.openrouter.is_some() {
            let req = Request {
                model: "openrouter:mistralai/mistral-7b-instruct:free".to_string(),
                messages: vec![Message {
                    role: Role::User,
                    content: MessageContent::Text("Hello".to_string()),
                }],
                max_tokens: Some(10),
                system: None,
                tools: vec![],
            };

            let mut has_response = false;
            agent.stream_completion(req, |_| { has_response = true; })
                .await.unwrap();

            assert!(has_response);
        }
    }

    #[tokio::test]
    async fn test_list_models() {
        let agent = create_test_agent().await;
        if agent.anthropic.is_none()
            && agent.google.is_none()
            && agent.ollama.is_none()
            && agent.openai.is_none()
            && agent.openrouter.is_none()
        {
            return;
        }

        let models = agent.list_models().await.unwrap();
        assert!(!models.is_empty());

        if agent.anthropic.is_some() {
            assert!(models.iter().any(|m| m.provider == "Anthropic"));
        }
        if agent.google.is_some() {
            assert!(models.iter().any(|m| m.provider == "Google"));
        }
        if agent.ollama.is_some() {
            assert!(models.iter().any(|m| m.provider == "Ollama"));
        }
        if agent.openai.is_some() {
            assert!(models.iter().any(|m| m.provider == "OpenAI"));
        }
        if agent.openrouter.is_some() {
            assert!(models.iter().any(|m| m.provider == "OpenRouter"));
        }
    }
}
