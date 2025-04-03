import { ToolImplementation } from "@/lib/OpenRouter"
import { ChatCompletionTool } from "openai/resources/index.mjs"
import { FileTextIcon } from "lucide-react"
import { Button } from "@/components/ui/button"
import { SetStateAction, Dispatch } from "react"
import { Message } from "@/lib/OpenRouter"
import { DatabaseContextType } from "../DatabaseProvider"
import UUID from "@/lib/UUID"

// ToolCall is a message containing the results of an LLM tool call.
export type ToolCall = {
  tool    : 'createNote'
  id?     : number
  content : string
}

// CreateNoteToolCard is a message that appears in the chat denoting
// the creation of a new note.
const CreateNoteToolCard = ({ id, preview } :
{ id: number, preview: string }) => (
  <div className="break-words rounded-sm p-3 text-sm bg-[#F6F6F6] border border-[#979797] text-black grid grid-cols-[50px_auto_70px] gap-3 h-[70px] max-w-full">
    <FileTextIcon className='h-full w-full'/>
    <div className="flex flex-col min-w-0">
      <b>Created New Note</b>
      <p className='overflow-hidden text-ellipsis whitespace-nowrap'>
        {preview}
      </p>
    </div>
    <Button onClick={() => window.location.href=`/note?id=${id}`}>
      View
    </Button>
  </div>
)

// LLM tool-calling function definitions.
const toolDefinitions: ChatCompletionTool[] = [
  {
    type: 'function',
    function: {
      name: 'createNote',
      description: 'Create a new note',
      parameters: {
        type: 'object',
        properties: {
          title: { type: 'string' },
          content: { type: 'string' },
        },
        required: ['title', 'content']
      }
    }
  }
]

// LLM tool-calling i
const toolImplementations = (
  db : DatabaseContextType,
  setMessages: Dispatch<SetStateAction<(ToolCall | Message)[]>>
) => {
  return {
    createNote: async (args) => {
      const id = UUID()
      const content = args.content.slice(0,200)
      await db.notes.create({
        id      : id,
        title   : args.title,
        content : args.content,
        atime   : Math.floor(Date.now() / 1000),
        mtime   : Math.floor(Date.now() / 1000),
      })
      await db.history.create({
        id        : id,
        role      : 'tool',
        tool_name : 'createNote',
        content   : content,
        time      : Math.floor(Date.now() / 1000)
      })
      setMessages((prev) => {
        return [...prev, {
          tool    : 'createNote',
          id      : id,
          content : content,
        } as ToolCall]
      })
      return { success: true }
    }
  } as ToolImplementation
}

export { toolDefinitions, toolImplementations, CreateNoteToolCard }
