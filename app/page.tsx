"use client"

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-nocheck
import { Button } from "@/components/ui/button"
import { AlignJustify, MessageSquare, Settings } from 'lucide-react'
import { useState } from "react"

import {
  Command,
  CommandInput,
  CommandList,
  CommandEmpty
} from "@/components/ui/command"

import ChatOverlay from "@/components/ui/chat"

export default function Home() {
  const [isOpen, setIsOpen] = useState(false);
  const [source, setSource] = useState("GPT");

  const toggleChat = () => {
    setIsOpen(!isOpen);
  }

  const handleSourceChange = (newSource: string) => {
    setSource(newSource);
  };

  return (
    <div className='w-full flex flex-row p-3 justify-between'>
      <div className='flex flex-row gap-2'>
        <Button size='icon'><AlignJustify /></Button>
        <Command>
          <CommandInput placeholder='Search Notes' />
          <CommandList>
            <CommandEmpty>No results found.</CommandEmpty>
          </CommandList>
        </Command>
      </div>

      <div className='flex flex-row gap-1'>
        <Button onClick={toggleChat} size='icon'><MessageSquare /></Button>
        <Button size='icon'><Settings /></Button>
      </div>

      {/* Chat Overlay */}
      <ChatOverlay
        isOpen={isOpen}
        source={source}
        onClose={toggleChat}
        onSourceChange={handleSourceChange}
      />

    </div>
  )
}
