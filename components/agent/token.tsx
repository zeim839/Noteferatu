import { useAgentContext } from "./context"

import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/core/tooltip"

// Converts a token count into a string. If less than 1000, then
// `tokens` is returned, otherwise truncate and add a 'k' prefix.
const tokenStr = (tokens: number) => {
  return tokens < 1000
    ? tokens.toString()
    : (tokens / 1000).toFixed(tokens % 1000 == 0 ? 0 : 1).toString() + "K"
}

// States the number of tokens used vs. the current model's context window.
function TokenInfo() {
  const ctx = useAgentContext()
  const isOverflow = ctx.tokensUsed >= ctx.totalTokens
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <div
          data-is-overflow={isOverflow}
          data-is-hidden={ctx.selectedModel === null}
          className="data-[is-hidden=true]:hidden text-xs inline-flex items-center justify-center rounded-sm bg-[#D4D8E1] h-6 px-2 shadow-xs data-[is-overflow=true]:bg-red-400"
        >
          {tokenStr(ctx.tokensUsed)} / {tokenStr(ctx.totalTokens)}
        </div>
      </TooltipTrigger>
      <TooltipContent side="top">
        <p
          data-is-overflow={isOverflow}
          className="data-[is-overflow=true]:text-red-800"
        >
          {(!isOverflow) ? "Estimated Tokens" : "Context Window Exceeded!"}
        </p>
      </TooltipContent>
    </Tooltip>
  )
}

export { TokenInfo }
