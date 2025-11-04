//! Chat generation request.

use serde::Serialize;
use super::message::Message;

/// Ollama chat completion object.
///
/// A request to generate the next chat message in a conversation
/// between a user and an assistant.
///
/// See: [API Reference](https://docs.ollama.com/api/chat)
#[derive(Serialize, Debug, Clone, Default)]
pub struct Request {

    /// Model name.
    pub model: String,

    /// Chat history as an array of message objects.
    pub messages: Vec<Message>,

    /// Optional list of function tools the model may call during the
    /// chat
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<serde_json::Value>,

    /// Runtime options that control text generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Options>,

    /// Whether to stream the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// When true, returns separate thinking output in addition to
    /// content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub think: Option<bool>,

    /// Model keep-alive duration
    ///
    /// For example `5m` or `0` to unload immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<String>,
}

impl Request {

    /// Create a new [Request] by specifying the model to use.
    pub fn from_model(model: &str) -> Self {
        let mut req = Request::default();
        req.model = model.to_string();
        req
    }

    /// Push a message to the [Request]'s list of messages.
    pub fn push_message(self, message: Message) -> Self {
        let mut req = self;
        req.messages.push(message);
        req
    }

    /// Append a vector of messages to the [Request]'s messages.
    ///
    /// Consumes `other`.
    pub fn append_messages(self, other: &mut Vec<Message>) -> Self {
        let mut req = self;
        req.messages.append(other);
        req
    }

    /// Push a tool to the [Request]'s tools.
    pub fn push_tool(self, tool: serde_json::Value) -> Self {
        let mut req = self;
        req.tools.push(tool);
        req
    }

    /// Append a vector of tools to the [Request]'s tools.
    ///
    /// Consumes `other`.
    pub fn append_tools(self, other: &mut Vec<serde_json::Value>) -> Self {
        let mut req = self;
        req.tools.append(other);
        req
    }

    /// Enable/disable response streaming.
    pub fn stream(self, stream: bool) -> Self {
        Self { stream: Some(stream), ..self }
    }

    /// Enable/disable extended reasoning (when available).
    pub fn think(self, think: bool) -> Self {
        Self { think: Some(think), ..self }
    }

    /// Set model keep-alive duration.
    ///
    /// For example, `5m` or `0` to unload immediately.
    pub fn keep_alive(self, duration: String) -> Self {
        Self { keep_alive: Some(duration), ..self }
    }

    /// Set a random seed for reproducible outputs.
    pub fn seed(self, seed: i64) -> Self {
        let mut req = self;
        req.options = Some(req.options.map(|opts| opts.seed(seed))
            .unwrap_or(Options::default().seed(seed)));

        req
    }

    /// Set sampling temperature.
    pub fn temperature(self, temp: f64) -> Self {
        let mut req = self;
        req.options = Some(req.options.map(|opts| opts.temperature(temp))
            .unwrap_or(Options::default().temperature(temp)));

        req
    }

    /// Limits the next token selection to the K most likely.
    pub fn top_k(self, top_k: u64) -> Self {
        let mut req = self;
        req.options = Some(req.options.map(|opts| opts.top_k(top_k))
            .unwrap_or(Options::default().top_k(top_k)));

        req
    }

    /// Cumulative probability threshold for nucleus sampling.
    pub fn top_p(self, top_p: f64) -> Self {
        let mut req = self;
        req.options = Some(req.options.map(|opts| opts.top_p(top_p))
            .unwrap_or(Options::default().top_p(top_p)));

        req
    }

    /// Minimum probability threshold for token selection.
    pub fn min_p(self, min_p: f64) -> Self {
        let mut req = self;
        req.options = Some(req.options.map(|opts| opts.min_p(min_p))
            .unwrap_or(Options::default().min_p(min_p)));

        req
    }

    /// Append stop sequences that will halt generation.
    ///
    /// Consumes `stop`.
    pub fn append_stop_sequences(self, stop: &mut Vec<String>) -> Self {
        let mut req = self;
        req.options = Some(req.options.map(|opts| opts.append_stop_sequences(stop))
            .unwrap_or(Options::default().append_stop_sequences(stop)));

        req
    }

    /// Push a stop sequence that will halt generation.
    pub fn push_stop_sequence(self, stop: String) -> Self {
        let mut req = self;
        req.options = Some(req.options.map(|opts| opts.push_stop_sequence(stop.clone()))
            .unwrap_or(Options::default().push_stop_sequence(stop)));

        req
    }

    /// Set context length size (number of tokens).
    pub fn num_ctx(self, num_ctx: u64) -> Self {
        let mut req = self;
        req.options = Some(req.options.map(|opts| opts.num_ctx(num_ctx))
            .unwrap_or(Options::default().num_ctx(num_ctx)));

        req
    }

    /// Set maximum number of tokens to generate.
    pub fn num_predict(self, num_predict: u64) -> Self {
        let mut req = self;
        req.options = Some(req.options.map(|opts| opts.num_predict(num_predict))
            .unwrap_or(Options::default().num_predict(num_predict)));

        req
    }
}

/// Response generation options.
#[derive(Serialize, Debug, Clone, Default)]
pub struct Options {

    /// Random seed used for reproducible outputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,

    /// Controls randomness in generation (higher = more random).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,

    /// Limits next token selection to the K most likely.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u64>,

    /// Cumulative probability threshold for nucleus sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,

    /// Minimum probability threshold for token selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_p: Option<f64>,

    /// Stop sequences that will halt generation.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub stop: Vec<String>,

    /// Context length size (number of tokens).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_ctx: Option<u64>,

    /// Maximum number of tokens to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_predict: Option<u64>,
}

impl Options {

    /// Set a random seed for reproducible outputs.
    pub fn seed(self, seed: i64) -> Self {
        Self { seed: Some(seed), ..self }
    }

    /// Set generation temperature.
    pub fn temperature(self, temp: f64) -> Self {
        Self { temperature: Some(temp), ..self }
    }

    /// Limits the next token selection to the K most likely.
    pub fn top_k(self, top_k: u64) -> Self {
        Self { top_k: Some(top_k), ..self }
    }

    /// Cumulative probability threshold for nucleus sampling.
    pub fn top_p(self, top_p: f64) -> Self {
        Self { top_p: Some(top_p), ..self }
    }

    /// Minimum probability threshold for token selection.
    pub fn min_p(self, min_p: f64) -> Self {
        Self { min_p: Some(min_p), ..self }
    }

    /// Append stop sequences that will halt generation.
    ///
    /// Consumes `other`.
    pub fn append_stop_sequences(self, stop: &mut Vec<String>) -> Self {
        let mut options = self;
        options.stop.append(stop);
        options
    }

    /// Push a stop sequence that will halt generation.
    pub fn push_stop_sequence(self, stop: String) -> Self {
        let mut options = self;
        options.stop.push(stop);
        options
    }

    /// Set context length size (number of tokens).
    pub fn num_ctx(self, num_ctx: u64) -> Self {
        Self { num_ctx: Some(num_ctx), ..self }
    }

    /// Set maximum number of tokens to generate.
    pub fn num_predict(self, num_predict: u64) -> Self {
        Self { num_predict: Some(num_predict), ..self }
    }
}
