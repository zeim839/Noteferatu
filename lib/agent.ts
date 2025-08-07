import { invoke } from "@tauri-apps/api/core"

export type Model = {
  id: string
  displayName: string
  provider: string
  contextSize: number
}

export type AgentErr = {
  type: string,
  error: {
    type?: string,
    message?: string,
  },
}

// A conversation is a series of user, system, tool and response
// messages between a user and LLM.
export type Conversation = {
  id: number
  name: string
  createdAt: number
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
  return await invoke("plugin:agent|try_connect", {
    payload: { provider, apiKey },
  })
}

// List all available models from registered providers.
//
// To register a new LLM provider, use `tryConnect`.
export async function listModels(): Promise<Array<Model>> {
  return await invoke<Array<Model>>("plugin:agent|list_models")
}

// List conversation threads.
export async function listConversations(): Promise<Array<Conversation>> {
  return await invoke<Array<Conversation>>("plugin:agent|list_conversations")
}

// Create new conversation thread.
export async function createConversation(name: string): Promise<Conversation> {
  return await invoke<Conversation>("plugin:agent|create_conversation", {
    payload: { name }
  })
}

// Rename an existing conversation thread.
export async function renameConversation(id: number, newName: string): Promise<void> {
  return await invoke("plugin:agent|rename_conversation", {
    payload: { id, newName }
  })
}

// Delete a conversation thread.
export async function removeConversation(id: number): Promise<void> {
  return await invoke("plugin:agent|remove_conversation", {
    payload: { id }
  })
}

// Fetch conversation messages.
export async function fetchConversationMessages() {
}

// Send a message.
export async function sendMessage() {
}
