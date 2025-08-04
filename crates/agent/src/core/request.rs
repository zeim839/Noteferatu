use super::message::*;
use super::tools::*;

/// A generic LLM chat completion request.
#[derive(Debug, Clone)]
pub struct Request {

    /// Model ID used to generate the response.
    pub model: String,

    /// A list of messages comprising the conversation so far.
    pub messages: Vec<Message>,

    /// An upper bound for the number of tokens that can be generated
    /// for a completion, including visible output tokens an reasoning
    /// tokens.
    pub max_tokens: Option<i64>,

    /// A list of tools that the model may call.
    ///
    /// Currently, only functions are supported as a tool. Use this to
    /// provide a list of functions the model may generate JSON inputs
    /// for.
    pub tools: Vec<ToolDefinition>,

    /// System prompt.
    pub system: Option<String>,
}
