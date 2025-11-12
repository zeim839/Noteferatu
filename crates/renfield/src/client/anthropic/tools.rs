//! Tool call definitions, configurations, and results.

use serde::{Serialize, Deserialize};
use super::message::{CacheControl, Content, Document};

/// How the model should use the tools provided in
/// [Request](super::Request).
#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ToolChoice {

    /// The model is prohibited from using any tools.
    None,

    /// The model will automatically decide whether to use tools.
    Auto {

        /// Whether to disable parallel tool use.
        #[serde(skip_serializing_if = "Option::is_none")]
        disable_parallel_tool_use: Option<bool>,
    },

    /// The model will use any available tools.
    Any {

        /// Whether to disable parallel tool use.
        #[serde(skip_serializing_if = "Option::is_none")]
        disable_parallel_tool_use: Option<bool>,
    },

    /// The model will use the specified tool.
    Tool {

        /// The name of the tool to use.
        name: String,

        /// Whether to disable parallel tool use.
        #[serde(skip_serializing_if = "Option::is_none")]
        disable_parallel_tool_use: Option<bool>,
    },
}

impl ToolChoice {

    /// Create a [ToolChoice] instance that prohibits all tool use.
    pub fn none() -> Self {
        Self::None
    }

    /// Create a [ToolChoice] instance that automatically decides
    /// whether to use tools.
    pub fn auto(disable_parallel_tool_use: Option<bool>) -> Self {
        Self::Auto { disable_parallel_tool_use }
    }

    /// Create a [ToolChoice] instance that will use any available
    /// tools.
    pub fn any(disable_parallel_tool_use: Option<bool>) -> Self {
        Self::Any { disable_parallel_tool_use }
    }

    /// Create a [ToolChoice] instance that will use a specified tool.
    pub fn tool(name: &str, disable_parallel_tool_use: Option<bool>) -> Self {
        Self::Tool { name: name.to_string(), disable_parallel_tool_use }
    }
}

/// Configuration for calling MCP server tools.
///
/// See also: [McpServerUrl](super::McpServerUrl) and
/// [Request](super::Request).
#[derive(Serialize, Debug, Clone, Default)]
pub struct McpToolConfig {

    /// A list of server tools that the model is allowed to call.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_tools: Vec<String>,

    /// Whether tool calling is enabled for the associated server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}


/// Tool use [ContentPart](super::message::ContentPart).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolUse {
    pub id: String,
    pub input: serde_json::Value,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<CacheControl>,
}

/// Tool call result [ContentPart](super::message::ContentPart).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolResult {
    pub tool_use_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<CacheControl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}

/// Server tool use [ContentPart](super::message::ContentPart).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerToolUse {
    pub id: String,
    pub input: serde_json::Value,
    pub name: ServerTool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<CacheControl>,
}

/// Names of Anthropic server tools.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ServerTool {
    WebSearch,
    WebFetch,
    CodeExecution,
    BashCodeExecution,
    TextEditorCodeExecution,
}

/// Web search tool result [ContentPart](super::message::ContentPart).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebSearchToolResult {
    pub content: Vec<WebSearchToolResultBlock>,
    pub tool_use_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<CacheControl>,
}

/// [WebSearchToolResult] result content block.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WebSearchToolResultBlock {
    WebSearchResult {
        encrypted_content: String,
        title: String,
        url: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        page_age: Option<String>,
    },
    WebSearchToolResultError {
        error_code: String,
    },
}

/// Web fetch tool result [ContentPart](super::message::ContentPart).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebFetchToolResult {
    pub content: WebFetchToolResultBlock,
    pub tool_use_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<CacheControl>,
}

/// [WebFetchToolResult] content block.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WebFetchToolResultBlock {
    WebFetchToolResultError {
        error_code: String,
    },
    WebFetchResult {
        url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        retrieved_at: Option<String>,
        content: WebFetchResultContent,
    },
}

/// [WebFetchToolResultBlock] result content.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WebFetchResultContent {
    Document {
        source: Document,
        title: String,
    },
}

/// Code execution tool result [ContentPart](super::message::ContentPart).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeExecutionToolResult {
    pub content: CodeExecutionToolResultBlock,
    pub tool_use_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<CacheControl>,
}

/// [CodeExecutionToolResult] content block.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CodeExecutionToolResultBlock {
    CodeExecutionToolResultError {
        error_code: String,
    },
    CodeExecutionResult {
        return_code: u64,
        stderr: String,
        stdout: String,
        content: CodeExecutionOutputBlock,
    },
}

/// [CodeExecutionToolResultBlock] output block.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CodeExecutionOutputBlock {
    CodeExecutionOutput { file_id: String }
}

/// Bash code execution tool result [ContentPart](super::message::ContentPart).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BashCodeExecutionToolResult {
    pub tool_use_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<CacheControl>,
    pub content: BashCodeExecutionToolResultBlock,
}

/// [BashCodeExecutionToolResult] content block.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BashCodeExecutionToolResultBlock {
    BashCodeExecutionToolResultError {
        error_code: String,
    },
    BashCodeExecutionOutput {
        content: BashCodeExecutionOutputBlock,
        return_code: u64,
        stderr: String,
        stdout: String,
    },
}

/// [BashCodeExecutionToolResultBlock] output block.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BashCodeExecutionOutputBlock {
    BashCodeExecutionOutput { file_id: String },
}

/// Text editor execution tool result [ContentPart](super::message::ContentPart).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextEditorExecutionToolResult {
    pub tool_use_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<CacheControl>,
    pub content: TextEditorExecutionToolResultBlock,
}

/// [TextEditorExecutionToolResult] content block.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TextEditorExecutionToolResultBlock {
    TextEditorCodeExecutionToolResultError {
        error_code: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        error_message: Option<String>,
    },
    TextEditorCodeExecutionViewResult {
        content: String,
        file_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        num_lines: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        start_line: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        total_lines: Option<u64>,
    },
    TextEditorCodeExecutionCreateResult {
        is_file_update: bool,
    },
    TextEditorCodeExecutionStrReplaceResult {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        lines: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        new_lines: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        new_start: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        old_lines: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        old_start: Option<u64>,
    },
}

/// MCP tool use [ContentPart](super::message::ContentPart).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct McpToolUse {
    pub id: String,
    pub input: serde_json::Value,
    pub name: String,
    pub server_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<CacheControl>,
}

/// MCP tool result [ContentPart](super::message::ContentPart).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct McpToolResult {
    pub tool_use_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<CacheControl>,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}
