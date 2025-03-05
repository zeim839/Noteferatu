"use client"

import { useState, useEffect, useRef } from "react"
import { Button } from "@/components/ui/button"
import { Stream, Message, Model } from "@/lib/OpenRouter"
import { MessageView } from "./messages"

type FormEvent = React.KeyboardEvent<HTMLInputElement>

export default function Chat() {
  const [messages, setMessages] = useState<Message[]>([])
  const [source/*, setSource */] = useState<Model>('ChatGPT')
  const [input, setInput] = useState<string>('')
  const [isTyping, setIsTyping] = useState<boolean>(false)

  // Ref for the dummy div at the bottom of the message list.
  // allows us to automatically scroll to bottom.
  const bottomRef = useRef<HTMLDivElement>(null)
  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" })
  }, [messages])

  const onSubmit = async () => {
    if (!input.trim()) return
    const userMessage = { role: "user", content: input }
    const updatedMessages = [...messages, userMessage as Message]
    setMessages(updatedMessages)
    setInput("")
    setIsTyping(true)
    let res : string = ""
    await Stream(updatedMessages, source, (chunk: string, i: number) => {
      res += chunk
      if (i == 0) {
        setIsTyping(false)
        setMessages((prev) => [...prev, { role: 'assistant', content: res }])
        return
      }
      setMessages((prev) => {
        const newMessages = [...prev]
        newMessages[newMessages.length - 1] = {
          ...newMessages[newMessages.length - 1],
          content: res,
        }
        return newMessages
      })
    })
  }

  return (
    <div className="pt-12 min-h-full grid grid-rows-[auto_40px]">
      { MessageView(messages, isTyping, bottomRef) }
      <div className="flex items-center">
        <input
          type="text"
          placeholder={`Message ${source}`}
          className="flex-1 border border-gray-300 rounded-md px-3 py-2 text-sm focus:outline-none"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={(event: FormEvent) => {
            if (event.key !== 'Enter') {
              return
            }
            event.preventDefault()
            onSubmit()
          }}
        />
        <Button className="ml-2 text-sm" onClick={onSubmit}>Send</Button>
      </div>
    </div>
  )
}
