// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-nocheck
"use client"

import { Button } from "@/components/ui/button"
import { AlignJustify, MessageSquare, Settings } from "lucide-react"
import { ReactNode, useContext, createContext, useState } from "react"
import { Command, CommandInput, CommandList, CommandEmpty } from "@/components/ui/command"

// Handles layout state.
type LayoutContext = {
  setRecentsOpen : (open: boolean) => void
  setChatOpen    : (open: boolean) => void
  isRecentsOpen  : boolean
  isChatOpen     : boolean
}

// Handles navigation layout state context.
const LayoutContext = createContext<LayoutContext | null>(null)

// Exposes layout context data within a LayoutProvider.
const useLayout = () => {
  const context = useContext(LayoutContext)
  if (!context) {
    throw new Error('useNav must be used within a NavProvider')
  }
  return context
}

// LayoutProvider handles its own LayoutContext and exposes it to
// children via a React context provider.
const LayoutProvider = ({ children } : { children: ReactNode }) => {
  const [isRecentsOpen, setRecentsOpen] = useState<boolean>(false)
  const [isChatOpen, setChatOpen] = useState<boolean>(false)
  return (
    <LayoutContext.Provider
      value={{ isChatOpen, setChatOpen, isRecentsOpen, setRecentsOpen }}>
      {children}
    </LayoutContext.Provider>
  )
}

// Navigation bar element.
const Layout = () => {
  const context = useLayout()
  const toggleChat = () => {context.setChatOpen(!context.isChatOpen)}
  const toggleRecents = () =>{context.setRecentsOpen(!context.isRecentsOpen)}
  return (
    <div className="fixed z-100 w-full flex flex-row p-3 justify-between">
      <div className="flex flex-row gap-2">
        <Button size="icon" onClick={toggleRecents}><AlignJustify /></Button>
        <Command>
          <CommandInput placeholder="Search Notes" />
          <CommandList>
            <CommandEmpty>No results found.</CommandEmpty>
          </CommandList>
        </Command>
      </div>
      <div className="flex flex-row gap-1">
        <Button size="icon" onClick={toggleChat}><MessageSquare /></Button>
        <Button size="icon"><Settings /></Button>
      </div>
    </div>
  )
}

// Arranges content according to a layout's state.
const LayoutContent = ({ children } : { children?: ReactNode }) => {
  const { isChatOpen, isRecentsOpen } = useLayout()
    return (
      <div className="flex justify-between">
        { (isRecentsOpen) ? (
          <div className="w-[450px] h-screen bg-[#f5f5f5cc]" />
        ) : null
        }
        <div className="w-full h-full pt-16">
          {children}
        </div>
        { (isChatOpen) ? (
          <div className="w-[370px] h-screen bg-[#f5f5f5cc]" />
        ) : null
        }
      </div>
    )}

export { LayoutProvider, LayoutContext, Layout, LayoutContent, useLayout }
