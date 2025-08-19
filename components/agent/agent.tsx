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
import { TokenInfo } from "./token"
import { ErrorCard } from "./error"

import {
  Message as MessageData,
  sendStreamMessage,
  createConversation,
  StreamEvent,
  Error as AgentError,
  stopMessages,
  updateMessage,
} from "@/lib/agent"

import {
  ChevronRightIcon,
  SlidersHorizontalIcon,
  SendHorizontalIcon,
  PlusIcon,
  SquareIcon,
} from "lucide-react"

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

  // Erase any errors or uncommitted responses when conversation changes.
  React.useEffect(() => {
    setLatestRes(null)
    setError(null)
  }, [ctx.selectedConv])

  // Sends a chat completion request and listens for streaming events.
  const sendMessage = async (conversation_id: number, newMessage?: MessageData, messages?: Array<MessageData>) => {
    setError(null)

    const _messages = (messages) ? messages :  ctx.messages
    let latestRes: MessageData | null = null

    // Update component message history state.
    if (newMessage) {
      _messages.push(newMessage)
      ctx.setMessages(_messages)
    }

    // Clear the input field.
    setInputValue("")

    // Enables the "stop message" button and shows loading animation.
    setIsStreaming(true)

    // Listen for stream events using a Tauri channel.
    const model = ctx.selectedModel
    const onEvent = new Channel<StreamEvent>()
    onEvent.onmessage = (event) => {
      if (event.event === 'content') {
        if (event.data?.messages[0]?.content) {
          const chunk = event.data.messages[0].content as string
          latestRes = {
            role: 'assistant',
            content: (latestRes?.content ?? "") + chunk
          }
          setLatestRes(latestRes)
          if (event.data.usage.totalTokens) {
            ctx.setTokensUsed(event.data.usage.totalTokens)
          }
        }
        if (event.data?.error) {
          setError(event.data?.error)
        }
      }
    }

    // The backend keeps track of the message history, so we only need to
    // provide the latest message. Once the stream is finished, the
    // final/accumulated response is returned.
    return sendStreamMessage(conversation_id, {
      model: `${model?.provider}:${model?.id}`,
      messages: (newMessage) ? [newMessage] : [],
      tools: [],
    }, onEvent)
      .then((res) => {
        if (res.usage.totalTokens > 0) {
          ctx.setTokensUsed(res.usage.totalTokens)
        }
      })
      .catch((err: AgentError) => {
        setError(err)
      })
      .finally(() => {
        setIsStreaming(false)
        ctx.setMessages(latestRes ?
          [..._messages, latestRes] :
          _messages
        )
        setLatestRes(null)
      })
  }

  // Handles clicking the "Send Message" button.
  //
  // If no conversation is selected, then a conversation is created prior
  // to sending the message. Otherwise, the message is added to the
  // current message history.
  const handleSendMessage = () => {
    let conversation = ctx.selectedConv
    const newMessage = { role: "user", content: inputValue.trim() } as MessageData
    if (!isStreaming && inputValue.trim() && conversation !== null) {
      sendMessage(conversation.id, newMessage)
    }

    if (!isStreaming && inputValue.trim() && conversation === null) {
      createConversation("New Conversation")
        .then((conversation) => {
          // (STUPID) HACK: sets a delay of 100ms, allowing the new
          // conversation to be created and the component state to update.
          ctx.setSelectedConv(conversation)
          setTimeout(() => {sendMessage(conversation.id, newMessage)}, 100)
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

  // Retry sending a failed message.
  const onRetry = () => {
    let conversation = ctx.selectedConv
    if (conversation) {
      setError(null)
      sendMessage(conversation.id)
    }
  }

  // Edit a message.
  const onEditMessage = (index: number, content: string) => {
    const conversation = ctx.selectedConv
    if (conversation !== null) {
      const newMessage: MessageData = { role: 'user', content }
      updateMessage(index, conversation.id, newMessage)
        .then(() => {
          setLatestRes(null)
          ctx.setMessages(ctx.messages.slice(0, index+1))
          sendMessage(conversation.id, undefined, ctx.messages.slice(0, index+1))
        })
    }
  }

  return (
    <div className="w-full min-w-70 h-full flex flex-col">
      <Sidebar.Header className="flex flex-row justify-between items-center px-1 w-full min-h-[29px] z-1">
        <Button
          tooltip="Choose Conversation"
          tooltipSide="bottom"
          onClick={onToggleConvs}
          variant="outline"
          className="p-2 rounded-sm h-6 px-1 flex items-center justify-between pr-2"
        >
          <ChevronRightIcon
            data-is-open={ctx.isConvsOpen}
            strokeWidth={1.6}
            className="data-[is-open=true]:rotate-90 transition-all"
          />
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
              // Stop streaming before changing.
              if (ctx.selectedConv !== null) {
                stopMessages(ctx.selectedConv.id).then(() => {
                  ctx.setSelectedConv(null)
                  ctx.setConvsOpen(false)
                  setLatestRes(null)
                })
                return
              }
              ctx.setSelectedConv(null)
              ctx.setConvsOpen(false)
              setLatestRes(null)
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
      <div className="flex-1 w-full overflow-y-auto max-h-[calc(100vh-35px-30px-150px)] break-words">
        {ctx.messages.map((msg, index) => <Message key={index} index={index} data={msg} onEdit={onEditMessage} />)}
        {isStreaming && !latestRes && (<MessageLoadingIndicator />)}
        {latestRes && <Message index={ctx.messages.length} data={latestRes} onEdit={onEditMessage} />}
        <div ref={messagesEndRef} />
      </div>

      {
        /* Error Description Card */
        error !== null ?
          <ErrorCard onRetry={onRetry} error={error} /> : null
      }

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
          <div className="flex flex-row items-center gap-1">
            <ModelSelector />
            <TokenInfo />
          </div>
          <div className="flex items-center gap-1">
            {
              (!isStreaming) ?
                <Button
                  variant="outline"
                  size="icon"
                  tooltip="Send Message"
                  className="bg-[#D4D8E1] hover:bg-[#ABB0BE] shadow-xs rounded-sm"
                  onClick={handleSendMessage}
                >
                  <SendHorizontalIcon strokeWidth={1.6} />
                </Button> :
                <Button
                  variant="outline"
                  size="icon"
                  tooltip="Stop Message"
                  className="bg-red-300 text-red-500 hover:bg-red-300 animate-pulse size-6"
                  onClick={() => {
                    const conversation = ctx.selectedConv
                    if (conversation !== null) {
                      stopMessages(conversation.id)
                    }
                  }}
                >
                  <SquareIcon fill="currentColor" strokeWidth={1.6} />
                </Button>
            }
          </div>
        </div>
      </div>
    </div>
  )
}

export { Agent }
