"use client"

import { useState } from "react"
import { Button } from "@/components/ui/button"

type Message = {
  role    : "user" | "assistant"
  content : string
}

export default function Chat() {
  const [messages, setMessages] = useState<Message[]>([])
  const [source/*, setSource*/] = useState<string>('GPT')
  const [inputValue, setInputValue] = useState<string>("")
  const [isTyping, setIsTyping] = useState(false)

  // Send userMessage to your Next.js API route
  async function callChatAPI(userMessage: string, source: string) {
    const response = await fetch("/api/chat", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ userMessage, source })
    })

    if (!response.ok) {
      console.error("API error:", await response.text())
      return { text: "Sorry, something went wrong." }
    }

    const data = await response.json()
    return data
  }

  const handleSend = async () => {
    if (!inputValue.trim()) return

    // Show user's message immediately
    const userMessage: Message = { role: "user", content: inputValue }
    setMessages((prev) => [...prev, userMessage])
    setInputValue("")

    setIsTyping(true)

    // Call server route to get AI response
    const { text } = await callChatAPI(userMessage.content, source)

    // Add AI response
    const aiMessage: Message = { role: "assistant", content: text }
    setMessages((prev) => [...prev, aiMessage])
    setIsTyping(false)
  }

  // Pressing "Enter" triggers handleSend
  const handleKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (event.key !== "Enter") {
      return
    }
    event.preventDefault()
    handleSend();
  }

  const Messages = () => {
    // Default message when no messages
    if (messages.length === 0) {
      return (
        <div className="flex flex-col justify-center items-center text-center text-gray-700">
          <h2 className="text-xl font-bold">Chat with your Notes</h2>
          <p className="mt-2">
            Enter a message to start chatting with Notefaratu
          </p>
        </div>
      )
    }

    return (
      <div className="mt-12 flex-1 overflow-auto flex flex-col gap-3">
        {messages.map((msg, index) => {
          const isUser = msg.role === "user"
          return (
            <div
              key={index}
              className={`
                    max-w-[75%] rounded-sm p-3 text-sm
                    ${isUser ? "self-end bg-black text-white" : "self-start bg-[#F6F6F6] border border-[#979797] text-black"}
                  `}
            >
              {msg.content}
            </div>
          )
        })
        }
        {isTyping && (
          <div className="self-start border border-[#979797] bg-[#F6F6F6] text-black rounded-md p-3 text-sm">
            <div className="">
              <span className="animate-pulse">Typing</span>
              <span className="animate-pulse">...</span>
            </div>
          </div>
          )}
      </div>
    )
  }

  return (
    <div className="min-h-full grid grid-rows-[auto_40px]">
      <Messages />
      <div className="flex items-center">
        <input
          type="text"
          placeholder={`Message ${source}`}
          className="flex-1 border border-gray-300 rounded-md px-3 py-2 text-sm focus:outline-none"
          value={inputValue}
          onChange={(e) => setInputValue(e.target.value)}
          onKeyDown={handleKeyDown}
        />
        <Button className="ml-2 text-sm">Send</Button>
      </div>
    </div>
  )
}
