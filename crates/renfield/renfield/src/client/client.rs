use super::request::Request;
use super::response::Response;
use super::model::Model;

use std::sync::mpsc::Sender;
use std::result::Result;
use std::future::Future;

/// Common LLM client interface.
pub trait Client {

    /// The client-specific error implementation
    type Error: std::error::Error;

    /// The client's chat completion request schema.
    type Request: From<Request>;

    /// The client's chat completion response schema.
    type Response: Into<Response>;

    /// List the models available to the client.
    fn list_models(&self) -> impl Future<Output = Result<Vec<Model>, Self::Error>>;

    /// Generate a chat completion.
    fn generate(&self, req: Self::Request) -> impl Future<Output = Result<Self::Response, Self::Error>>;

    /// Stream a chat completion.
    ///
    /// LLM streams use server-sent events, meaning a single HTTP
    /// request elicits multiple responses (called events) from the
    /// API.
    ///
    /// Calling [`stream`](Self) means an LLM's response may be read
    /// as it is being generated in real-time.
    ///
    /// # Example
    ///
    /// ```
    /// todo!();
    /// ```
    fn stream(&self, req: Self::Request, pipe: Sender<Response>) -> impl Future<Output = Result<(), Self::Error>>;
}
