use serde::{Serialize, Deserialize};

/// Specifies the producer of message content.
#[derive(Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "model")]
    Model,
    #[serde(rename = "function")]
    Function,
}

/// Body of a chat completion request.
#[derive(Default, Serialize, Deserialize)]
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
    #[serde(rename = "toolConfig")]
    pub tool_config: Option<ToolConfig>,

    /// Developer set system instruction(s). Currently, text only.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "systemInstruction")]
    pub system_instruction: Option<Content>,

    /// Configuration options for model generation and outputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "generationConfig")]
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
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
pub struct Part {

    /// Indicates if the part is thought from the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thought: Option<bool>,

    /// An opaque signature for the thought so it can be reused in
    /// subsequent requests.
    ///
    /// A base64-encoded string.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "thoughtSignature")]
    pub thought_signature: Option<String>,

    /// The data contained by this [Part].
    #[serde(flatten)]
    pub data: PartData,
}

#[derive(Default, Serialize, Deserialize)]
pub struct PartData {

    /// Inline text.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "text")]
    pub text: Option<String>,

    /// Inline media bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "inlineData")]
    pub inline_data: Option<BlobPartData>,

    /// A predicted FunctionCall returned from the model that contains
    /// a string representing the `FunctionDeclaration.name` with the
    /// arguments and their values.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "functionCall")]
    pub function_call: Option<FunctionCallPartData>,

    /// The result output of a `FunctionCall` that contains a string
    /// representing the `FunctionDeclaration.name` and a structured
    /// JSON object containing any output from the function is used as
    /// context to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "functionReponse")]
    pub function_response: Option<FunctionResponsePartData>,

    /// URI based data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fileData")]
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
#[derive(Serialize, Deserialize)]
pub struct BlobPartData {

    /// The IANA standard MIME type of the source data.
    #[serde(rename = "mimeType")]
    pub mime_type: String,

    /// Raw bytes for media formats.
    pub data: String,
}

/// A predicted `FunctionCall` returned from the model that contains a
/// string representing the `FunctionDeclaration.name` with the
/// arguments and their values.
#[derive(Serialize, Deserialize)]
pub struct FunctionCallPartData {

    /// The unique id of the function call. If populated, the client
    /// to execute the functionCall and return the response with the
    /// matching id.
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
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
pub enum Scheduling {

    /// This value is unused.
    #[serde(rename = "SCHEDULING_UNSPECIFIED")]
    SchedulingUnspecified,

    /// Only add the result to the conversation context, do not
    /// interrupt or trigger generation.
    #[serde(rename = "SILENT")]
    Silent,

    /// Add the result to the conversation context, and prompt to
    /// generate output without interrupting ongoing generation.
    #[serde(rename = "WHEN_IDLE")]
    WhenIdle,

    /// Add the result to the conversation context, interrupt ongoing
    /// generation and prompt to generate output.
    #[serde(rename = "INTERRUPT")]
    Interrupt,
}

/// URI based data.
#[derive(Serialize, Deserialize)]
pub struct FilePartData {

    /// The IANA standard MIME type of the source data.
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    /// URI.
    #[serde(rename = "fileUri")]
    pub file_uri: String,
}

/// Tool details that the model may use to generate response.
///
/// A `Tool` is a piece of code that enables the system to interact with
/// external systems to perform an action, or set of actions, outside
/// of knowledge and scope of the [Model](super::models::Model).
#[derive(Serialize, Deserialize)]
pub struct Tool {

    /// A list of `FunctionDeclarations` available to the model that can
    /// be used for function calling.
    ///
    /// The model or system does not execute the function. Instead the
    /// defined function may be returned as a `FunctionCall` with
    /// arguments to the client side for execution.
    #[serde(rename = "functionDeclarations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_declarations: Option<Vec<FunctionDeclaration>>,

    /// Retrieval tool that is powered by Google search.
    #[serde(rename = "googleSearchRetrieval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_search_retrieval: Option<GoogleSearchRetrieval>,

    /// GoogleSearch tool type. Tool to support Google Search in
    /// Model. Powered by Google.
    #[serde(rename = "googleSearch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_search: Option<GoogleSearch>,

    /// Tool to support URL context retrieval.
    #[serde(rename = "urlContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_context: Option<serde_json::Value>,
}

/// Structured representation of a function declaration as defined by
/// the OpenAPI 3.03 specification. Included in this declaration are
/// the function name and parameters. This FunctionDeclaration is a
/// representation of a block of code that can be used as a Tool by
/// the model and executed by the client.
#[derive(Serialize, Deserialize)]
pub struct FunctionDeclaration {

    /// The name of the function.
    pub name: String,

    /// A brief description of the function.
    pub description: String,

    /// Describes the parameters to the function in JSON Schema
    /// format. The schema must describe an object where the
    /// properties are the parameters to the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parametersJsonSchema")]
    pub parameters_json_schema: Option<serde_json::Value>,

    /// Describes the output from this function in JSON Schema
    /// format. The value specified by the schema is the response
    /// value of the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "responseJsonSchema")]
    pub response_json_schema: Option<serde_json::Value>,
}

/// Tool to retrieve public web data for grounding, powered by Google.
#[derive(Serialize, Deserialize)]
pub struct GoogleSearchRetrieval {

    /// Specifies the dynamic retrieval configuration for the given
    /// source.
    #[serde(rename = "dynamicRetrievalConfig")]
    pub dynamic_retrieval_config:  DynamicRetrievalConfig,
}

/// Describes the options to customize dynamic retrieval.
#[derive(Serialize, Deserialize)]
pub struct DynamicRetrievalConfig {

    /// The mode of the predictor to be used in dynamic retrieval.
    pub mode: DynamicRetrievalMode,

    /// The threshold to be used in dynamic retrieval. If not set, a
    /// system default value is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dynamicThreshold")]
    pub dynamic_threshold: Option<i64>,
}

/// The mode of the predictor to be used in dynamic retrieval.
#[derive(Serialize, Deserialize)]
pub enum DynamicRetrievalMode {

    /// Always trigger retrieval.
    #[serde(rename = "MODE_UNSPECIFIED")]
    ModeUnspecified,

    /// Run retrieval only when system decides it is necessary.
    #[serde(rename = "MODE_DYNAMIC")]
    ModeDynamic,
}

/// GoogleSearch tool type. Tool to support Google Search in
/// Model. Powered by Google.
#[derive(Serialize, Deserialize)]
pub struct GoogleSearch {

    /// Filter search results to a specific time range. If customers
    /// set a start time, they must set an end time (and vice versa).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timeRangeFilter")]
    time_range_filter: Option<Interval>,
}

/// Represents a time interval, encoded as a Timestamp start
/// (inclusive) and a Timestamp end (exclusive).
///
/// The start must be less than or equal to the end. When the start
/// equals the end, the interval is empty (matches no time). When both
/// start and end are unspecified, the interval matches any time.
#[derive(Serialize, Deserialize)]
pub struct Interval {

    /// Inclusive start of the interval.
    ///
    /// If specified, a Timestamp matching this interval will have to be
    /// the same or after the start.
    ///
    /// Uses RFC 3339, where generated output will always be Z-normalized
    /// and uses 0, 3, 6 or 9 fractional digits. Offsets other than "Z"
    /// are also accepted. Examples: "2014-10-02T15:01:23Z",
    /// "2014-10-02T15:01:23.045123456Z" or "2014-10-02T15:01:23+05:30".
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,

    /// Exclusive end of the interval.
    ///
    /// If specified, a Timestamp matching this interval will have to
    /// be before the end.
    ///
    /// Uses RFC 3339, where generated output will always be
    /// Z-normalized and uses 0, 3, 6 or 9 fractional digits. Offsets
    /// other than "Z" are also accepted. Examples:
    /// "2014-10-02T15:01:23Z", "2014-10-02T15:01:23.045123456Z" or
    /// "2014-10-02T15:01:23+05:30".
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
}

/// The Tool configuration containing parameters for specifying [Tool]
/// use in the request.
#[derive(Serialize, Deserialize)]
pub struct ToolConfig {

    /// Function calling config.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "functionCallingConfig")]
    pub function_calling_config: Option<FunctionCallingConfig>,
}

/// Configuration for specifying function calling behavior.
#[derive(Serialize, Deserialize)]
pub struct FunctionCallingConfig {

    /// Specifies the mode in which function calling should
    /// execute. If unspecified, the default value will be set to
    /// `AUTO`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<FunctionCallingMode>,

    /// A set of function names that, when provided, limits the
    /// functions the model will call.
    ///
    /// This should only be set when the Mode is ANY. Function names
    /// should match `FunctionDeclaration.name`. With mode set to `ANY`,
    /// model will predict a function call from the set of function names
    /// provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allowedFunctionNames")]
    pub allowed_function_names: Option<Vec<String>>,
}

/// Defines the execution behavior for function calling by defining
/// the execution mode.
#[derive(Serialize, Deserialize)]
pub enum FunctionCallingMode {

    /// Unspecified function calling mode. This value should not be
    /// used.
    #[serde(rename = "MODE_UNSPECIFIED")]
    ModeUnspecified,

    /// Default model behavior, model decides to predict either a
    /// function call or a natural language response.
    #[serde(rename = "AUTO")]
    Auto,

    /// [Model](super::models::Model) is constrained to always
    /// predicting a function call only. If `allowedFunctionNames` are
    /// set, the predicted function call will be limited to any one of
    /// `allowedFunctionNames`, else the predicted function call will
    /// be any one of the provided `functionDeclarations`.
    #[serde(rename = "ANY")]
    Any,

    /// [Model](super::models::Model) will not predict any function
    /// call. Model behavior is same as when not passing any function
    /// declarations.
    #[serde(rename = "NONE")]
    None,

    /// Model decides to predict either a function call or a natural
    /// language response, but will validate function calls with
    /// constrained decoding.
    #[serde(rename = "VALIDATED")]
    Validated,
}

/// Configuration options for model generation and outputs. Not all
/// parameters are configurable for every model.
#[derive(Default, Serialize, Deserialize)]
pub struct GenerationConfig {

    /// The requested modalities of the response. Represents the set
    /// of modalities that the model can return, and should be
    /// expected in the response. This is an exact match to the
    /// modalities of the response.
    ///
    /// A model may have multiple combinations of supported modalities. If
    /// the requested modalities do not match any of the supported
    /// combinations, an error will be returned.
    ///
    /// An empty list is equivalent to requesting only text.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "responseModalities")]
    pub response_modalities: Option<Vec<Modality>>,

    /// Number of generated responses to return. If unset, this will
    /// default to 1. Please note that this doesn't work for previous
    /// generation models (Gemini 1.0 family)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "candidateCount")]
    pub candidate_count: Option<i64>,

    /// The maximum number of tokens to include in a response
    /// candidate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxOutputTokens")]
    pub max_output_tokens: Option<i64>,

    /// Controls the randomness of the output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,

    /// The maximum cumulative probability of tokens to consider when
    /// sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "topP")]
    pub top_p: Option<f64>,

    /// The maximum number of tokens to consider when sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "topK")]
    pub top_k: Option<f64>,

    /// Seed used in decoding. If not set, the request uses a randomly
    /// generated seed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,

    /// Presence penalty applied to the next token's logprobs if the
    /// token has already been seen in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "presencePenalty")]
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
    #[serde(rename = "frequency_penalty")]
    pub frequency_penalty: Option<f64>,

    /// Config for thinking features. An error will be returned if
    /// this field is set for models that don't support thinking.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "thinkingConfig")]
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

    /// Populates the [Self::candidate_count] field with the given value.
    pub fn with_candidate_count(self, candidate_count: Option<i64>) -> Self {
        Self { candidate_count, ..self }
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

/// Config for thinking features.
#[derive(Serialize, Deserialize)]
pub struct ThinkingConfig {

    /// Indicates whether to include thoughts in the response. If
    /// true, thoughts are returned only when available.`
    #[serde(rename = "includeThoughts")]
    pub include_thoughts: bool,

    /// The number of thoughts tokens that the model should generate.
    #[serde(rename = "thinkingBudget")]
    pub thinking_budget: i64,
}

/// Content Part modality
#[derive(Serialize, Deserialize)]
pub enum Modality {

    /// Unspecified modality.
    #[serde(rename = "MODALITY_UNSPECIFIED")]
    ModalityUnspecified,

    /// Plain text.
    #[serde(rename = "TEXT")]
    Text,

    /// Image.
    #[serde(rename = "IMAGE")]
    Image,

    /// Video.
    #[serde(rename = "VIDEO")]
    Video,

    /// Audio.
    #[serde(rename = "AUDIO")]
    Audio,

    /// Document, e.g. PDF.
    #[serde(rename = "DOCUMENT")]
    Document,
}

/// Response from the [Model](super::models::Model) supporting
/// multiple candidate responses.
#[derive(Serialize, Deserialize)]
pub struct ChatResponse {

    /// Candidate responses from the model.
    pub candidates: Vec<Candidate>,

    /// Metadata on the generation requests' token usage.
    #[serde(rename = "usageMetadata")]
    pub usage_metadata: UsageMetadata,

    /// The model version used to generate the response.
    #[serde(rename = "modelVersion")]
    pub model_version: String,

    /// Used to identify each response.
    #[serde(rename = "responseId")]
    pub response_id: String,
}

/// A response candidate generated from the model.
#[derive(Serialize, Deserialize)]
pub struct Candidate {

    /// Generated content returned from the model.
    pub content: Content,

    /// The reason why the model stopped generating tokens.
    ///
    /// If empty, the model has not stopped generating tokens.
    #[serde(rename = "finishReason")]
    pub finish_reason: Option<FinishReason>,
}

/// Defines the reason why the model stopped generating tokens.
#[derive(Serialize, Deserialize)]
pub enum FinishReason {

    /// This value is unused.
    #[serde(rename = "FINISH_REASON_UNSPECIFIED")]
    FinishReasonUnspecified,

    /// Natural stop point of the model or provided stop sequence.
    #[serde(rename = "STOP")]
    Stop,

    /// The maximum number of tokens as specified in the request was
    /// reached.
    #[serde(rename = "MAX_TOKENS")]
    MaxTokens,

    /// The response candidate content was flagged for safety reasons.
    #[serde(rename = "SAFETY")]
    Safety,

    /// The response candidate content was flagged for recitation
    /// reasons.
    #[serde(rename = "RECITATION")]
    Recitation,

    /// The response candidate content was flagged for using an
    /// unsupported language.
    #[serde(rename = "LANGUAGE")]
    Language,

    /// Unknown reason.
    #[serde(rename = "OTHER")]
    Other,

    /// Token generation stopped because the content contains
    /// forbidden terms.
    #[serde(rename = "BLOCKLIST")]
    Blocklist,

    /// Token generation stopped for potentially containing prohibited
    /// content.
    #[serde(rename = "PROHIBITED_CONTENT")]
    ProhibitedContent,

    /// Token generation stopped because the content potentially
    /// contains Sensitive Personally Identifiable Information (SPII).
    #[serde(rename = "SPII")]
    Spii,

    /// The function call generated by the model is invalid.
    #[serde(rename = "MALFORMED_FUNCTION_CALL")]
    MalformedFunctionCall,

    /// Token generation stopped because generated images contain
    /// safety violations.
    #[serde(rename = "IMAGE_SAFETY")]
    ImageSafety,

    /// Model generated a tool call but no tools were enabled in the
    /// request.
    #[serde(rename = "UNEXPECTED_TOOL_CALL")]
    UnexpectedToolCall,
}

/// Metadata on the generation request's token usage.
#[derive(Serialize, Deserialize)]
pub struct UsageMetadata {

    /// Number of tokens in the prompt.
    #[serde(rename = "promptTokenCount")]
    pub prompt_token_count: i64,

    /// Number of tokens in the cached part of the prompt (the cached
    /// content)
    #[serde(rename = "cachedContentTokenCount")]
    pub cached_content_token_count: Option<i64>,

    /// Total number of tokens across all the generated response
    /// candidates.
    #[serde(rename = "candidatesTokenCount")]
    pub candidates_token_count: Option<i64>,

    /// Number of tokens present in tool-use prompt(s).
    #[serde(rename = "toolUsePromptTokenCount")]
    pub tool_use_prompt_token_count: Option<i64>,

    /// Number of tokens of thoughts for thinking models.
    #[serde(rename = "thoughtsTokenCount")]
    pub thoughts_token_count: Option<i64>,

    /// Total token count for the generation request (prompt +
    /// response candidates)
    #[serde(rename = "totalTokenCount")]
    pub total_token_count: i64,
}
