//! Generation request.

use serde::Serialize;
use super::content::Content;
use super::safety::SafetySettings;
use super::tools::*;

/// Model response generation request.
///
/// See: [API Reference](https://ai.google.dev/api/generate-content#request-body)
#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Request {

    /// The name of the [Model](super::model::Model) to use for
    /// generating the completion.
    #[serde(skip_serializing)]
    pub model: String,

    /// The content of the current conversation with the model.
    ///
    /// For single-turn queries, this is a single instance. For
    /// multi-turn queries like chat, this is a repeated field that
    /// contains the conversation history and the latest request.
    pub contents: Vec<Content>,

    /// A list of Tools the Model may use to generate the next
    /// response.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Tool>,

    /// Tool configuration for any tool specified in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_config: Option<ToolConfig>,

    /// A list of unique [SafetySetting] instances for blocking unsafe
    /// content.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub safety_settings: Vec<SafetySettings>,

    /// Developer set system instruction(s). Currently, text only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<Content>,

    /// Configuration options for model generation and outputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<GenerationConfig>,

    /// The name of the content cached to use as context to serve the
    /// prediction.
    ///
    /// Format: `cachedContents/{cachedContent}`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_content: Option<String>,
}

impl Request {

    /// Create a [Request] by specifying a model.
    pub fn from_model(model: &str) -> Self {
        Self { model: model.to_string(), ..Self::default() }
    }

    /// Push content to the [Request] conversation history.
    pub fn push_content(self, content: Content) -> Self {
        let mut req = self;
        req.contents.push(content);
        req
    }

    /// Append content to the [Request] conversation history.
    ///
    /// Consumes `contents`.
    pub fn append_content(self, contents: &mut Vec<Content>) -> Self {
        let mut req = self;
        req.contents.append(contents);
        req
    }

    /// Push a tool to the [Request] available tools.
    pub fn push_tool(self, tool: Tool) -> Self {
        let mut req = self;
        req.tools.push(tool);
        req
    }

    /// Append a vector of tools to the [Request] available tools.
    ///
    /// Consumes `tools`.
    pub fn append_tools(self, tools: &mut Vec<Tool>) -> Self {
        let mut req = self;
        req.tools.append(tools);
        req
    }

    /// Set the [Request] tool config.
    ///
    /// Overwrites [`function_calling_config`] and [`retrieval_config`].
    pub fn tool_config(self, config: ToolConfig) -> Self {
        Self { tool_config: Some(config), ..self }
    }

    /// Set a function calling config (part of [`tool_config`]).
    pub fn function_calling_config(self, config: FunctionCallingConfig) -> Self {
        let mut req = self;
        match &mut req.tool_config {
            Some(tool_config) => {
                tool_config.function_calling_config = Some(config);
            },
            None => {
                req.tool_config = Some(ToolConfig {
                    function_calling_config: Some(config),
                    retrieval_config: None,
                });
            }
        }
        req
    }

    /// Set a retrieval config (part of [`tool_config`])
    pub fn retrieval_config(self, config: RetrievalConfig) -> Self {
        let mut req = self;
        match &mut req.tool_config {
            Some(tool_config) => {
                tool_config.retrieval_config = Some(config);
            },
            None => {
                req.tool_config = Some(ToolConfig {
                    function_calling_config: None,
                    retrieval_config: Some(config),
                });
            },
        }
        req
    }

    /// Push [Request] safety setting.
    pub fn push_safety_setting(self, setting: SafetySettings) -> Self {
        let mut req = self;
        req.safety_settings.push(setting);
        req
    }

    /// Append [Request] safety settings.
    ///
    /// Consumes `settings`.
    pub fn append_safety_settings(self, settings: &mut Vec<SafetySettings>) -> Self {
        let mut req = self;
        req.safety_settings.append(settings);
        req
    }

    /// Set the system instruction.
    pub fn system_instruction(self, content: Content) -> Self {
        Self { system_instruction: Some(content), ..self }
    }

    /// Set the [Request] generation config.
    pub fn generation_config(self, config: GenerationConfig) -> Self {
        Self { generation_config: Some(config), ..self }
    }

    /// Push a stop sequence to the [GenerationConfig].
    pub fn push_stop_sequence(self, sequence: &str) -> Self {
        let mut req = self;
        req.generation_config
            .get_or_insert_with(Default::default)
            .stop_sequences
            .push(sequence.to_string());

        req
    }

    /// Set the response mime type in the [GenerationConfig].
    pub fn response_mime_type(self, mime_type: &str) -> Self {
        let mut req = self;
        req.generation_config
            .get_or_insert_with(Default::default)
            .response_mime_type = Some(mime_type.to_string());

        req
    }

    /// Set the response schema in the [GenerationConfig].
    pub fn response_schema(self, schema: serde_json::Value) -> Self {
        let mut req = self;
        req.generation_config
            .get_or_insert_with(Default::default)
            .response_schema = Some(schema);

        req
    }

    /// Set the response JSON schema in the [GenerationConfig].
    pub fn response_json_schema(self, schema: serde_json::Value) -> Self {
        let mut req = self;
        req.generation_config
            .get_or_insert_with(Default::default)
            .response_json_schema = Some(schema);

        req
    }

    /// Push a response [Modality] to the [GenerationConfig].
    pub fn push_response_modality(self, modality: Modality) -> Self {
        let mut req = self;
        req.generation_config
            .get_or_insert_with(Default::default)
            .response_modalities
            .push(modality);

        req
    }

    /// Append modalities to the [GenerationConfig].
    ///
    /// Consumes `modalities`.
    pub fn append_response_modalities(self, modalities: &mut Vec<Modality>) -> Self {
        let mut req = self;
        req.generation_config
            .get_or_insert_with(Default::default)
            .response_modalities
            .append(modalities);

        req
    }

    /// Set the [GenerationConfig] candidate count.
    pub fn candidate_count(self, count: u64) -> Self {
        let mut req = self;
        req.generation_config
            .get_or_insert_with(Default::default)
            .candidate_count = Some(count);

        req
    }

    /// Set the [GenerationConfig] max output tokens.
    pub fn max_output_tokens(self, tokens: u64) -> Self {
        let mut req = self;
        req.generation_config
            .get_or_insert_with(Default::default)
            .max_output_tokens = Some(tokens);

        req
    }

    /// Set the [GenerationConfig] temperature.
    pub fn temperature(self, temp: f64) -> Self {
        let mut req = self;
        req.generation_config
            .get_or_insert_with(Default::default)
            .temperature = Some(temp);

        req
    }

    /// Set the `top_p` sampling parameter in [GenerationConfig].
    pub fn top_p(self, top_p: f64) -> Self {
        let mut req = self;
        req.generation_config
            .get_or_insert_with(Default::default)
            .top_p = Some(top_p);

        req
    }

    /// Set the generation seed in [GenerationConfig].
    pub fn seed(self, seed: u64) -> Self {
        let mut req = self;
        req.generation_config
            .get_or_insert_with(Default::default)
            .seed = Some(seed);

        req
    }

    /// Set the `presence_penalty` sampling parameter in
    /// [GenerationConfig].
    pub fn presence_penalty(self, penalty: f64) -> Self {
        let mut req = self;
        req.generation_config
            .get_or_insert_with(Default::default)
            .presence_penalty = Some(penalty);

        req
    }

    /// Set the `frequency_penalty` sampling parameter in
    /// [GenerationConfig].
    pub fn frequency_penalty(self, penalty: f64) -> Self {
        let mut req = self;
        req.generation_config
            .get_or_insert_with(Default::default)
            .frequency_penalty = Some(penalty);

        req
    }

    /// Enable enhanced civic answers in [GenerationConfig].
    pub fn enhanced_civic_answers(self, enable: bool) -> Self {
        let mut req = self;
        req.generation_config
            .get_or_insert_with(Default::default)
            .enable_enhanced_civic_answers = Some(enable);

        req
    }

    /// Set `speech_config` in [GenerationConfig].
    pub fn speech_config(self, config: SpeechConfig) -> Self {
        let mut req = self;
        req.generation_config
            .get_or_insert_with(Default::default)
            .speech_config = Some(config);

        req
    }

    /// Set the `thinking_config` in [GenerationConfig].
    pub fn thinking_config(self, config: ThinkingConfig) -> Self {
        let mut req = self;
        req.generation_config
            .get_or_insert_with(Default::default)
            .thinking_config = Some(config);

        req
    }

    /// Set the `image_config` in [GenerationConfig].
    pub fn image_config(self, config: ImageConfig) -> Self {
        let mut req = self;
        req.generation_config
            .get_or_insert_with(Default::default)
            .image_config = Some(config);

        req
    }

    /// Specify the `media_resolution` in [GenerationConfig].
    pub fn media_resolution(self, res: MediaResolution) -> Self {
        let mut req = self;
        req.generation_config
            .get_or_insert_with(Default::default)
            .media_resolution = Some(res);

        req
    }

    /// Set the [Request] cached content.
    pub fn cached_content(self, cached_content: String) -> Self {
        Self { cached_content: Some(cached_content), ..self }
    }
}

/// Configuration options for model generation and outputs. Not all
/// parameters are configurable for every model.
#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct GenerationConfig {

    /// The set of character sequences (up to 5) that will stop output
    /// generation.
    ///
    /// If specified, the API will stop at the first appearance of a
    /// stop_sequence. The stop sequence will not be included as part
    /// of the response.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub stop_sequences: Vec<String>,

    /// MIME type of the generated candidate text.
    ///
    /// Supported MIME types are: `text/plain`: (default) Text
    /// output. `application/json`: JSON response in the response
    /// candidates. `text/x.enum`: ENUM as a string response in the
    /// response candidates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_mime_type: Option<String>,

    /// Output schema of the generated candidate text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_schema: Option<serde_json::Value>,

    /// Output schema of the generated response. This is an
    /// alternative to [`response_schema`] that accepts JSON Schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_json_schema: Option<serde_json::Value>,

    /// The requested modalities of the response.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub response_modalities: Vec<Modality>,

    /// Number of generated responses to return.
    ///
    /// If unset, this will default to 1. Please note that this
    /// doesn't work for previous generation models (Gemini 1.0
    /// family)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_count: Option<u64>,

    /// The maximum number of tokens to include in a response
    /// candidate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u64>,

    /// Controls the randomness of the output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,

    /// The maximum cumulative probability of tokens to consider when
    /// sampling.
    ///
    /// The model uses combined Top-k and Top-p (nucleus) sampling.
    ///
    /// Tokens are sorted based on their assigned probabilities so
    /// that only the most likely tokens are considered. Top-k
    /// sampling directly limits the maximum number of tokens to
    /// consider, while Nucleus sampling limits the number of tokens
    /// based on the cumulative probability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,

    /// The maximum number of tokens to consider when sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i64>,

    /// Seed used in decoding.
    ///
    /// If not set, the request uses a randomly generated seed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,

    /// Presence penalty applied to the next token's logprobs if the
    /// token has already been seen in the response.
    ///
    /// This penalty is binary on/off and not dependant on the number
    /// of times the token is used (after the first). Use
    /// [`frequency_penalty`] for a penalty that increases with each
    /// use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,

    /// Frequency penalty applied to the next token's logprobs,
    /// multiplied by the number of times each token has been seen in
    /// the respponse so far.
    ///
    /// A positive penalty will discourage the use of tokens that have
    /// already been used, proportional to the number of times the
    /// token has been used: The more a token is used, the more
    /// difficult it is for the model to use that token again
    /// increasing the vocabulary of responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,

    /// Enables enhanced civic answers. It may not be available for
    /// all models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_enhanced_civic_answers: Option<bool>,

    /// The speech generation config.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_config: Option<SpeechConfig>,

    /// Config for thinking features.
    ///
    /// An error will be returned if this field is set for models that
    /// don't support thinking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking_config: Option<ThinkingConfig>,

    /// Config for image generation.
    ///
    /// An error will be returned if this field is set for models that
    /// don't support these config options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_config: Option<ImageConfig>,

    /// If specified, the media resolution specified will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_resolution: Option<MediaResolution>,
}

/// Supported modalities of the response.
///
/// See: [API Reference](https://ai.google.dev/api/generate-content#Modality)
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Modality {

    /// Indicates the model should return text.
    Text,

    /// Indicates the model should return images.
    Image,

    /// Indicates the model should return audio.
    Audio,
}

/// The speech generation config.
///
/// See: [API Reference](https://ai.google.dev/api/generate-content#SpeechConfig)
#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SpeechConfig {

    /// The configuration in case of single-voice output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_config: Option<VoiceConfig>,

    /// The configuration for the multi-speaker setup.
    ///
    /// It is mutually exclusive with the [`voice_config`] field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_speaker_voice_config: Option<MultiSpeakerVoiceConfig>,

    /// Language code (in BCP 47 format, e.g. "en-US") for speech
    /// synthesis.
    ///
    /// Valid values are: de-DE, en-AU, en-GB, en-IN, en-US, es-US,
    /// fr-FR, hi-IN, pt-BR, ar-XA, es-ES, fr-CA, id-ID, it-IT, ja-JP,
    /// tr-TR, vi-VN, bn-IN, gu-IN, kn-IN, ml-IN, mr-IN, ta-IN, te-IN,
    /// nl-NL, ko-KR, cmn-CN, pl-PL, ru-RU, and th-TH.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

impl From<VoiceConfig> for SpeechConfig {
    fn from(value: VoiceConfig) -> Self {
        Self { voice_config: Some(value), ..Default::default() }
    }
}

impl From<MultiSpeakerVoiceConfig> for SpeechConfig {
    fn from(value: MultiSpeakerVoiceConfig) -> Self {
        Self { multi_speaker_voice_config: Some(value), ..Default::default() }
    }
}

/// The configuration for the voice to use.
///
/// See: [API Reference](https://ai.google.dev/api/generate-content#VoiceConfig)
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum VoiceConfig {

    /// The configuration for the prebuilt voice to use.
    PrebuiltVoiceConfig {

        /// The name of the preset voice to use.
        voice_name: String,
    }
}

impl From<String> for VoiceConfig {
    fn from(value: String) -> Self {
        Self::PrebuiltVoiceConfig { voice_name: value }
    }
}

impl From<&str> for VoiceConfig {
    fn from(value: &str) -> Self {
        Self::PrebuiltVoiceConfig { voice_name: value.to_string() }
    }
}

/// The configuration for the multi-speaker setup.
///
/// See: [API Reference](https://ai.google.dev/api/generate-content#MultiSpeakerVoiceConfig)
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MultiSpeakerVoiceConfig {

    /// All the enabled speaker voices.
    pub speaker_voice_configs: Vec<SpeakerVoiceConfig>,
}

impl From<Vec<SpeakerVoiceConfig>> for MultiSpeakerVoiceConfig {
    fn from(value: Vec<SpeakerVoiceConfig>) -> Self {
        Self { speaker_voice_configs: value }
    }
}

/// The configuration for a single speaker in a multi speaker setup.
///
/// See: [API Reference](https://ai.google.dev/api/generate-content#SpeakerVoiceConfig)
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerVoiceConfig {

    /// The name of the speaker to use.
    ///
    /// Should be the same as in the prompt.
    pub speaker: String,

    /// The configuration for the voice to use.
    pub voice_config: VoiceConfig,
}

/// Config for thinking features.
///
/// See: [API Reference](https://ai.google.dev/api/generate-content#ThinkingConfig)
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ThinkingConfig {

    /// Indicates whether to include thoughts in the response.
    ///
    /// If true, thoughts are returned only when available.
    pub include_thoughts: bool,

    /// The number of thoughts tokens that the model should generate.
    pub thinking_budget: Option<u64>,
}

impl ThinkingConfig {

    /// Create a [ThinkingConfig].
    pub fn new(include_thoughts: bool, thinking_budget: Option<u64>) -> Self {
        Self { include_thoughts, thinking_budget }
    }
}

/// Config for image generation features.
///
/// See: [API Reference](https://ai.google.dev/api/generate-content#ImageConfig)
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageConfig {

    /// The aspect ratio of the image to generate.
    ///
    /// Supported aspect ratios: `1:1`, `2:3`, `3:2`, `3:4`, `4:3`,
    /// `9:16`, `16:9`, `21:9`.
    ///
    /// If not specified, the model will choose a default aspect ratio
    /// based on any reference images provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
}

/// Media resolution for the input media.
///
/// See: [API Reference](https://ai.google.dev/api/generate-content#MediaResolution)
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaResolution {

    /// Media resolution set to low (64 tokens).
    MediaResolutionLow,

    /// Media resolution set to medium (256 tokens).
    MediaResolutionMedium,

    /// Media resolution set to high (zoomed reframing with 256
    /// tokens).
    MediaResolutionHigh,
}
