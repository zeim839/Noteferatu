//! LLM chat completion library for Rust, with support for Anthropic,
//! Google AI, Ollama, OpenAI, and OpenRouter.
//!
//! Support both static and streaming responses.
//!
//! ## providers
//!
//! The [providers] module exposes structs for interacting with the
//! [Anthropic](providers::anthropic), [Google](providers::google),
//! [Ollama](providers::ollama), [OpenAI](providers::openai), and
//! [OpenRouter](providers::openrouter) chat completion APIs. It
//! supports both static and streaming responses.
//!
//! Each provider API is implemented as a [Client](core::Client) trait
//! that accepts [Request](core::Request), [Response](core::Response)
//! types that are common across all API implementations.
//!
//! To enable a provider API, you must specify the relevant Cargo
//! features:
//!  - `providers`: enables all clients.
//!  - `anthropic`: enables the [Anthropic](providers::anthropic) client.
//!  - `google`: enables the [Google](providers::google) client.
//!  - `ollama`: enables the [Ollama](providers::ollama) client.
//!  - `openai`: enables the [OpenAI](providers::openai) client.
//!  - `openrouter`: enables the [OpenRouter](providers::openrouter) client.
//!
//! ## agent
//!
//! The [agent] module is a single, unified interface for interacting
//! with any of the available model providers. It is backed by an
//! [SQLite database](database) that persists LLM conversation threads
//! and their respective message histories.
//!
//! Enable the [agent] module by activating the `agent` Cargo
//! feature. Enabling `agent` automatically enables all providers.
//!
//! ## plugin
//!
//! The [plugin] module implements a [tauri] plugin that exposes the
//! Agent library to Desktop and Mobile apps.

pub mod core;
pub mod providers;

#[cfg(feature = "agent")]
pub mod agent;

#[cfg(feature = "plugin")]
pub mod plugin;
