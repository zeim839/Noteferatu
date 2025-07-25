use serde::{Serialize, Deserialize};

/// Specifies the producer of message content.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    User,
    Model,
    Function,
}

/// Body of a chat completion request.
#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatRequest {

    /// The content of the current conversation with the model.
    ///
    /// For single-turn queries, this is a single instance. For multi-turn
    /// queries like chat, this is a repeated field that contains the
    /// conversation history and the latest request.
    pub contents: Vec<Content>,

    /// A list of Tools the [Model](super::models::Model) may use to
    /// generate the next response.
    ///
    /// A [Tool] is a piece of code that enables the system to
    /// interact with external systems to perform an action, or set of
    /// actions, outside of knowledge and scope of the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,

    /// Tool configuration for any Tool specified in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_config: Option<ToolConfig>,

    /// Developer set system instruction(s). Currently, text only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<Content>,

    /// Configuration options for model generation and outputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<GenerationConfig>,
}

impl ChatRequest {

    /// Creates a [ChatRequest] from a single text prompt.
    pub fn from_prompt(prompt: &str) -> Self {
        let mut req = Self::default();
        req.contents = vec![Content {
            role: Role::User,
            parts: vec![Part {
                thought: None,
                thought_signature: None,
                data: PartData::new()
                    .with_text(Some(prompt.to_string())),
            }],
        }];
        req
    }

    /// Creates a [ChatRequest] from a vector of messages.
    pub fn from_contents(contents: Vec<Content>) -> Self {
        let mut req = Self::default();
        req.contents = contents;
        req
    }

    /// Populates the [Self::tools] field with the given value.
    pub fn with_tools(self, tools: Option<Vec<Tool>>) -> Self {
        Self { tools, ..self }
    }

    /// Populates the [Self::tool_config] field with the given value.
    pub fn with_tool_config(self, tool_config: Option<ToolConfig>) -> Self {
        Self { tool_config, ..self }
    }

    /// Populates the [Self::system_instruction] field with the given value.
    pub fn with_system_instruction(self, system: Option<Content>) -> Self {
        Self { system_instruction: system, ..self }
    }

    /// Populates the [Self::generation_config] field with the given value.
    pub fn with_generation_config(self, config: Option<GenerationConfig>) -> Self {
        Self { generation_config: config, ..self }
    }
}

/// The base structured datatype containing multi-part content of a
/// message.
///
/// A `Content` includes a `role` field designating the producer of
/// the `Content` and a `parts` field containing multi-part data that
/// contains the content of the message turn.
#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub parts: Vec<Part>,
    pub role: Role,
}

/// A datatype containing media that is part of a multi-part Content
/// message.
///
/// A Part consists of data which has an associated datatype. A Part
/// can only contain one of the accepted types in Part.data.
///
/// A Part must have a fixed IANA MIME type identifying the type and
/// subtype of the media if the inlineData field is filled with raw
/// bytes.
///
/// API Reference: [Part](https://ai.google.dev/api/caching#Part)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Part {

    /// Indicates if the part is thought from the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thought: Option<bool>,

    /// An opaque signature for the thought so it can be reused in
    /// subsequent requests.
    ///
    /// A base64-encoded string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thought_signature: Option<String>,

    /// The data contained by this [Part].
    #[serde(flatten)]
    pub data: PartData,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartData {

    /// Inline text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Inline media bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_data: Option<BlobPartData>,

    /// A predicted FunctionCall with arguments and their values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCallPartData>,

    /// The result output of a `FunctionCall`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_response: Option<FunctionResponsePartData>,

    /// URI based data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_data: Option<FilePartData>,
}

impl PartData {

    /// Instantiates a new [PartData] with all `None`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Populates the [Self::text] field with the given value.
    pub fn with_text(self, text: Option<String>) -> Self {
        Self { text, ..self}
    }

    /// Populates the [Self::inline_data] field with the given value.
    pub fn with_inline_data(self, inline_data: Option<BlobPartData>) -> Self {
        Self { inline_data, ..self }
    }

    /// Populates the [Self::function_call] field with the given value.
    pub fn with_function_call(self, function_call: Option<FunctionCallPartData>) -> Self {
        Self { function_call, ..self }
    }

    /// Populates the [Self::function_response] field with the given value.
    pub fn with_function_response(self, function_response: Option<FunctionResponsePartData>) -> Self {
        Self { function_response, ..self }
    }

    /// Populates the [Self::file_data] field with the given value.
    pub fn with_file_data(self, file_data: Option<FilePartData>) -> Self {
        Self { file_data, ..self }
    }
}

/// Raw media bytes.
///
/// Text should not be sent as raw bytes, use the 'text' field.
#[derive(Debug, Serialize, Deserialize)]
pub struct BlobPartData {

    /// The IANA standard MIME type of the source data.
    #[serde(rename = "mimeType")]
    pub mime_type: String,

    /// Raw bytes for media formats.
    pub data: String,
}

/// A predicted `FunctionCall` with the arguments and their values.
#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionCallPartData {

    /// The unique id of the function call, which should be referenced
    /// when returning a tool call result.
    pub id: Option<String>,

    /// The name of the function to call.
    pub name: String,

    /// The function parameters and values in JSON object format.
    pub args: Option<serde_json::Value>,
}

/// The result output from a `FunctionCall` that contains a string
/// representing the `FunctionDeclaration.name` and a structured JSON
/// object containing any output from the function is used as context
/// to the model. This should contain the result of a `FunctionCall` made
/// based on model prediction.
#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionResponsePartData {

    /// The id of the function call this response is for. Populated by
    /// the client to match the corresponding function call `id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The name of the function to call.
    pub name: String,

    /// The function response in JSON object format.
    pub response: serde_json::Value,

    /// Signals that function call continues, and more responses will
    /// be returned, turning the function call into a generator.
    ///
    /// Is only applicable to `NON_BLOCKING` function calls, is ignored
    /// otherwise. If set to `false`, future responses will not be
    /// considered. It is allowed to return empty response with
    /// `willContinue=False` to signal that the function call is
    /// finished. This may still trigger the model generation. To
    /// avoid triggering the generation and finish the function call,
    /// additionally set scheduling to `SILENT`.
    #[serde(rename="willContinue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub will_continue: Option<bool>,

    /// Specifies how the response should be scheduled in the
    /// conversation. Only applicable to `NON_BLOCKING` function calls,
    /// is ignored otherwise. Defaults to `WHEN_IDLE`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling: Option<Scheduling>,
}

/// Specifies how the response should be scheduled in the conversation.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Scheduling {

    /// This value is unused.
    SchedulingUnspecified,

    /// Only add the result to the conversation context, do not
    /// interrupt or trigger generation.
    Silent,

    /// Add the result to the conversation context, and prompt to
    /// generate output without interrupting ongoing generation.
    WhenIdle,

    /// Add the result to the conversation context, interrupt ongoing
    /// generation and prompt to generate output.
    Interrupt,
}

/// URI based data.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilePartData {

    /// The IANA standard MIME type of the source data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    /// URI.
    pub file_uri: String,
}

/// Tool details that the model may use to generate response.
///
/// A `Tool` is a piece of code that enables the system to interact with
/// external systems to perform an action, or set of actions, outside
/// of knowledge and scope of the [Model](super::models::Model).
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tool {

    /// A list of `FunctionDeclarations` available to the model that can
    /// be used for function calling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_declarations: Option<Vec<FunctionDeclaration>>,

    /// Retrieval tool that is powered by Google search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_search_retrieval: Option<GoogleSearchRetrieval>,

    /// GoogleSearch tool type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_search: Option<GoogleSearch>,

    /// Tool to support URL context retrieval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_context: Option<serde_json::Value>,
}

/// Structured representation of a function declaration.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionDeclaration {

    /// The name of the function.
    pub name: String,

    /// A brief description of the function.
    pub description: String,

    /// Describes the parameters to the function as a JSON Schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}

/// Tool to retrieve public web data for grounding, powered by Google.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleSearchRetrieval {

    /// Specifies the dynamic retrieval configuration for the source.
    pub dynamic_retrieval_config:  DynamicRetrievalConfig,
}

/// Describes the options to customize dynamic retrieval.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicRetrievalConfig {

    /// The mode of the predictor to be used in dynamic retrieval.
    pub mode: DynamicRetrievalMode,

    /// The threshold to be used in dynamic retrieval. If not set, a
    /// system default value is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_threshold: Option<i64>,
}

/// The mode of the predictor to be used in dynamic retrieval.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DynamicRetrievalMode {

    /// Always trigger retrieval.
    ModeUnspecified,

    /// Run retrieval only when system decides it is necessary.
    ModeDynamic,
}

/// GoogleSearch tool type.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleSearch {

    /// Filter search results to a specific time range. If customers
    /// set a start time, they must set an end time (and vice versa).
    #[serde(skip_serializing_if = "Option::is_none")]
    time_range_filter: Option<Interval>,
}

/// Represents a time interval, encoded as a Timestamp start
/// (inclusive) and a Timestamp end (exclusive).
///
/// The start must be less than or equal to the end. When the start
/// equals the end, the interval is empty (matches no time). When both
/// start and end are unspecified, the interval matches any time.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interval {

    /// Inclusive start of the interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,

    /// Exclusive end of the interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

/// The Tool configuration containing parameters for specifying [Tool]
/// use in the request.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolConfig {

    /// Function calling config.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_calling_config: Option<FunctionCallingConfig>,
}

/// Configuration for specifying function calling behavior.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCallingConfig {

    /// Specifies the mode in which function calling should
    /// execute. If unspecified, the default value will be set to
    /// `AUTO`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<FunctionCallingMode>,

    /// A set of function names that, when provided, limits the
    /// functions the model will call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_function_names: Option<Vec<String>>,
}

/// Defines the execution behavior for function calling by defining
/// the execution mode.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FunctionCallingMode {

    /// Default model behavior, model decides to predict either a
    /// function call or a natural language response.
    Auto,

    /// [Model](super::models::Model) is constrained to always
    /// predicting a function call only. If `allowedFunctionNames` are
    /// set, the predicted function call will be limited to any one of
    /// `allowedFunctionNames`, else the predicted function call will
    /// be any one of the provided `functionDeclarations`.
    Any,

    /// The model will never specify a function call.
    None,

    /// Model decides to predict either a function call or a natural
    /// language response, but will validate function calls with
    /// constrained decoding.
    Validated,
}

/// Configuration options for model generation and outputs. Not all
/// parameters are configurable for every model.
#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationConfig {

    /// The requested modalities of the response. Represents the set
    /// of modalities that the model can return, and should be
    /// expected in the response. This is an exact match to the
    /// modalities of the response.
    ///
    /// An empty list is equivalent to requesting only text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_modalities: Option<Vec<Modality>>,

    /// The maximum number of tokens to include in a response
    /// candidate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<i64>,

    /// Controls the randomness of the output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,

    /// The maximum cum. probability of tokens to consider when sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,

    /// The maximum number of tokens to consider when sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<f64>,

    /// Seed used in decoding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,

    /// Presence penalty applied to the next token's logprobs if the
    /// token has already been seen in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,

    /// Frequency penalty applied to the next token's logprobs,
    /// multiplied by the number of times each token has been seen in
    /// the response so far.
    ///
    /// A positive penalty will discourage the use of tokens that have
    /// already been used, proportional to the number of times the token
    /// has been used: The more a token is used, the more difficult it is
    /// for the model to use that token again increasing the
    /// vocabulary of responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,

    /// Config for thinking features. An error will be returned if
    /// this field is set for models that don't support thinking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking_config: Option<ThinkingConfig>,
}

impl GenerationConfig {

    /// Instantiates a new [GenerationConfig].
    pub fn new() -> Self {
        Self::default()
    }

    /// Populates the [Self::response_modalities] field with the given value.
    pub fn with_response_modalities(self, modalities: Option<Vec<Modality>>) -> Self {
        Self { response_modalities: modalities, ..self }
    }

    /// Populates the [Self::max_output_tokens] field with the given value.
    pub fn with_max_output_tokens(self, max_output_tokens: Option<i64>) -> Self {
        Self { max_output_tokens, ..self }
    }

    /// Populates the [Self::temperature] field with the given value.
    pub fn with_temperature(self, temperature: Option<f64>) -> Self {
        Self { temperature, ..self }
    }

    /// Populates the [Self::top_p] field with the given value.
    pub fn with_top_p(self, top_p: Option<f64>) -> Self {
        Self { top_p, ..self }
    }

    /// Populates the [Self::top_k] field with the given value.
    pub fn with_top_k(self, top_k: Option<f64>) -> Self {
        Self { top_k, ..self }
    }

    /// Populates the [Self::seed] field with the given value.
    pub fn with_seed(self, seed: Option<i64>) -> Self {
        Self { seed, ..self }
    }

    /// Populates the [Self::presence_penalty] field with the given value.
    pub fn with_presence_penalty(self, presence_penalty: Option<f64>) -> Self {
        Self { presence_penalty, ..self }
    }

    /// Populates the [Self::frequency_penalty] field with the given value.
    pub fn with_frequency_penalty(self, frequency_penalty: Option<f64>) -> Self {
        Self { frequency_penalty, ..self }
    }

    /// Populates the [Self::thinking_config] field with the given value.
    pub fn with_thinking_config(self, thinking_config: Option<ThinkingConfig>) -> Self {
        Self { thinking_config, ..self }
    }
}

/// Supported modalities of the response.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Modality {
    Text,
    Image,
    Audio,
}

/// Config for thinking features.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThinkingConfig {

    /// Indicates whether to include thoughts in the response.
    pub include_thoughts: bool,

    /// The number of thoughts tokens that the model should generate.
    pub thinking_budget: i64,
}

/// Response from the [Model](super::models::Model) supporting
/// multiple candidate responses.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatResponse {

    /// Candidate responses from the model.
    pub candidates: Vec<Candidate>,

    /// Metadata on the generation requests' token usage.
    pub usage_metadata: UsageMetadata,

    /// The model version used to generate the response.
    pub model_version: String,

    /// Used to identify each response.
    pub response_id: String,
}

/// A response candidate generated from the model.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {

    /// Generated content returned from the model.
    pub content: Content,

    /// The reason why the model stopped generating tokens.
    /// If empty, the model has not stopped generating tokens.
    pub finish_reason: Option<FinishReason>,
}

/// Defines the reason why the model stopped generating tokens.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FinishReason {

    /// This value is unused.
    FinishReasonUnspecified,

    /// Natural stop point of the model or provided stop sequence.
    Stop,

    /// The maximum number of tokens as specified in the request was
    /// reached.
    MaxTokens,

    /// The response candidate content was flagged for safety reasons.
    Safety,

    /// The response candidate content was flagged for recitation
    /// reasons.
    Recitation,

    /// The response candidate content was flagged for using an
    /// unsupported language.
    Language,

    /// Unknown reason.
    Other,

    /// Token generation stopped because the content contains
    /// forbidden terms.
    Blocklist,

    /// Token generation stopped for potentially containing prohibited
    /// content.
    ProhibitedContent,

    /// Token generation stopped because the content potentially
    /// contains Sensitive Personally Identifiable Information (SPII).
    Spii,

    /// The function call generated by the model is invalid.
    MalformedFunctionCall,

    /// Token generation stopped because generated images contain
    /// safety violations.
    ImageSafety,

    /// Model generated a tool call but no tools were enabled in the
    /// request.
    UnexpectedToolCall,
}

/// Metadata on the generation request's token usage.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageMetadata {

    /// Number of tokens in the prompt.
    pub prompt_token_count: i64,

    /// Number of tokens in the cached part of the prompt (the cached
    /// content)
    pub cached_content_token_count: Option<i64>,

    /// Total number of tokens across all the generated response
    /// candidates.
    pub candidates_token_count: Option<i64>,

    /// Number of tokens present in tool-use prompt(s).
    pub tool_use_prompt_token_count: Option<i64>,

    /// Number of tokens of thoughts for thinking models.
    pub thoughts_token_count: Option<i64>,

    /// Total token count for the generation request (prompt +
    /// response candidates)
    pub total_token_count: i64,
}
