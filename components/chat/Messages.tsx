import { Message } from "@/lib/OpenRouter"
import { ToolCall } from "./tools"

// MessageField is a single message bubble. Its style depends on whether
// the message is from user or system (LLM response).
export const MessageField = (data: Message | ToolCall, index: number) => {
  // Skip rendering any tool calls
  if ('tool' in data) {
    return null;
  }
  
  const isUser = data.role === "user"
  return (
    <div
      key={index}
      className={`max-w-[75%] break-words rounded-sm p-3 text-sm ${isUser ? "self-end bg-black text-white" : "self-start bg-[#F6F6F6] border border-[#979797] text-black"}`}
    >
      {data.content}
    </div>
  )
}

type BottomRef = React.RefObject<HTMLDivElement | null>

// MessageView contains all LLM chat messages.
export const MessageView = (
  messages: (Message | ToolCall)[],
  isTyping: boolean,
  ref: BottomRef
) => {
  // Filter out all tool calls
  const displayMessages = messages.filter(msg => !('tool' in msg));
  
  // Show a placeholder message whenever there are no messages.
  if (displayMessages.length === 0) {
    return (
      <div className="flex flex-col justify-center items-center text-center text-gray-700">
        <h2 className="text-xl font-bold">Chat with your Notes</h2>
        <p className="mt-2">
          Enter a message to start chatting with Noteferatu
        </p>
      </div>
    )
  }
  
  return (
    <div className="flex-1 overflow-auto flex flex-col gap-3"
      style={{maxHeight: 'calc(100vh - 120px)'}}>
      { /* Render each message as a MessageField object. */ }
      { displayMessages.map(MessageField) }
      { /* Show a grayed-out 'typing' placeholder when loading */ }
      {isTyping && (
        <div className="self-start border border-[#979797] bg-[#F6F6F6] text-black rounded-md p-3 text-sm">
          <div>
            <span className="animate-pulse">Typing</span>
            <span className="animate-pulse">...</span>
          </div>
        </div>
      )}
      <div ref={ref} />
    </div>
  )
}