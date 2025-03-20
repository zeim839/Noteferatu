"use client"

import { ReactNode, useState } from "react"
import { Button } from "@/components/ui/button"
import { Command, CommandInput } from "@/components/ui/command"
import { useRouter, usePathname } from "next/navigation"
import { useDB } from "@/components/DatabaseProvider"
import { cn } from "@/lib/utils"

import Recents from "@/components/recents/Recents"
import Chat from "@/components/chat/chat"

import {
  AlignJustifyIcon,
  BeanIcon,
  HouseIcon,
  MessageSquareIcon,
  PlusIcon,
  SettingsIcon
} from "lucide-react"

// NavigationState conveniently exposes the state of the Navigation
// component to LeftNavigation and RightNavigation.
type NavigationState = {
  isLeftOpen   : boolean
  isRightOpen  : boolean
  setLeftOpen  : (open: boolean) => void
  setRightOpen : (open: boolean) => void
}

// LeftNavigation consists of the 'recents' hamburger menu and search bar.
const LeftNavigation = ({ state } : { state: NavigationState }) => {
  const isNotePage = usePathname() === '/note'
  const router = useRouter()
  return (
    <div className='flex flex-row gap-1 z-20 fixed left-2 top-2'>
      { /* Show a 'home' button when on the note page */ }
      { (isNotePage) ? (
        <Button size='icon' onClick={() => router.push('/')}>
          <HouseIcon />
        </Button>
      ) : null
      }
      { /* Toggles the left sidebar */ }
      <Button
        onClick={() => state.setLeftOpen(!state.isLeftOpen)}
        size='icon'
      >
        <AlignJustifyIcon />
      </Button>
      { /* Shrink the search bar when the 'home' button is shown */ }
      { /* @ts-expect-error: Don't care. */ }
      <Command className={`${isNotePage ? 'w-[263px]' : 'w-[305px]'}`}>
        <CommandInput placeholder="Search Notes" />
      </Command>
    </div>
  )
}

// RightNavigation consists of the create note, LLM chat, and
// settings buttons.
const RightNavigation = ({ state } : { state: NavigationState }) => {
  const router = useRouter()
  const db = useDB()

  const temporarySeed = async () => {
    if (!db) return

    if (await db.notes.count() > 0) {
      await db.notes.deleteAll()
      return
    }

    await db.notes.create({
      title: 'Open Source Club',
      content: "Embrace the power of collaborative creation at the University of Florida's Open Source Club. Meet new friends, propose ideas, learn programming, and work on open source projects.",
      atime: Math.floor(Date.now() / 1000),
      mtime: Math.floor(Date.now() / 1000)
    })

    await db.notes.create({
      title: 'TikTok Rizz Party',
      content: "A Rizz party is a group of friends having fun on prom night, dancing to the song Carnival by Kanye and it went super viral to the point where",
      atime: Math.floor(Date.now() / 1000),
      mtime: Math.floor(Date.now() / 1000)
    })

    await db.notes.create({
      title: 'Quandale Dingle Lore',
      content: "Quandale Dingle Lore: The Movie may contain material that could be considered sensitive or inappropriate for certain audiences, such as content related to the story behind the subject. If you are sensitive to this type of material, you should refrain from proceeding further.",
      atime: Math.floor(Date.now() / 1000),
      mtime: Math.floor(Date.now() / 1000)
    })

    await db.notes.create({
      title: 'Skibidi Toilet Lore',
      content: "skibidi toilet is the very first episode Episode in the Skibidi Toilet series, created by DaFuq!?Boom!. It marks the start of the war between The Alliance and Skibidi Toilets.",
      atime: Math.floor(Date.now() / 1000),
      mtime: Math.floor(Date.now() / 1000)
    })
  }

  return (
    <div className='flex flex-row gap-1 fixed top-2 right-2 z-20'>
      { /* Redirecting to the note page creates a new note */ }
      <Button size='icon' onClick={() => router.push('/note')}>
        <PlusIcon />
      </Button>
      <Button size='icon' onClick={() => temporarySeed()}>
        <BeanIcon />
      </Button>
      { /* Toggle the right sidebar */ }
      <Button
        onClick={() => state.setRightOpen(!state.isRightOpen)}
        size='icon'
      >
        <MessageSquareIcon />
      </Button>
      <Button size='icon'>
        <SettingsIcon />
      </Button>
    </div>
  )
}

// LeftSidebar shows recently accessed notes or search results.
const LeftSidebar = () => {
  // Use fixed positioning for the GraphView to prevent shifting
  // nodes and edges.
  const fixedPos = usePathname() === '/' ? 'fixed left-0' : ''
  return (
    <div className={cn(fixedPos, 'min-w-[366px] w-[366px] h-screen bg-[rgba(245,245,245,0.75)] p-2 border border-r-gray-300 z-10')}>
      <Recents />
    </div>
  )
}

// RightSidebar shows the LLM chat sidebar.
const RightSidebar = () => (
  <div className='min-w-[420px] w-[420px] h-screen bg-[rgba(245,245,245,0.75)] p-2 border border-l-gray-300'>
    <Chat />
  </div>
)

const Navigation = ({ children } : { children?: ReactNode }) => {
  const [isLeftOpen, setLeftOpen] = useState<boolean>(false)
  const [isRightOpen, setRightOpen] = useState<boolean>(false)

  // Use a solid background color for notes page otherwise text
  // visibility is poor.
  const background = usePathname() === '/note' ?
    'bg-[#FBF9F3]' : 'bg-transparent'

  // Wrap Navigation state to conveniently pass it to LeftNavigation
  // and RightNavigation.
  const navState = () => ({
    isLeftOpen, setLeftOpen, isRightOpen, setRightOpen
  } as NavigationState)

  return (
    <div>
      <LeftNavigation state={navState()} />
      <RightNavigation state={navState()} />
      <div className={cn(background, 'flex justify-between')}>
        { isLeftOpen ? <LeftSidebar /> : null }
        <div className='w-full h-full overflow-hidden'>
          {children}
        </div>
        { isRightOpen ? <RightSidebar /> : null }
      </div>
    </div>
  )
}

export default Navigation
