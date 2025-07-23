import * as React from "react"
import { Conversation } from "./conversation"
import { Model } from "@/lib/agent"

// Defines a common context for agent subcomponents.
export type AgentContextType = {

  // Control the conversation history subpanel. The conversation and
  // settings sidepanels should never be open simultaneously.
  isConvsOpen: () => boolean
  setConvsOpen: (open: boolean) => void
  toggleConvs: () => void,

  // Control the number of tokens used by the current conversation.
  tokensUsed: () => number
  setTokensUsed: (usage: number) => void

  // Control the number of tokens that fit into the current model's
  // context window
  totalTokens: () => number
  setTotalTokens: (total: number) => void

  // Whether the settings subpanel is open. The conversation and settings
  // sidepanels should never be open simultaneously
  isSettingsOpen: () => boolean
  setSettingsOpen: (open: boolean) => void
  toggleSettings: () => void

  // Control the conversation history.
  convHistory: () => Array<Conversation>
  setConvHistory: (convs: Array<Conversation>) => void

  // Control the LLM models available to the user.
  models: () => Record<string, Array<Model>>
  setModels: (provider: string, models: Array<Model>) => void
}

// Implements AgentContextType.
export const AgentContext = React.createContext<AgentContextType | null>(null)

// Exposes AgentContext to agent subcomponents.
export function AgentProvider({ children }: { children: React.ReactNode }) {
  const [isConvsOpen, setConvsOpen] = React.useState<boolean>(false)
  const [tokensUsed, setTokensUsed] = React.useState<number>(0)
  const [totalTokens, setTotalTokens] = React.useState<number>(200000)
  const [isSettingsOpen, setSettingsOpen] = React.useState<boolean>(false)
  const [models, setModels] = React.useState<Record<string, Array<Model>>>({})
  const [convHistory, setConvHistory] = React.useState<Conversation[]>([
    {
      id: "0",
      name: "Software Engineering Assistance Request (need help)",
      createdAt: 1752250136,
    },
    { id: "1", name: "Software Engineering Help", createdAt: 1752240136 },
    { id: "2", name: "Another Example Conversation", createdAt: 1752230136 },
  ])

  const context: AgentContextType = {
    isConvsOpen: () => { return isConvsOpen },
    setConvsOpen: (open) => setConvsOpen(open),
    toggleConvs: () => {
      setConvsOpen(!isConvsOpen)
      if (isSettingsOpen) {
        setSettingsOpen(false)
      }
    },

    tokensUsed: () => { return tokensUsed },
    setTokensUsed: (usage) => setTokensUsed(usage),

    totalTokens: () => { return totalTokens },
    setTotalTokens: (total) => setTotalTokens(total),

    isSettingsOpen: () => { return isSettingsOpen },
    setSettingsOpen: (open) => setSettingsOpen(open),
    toggleSettings: () => {
      setSettingsOpen(!isSettingsOpen)
      if (isConvsOpen) {
        setConvsOpen(false)
      }
    },

    convHistory: () => { return convHistory },
    setConvHistory: (convs) => setConvHistory(convs),

    models: () => { return models },
    setModels: (provider, models) => {
      if (models.length == 0) {
        setModels((prev) => {
          // eslint-disable-next-line @typescript-eslint/no-unused-vars
          const { [provider]: _, ...rest } = prev
          return rest
        })
        return
      }
      setModels((prev) => ({ ...prev, [provider]: models}))
    }
  }

  return (
    <AgentContext.Provider value={context}>
      {children}
    </AgentContext.Provider>
  )
}

export function useAgentContext() {
  const context = React.useContext(AgentContext)
  if (!context) {
    throw new Error("useAgentContext must be called within AgentProvider")
  }
  return context
}
