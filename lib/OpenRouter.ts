import OpenAI from 'openai'

// Message is an OpenRouter message history record. User indicates
// messages sent by users, assistant are the LLM responses.
export type Message = {
  role    : 'user' | 'assistant',
  content : string,
}

// StreamFn is a callback function that consumes tokens from a chat
// completion's stream.
export type StreamFn = (chunk: string, index: number) => void

// systemPrompt is the first message prepended to a chat completions
// request.
const systemPrompt = "You are NoteFeratu, a note-taking app with a built-in helpful AI assistant"

// Chat sends a chat completions API request and returns response.
export const Chat = async (client: OpenAI, messages: Message[],
  model?: string) => {
    const completion = await client.chat.completions.create({
      model: model || 'deepseek/deepseek-r1:free',
      messages: [
        { role: 'system', content: systemPrompt },
        ...messages
      ],
    })
    return completion.choices[0]?.message?.content || ""
  }

// Stream sends a chat completions API request and streams the token
// responses. cb consumes the tokens as they arrive.
export const Stream = async (client: OpenAI, messages: Message[],
  model?: string, cb?: StreamFn) => {
    const stream = await client.chat.completions.create({
      model: model || 'deepseek/deepseek-r1:free',
      messages: [
        { role: 'system', content: systemPrompt },
        ...messages
      ],
      stream: true
    })
    if (typeof cb === 'undefined') {
      return stream
    }
    let i : number = 0
    for await (const chunk of stream) {
      cb(chunk.choices[0]?.delta?.content || '', i)
      i += 1
    }
    return stream
  }
