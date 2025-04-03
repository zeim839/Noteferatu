"use client"

import { useState, useEffect, useRef } from "react"
import { toolDefinitions, toolImplementations, ToolCall } from "./tools"
import { Stream, Message } from "@/lib/OpenRouter"
import { MessageView } from "./Messages"
import { useDB } from "@/components/DatabaseProvider"
import { toast } from "sonner"
import SourceDropdown from "./SourceDropdown"
import OpenAI from "openai"

import {
  WandSparklesIcon,
  SendHorizontalIcon,
  GlobeIcon,
  WrenchIcon,
  CheckIcon
} from "lucide-react"

import {
  Popover,
  PopoverTrigger,
  PopoverContent,
  PopoverAnchor
} from "@/components/ui/popover"

type FormEvent = React.KeyboardEvent<HTMLInputElement>

export default function Chat() {
  const [messages, setMessages] = useState<(Message|ToolCall)[]>([])
  const [source, setSource] = useState<string>('google/gemini-2.5-pro-exp-03-25:free')
  const [input, setInput] = useState<string>('')
  const [isTyping, setIsTyping] = useState<boolean>(true)
  const [isStreaming, setIsStreaming] = useState<boolean>(false)
  const [webSearchEnabled, setWebSearchEnabled] = useState<boolean>(false)
  const [toolCallingEnabled, setToolCallingEnabled] = useState<boolean>(true)
  const [client, setClient] = useState<OpenAI>(new OpenAI({
    baseURL: "https://openrouter.ai/api/v1",
    dangerouslyAllowBrowser: true,
    apiKey: '',
  }))

  const db = useDB()

  // Fetch messages and API key from database.
  useEffect(() => {
    // TODO: HANDLE DATABASE ERRORS.
    const fetchMessages = async () => {
      const msgs = await db.history.readAll()
      setMessages(msgs.map(msg => {
        if (msg.role === 'tool') {
          return {
            tool    : msg.tool_name,
            id      : msg.id,
            content : msg.content,
          } as ToolCall
        }
        return {
          role    : msg.role,
          content : msg.content
        } as Message
      }))
      setIsTyping(false)
    }
    const fetchKey = async () => {
      const keys = await db.keys.readAll()
      if (keys.length === 0) {
        // TODO: SHOW DIALOG
        return
      }
      setClient(new OpenAI({
        baseURL: "https://openrouter.ai/api/v1",
        dangerouslyAllowBrowser: true,
        apiKey: keys[0].key_hash,
      }))
    }
    fetchMessages()
    fetchKey()
  }, [])

  // Ref for the dummy div at the bottom of the message list.
  // allows us to automatically scroll to bottom.
  const bottomRef = useRef<HTMLDivElement>(null)
  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" })
  }, [messages])

  // onSubmit sends a chat completion request to the OpenRouter API.
  // It streams the response or shows a 'typing' placeholder whenever
  // the response is still loading.
  const onSubmit = async () => {
    if (!input.trim()) return
    const userMessage = { role: "user", content: input }
    const updatedMessages = [...messages, userMessage as Message]
    setMessages(updatedMessages)
    setInput("")
    setIsTyping(true)
    setIsStreaming(true)

    // Insert user message into database.
    await db.history.create({
      role    : 'user',
      content : input,
      time    : Math.floor(Date.now() / 1000)
    })

    // Filter non-message values -- cannot pass these to OpenRouter.
    const context : Message[] = updatedMessages.filter(
      (data): data is Message => !('tool' in data)
    )

    // Chunks are appended to res because reading the 'messages'
    // does not return the latest data.
    let res : string = ""

    // Take timestamp early; this way the LLM response is saved
    // before any tool calls.
    const time : number = Math.floor(Date.now() / 1000)

    try {
      await Stream(client, context,
        // Append ":online" to model name to enable web search.
        `${source}${(webSearchEnabled) ? ':online' : ''}`,
        (chunk: string, i: number) => {
          res += chunk
          if (i == 0) {
            setMessages((prev) => [...prev, {
              role: 'assistant', content: res
            }])
            setIsTyping(false)
            return
          }
          // Update the last message to include new chunks.
          setMessages((prev) => {
            const newMessages = [...prev]
            newMessages[newMessages.length - 1] = {
              ...newMessages[newMessages.length - 1],
              content: res,
            }
            return newMessages
          })
        },
        (toolCallingEnabled) ? toolDefinitions : undefined,
        toolImplementations(db, setMessages))

      if (res === '') {
        return
      }

      // Insert response into chat history.
      await db.history.create({
        role    : 'assistant',
        content : res,
        time    : time
      })

    } catch (error: unknown) {
      let description = 'An unknown error has occurred'
      if (error instanceof Error) {
        description = error.message
      }
      toast('Error: Could not Send Message', {description})
      setIsTyping(false)
    } finally {
      setIsTyping(false)
      setIsStreaming(false)
    }
  }

  return (
    <div className="pt-12 min-h-full grid grid-rows-[auto_40px]">
      <div className="absolute top-2">
        <SourceDropdown
          onValueChange={(v) => setSource(v)}
          value={source}
        />
      </div>
      { MessageView(messages, isTyping, bottomRef) }
      <div className="grid grid-cols-[24px_auto_24px] gap-2 border border-gray-300 rounded-md px-3 py-2 text-sm focus:outline-none bg-white relative">
        <Popover>
          <PopoverTrigger asChild>
            <button className="flex items-center justify-center">
              <WandSparklesIcon className="text-[#ADADAD] hover:text-gray-600 transition-colors"/>
            </button>
          </PopoverTrigger>
          <PopoverAnchor asChild>
            <div className="absolute left-0 bottom-full" />
          </PopoverAnchor>
          <PopoverContent
            className="w-[180px] p-0"
            align="start"
            alignOffset={0}
            side="top"
            sideOffset={0}
          >
            <div className="py-1 text-sm">
              <button
                onClick={() => setWebSearchEnabled(!webSearchEnabled)}
                className="flex items-center justify-between w-full px-2 py-1.5 hover:bg-gray-100"
              >
                <div className="flex items-center gap-2">
                  <GlobeIcon className="h-4 w-4" />
                  <span>Web Search</span>
                </div>
                {webSearchEnabled && <CheckIcon className="h-4 w-4" />}
              </button>
              <button
                onClick={() => setToolCallingEnabled(!toolCallingEnabled)}
                className="flex items-center justify-between w-full px-2 py-1.5 hover:bg-gray-100"
              >
                <div className="flex items-center gap-2">
                  <WrenchIcon className="h-4 w-4" />
                  <span>Tool Calling</span>
                </div>
                {toolCallingEnabled && <CheckIcon className="h-4 w-4" />}
              </button>
            </div>
          </PopoverContent>
        </Popover>
        <input
          type="text"
          disabled={isTyping || isStreaming}
          placeholder={(isTyping || isStreaming) ? "Processing..." : `Message ${source}`}
          className="flex-1 focus:outline-none focus:ring-0"
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
        <SendHorizontalIcon onClick={onSubmit} className="cursor-pointer text-[#ADADAD]"/>
      </div>
    </div>
  )
}
