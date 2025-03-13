// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-nocheck
"use client"

import { Button } from "@/components/ui/button"
import { ReactNode, useContext, createContext, useState } from "react"
import { Command, CommandInput, CommandList, CommandEmpty } from "@/components/ui/command"
import { useRouter } from "next/navigation"
import Chat from "@/components/chat/chat"
import Recents from "./recents/Recents"
import { useEditorBackground } from "@/components/background"
import NoteController from "@/lib/controller/NoteController"
import { appLocalDataDir } from "@tauri-apps/api/path"
import path from "path"

import {
  AlignJustify,
  HouseIcon,
  PlusIcon,
  MessageSquare,
  Settings,
  BeanIcon
} from "lucide-react"

// Handles layout state.
type LayoutContext = {
  setRecentsOpen : (open: boolean) => void
  setChatOpen    : (open: boolean) => void
  setBackButton  : (open: boolean) => void
  isRecentsOpen  : boolean
  isChatOpen     : boolean
  isBackButton   : boolean
}

// Handles navigation layout state context.
const LayoutContext = createContext<LayoutContext | null>(null)

// Exposes layout context data within a LayoutProvider.
const useLayout = () => {
  const context = useContext(LayoutContext)
  if (!context) {
    throw new Error('useLayout must be used within a LayoutProvider')
  }
  return context
}

// LayoutProvider handles its own LayoutContext and exposes it to
// children via a React context provider.
const LayoutProvider = ({ children } : { children: ReactNode }) => {
  const [isRecentsOpen, setRecentsOpen] = useState<boolean>(false)
  const [isChatOpen, setChatOpen] = useState<boolean>(false)
  const [isBackButton, setBackButton] = useState<boolean>(false)
  return (
    <LayoutContext.Provider
      value={{ isChatOpen, setChatOpen, setBackButton, isBackButton,
        isRecentsOpen, setRecentsOpen }}>
      {children}
    </LayoutContext.Provider>
  )
}

// LeftNavigation includes the search bar and "recents" button.
const LeftNavigation = () => {
  const { isRecentsOpen, setRecentsOpen } = useLayout()
  const { isBackButton, setBackButton } = useLayout()
  const router = useRouter()

  const onBackButton = () => {
    if (!isBackButton) {
      return
    }
    router.push('/')
    setBackButton(false)
  }

  return (
    <div className="flex flex-row gap-2">
      { (isBackButton) ? (
        <Button size="icon" onClick={onBackButton}>
          <HouseIcon />
        </Button>
      ) : null
      }
      <Button size="icon" onClick={() =>{setRecentsOpen(!isRecentsOpen)}}>
        <AlignJustify />
      </Button>
      <Command className={`${(isBackButton) ? 'w-[256]' : 'w-[300]'}`}>
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
  const router = useRouter()

  const temporarySeed = async () => {
    const dbDir = await appLocalDataDir()
    const controller = new NoteController(path.join(dbDir, 'db.sqlite'))
    const count = await controller.count()
    if (count > 0) {
      await controller.deleteAll()
      return
    }
    await controller.create({
      title: 'Open Source Club',
      content: "Embrace the power of collaborative creation at the University of Florida's Open Source Club. Meet new friends, propose ideas, learn programming, and work on open source projects.",
      atime: Math.floor(Date.now() / 1000),
      mtime: Math.floor(Date.now() / 1000)
    })
    await controller.create({
      title: 'TikTok Rizz Party',
      content: "A Rizz party is a group of friends having fun on prom night, dancing to the song Carnival by Kanye and it went super viral to the point where",
      atime: Math.floor(Date.now() / 1000),
      mtime: Math.floor(Date.now() / 1000)
    })
    await controller.create({
      title: 'Quandale Dingle Lore',
      content: "Quandale Dingle Lore: The Movie may contain material that could be considered sensitive or inappropriate for certain audiences, such as content related to the story behind the subject. If you are sensitive to this type of material, you should refrain from proceeding further.",
      atime: Math.floor(Date.now() / 1000),
      mtime: Math.floor(Date.now() / 1000)
    })
    await controller.create({
      title: 'Skibidi Toilet Lore',
      content: "skibidi toilet is the very first episode Episode in the Skibidi Toilet series, created by DaFuq!?Boom!. It marks the start of the war between The Alliance and Skibidi Toilets.",
      atime: Math.floor(Date.now() / 1000),
      mtime: Math.floor(Date.now() / 1000)
    })
  }

  return (
    <div className="flex flex-row gap-1">
      <Button size="icon" onClick={() => {router.push('/note')}}>
        <PlusIcon />
      </Button>
      <Button size="icon" onClick={() => { temporarySeed() }}>
        <BeanIcon />
      </Button>
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
    <div className="min-w-[372px] w-[372px] h-screen bg-[rgba(245,245,245,0.75)] p-3 border border-r-gray-300">
      <div className="fixed z-101 h-screen">
        <LeftNavigation />
      </div>
      <Recents />
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
    <div className="min-w-[420px] w-[420px] h-screen bg-[rgba(245,245,245,0.75)] p-3 border border-l-gray-300">
      <div className="fixed z-101 right-3 h-screen">
        <RightNavigation />
      </div>
      <Chat />
    </div>
  )
}

// Navigation bar element.
const Layout = ({ children } : { children?: ReactNode }) => {
  const { isRecentsOpen, isChatOpen }= useLayout()
  const { isEditorMode } = useEditorBackground()
  return (
    <div>
      <div className="fixed z-100 w-full flex flex-row p-3 justify-between">
        {(isRecentsOpen) ? <div /> : <LeftNavigation />}
        {(isChatOpen) ? <div />: <RightNavigation />}
      </div>
      <div className="flex justify-between">
        {
          // Set the LeftSidebar to a fixed position when using
          // the graph view to prevent the graph shifting to the
          // right.
          (isEditorMode) ? <LeftSidebar /> : (
            <div className='fixed z-10'>
              <LeftSidebar />
            </div>
          )
        }
        <div className="w-full h-full pt-16 overflow-x-auto" style={{ backgroundColor: isEditorMode ? '#FBF9F3' : 'transparent' }}>
          {children}
        </div>
        <RightSidebar />
      </div>
    </div>
  )
}

export { LayoutProvider, LayoutContext, Layout, useLayout }
