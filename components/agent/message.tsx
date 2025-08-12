import * as React from "react"
import { Message as MessageData } from "@/lib/agent"

interface MessageProps extends React.ComponentProps<"div"> {
  data: MessageData;
  index: number;
  onSave: (index: number, content: string) => void;
}

function Message({ data, index, onSave }: MessageProps) {
  const [isEditing, setIsEditing] = React.useState(false)
  const [editedContent, setEditedContent] = React.useState(data.content)
  const textAreaRef = React.useRef<HTMLTextAreaElement>(null)

  React.useEffect(() => {
    if (isEditing && textAreaRef.current) {
      const textarea = textAreaRef.current
      textarea.focus()
      textarea.style.height = "auto"
      textarea.style.height = `${textarea.scrollHeight}px`
    }
  }, [isEditing])

  const handleSave = () => {
    if (editedContent.trim() && editedContent !== data.content) {
      onSave(index, editedContent)
    }
    setIsEditing(false)
  }

  const handleCancel = () => {
    setEditedContent(data.content)
    setIsEditing(false)
  }

  const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setEditedContent(e.target.value)
    e.target.style.height = "auto"
    e.target.style.height = `${e.target.scrollHeight}px`
  }

  // handleSave is called on blur, so we only need to handle the enter key
  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault()
      handleSave()
    } else if (e.key === "Escape") {
      e.preventDefault()
      handleCancel()
    }
  }

  if (data.role === "user") {
    return (
      <div className="flex my-6 mx-2 text-sm">
        <div className={`p-2 w-full border rounded-sm shadow-xs select-text bg-[#EDF0F4] ${isEditing ? "border-blue-500" : "border-[#AEB3C0]"}`}>
          {isEditing ? (
            <div>
              <textarea
                ref={textAreaRef}
                value={editedContent}
                onChange={handleChange}
                onKeyDown={handleKeyDown}
                onBlur={handleSave}
                className="w-full p-0 m-0 text-sm resize-none bg-inherit focus:outline-none overflow-y-hidden"
              />
              <div className="text-right text-xs text-gray-500 opacity-75">
                press enter to save ⏎
              </div>
            </div>
          ) : (
            <div onClick={() => setIsEditing(true)} className="w-full h-full cursor-text">
              <p className="whitespace-pre-wrap">{data.content}</p>
            </div>
          )}
        </div>
      </div>
    )
  }

  if (data.role === "assistant") {
    return (
      <div className="flex justify-start my-4 mx-2 text-sm">
        <div className="whitespace-pre-wrap cursor-text select-text">{data.content}</div>
      </div>
    )
  }

  return null
}

export { Message }
