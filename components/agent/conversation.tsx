import * as React from "react"
import { Button } from "@/components/core/button"
import {Edit2Icon, Trash2Icon, CheckIcon, XIcon} from "lucide-react"

// A conversation is a series of user, system, tool and response
// messages between a user and LLM.
export type Conversation = {
  id: String
  name: String
  createdAt: number
}

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

  const onMouseEnter = () => {
    setIsHovered(true)
  }

  const onMouseLeave = () => {
    setIsHovered(false)
    setIsBeingDeleted(false)
    setIsBeingRenamed(false)
  }

  const onDelete = () => {
    // TODO
    setIsBeingDeleted(false)
  }

  const onRename = () => {
    // TODO
    setIsBeingRenamed(false)
  }

  return (
    <div
      onMouseEnter={onMouseEnter}
      onMouseLeave={onMouseLeave}
      className="flex flex-row justify-between w-full p-1.5 px-2 rounded-md hover:bg-[#DCE0E8]"
    >
      <p className="text-xs text-nowrap text-ellipsis overflow-x-hidden overflow-y-hidden">
        {name}
      </p>
      <p
        data-is-hovered={isHovered}
        className="text-xs min-w-15 text-right data-[is-hovered=true]:hidden"
      >
        {timeAgo(createdAt)}
      </p>
      <div
        className="hidden data-[is-hovered=true]:flex gap-2 min-w-15 justify-end"
        data-is-hovered={isHovered}
      >
        { (!isBeingDeleted && !isBeingRenamed) ?
          <Button
            onClick={() => setIsBeingRenamed(true)}
            variant="ghost"
            size="icon"
            className="p-0 m-0 size-4"
          >
            <Edit2Icon strokeWidth={1} className="max-h-3 max-w-3"/>
          </Button> : null
        }
        {
          (!isBeingDeleted && !isBeingRenamed) ?
            <Button
              onClick={() => setIsBeingDeleted(true)}
              variant="ghost"
              size="icon"
              className="p-0 m-0 size-4"
            >
              <Trash2Icon strokeWidth={1} className="max-h-3 max-w-3" />
          </Button> : null
        }
        {
          (isBeingDeleted && !isBeingRenamed) ?
            <Button
              onClick={onDelete}
              variant="destructive"
              size="icon"
              className="p-0 m-0 size-4"
            >
              <Trash2Icon strokeWidth={2} className="max-h-3 max-w-3" />
          </Button> : null
        }
        {
          (!isBeingDeleted && isBeingRenamed) ?
            <>
              <Button
                onClick={onRename}
                variant="confirmation"
                size="icon"
                className="p-0 m-0 size-4"
              >
                <CheckIcon strokeWidth={2.5} className="max-h-3 max-w-3" />
              </Button>
              <Button
                onClick={() => setIsBeingRenamed(false)}
                variant="destructive"
                size="icon"
                className="p-0 m-0 size-4"
              >
                <XIcon strokeWidth={2.5} className="max-h-3 max-w-3" />
              </Button>
            </> : null
        }
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
        body.map((item, index) =>
          <Entry
            key={index}
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
