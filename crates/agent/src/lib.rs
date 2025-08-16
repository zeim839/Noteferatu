//! LLM chat completion library for Rust, with support for Anthropic,
//! Google AI, Ollama, OpenAI, and OpenRouter.
//!
//! Support both static and streaming responses.

pub mod providers;
pub mod core;

#[cfg(feature = "agent")]
pub mod agent;

#[cfg(feature = "plugin")]
pub mod plugin;
