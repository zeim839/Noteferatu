import OpenAI from 'openai'
import { ChatCompletionTool } from 'openai/resources/index.mjs'

// Message is an OpenRouter message history record. User indicates
// messages sent by users, assistant are the LLM responses.
export type Message = {
  role    : 'user' | 'assistant',
  content : string,
}

// StreamFn is a callback function that consumes tokens from a chat
// completion's stream.
export type StreamFn = (chunk: string, index: number) => void

// ToolImplementation defines the structure for tool functions
// provided by the user.
export type ToolImplementation = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  [key: string]: (args: Record<string, any>) => Promise<any>
}

// systemPrompt is the first message prepended to a chat completions
// request.
const systemPrompt = "You are NoteFeratu, a note-taking app with a built-in helpful AI assistant"

// Chat sends a chat completions API request and returns response.
export const Chat = async (
  client   : OpenAI,
  messages : Message[],
  model?   : string,
  tools?   : ChatCompletionTool[],
  impl?    : ToolImplementation
) => {
  const completion = await client.chat.completions.create({
    model: model || 'google/gemini-2.5-pro-exp-03-25:free',
    messages: [
      { role: 'system', content: systemPrompt },
      ...messages
    ],
    tools: tools,
  })
  const message = completion.choices[0].message
  if (message.tool_calls && message.tool_calls.length > 0 && impl) {
    for (const toolCall of message.tool_calls) {
      const fnName = toolCall.function.name
      const args = JSON.parse(toolCall.function.arguments)
      if (impl[fnName]) {
        await impl[fnName](args)
      }
    }
  }
  return completion.choices[0]?.message?.content || ""
}

// Stream sends a chat completions API request and streams the token
// responses. cb consumes the tokens as they arrive.
export const Stream = async (
  client   : OpenAI,
  messages : Message[],
  model?   : string,
  cb?      : StreamFn,
  tools?   : ChatCompletionTool[],
  impl?    : ToolImplementation
) => {
  const stream = await client.chat.completions.create({
    model: model || 'google/gemini-2.5-pro-exp-03-25:free',
    messages: [
      { role: 'system', content: systemPrompt },
      ...messages
    ],
    stream: true,
    tools: tools,
  })
  if (typeof cb === 'undefined') {
    return stream
  }
  let i : number = 0
  const toolCalls: {id?: string, function: { name?: string; arguments: string }}[] = []
  let finishReason: string | null = null
  for await (const chunk of stream) {
    const delta = chunk.choices[0].delta
    if (delta.content) {
      cb(delta.content, i)
    }
    if (delta.tool_calls) {
      delta.tool_calls.forEach((toolCallDelta) => {
        const index = toolCallDelta.index
        if (!toolCalls[index]) {
          toolCalls[index] = { function: { arguments: '' }}
        }
        const toolCall = toolCalls[index]
        if (toolCallDelta.id) {
          toolCall.id = toolCallDelta.id
        }
        if (toolCallDelta.function) {
          if (toolCallDelta.function.name) {
            toolCall.function.name = toolCallDelta.function.name
          }
          if (toolCallDelta.function.arguments) {
            toolCall.function.arguments += toolCallDelta.function.arguments
          }
        }
      })
    }
    if (chunk.choices[0].finish_reason) {
      finishReason = chunk.choices[0].finish_reason
    }
    i += 1
  }
  if (finishReason === "tool_calls" && impl) {
    for (const toolCall of toolCalls) {
      if (toolCall.function.name) {
        const fnName = toolCall.function.name
        const args = JSON.parse(toolCall.function.arguments)
        if (impl[fnName]) {
          await impl[fnName](args)
        }
      }
    }
  }
  return stream
}
