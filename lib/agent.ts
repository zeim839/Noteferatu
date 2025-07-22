import { invoke } from '@tauri-apps/api/core'

export async function tryConnect(provider: string, apiKey: string): Promise<null> {
  return await invoke<{value?: string}>('plugin:plugin-agent|try_connect', {
    payload: {
      provider,
      apiKey,
    },
  }).then(() => null)
}
