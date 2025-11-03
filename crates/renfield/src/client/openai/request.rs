//! Chat completion request.

use serde::{Serialize, Deserialize};

use super::message::Message;

/// Chat completion request.
///
/// See: [API Reference](https://platform.openai.com/docs/api-reference/chat/create)
#[derive(Serialize, Debug, Clone, Default)]
pub struct Request {

    /// A list of messages comprising the conversation so far.
    ///
    /// Depending on the model you use, different message types
    /// (modalities) are supported, like text, images, and audio.
    pub messages: Vec<Message>,

    /// Model ID used to generate the response, like `gpt-4o` or `o3`.
    pub model: String,

    /// Parameters for audio output.
    ///
    /// Required when audio output is requested with modalities:
    /// ["audio"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<AudioConfig>,

    /// Positive values penalize new tokens based on their existing
    /// frequency in the text so far, decreasing the model's
    /// likelihood to repeat the same line verbatim.
    ///
    /// Number between -2.0 and 2.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,

    /// An upper bound for the number of tokens that can be generated
    /// for a completion, including visible output tokens and
    /// reasoning tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<i64>,

    /// Output types that you would like the model to generate.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub modalities: Vec<Modality>,

    /// How many chat completion choices to generate for each input
    /// message.
    ///
    /// Note that you will be charged based on the number of generated
    /// tokens across all of the choices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i64>,

    /// Whether to enable parallel function calling during tool use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    /// Positive values penalize new tokens based on whether they
    /// appear in the text so far, increasing the model's likelihood
    /// to talk about new topics.
    ///
    /// Number between -2.0 and 2.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,

    /// Constrains effort on reasoning for reasoning models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,

    /// Specifies the processing type used for serving the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,

    /// If set to true, the model response data will be streamed to
    /// the client as it is generated using server-sent events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Options for streaming response.
    ///
    /// Only set this when you set stream: true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,

    /// What sampling temperature to use.
    ///
    /// Number between 0 and 2. Higher values like 0.8 will make the
    /// output more random, while lower values like 0.2 will make it
    /// more focused and deterministic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,

    /// A list of tools the model may call.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<serde_json::Value>,

    /// An alternative to sampling with temperature, called nucleus
    /// sampling, where the model considers the results of the tokens
    /// with top_p probability mass. So 0.1 means only the tokens
    /// comprising the top 10% probability mass are considered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,

    /// Constrains the verbosity of the model's response.
    ///
    /// Lower values will result in more concise responses, while
    /// higher values will result in more verbose responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<Verbosity>,
}

impl Request {

    /// Create a new request by specifying a model.
    pub fn from_model(model: &str) -> Self {
        let mut req = Request::default();
        req.model = String::from(model);
        req
    }

    /// Push a message to the [Request]'s message vector.
    pub fn push_message(self, message: Message) -> Self {
        let mut req = self;
        req.messages.push(message);
        req
    }

    /// Append a vector of messages to the request.
    ///
    /// Moves all messages in `other` into `self`, leaving `other`
    /// empty.
    pub fn append_messages(self, other: &mut Vec<Message>) -> Self {
        let mut req = self;
        req.messages.append(other);
        req
    }

    /// Configure parameters for audio output.
    pub fn audio(self, audio: AudioConfig) -> Self {
        Self { audio: Some(audio), ..self }
    }

    /// Set a frequency penalty.
    pub fn frequency_penalty(self, penalty: f64) -> Self {
        Self { frequency_penalty: Some(penalty), ..self }
    }

    /// Set the maximum completion tokens.
    pub fn max_completion_tokens(self, tokens: i64) -> Self {
        Self { max_completion_tokens: Some(tokens), ..self }
    }

    /// Push a supported output modality.
    pub fn push_modality(self, modality: Modality) -> Self {
        let mut req = self;
        req.modalities.push(modality);
        req
    }

    /// Append a vector of modalities, consuming `other`.
    pub fn append_modalities(self, other: &mut Vec<Modality>) -> Self {
        let mut req = self;
        req.modalities.append(other);
        req
    }

    /// Configure how many chat completions to generate per request.
    pub fn n(self, n: i64) -> Self {
        Self { n: Some(n), ..self }
    }

    /// Enable/disable parallel tool calls.
    pub fn parallel_tool_calls(self, enable: bool) -> Self {
        Self { parallel_tool_calls: Some(enable), ..self }
    }

    /// Set a presence penalty.
    pub fn presence_penalty(self, penalty: f64) -> Self {
        Self { presence_penalty: Some(penalty), ..self }
    }

    /// Set the reasoning effort.
    pub fn reasoning_effort(self, effort: ReasoningEffort) -> Self {
        Self { reasoning_effort: Some(effort), ..self }
    }

    /// Set the service tier.
    pub fn service_tier(self, service_tier: ServiceTier) -> Self {
        Self { service_tier: Some(service_tier), ..self }
    }

    /// Enable/disable streaming responses.
    pub fn stream(self, enable: bool) -> Self {
        Self { stream: Some(enable), ..self }
    }

    /// Set streaming options.
    pub fn stream_options(self, options: StreamOptions) -> Self {
        Self { stream_options: Some(options), ..self }
    }

    /// Set a sampling temperature.
    pub fn temperature(self, temp: f64) -> Self {
        Self { temperature: Some(temp), ..self }
    }

    /// Push a tool to the [Request]'s available tools.
    pub fn push_tool(self, tool: serde_json::Value) -> Self {
        let mut req = self;
        req.tools.push(tool);
        req
    }

    /// Append a vector of tools to the request.
    ///
    /// Moves elements of `tools` to `self`, leaving `tools` empty.
    pub fn append_tools(self, tools: &mut Vec<serde_json::Value>) -> Self {
        let mut req = self;
        req.tools.append(tools);
        req
    }

    /// Set a nucleus sampling value.
    pub fn top_p(self, top_p: f64) -> Self {
        Self { top_p: Some(top_p), ..self }
    }

    /// Configure the model verbosity.
    pub fn verbosity(self, verbosity: Verbosity) -> Self {
        Self { verbosity: Some(verbosity), ..self }
    }
}

impl From<crate::client::Request> for Request {
    fn from(_value: crate::client::Request) -> Self {
        todo!();
    }
}

/// Parameters for audio output.
#[derive(Serialize, Debug, Clone, Copy, Default)]
pub struct AudioConfig {

    /// Specifies the output audio format.
    pub format: AudioFormat,

    /// The voice the model uses to respond.
    pub voice: Voice,
}

impl AudioConfig {

    /// Create a new [AudioConfig].
    pub fn new(format: AudioFormat, voice: Voice) -> Self {
        AudioConfig { format, voice }
    }
}

/// Specifies the input/output audio format.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
#[serde(rename_all = "lowercase")]
pub enum AudioFormat {
    #[default]
    Wav,
    Mp3,
    Flac,
    Opus,
    Pcm16,
}

/// The voice the model uses when generating speech output.
#[derive(Serialize, Debug, Clone, Copy, Default)]
#[serde(rename_all = "lowercase")]
pub enum Voice {
    #[default]
    Alloy,
    Ash,
    Ballad,
    Coral,
    Echo,
    Fable,
    Nova,
    Onyx,
    Sage,
    Shimmer,
}

/// Output types that you would like the model to generate.
#[derive(Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Modality {
    Text,
    Audio,
}

/// Constrains effort on reasoning for reasoning models.
#[derive(Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
    Minimal,
    Low,
    Medium,
    High,
}

/// Specifies the processing type used for serving the request.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
#[serde(rename_all = "lowercase")]
pub enum ServiceTier {
    #[default]
    Auto,
    Default,
    Flex,
    Priority,
}

/// Options for streaming response.
#[derive(Serialize, Debug, Clone)]
pub struct StreamOptions {

    /// The usage field on this chunk shows the token usage statistics
    /// for the entire request, and the choices field will always be
    /// an empty array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_usage: Option<bool>,
}

impl StreamOptions {

    /// Create a [StreamOptions] by specifying whether to include usage.
    pub fn new(include_usage: bool) -> Self {
        Self { include_usage: Some(include_usage) }
    }
}

/// Constrains the verbosity of the model's response.
///
/// Lower values will result in more concise responses, while higher
/// values will result in more verbose responses.
#[derive(Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Verbosity {
    Low,
    Medium,
    High,
}
