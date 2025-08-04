use super::response::Response;
use super::request::Request;
use super::model::Model;

use std::future::Future;

/// An LLM provider client.
///
/// Generates chat completions (streaming or non-streaming) using a
/// common [Request](super::Request)/[Response](super::Response)
/// interface using models made available by a provider.
pub trait Client {

    /// The client-specific error implementation.
    type Error: std::error::Error;

    /// The client-specific SSE implementation.
    type StreamResponse;

    /// The client-specific API request.
    type Request: From<Request>;

    /// The client-specific API response.
    type Response: Into<Response>;

    /// Generate a non-streaming chat completion.
    fn completion(&self, req: Self::Request) ->
    impl Future<Output = Result<Self::Response, Self::Error>>;

    /// Generate a streaming chat completion.
    fn stream_completion<T: FnMut(Self::StreamResponse)>
        (&self, req: Self::Request, cb: T) ->
    impl Future<Output = Result<(), Self::Error>>;

    /// List the models available on the client.
    fn list_models(&self) ->
    impl Future<Output = Result<Vec<Model>, Self::Error>>;

    /// Check whether the client is connected.
    fn check(&self) -> impl Future<Output = Result<(), Self::Error>>;
}
