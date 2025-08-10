"use client"

import * as React from "react"
import { Sidebar } from "@/components/window/sidebar"
import { Button } from "@/components/core/button"
import * as Conversation from "./conversation"
import { AgentSettings } from "./settings"
import { useAgentContext } from "./context"
import { Message } from "./message"
import { Channel } from "@tauri-apps/api/core"
import { MessageLoadingIndicator } from "./indicator"
import { ModelSelector } from "./model_selector"

import {
  Message as MessageData,
  sendStreamMessage,
  createConversation,
  StreamEvent,
  Error as AgentError,
  stopMessages,
} from "@/lib/agent"

import {
  ChevronDownIcon,
  ChevronRightIcon,
  SlidersHorizontalIcon,
  SendHorizontalIcon,
  PlusIcon,
  SquareIcon,
} from "lucide-react"

// Converts a token count into a string. If less than 1000, then
// `tokens` is returned, otherwise truncate and add a 'k' prefix.
const tokenStr = (tokens: number) => {
  return tokens < 1000
    ? tokens.toString()
    : (tokens / 1000).toFixed(tokens % 1000 == 0 ? 0 : 1).toString() + "K"
}

// Agent is the parent element of the chat sidebar.
function Agent() {
  const [inputValue, setInputValue] = React.useState("")
  const [isStreaming, setIsStreaming] = React.useState<boolean>(false)
  const [latestRes, setLatestRes] = React.useState<MessageData | null>(null)
  const [error, setError] = React.useState<AgentError | null>(null)

  const ctx = useAgentContext()

  // Reference to a dummy element that tracks the bottom of the message
  // history. Used to conveniently scroll to the latest message.
  const messagesEndRef = React.useRef<HTMLDivElement>(null)

  React.useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" })
  }, [ctx.messages, latestRes, isStreaming])

  // When the conversation ID changes, the latest message will be fetched
  // from the database. This wipes it from the react state.
  React.useEffect(() => {
    setLatestRes(null)
  }, [ctx.selectedConv])

  // Sends a chat completion request and listens for streaming events.
  const sendMessage = async (conversation_id: number) => {

    // Add the last response to the message history before
    // sending a new one.
    const allMessages = latestRes ?
      [...ctx.messages, latestRes] :
      ctx.messages

    const newUserMessage = {
      role: "user",
      content: inputValue.trim()
    } as MessageData

    // Update component message history state.
    ctx.setMessages([...allMessages, newUserMessage])
    setLatestRes(null)

    // Clear the input field.
    setInputValue("")

    // Enables the "stop message" button and shows loading animation.
    setIsStreaming(true)

    // Listen for stream events using a Tauri channel.
    const model = ctx.selectedModel
    const onEvent = new Channel<StreamEvent>()
    onEvent.onmessage = (event) => {
      if (event.event === 'content' && event.data?.messages[0]?.content) {
        const chunk = event.data.messages[0].content as string
        setLatestRes(prev => ({
          role: 'assistant',
          content: (prev?.content ?? "") + chunk
        }))
      }
    }

    // The backend keeps track of the message history, so we only need to
    // provide the latest message. Once the stream is finished, the
    // final/accumulated response is returned.
    return sendStreamMessage(conversation_id, {
        model: `${model?.provider}:${model?.id}`,
        messages: [newUserMessage],
        tools: [],
      }, onEvent)
      .then((res) => {
        setLatestRes(res.messages[0])
      })
      .catch((err: AgentError) => {
        console.log(err)
        setError(err)
      })
      .finally(() => {
        setIsStreaming(false)
      })
  }

  // Handles clicking the "Send Message" button.
  //
  // If no conversation is selected, then a conversation is created prior
  // to sending the message. Otherwise, the message is added to the
  // current message history.
  const handleSendMessage = () => {
    let conversation = ctx.selectedConv
    if (inputValue.trim() && conversation !== null) {
      sendMessage(conversation.id)
    }

    if (inputValue.trim() && conversation === null) {
      createConversation("New Conversation")
        .then((conversation) => {
          ctx.setSelectedConv(conversation)
          setTimeout(() => {sendMessage(conversation.id)}, 100)
        })
    }
  }

  // Toggle the conversation history panel.
  const onToggleConvs = () => {
    if (!ctx.isConvsOpen) {
      ctx.setIsModelSelectorOpen(false)
    }
    ctx.toggleConvs()
  }

  // Toggle the settings panel.
  const onToggleSettings = () => {
    if (!ctx.isSettingsOpen) {
      ctx.setIsModelSelectorOpen(false)
    }
    ctx.toggleSettings()
  }

  return (
    <div className="w-full min-w-[250px] h-full flex flex-col">
      <Sidebar.Header className="flex flex-row justify-between items-center px-1 w-full min-h-[29px] z-1">
        <Button
          tooltip="Choose Conversation"
          tooltipSide="bottom"
          onClick={onToggleConvs}
          variant="outline"
          className="p-2 rounded-sm h-6 px-1 flex items-center justify-between pr-2"
        >
          {ctx.isConvsOpen ? (
            <ChevronDownIcon strokeWidth={1.6} />
          ) : (
            <ChevronRightIcon strokeWidth={1.6} />
          )}
          <p className="text-xs max-h-[15px] max-w-[150px] text-nowrap text-ellipsis overflow-x-hidden overflow-y-hidden">
            {
              /* Conversation Title */
              ctx.selectedConv !== null ?
                ctx.selectedConv?.name :
                "New Conversation"
            }
          </p>
        </Button>
        <div className="flex items-center gap-1">
          <Button
            variant="ghost"
            size="icon"
            tooltip="New Conversation"
            onClick={() => {
              ctx.setSelectedConv(null)
              ctx.setConvsOpen(false)
            }}
          >
            <PlusIcon strokeWidth={1.6} />
          </Button>
          <Button
            variant="ghost"
            size="icon"
            tooltip="Agent Settings"
            onClick={onToggleSettings}
          >
            <SlidersHorizontalIcon strokeWidth={1.6} />
          </Button>
        </div>
      </Sidebar.Header>

      {/* Conversation history (panel) */}
      <div
        className="absolute top-[29px] border-[#ABB0BE] border-t-1 w-full h-0 bg-[#E5E9EF] hidden data-[is-expanded=true]:block data-[is-expanded=true]:h-[calc(100vh-35px-30px)] z-1"
        data-is-expanded={ctx.isConvsOpen}
      >
        <Conversation.Body body={ctx.convHistory} />
      </div>

      {/* Agent Settings (panel) */}
      <div
        className="absolute top-[29px] border-[#ABB0BE] border-t-1 w-full h-0 bg-[#E5E9EF] hidden data-[is-expanded=true]:block data-[is-expanded=true]:h-[calc(100vh-35px-30px)] z-1"
        data-is-expanded={ctx.isSettingsOpen}
      >
        <AgentSettings />
      </div>

      {/* Message History */}
      <div className="flex-1 w-full overflow-y-auto max-h-[calc(100vh-35px-30px-150px)]">
        {ctx.messages.map((msg, index) => <Message key={index} data={msg}/>)}
        {isStreaming && !latestRes && (<MessageLoadingIndicator />)}
        {latestRes && <Message data={latestRes} />}
        <div ref={messagesEndRef} />
      </div>

      {/* Message Input Field */}
      <div className="w-full h-[150px] bg-[#EDF0F4] border-t border-[#AEB3C0] p-2 flex flex-col justify-between">
        <div className="flex-1 relative">
          <textarea
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter" && !e.shiftKey) {
                e.preventDefault()
                handleSendMessage()
              }
            }}
            placeholder="Ask anything..."
            className="w-full h-full min-h-[60px] p-2 text-sm resize-none bg-none focus:outline-none"
          />
        </div>
        <div className="flex flex-row items-center justify-between">
          <ModelSelector />
          <div className="flex items-center gap-1">
            {
              (!isStreaming) ?
                <Button
                  variant="outline"
                  size="icon"
                  tooltip="Send Message"
                  disabled={ctx.selectedModel === null || !inputValue.trim()}
                  onClick={handleSendMessage}
                >
                  <SendHorizontalIcon strokeWidth={1.6} />
                </Button> :
                <Button
                  variant="outline"
                  size="icon"
                  tooltip="Stop Message"
                  className="text-red-600"
                  onClick={() => {
                    const conversation = ctx.selectedConv
                    if (conversation !== null) {
                      stopMessages(conversation.id)
                    }
                  }}
                >
                  <SquareIcon strokeWidth={1.6} />
                </Button>
            }
          </div>
        </div>
      </div>
    </div>
  )
}

export { Agent }
