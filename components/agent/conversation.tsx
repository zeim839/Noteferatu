import * as React from "react"
import { Button } from "@/components/core/button"
import { Conversation, removeConversation, renameConversation } from "@/lib/agent"
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

  React.useEffect(() => {
    if (isBeingRenamed) {
      inputRef.current?.focus()
      inputRef.current?.select()
    }
  }, [isBeingRenamed])

  const onMouseEnter = () => {
    setIsHovered(true)
  }

  const onMouseLeave = () => {
    setIsHovered(false)
    setIsBeingDeleted(false)
  }

  const handleDelete = () => {
    removeConversation(id)
    setIsBeingDeleted(false)
  }

  const handleRename = () => {
    if (newName.trim() && newName.trim() !== name) {
      renameConversation(id, newName.trim())
    }
    setIsBeingRenamed(false)
  }

  const handleCancelRename = () => {
    setNewName(name)
    setIsBeingRenamed(false)
  }

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      handleRename()
    } else if (e.key === 'Escape') {
      handleCancelRename()
    }
  }

  const showActions = isHovered || isBeingDeleted || isBeingRenamed

  return (
    <div
      key={id}
      onMouseEnter={onMouseEnter}
      onMouseLeave={onMouseLeave}
      className="flex flex-row justify-between items-center w-full p-1.5 px-2 rounded-md hover:bg-[#DCE0E8] min-w-[250px]"
    >
      <div className="flex-grow overflow-hidden mr-2">
        {isBeingRenamed ? (
          <input
            ref={inputRef}
            type="text"
            value={newName}
            onChange={(e) => setNewName(e.target.value)}
            onBlur={handleRename}
            onKeyDown={handleKeyDown}
            className="text-sm bg-transparent outline-none w-full p-0 m-0"
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
                onClick={() => setIsBeingRenamed(true)}
                variant="ghost"
                size="icon"
                className="p-0 m-0 size-4"
                tooltip="Rename"
              >
                <Edit2Icon strokeWidth={1} className="max-h-3 max-w-3"/>
              </Button>
              <Button
                onClick={() => setIsBeingDeleted(true)}
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
              tooltip="Confirm"
            >
              <Trash2Icon strokeWidth={2} className="max-h-3 max-w-3" />
            </Button>
          ) : null}

          {isBeingRenamed ? (
            <>
              <Button
                onClick={handleRename}
                variant="confirmation"
                size="icon"
                className="p-0 m-0 size-4"
                tooltip="Confirm"
              >
                <CheckIcon strokeWidth={2.5} className="max-h-3 max-w-3" />
              </Button>
              <Button
                onClick={handleCancelRename}
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
