//! LLM provider [Client](crate::core::Client) APIs

#[cfg(feature = "anthropic")]
pub mod anthropic;

#[cfg(feature = "google")]
pub mod google;

#[cfg(feature = "ollama")]
pub mod ollama;

#[cfg(feature = "openai")]
pub mod openai;

#[cfg(feature = "openrouter")]
pub mod openrouter;
