import { Message } from "@/lib/OpenRouter"

// MessageField is a single message bubble. Its style depends on whether
// the message is from user or system (LLM response).
export const MessageField = (message: Message, index: number) => {
  const isUser = message.role === "user"
  return (
    <div
      key={index}
      className={`max-w-[75%] break-words rounded-sm p-3 text-sm ${isUser ? "self-end bg-black text-white" : "self-start bg-[#F6F6F6] border border-[#979797] text-black"}`}
    >
      {message.content}
    </div>
  )
}

type BottomRef = React.RefObject<HTMLDivElement | null>

// MessageView contains all LLM chat messages.
export const MessageView = (messages: Message[], isTyping: boolean, ref: BottomRef) => {

  // Show a placeholder message whenever there are no messages.
  if (messages.length === 0) {
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
      {messages.map(MessageField)}

      { /* Show a grayed-out 'typing' placeholder when loading */ }
      {isTyping && (
        <div className="self-start border border-[#979797] bg-[#F6F6F6] text-black rounded-md p-3 text-sm">
          <div className="">
            <span className="animate-pulse">Typing</span>
            <span className="animate-pulse">...</span>
          </div>
        </div>
      )}
      <div ref={ref} />
    </div>
  )
}
