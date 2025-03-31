"use client"

import { useState, useEffect, useRef } from "react"
import { Stream, Message } from "@/lib/OpenRouter"
import { WandSparklesIcon, SendHorizontalIcon } from "lucide-react"
import { MessageView } from "./Messages"
import { useDB } from "@/components/DatabaseProvider"
import { toast } from "sonner"

import SourceDropdown from "./SourceDropdown"
import OpenAI from "openai"

type FormEvent = React.KeyboardEvent<HTMLInputElement>

export default function Chat() {
  const [messages, setMessages] = useState<Message[]>([])
  const [source, setSource] = useState<string>('deepseek/deepseek-r1:free')
  const [input, setInput] = useState<string>('')
  const [isTyping, setIsTyping] = useState<boolean>(true)
  const [isStreaming, setIsStreaming] = useState<boolean>(false)
  const [client, setClient] = useState<OpenAI>(new OpenAI({
    baseURL: "https://openrouter.ai/api/v1",
    dangerouslyAllowBrowser: true,
    apiKey: '',
  }))

  const db = useDB()
  
  //ensure each notification only shows up once
  const notifInit = useRef(false);

  // Fetch messages and API key from database.
  useEffect(() => {
    // TODO: HANDLE DATABASE ERRORS.
    if (notifInit.current) return; 
    notifInit.current = true;

    const fetchMessages = async () => {
      try{
        const msgs = await db.history.readAll()
        setMessages(msgs.map(msg => ({
          role    : msg.role,
          content : msg.content
        } as Message)))
      }
      catch{
        toast('Error: Failed to Load Chat Message', {
          description: 'The database failed to load chat message'
        })
      }
      finally{
        setIsTyping(false)
      }

    }
    const fetchKey = async () => {
      try{
        const keys = await db.keys.readAll()
        if (keys.length === 0) {
          toast('No API Key Found', {
            description: 'Please go to settings and add your OpenRouter API Key'
          })
          return
        }
        setClient(new OpenAI({
          baseURL: "https://openrouter.ai/api/v1",
          dangerouslyAllowBrowser: true,
          apiKey: keys[0].key_hash,
        }))
      }
      catch(error){
        toast('Error: Failed to load API Key', {
          description: 'Ensure your API Key is correct'
        })
      }

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

    // Chunks are appended to res because reading the 'messages'
    // does not return the latest data.
    let res : string = ""

    try {
      await Stream(client, updatedMessages, source,
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
        })

      // Insert response into chat history.
      await db.history.create({
        role    : 'assistant',
        content : res,
        time    : Math.floor(Date.now() / 1000)
      })

    } catch (error: unknown) {
      let description = 'An unknown error has occurred'
      if (error instanceof Error) {
        description = error.message
      }
      toast('Error: Could not Send Message', {description})
      setIsTyping(false)
    }
    setIsStreaming(false)
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
      <div className="grid grid-cols-[24px_auto_24px] gap-2 border border-gray-300 rounded-md px-3 py-2 text-sm focus:outline-none bg-white">
        <WandSparklesIcon className="text-[#ADADAD]"/>
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
