import { Button } from "@/components/ui/button"
import { NavigationState } from "./NavigationState"
import { MessageSquareIcon, PlusIcon, SettingsIcon } from "lucide-react"
import Chat from "@/components/chat/Chat"
import Settings from "../chat/Settings"

// RightNavigation consists of the create note, LLM chat, and
// settings buttons.
const RightNavigation = ({ state } : { state: NavigationState }) => (
  <div className='flex flex-row gap-1 fixed top-2 right-2 z-20'>
    { /* Redirecting to the note page creates a new note */ }
    <Button size='icon' onClick={() => window.location.href = '/note'}>
      <PlusIcon />
    </Button>
    { /* Toggle the right sidebar */ }
    <Button
      onClick={() => state.setRightOpen(!state.isRightOpen)}
      size='icon'
    >
      <MessageSquareIcon />
    </Button>
    <Settings />
  </div>
)

// RightSidebar shows the LLM chat sidebar.
const RightSidebar = () => (
  <div className='min-w-[420px] w-[420px] h-screen bg-[rgba(245,245,245,0.75)] p-2 border border-l-gray-300'>
    <Chat />
  </div>
)

export { RightNavigation, RightSidebar }
