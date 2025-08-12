import * as React from "react"
import { Button } from "../core/button"
import { ChevronRightIcon, RotateCcwIcon, CopyIcon } from "lucide-react"
import { cn } from "@/lib/utils"

import {
  Error,
  ClientError,
  AnthropicError,
  GoogleError,
  OpenAIError,
  OpenRouterError,
} from "@/lib/agent"

interface ErrorCardProps extends React.ComponentProps<"div"> {
  error: Error
  onRetry: () => void
}

// Display information related to an agent/conversation error.
function ErrorCard({ error, onRetry, className, ...props }: ErrorCardProps) {
  const [isExpanded, setIsExpanded] = React.useState<boolean>(false)
  const { title, description } = parseError(error)

  // Toggles the error description.
  const toggleExpanded = () => {
    setIsExpanded(!isExpanded)
  }

  // Copies the error to the user's clipboard.
  const onCopy = (e: React.MouseEvent) => {
    e.preventDefault()
    e.stopPropagation()
    if (typeof navigator === "undefined") {
      return
    }
    navigator.clipboard.writeText(`${title}\n${description}`);
  }

  // Calls the `onRetry` callback after preventing onClick propagation.
  const onRetryWrapper = (e: React.MouseEvent) => {
    e.preventDefault()
    e.stopPropagation()
    onRetry()
  }

  return (
    <div onClick={toggleExpanded} className={cn("group/error-card p-2 rounded-sm shadow-sm bg-red-100 m-2 outline outline-red-300 text-sm text-red-900", className)} {...props}>
      <div className="flex justify-between gap-1.5">
        <div className="flex items-center gap-2 group-hover/error-card:underline min-w-0">
          <ChevronRightIcon
            data-is-expanded={isExpanded}
            strokeWidth={2}
            className="size-4 min-w-4 min-h-4 transition-all data-[is-expanded=true]:rotate-90"
          />
          <p className="font-bold text-red-900 truncate">{title}</p>
        </div>
        <div className="flex items-center">
          <Button
            variant="outline"
            size="icon"
            tooltip="Retry Request"
            className="text-red-900 hover:bg-red-200"
            onClick={onRetryWrapper}
          >
            <RotateCcwIcon strokeWidth={2} className="size-3" />
          </Button>
          <Button
            variant="outline"
            size="icon"
            tooltip="Copy"
            className="text-red-900 hover:bg-red-200"
            onClick={onCopy}
          >
            <CopyIcon strokeWidth={2} className="size-3" />
          </Button>
        </div>
      </div>

      { /* Expandable Error Description */ }
      {(isExpanded) ?
        <div className="mt-2 wrap-break-word">
          <p>{title}</p>
          <br/>
          <p>{description}</p>
        </div> : null
      }

    </div>
  )
}

// Retrieves a user-friendly title and description from an agent error.
function parseError(error: Error) {
  let title = "Unknown error"
  let description = "Error sending message"
  if (error.type === 'client') {
    const data = error.data as ClientError
    if (data.message === "request timed out" || data.message === "connection error") {
      title = "Connection Failed"
      description = "Please check your internet connection before trying again. If the issue persists, the LLM server could be temporarily unavailable."
    } else if (data.message === "received error status response") {
      title = "Failed to Send"
      description = "Message was rejected by the server, please try again later. If the issue persists, the server could be temporarily unavailable"
    } else if (data.message === "redirect policy error") {
      title = "Redirect Policy Error"
      description = "Server responded with an unexpected redirect. Please try again later or try using a different provider"
    } else if (data.message === "bad request") {
      title = "Bad Request"
      description = "NoteFeratu encountered an error while sending your message. Please try again later or try using a different provider"
    } else {
      title = "Failed to Send"
      description = "NoteFeratu encountered an error while decoding the server response. Please try again later or try using a different provider."
    }
  }

  if (error.type === 'anthropic') {
    const data = error.data as AnthropicError
    title = data.type
    description = data.message
  }

  if (error.type === 'google') {
    const data = error.data as GoogleError
    title = data.status
    description = data.message
  }

  if (error.type === 'ollama') {
    title = "Ollama Error"
    description = error.data as string
  }

  if (error.type === 'openAI') {
    const data = error.data as OpenAIError
    title = data.type
    description = data.message
  }

  if (error.type === 'openRouter') {
    const data = error.data as OpenRouterError
    title = data.message
    description = typeof data.metadata !== "undefined" ?
      JSON.stringify(data.metadata) : "Failed to send message"
  }

  if (error.type === 'json') {
    title = "Decoding Error"
    description = "NoteFeratu had trouble decoding the server's response. Please try again later or try using a different provider"
  }

  if (error.type === 'invalidModelId') {
    title = "Invalid Model"
    description = "The selected model is no longer available. Try selecting a different model or try using a different provider"
  }

  if (error.type === 'providerNotConfigured') {
    title = "Provider not configured"
    description = "The selected model is no longer available because its provider has not been properly configured. Please update your agent settings or try using a different model."
  }

  if (error.type === 'sql') {
    title = "Database Error"
    description = `NoteFeratu encountered the following error while trying to write a message to the database: ${error.data as string}`
  }

  return { title, description }
}

export { ErrorCard }
