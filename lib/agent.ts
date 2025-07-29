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

export async function tryConnect(
  provider: string,
  apiKey: string,
): Promise<null> {
  return await invoke("plugin:agent|try_connect", {
    payload: {
      provider,
      apiKey,
    },
  })
}

export async function listModels(provider?: string): Promise<Array<Model>> {
  return await invoke<Array<Model>>("plugin:agent|list_models", {
    payload: { provider },
  })
}
