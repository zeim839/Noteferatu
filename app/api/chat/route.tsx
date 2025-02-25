import { NextResponse } from 'next/server'
import OpenAI from 'openai'

/**
 * Initialize OpenAI (DeepSeek) on the server.
 * Make sure you have DEEPSEEK_API_KEY in your .env
 */
const openai = new OpenAI({
  baseURL: "https://openrouter.ai/api/v1",      // <-- note the new base
  apiKey: process.env.OPENROUTER_API_KEY,       // from your .env
  // dangerouslyAllowBrowser: false (default)
})

const MODEL_MAP: Record<string, string> = {
  GPT: "openai/gpt-3.5-turbo",
  // Gemini: "google/palm-2-chat-bison", // idk real one
  DeepSeek: "deepseek/deepseek-r1:free",
  Claude: "anthropic/claude-instant:free",
};

/**
 * POST /api/chat
 * Expects { userMessage: string } in JSON body.
 * Returns { text: string } in JSON.
 */
// export async function POST(request: Request) {
//   try {
//     const { userMessage } = await request.json();

//     // call DeepSeek with the userMessage
//     const completion = await openai.chat.completions.create({
//       messages: [
//         { role: "system", content: "You are a helpful assistant." },
//         { role: "user", content: userMessage },
//       ],
//       model: "deepseek-chat",
//     });

//     // Extract the text from the AI response
//     const text = completion.choices[0]?.message?.content || "";
//     return NextResponse.json({ text });
//   } catch (err: any) {
//     return NextResponse.json({ error: err.message }, { status: 500 });
//   }
// }

export async function POST(request: Request) {
  try {
    const { userMessage, source } = await request.json()

    const chosenModel = MODEL_MAP[source] || "openai/gpt-3.5-turbo";

    // The model name, per the Medium article: "deepseek/deepseek-r1:free"
    // If you want to pass extra headers for analytics, see below.
    const completion = await openai.chat.completions.create({
      model: chosenModel,
      messages: [
        { role: "system", content: "You are a helpful assistant." },
        { role: "user", content: userMessage }
      ],
      // If you want the "Referer" or "X-Title" that the Medium post mentions:
      // extra_headers: {
      //   "HTTP-Referer": "your-site-url.com",
      //   "X-Title": "YourSiteName"
      // },
    })

    const text = completion.choices[0]?.message?.content || ""
    console.log("helloooooo")
    return NextResponse.json({ text })
  } catch (err: unknown) {
    if (err instanceof Error) {
      return NextResponse.json({ error: err.message }, { status: 500 });
    }
    // fallback if it’s not an Error instance
    return NextResponse.json({ error: String(err) }, { status: 500 });
  }
}