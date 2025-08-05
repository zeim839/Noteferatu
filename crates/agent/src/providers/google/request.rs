use serde::{Serialize, Deserialize};
use super::content::*;
use crate::core::{self, MessageContent};

/// Body of a chat completion request.
#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {

    /// Specifies the model that will respond to the request.
    #[serde(skip_serializing)]
    pub model: String,

    /// The content of the current conversation with the model.
    ///
    /// For single-turn queries, this is a single instance. For multi-turn
    /// queries like chat, this is a repeated field that contains the
    /// conversation history and the latest request.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contents: Vec<Content>,

    /// A list of Tools the [Model](super::model::Model) may use to
    /// generate the next response.
    ///
    /// A [Tool] is a piece of code that enables the system to
    /// interact with external systems to perform an action, or set of
    /// actions, outside of knowledge and scope of the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,

    /// Developer set system instruction(s). Currently, text only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<Content>,

    /// Configuration options for model generation and outputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<GenerationConfig>,
}

impl Request {

    /// Creates a [Request] from a single text prompt.
    pub fn from_prompt(model: &str, prompt: &str) -> Self {
        let mut req = Self::default();
        req.model = model.to_string();
        req.contents = vec![Content {
            role: Role::User,
            parts: vec![Part {
                thought: None,
                thought_signature: None,
                data: PartData::new().with_text(Some(prompt.to_string())),
            }],
        }];
        req
    }

    /// Creates a [Request] from a vector of messages.
    pub fn from_contents(model: &str, contents: Vec<Content>) -> Self {
        let mut req = Self::default();
        req.model = model.to_string();
        req.contents = contents;
        req
    }

    /// Populates the [Self::model] fields with the given model.
    pub fn with_model(self, model: &str) -> Self {
        Self { model: model.to_string(), ..self }
    }

    /// Populates the [Self::tools] field with the given value.
    pub fn with_tools(self, tools: Option<Vec<Tool>>) -> Self {
        Self { tools, ..self }
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

impl From<core::Request> for Request {
    fn from(value: core::Request) -> Self {
        let system_instruction = value.system.map(|s| Content {
            role: Role::User,
            parts: vec![Part {
                thought: None,
                thought_signature: None,
                data: PartData::new().with_text(Some(s)),
            }],
        });

        let tools = if value.tools.len() > 0 {
            Some(vec![Tool {
                function_declarations: Some(value.tools.into_iter().map(Into::into).collect()),
            }])
        } else { None };

        let contents = value.messages.iter().map(|msg| {
            let role = msg.role.clone().into();
            let parts = match &msg.content {
                MessageContent::Text(text) => vec![Part {
                    thought: None,
                        thought_signature: None,
                    data: PartData::new().with_text(Some(text.clone())),
                }],
                MessageContent::ToolCall(tool_call) => {
                    vec![Part {
                        thought: None,
                        thought_signature: None,
                        data: PartData::new().with_function_call(Some(FunctionCallPartData {
                            id: Some(tool_call.id.clone()),
                            name: tool_call.name.clone(),
                                args: Some(tool_call.arguments.clone()),
                        })),
                    }]
                }
                MessageContent::ToolResponse(tool_response) => {
                    let tool_call_name = value.messages.iter().find_map(|m| {
                        if let MessageContent::ToolCall(tc) = &m.content {
                            if tc.id == tool_response.id {
                                return Some(tc.name.clone());
                            }
                        }
                        None
                    }).expect("Tool response for a call must be preceded by that call.");
                    let response_json: serde_json::Value =
                        serde_json::from_str(&tool_response.content).unwrap_or_else(|_| {
                            serde_json::json!({ "result": &tool_response.content })
                        });

                    vec![Part {
                        thought: None,
                        thought_signature: None,
                        data: PartData::new().with_function_response(Some(
                            FunctionResponsePartData {
                                id: Some(tool_response.id.clone()),
                                name: tool_call_name,
                                response: response_json,
                            },
                        )),
                    }]
                }
            };
            Content { role, parts }
        }).collect();

        let generation_config = GenerationConfig::default()
            .with_thinking_config(Some(ThinkingConfig{
                include_thoughts: true,
                thinking_budget: 128,
            }))
            .with_max_output_tokens(value.max_tokens);

        Request {
            model: value.model,
            contents,
            tools,
            system_instruction,
            generation_config: Some(generation_config),
        }
    }
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
