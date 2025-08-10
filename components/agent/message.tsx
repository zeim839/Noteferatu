import * as React from "react"
import { Message as MessageData } from "@/lib/agent"

interface MessageProps extends React.ComponentProps<"div"> {
  data: MessageData
}

function Message({ data }: MessageProps) {
  if (data.role === "user") {
    return (
      <div className="flex justify-end my-6 mx-2 text-sm">
        <div className="p-2 border border-[#ABB0BE] bg-[#E5E9EF] rounded-sm shadow-sm select-text cursor-text">
          <p className="whitespace-pre-wrap">{data.content}</p>
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
