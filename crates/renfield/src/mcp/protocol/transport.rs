/// Supported transport protocols.
pub enum Transport {

    /// MCP is a shorthand for the streamable HTTP transport protocol.
    MCP,

    /// SSE is the HTTP+SSE **deprecated** transport protocol.
    #[deprecated(note = "sse transport protocol is deprecated")]
    SSE,

    /// STD is the `stdio` transport protocol.
    STD,
}
