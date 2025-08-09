//! LLM chat completion library for Rust, with support for Anthropic,
//! Google AI, Ollama, OpenAI, and OpenRouter.
//!
//! Support both static and streaming responses.

pub mod core;
pub mod providers;

#[cfg(feature = "agent")]
pub mod agent;

#[cfg(feature = "plugin")]
pub mod plugin;
