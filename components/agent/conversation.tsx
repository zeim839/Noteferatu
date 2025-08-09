import * as React from "react"
import { Conversation, removeConversation, renameConversation } from "@/lib/agent"
import { useAgentContext } from "./context"
import { Button } from "@/components/core/button"
import { Edit2Icon, Trash2Icon, CheckIcon, XIcon } from "lucide-react"

// Convert a timestamp into a "[X] [units] ago" string, where "[units]"
// is the largest time denomination (seconds, minutes, hours, days,
// months, years) such that "[X]" is a positive integer.
const timeAgo = (timestamp: number): string => {
  const currentTimeStampSeconds = Math.floor(Date.now()/1000)
  const diffInSeconds = Math.floor((currentTimeStampSeconds - timestamp))
  if (diffInSeconds < 60) return `${diffInSeconds}s ago`
  const diffInMinutes = Math.floor(diffInSeconds / 60)
  if (diffInMinutes < 60) return `${diffInMinutes}m ago`
  const diffInHours = Math.floor(diffInMinutes / 60)
  if (diffInHours < 24) return `${diffInHours}h ago`
  const diffInDays = Math.floor(diffInHours / 24)
  if (diffInDays < 30) return `${diffInDays}d ago`
  const diffInMonths = Math.floor(diffInDays / 30)
  if (diffInMonths < 12) return `${diffInMonths}mo ago`
  const diffInYears = Math.floor(diffInMonths / 12)
  return `${diffInYears}y ago`
}

// An entry displays a conversation's name and allows a user to select,
// rename, or delete it. It appears in the conversation history panel.
const Entry = ({ id, name, createdAt } : Conversation) => {
  const [isHovered, setIsHovered] = React.useState<boolean>(false)
  const [isBeingDeleted, setIsBeingDeleted] = React.useState<boolean>(false)
  const [isBeingRenamed, setIsBeingRenamed] = React.useState<boolean>(false)
  const [newName, setNewName] = React.useState(name)
  const inputRef = React.useRef<HTMLInputElement>(null)
  const { selectedConv, setSelectedConv, toggleConvs } = useAgentContext()

  React.useEffect(() => {
    if (isBeingRenamed) {
      inputRef.current?.focus()
      inputRef.current?.select()
    }
  }, [isBeingRenamed])

  // Hover over a conversation entry.
  const onMouseEnter = () => {
    setIsHovered(true)
  }

  // Unhover over the conversation entry.
  const onMouseLeave = () => {
    setIsHovered(false)
    setIsBeingDeleted(false)
  }

  // Delete a conversation and its message history.
  const handleDelete = (e: React.MouseEvent) => {
    e.stopPropagation()
    removeConversation(id)
    if (selectedConv()?.id === id) {
      setSelectedConv(null)
    }
    setIsBeingDeleted(false)
  }

  const commitRename = () => {
    if (newName.trim() && newName.trim() !== name) {
      renameConversation(id, newName.trim())
    }
    setIsBeingRenamed(false)
  }

  const revertRename = () => {
    setNewName(name)
    setIsBeingRenamed(false)
  }

  const handleRenameClick = (e: React.MouseEvent) => {
    e.stopPropagation()
    commitRename()
  }

  const handleCancelRenameClick = (e: React.MouseEvent) => {
    e.stopPropagation()
    revertRename()
  }

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      commitRename()
    } else if (e.key === 'Escape') {
      revertRename()
    }
  }

  // Prevents the input field from losing focus (and triggering onBlur)
  // when the user clicks a button.
  const handleMouseDownOnButton = (e: React.MouseEvent) => {
    e.preventDefault()
  }

  // Clicking on a conversation entry sets the selected conversation.
  const handleSelect = () => {
    if (isBeingRenamed) return
    setSelectedConv({ id, name, createdAt })
    toggleConvs()
  }

  const showActions = isHovered || isBeingDeleted || isBeingRenamed

  return (
    <div
      key={id}
      onMouseEnter={onMouseEnter}
      onMouseLeave={onMouseLeave}
      onClick={handleSelect}
      className="flex flex-row justify-between items-center w-full p-1.5 px-2 rounded-md hover:bg-[#DCE0E8] min-w-[250px] cursor-pointer"
    >
      <div className="flex-grow overflow-hidden mr-2">
        {isBeingRenamed ? (
          <input
            ref={inputRef}
            type="text"
            value={newName}
            onChange={(e) => setNewName(e.target.value)}
            onBlur={commitRename}
            onKeyDown={handleKeyDown}
            onClick={(e) => e.stopPropagation()} // Prevent selection when clicking input
            className="text-sm bg-transparent outline-none w-full p-0 m-0 cursor-text"
          />
        ) : (
          <p className="text-sm text-nowrap text-ellipsis overflow-hidden">
            {name}
          </p>
        )}
      </div>

      <div className="flex-shrink-0">
        <p
          data-is-visible={!showActions}
          className="text-xs min-w-15 text-right data-[is-visible=false]:hidden"
        >
          {timeAgo(createdAt)}
        </p>
        <div
          className="hidden data-[is-visible=true]:flex gap-2 min-w-15 justify-end"
          data-is-visible={showActions}
        >
          {(!isBeingDeleted && !isBeingRenamed) ? (
            <>
              <Button
                onClick={(e) => { e.stopPropagation(); setIsBeingRenamed(true); }}
                variant="ghost"
                size="icon"
                className="p-0 m-0 size-4"
                tooltip="Rename"
              >
                <Edit2Icon strokeWidth={1} className="max-h-3 max-w-3"/>
              </Button>
              <Button
                onClick={(e) => { e.stopPropagation(); setIsBeingDeleted(true); }}
                variant="ghost"
                size="icon"
                className="p-0 m-0 size-4"
                tooltip="Delete"
              >
                <Trash2Icon strokeWidth={1} className="max-h-3 max-w-3" />
              </Button>
            </>
          ) : null}

          {(isBeingDeleted && !isBeingRenamed) ? (
            <Button
              onClick={handleDelete}
              variant="destructive"
              size="icon"
              className="p-0 m-0 size-4 border-transparent shadow-none bg-none hover:bg-transparent hover:text-red-600 hover:border-none"
              tooltip="Confirm Delete"
            >
              <Trash2Icon strokeWidth={2} className="max-h-3 max-w-3" />
            </Button>
          ) : null}

          {isBeingRenamed ? (
            <>
              <Button
                onMouseDown={handleMouseDownOnButton}
                onClick={handleRenameClick}
                variant="confirmation"
                size="icon"
                className="p-0 m-0 size-4"
                tooltip="Confirm"
              >
                <CheckIcon strokeWidth={2.5} className="max-h-3 max-w-3" />
              </Button>
              <Button
                onMouseDown={handleMouseDownOnButton}
                onClick={handleCancelRenameClick}
                variant="destructive"
                size="icon"
                className="p-0 m-0 size-4 border-transparent shadow-none bg-none hover:bg-transparent hover:text-red-600 hover:border-none"
                tooltip="Cancel"
              >
                <XIcon strokeWidth={2.5} className="max-h-3 max-w-3" />
              </Button>
            </>
          ) : null}
        </div>
      </div>
    </div>
  )
}

// Body organizes the contents of the agent panel's conversation history.
const Body = ({ body }: { body: Conversation[] }) => {
  if (body.length == 0) {
    return (
      <div
        className="h-full w-full flex justify-center items-center pb-16" >
        <p className="text-xs">No conversations yet</p>
      </div>
    )
  }
  return (
    <div className="mt-1">
      {
        body.map((item) =>
          <Entry
            key={item.id}
            id={item.id}
            name={item.name}
            createdAt={item.createdAt}
          />
        )
      }
    </div>
  )
}

export { Body, Entry }
