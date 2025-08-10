import * as React from "react"
import { listen } from '@tauri-apps/api/event'

import {
  Model,
  Conversation,
  Message,
  listConversations,
  listMessages
} from "@/lib/agent"

// Defines a common context for agent subcomponents.
export type AgentContextType = {

  // Control the conversation history subpanel. The conversation and
  // settings sidepanels should never be open simultaneously.
  isConvsOpen: boolean
  setConvsOpen: (open: boolean) => void
  toggleConvs: () => void,

  // Control the number of tokens used by the current conversation.
  tokensUsed: number
  setTokensUsed: (usage: number) => void

  // Control the number of tokens that fit into the current model's
  // context window
  totalTokens: number
  setTotalTokens: (total: number) => void

  // Whether the settings subpanel is open. The conversation and settings
  // sidepanels should never be open simultaneously
  isSettingsOpen: boolean
  setSettingsOpen: (open: boolean) => void
  toggleSettings: () => void

  // Control the LLM models available to the user.
  models: Record<string, Array<Model>>
  setModels: (provider: string, models: Array<Model>) => void

  // Control the selected model.
  selectedModel: Model | null
  setSelectedModel: (model: Model | null) => void

  // Control the conversation history.
  convHistory: Array<Conversation>
  setConvHistory: (convs: Array<Conversation>) => void

  // Control the currently selected conversation.
  selectedConv: Conversation | null
  setSelectedConv: (conv: Conversation | null) => void

  // Conversation messages.
  messages: Array<Message>
  setMessages: (messages: Array<Message>) => void

  // Control whether the model selector popover is open.
  isModelSelectorOpen: boolean
  setIsModelSelectorOpen: (open: boolean) => void
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
  const [selectedModel, setSelectedModel] = React.useState<Model | null>(null)
  const [convHistory, setConvHistory] = React.useState<Conversation[]>([])
  const [selectedConv, setSelectedConv] = React.useState<Conversation | null>(null)
  const [messages, setMessages] = React.useState<Array<Message>>([])
  const [isModelSelectorOpen, setIsModelSelectorOpen] = React.useState<boolean>(false)

  // Fetch conversation history.
  React.useEffect(() => {
    const fetchConversations = () => {
      listConversations().then((conversations) => {
        setConvHistory(conversations)
      })
    }
    const convEventPromise = listen("agent-conversations-change",
      () => { fetchConversations() }
    )
    fetchConversations()
    return () => {convEventPromise.then((unlisten) => unlisten())}
  }, [])

  // Fetch messages history for the currently-selected conversation.
  React.useEffect(() => {
    if (selectedConv === null) {
      setMessages([])
      return
    }
    listMessages(selectedConv.id).then(msgs => {
      setMessages(msgs)
    })
  }, [selectedConv])

  const context: AgentContextType = {
    isConvsOpen,
    setConvsOpen: (open) => setConvsOpen(open),
    toggleConvs: () => {
      setConvsOpen(!isConvsOpen)
      if (isSettingsOpen) {
        setSettingsOpen(false)
      }
    },

    tokensUsed,
    setTokensUsed: (usage) => setTokensUsed(usage),

    totalTokens,
    setTotalTokens: (total) => setTotalTokens(total),

    isSettingsOpen,
    setSettingsOpen: (open) => setSettingsOpen(open),
    toggleSettings: () => {
      setSettingsOpen(!isSettingsOpen)
      if (isConvsOpen) {
        setConvsOpen(false)
      }
    },

    models,
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
    },

    selectedModel,
    setSelectedModel: (model) => setSelectedModel(model),

    convHistory,
    setConvHistory: (convs) => setConvHistory(convs),

    selectedConv,
    setSelectedConv: (conv) => setSelectedConv(conv),

    messages,
    setMessages: (msgs) => setMessages(msgs),

    isModelSelectorOpen,
    setIsModelSelectorOpen: (open: boolean) => setIsModelSelectorOpen(open),
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
