import { Button } from "@/components/ui/button"
import { useRouter } from "next/navigation"
import { useDB } from "@/components/DatabaseProvider"
import Chat from "@/components/chat/Chat"
import { NavigationState } from "./NavigationState"

import {
  BeanIcon,
  MessageSquareIcon,
  PlusIcon,
  SettingsIcon
} from "lucide-react"

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
      id: 420,
      title: 'Quandale Dingle Lore',
      content: "Quandale Dingle Lore: The Movie may contain material that could be considered sensitive or inappropriate for certain audiences, such as content related to the story behind the subject. If you are sensitive to this type of material, you should refrain from proceeding further.",
      atime: Math.floor(Date.now() / 1000),
      mtime: Math.floor(Date.now() / 1000)
    })

    await db.notes.create({
      id: 69,
      title: 'Skibidi Toilet Lore',
      content: "skibidi toilet is the very first episode Episode in the Skibidi Toilet series, created by DaFuq!?Boom!. It marks the start of the war between The Alliance and Skibidi Toilets.",
      atime: Math.floor(Date.now() / 1000),
      mtime: Math.floor(Date.now() / 1000)
    })

    await db.edges.create({src: 420, dst: 69})
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

// RightSidebar shows the LLM chat sidebar.
const RightSidebar = () => (
  <div className='min-w-[420px] w-[420px] h-screen bg-[rgba(245,245,245,0.75)] p-2 border border-l-gray-300'>
    <Chat />
  </div>
)

export { RightNavigation, RightSidebar }
