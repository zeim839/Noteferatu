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
      id: 42,
      title: 'Abstract Syntax Tree',
      content: "An abstract syntax tree (AST) is a data structure used in computer science to represent the structure of a program or code snippet. It is a tree representation of the abstract syntactic structure of text (often source code) written in a formal language. Each node of the tree denotes a construct occurring in the text. It is sometimes called just a syntax tree.",
      atime: Math.floor(Date.now() / 1000),
      mtime: Math.floor(Date.now() / 1000)
    })

    await db.notes.create({
      id: 420,
      title: 'Regular Expression',
      content: "A regular expression (shortened as regex or regexp), sometimes referred to as rational expression, is a sequence of characters that specifies a match pattern in text.",
      atime: Math.floor(Date.now() / 1000),
      mtime: Math.floor(Date.now() / 1000)
    })

    await db.notes.create({
      id: 69,
      title: 'Formal Language Theory',
      content: "In logic, mathematics, computer science, and linguistics, a formal language consists of words whose letters are taken from an alphabet and are well-formed according to a specific set of rules called a formal grammar.",
      atime: Math.floor(Date.now() / 1000),
      mtime: Math.floor(Date.now() / 1000)
    })

    await db.edges.create({src: 420, dst: 69})
    await db.edges.create({src: 42, dst: 69})
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
