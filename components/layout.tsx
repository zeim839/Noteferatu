// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-nocheck
"use client"

import { Button } from "@/components/ui/button"
import { AlignJustify, MessageSquare, Settings } from "lucide-react"
import { ReactNode, useContext, createContext, useState } from "react"
import { Command, CommandInput, CommandList, CommandEmpty } from "@/components/ui/command"
import Chat from "@/components/chat/chat"

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

// LeftNavigation includes the search bar and "recents" button.
const LeftNavigation = () => {
  const { isRecentsOpen, setRecentsOpen } = useLayout()
  return (
    <div className="flex flex-row gap-2">
      <Button size="icon" onClick={() =>{setRecentsOpen(!isRecentsOpen)}}>
        <AlignJustify />
      </Button>
      <Command>
        <CommandInput placeholder="Search Notes" />
        <CommandList>
          <CommandEmpty>No results found.</CommandEmpty>
        </CommandList>
      </Command>
    </div>
  )
}

// RightNavigation includes the chat and settings buttons.
const RightNavigation = () => {
  const { isChatOpen, setChatOpen } = useLayout()
  return (
    <div className="flex flex-row gap-1">
      <Button size="icon" onClick={() => {setChatOpen(!isChatOpen)}}>
        <MessageSquare />
      </Button>
      <Button size="icon"><Settings /></Button>
    </div>
  )
}

// LeftSidebar contains the search function and recent documents content.
// It also includes its own LeftNavigation, which appears only when the
// sidebar is open.
const LeftSidebar = () => {
  const {isRecentsOpen} = useLayout()
  if (!isRecentsOpen) {
    return null
  }
  return (
    <div className="min-w-[372px] w-[372px] h-screen bg-background p-3">
      <div className="fixed z-101">
        <LeftNavigation />
      </div>
    </div>
  )
}

// RightSidebar contains the LLM chat component. It also includes its
// own RightNavigation, which appears only when the sidebar is open.
const RightSidebar = () => {
  const {isChatOpen} = useLayout()
  if (!isChatOpen) {
    return null
  }
  return (
    <div className="min-w-[420px] w-[420px] h-screen bg-background p-3">
      <div className="fixed z-101 right-3">
        <RightNavigation />
      </div>
      <Chat />
    </div>
  )
}

// Navigation bar element.
const Layout = ({ children } : { children?: ReactNode }) => {
  const { isRecentsOpen, isChatOpen }= useLayout()
  return (
    <div>
      <div className="fixed z-100 w-full flex flex-row p-3 justify-between">
        {(isRecentsOpen) ? <div /> : <LeftNavigation />}
        {(isChatOpen) ? <div />: <RightNavigation />}
      </div>
      <div className="flex justify-between">
        <LeftSidebar />
        <div className="w-full h-full pt-16">
          {children}
        </div>
        <RightSidebar />
      </div>
    </div>
  )
}

export { LayoutProvider, LayoutContext, Layout, useLayout }
