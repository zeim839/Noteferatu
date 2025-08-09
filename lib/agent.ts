import { invoke, Channel } from "@tauri-apps/api/core"

// Description of an LLM model.
export type Model = {
  id: string
  displayName: string
  provider: string
  contextSize: number
}

// A conversation is a series of user, system, tool and response
// messages between a user and LLM.
export type Conversation = {
  id: number
  name: string
  createdAt: number
}

// Chat completion request.
export type Request = {
  model: string
  messages: Array<Message>
  maxTokens?: number
  tools: Array<ToolDefinition>
  system?: string
}

// Response to an LLM completion request.
export type Response = {
  messages: Array<Message>
  usage: Usage
}

// Defines a function-calling tool.
export type ToolDefinition = {
  name: string
  description: string
  parameters?: object
}

// Conversation token usage information.
export type Usage = {
  promptTokens: number
  completionTokens: number
  totalTokens: number
}

// Message to/from an LLM.
export type Message = {
  role: Role
  content: MessageContent
}

// Error response types.
export type ErrorType = 'client' | 'anthropic' | 'google' | 'ollama' |
 'openai' | 'openrouter' | 'json' | 'invalidModelId' |
 'providerNotConfigured' | 'sql' | 'io' | 'pluginInvoke' | 'plugin'

// Enumeration of possible error contents.
export type ErrorDataType = string | AnthropicError |
GoogleError | OpenAIError | ClientError

// Google AI error response.
export type GoogleError = {

  // Error code (same as HTTP status).
  code: number

  // Message describing the error.
  message: string

  // Error status.
  status: string
}

// OpenAI error response.
export type OpenAIError = {

  // Message describing the error.
  message: string

  // The type of error.
  type: string

  // Error code.
  code?: string
}

// Anthropic error response.
export type AnthropicError = {

  // Message describing the error.
  message: string

  // The type of error.
  type: string
}

// HTTP client error.
export type ClientError = {

  // HTTP status for when the error is from an HTTP error response.
  status?: number

  // Error message.
  message: string

  // A possible URL related to this error.
  url?: string
}

// Error response.
export type Error = {
  type: ErrorType
  data: ErrorDataType
}

// Role of a message sender.
export type Role = "system" | "user" | "assistant" | "tool"

// Enumeration of the possible types of message contents.
export type MessageContent = string | ToolCall | ToolResponse

// Information passed to a tool function call.
export type ToolCall = {
  id: string
  name: string
  arguments: object
}

// Response to a ToolCall (i.e. result of executing the tool call).
export type ToolResponse = {
  id: string
  content: string
}

// A streaming completion event.
export type StreamEvent = {
  event: 'started' | 'content' | 'finished'
  data?: Response
}

// Attempt to connect to an LLM provider.
//
// If successful, the provider's models can be used in subsequent
// requests (i.e. listModels, sendMessage). Otherwise, an error is
// thrown.
export async function tryConnect(
  provider: string,
  apiKey: string,
): Promise<null> {
  return await invoke("plugin:agent|try_connect", { provider, apiKey })
}

// List all available models from registered providers.
//
// To register a new LLM provider, use `tryConnect`.
export async function listModels(provider?: string): Promise<Array<Model>> {
  return await invoke<Array<Model>>("plugin:agent|list_models", { provider })
}

// List conversation threads.
export async function listConversations(): Promise<Array<Conversation>> {
  return await invoke<Array<Conversation>>("plugin:agent|list_conversations")
}

// Create new conversation thread.
export async function createConversation(name: string): Promise<Conversation> {
  return await invoke<Conversation>("plugin:agent|create_conversation", { name })
}

// Rename an existing conversation thread.
export async function renameConversation(id: number, newName: string): Promise<void> {
  return await invoke("plugin:agent|rename_conversation", { id, newName })
}

// Delete a conversation thread.
export async function removeConversation(id: number): Promise<void> {
  return await invoke("plugin:agent|remove_conversation", { id })
}

// Fetch conversation messages.
export async function fetchConversationMessages() {
}

// Send a message.
export async function sendMessage(
  conversationId: number,
  req: Request
): Promise<Response> {
  return await invoke<Response>("plugin:agent|send_message", {
    conversationId, request: req
  })
}

// Send a message that accepts a Tauri IPC channel for
// streaming completions.
//
// If the completion succeeds, the finalized message is returned.
export async function sendStreamMessage(
  conversationId: number,
  request: Request,
  channel: Channel<StreamEvent>
): Promise<Response> {
  return await invoke<Response>("plugin:agent|send_stream_message", {
    conversationId, request, channel
  })
}

// Fetches the message history for the specified conversation.
export async function listMessages(conversationId: number): Promise<Array<Message>> {
  return await invoke<Array<Message>>("plugin:agent|list_messages", {
    conversationId
  })
}
