"use client";

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-nocheck
import { useState, useRef, useEffect } from "react";
import { Button } from "@/components/ui/button";

interface ChatOverlayProps {
  isOpen: boolean;
  source: string;
  onClose: () => void;
  onSourceChange: (newSource: string) => void;
}

interface Message {
  role: "user" | "assistant";
  content: string;
}

export default function ChatOverlay({
  isOpen,
  source,
  onClose,
  onSourceChange
  }: ChatOverlayProps) {
  const [messages, setMessages] = useState<Message[]>([]);
  const [inputValue, setInputValue] = useState("");

  const handleSelectSource = (event: React.ChangeEvent<HTMLSelectElement>) => {
    onSourceChange(event.target.value);
  };

  const chatContainerRef = useRef<HTMLDivElement>(null);

  //auto scrolling on message send
  useEffect(() => {
    if (chatContainerRef.current) {
      chatContainerRef.current.scrollTop = chatContainerRef.current.scrollHeight;
    }
  }, [messages]);

  async function callChatAPI(userMessage: string) {
    // Send userMessage to your Next.js API route
    const response = await fetch("/api/chat", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ userMessage })
    });

    if (!response.ok) {
      console.error("API error:", await response.text());
      return { text: "Sorry, something went wrong." };
    }

    const data = await response.json();
    return data;
  }

  const handleSend = async () => {
    if (!inputValue.trim()) return;

    // Show user's message immediately
    const userMessage: Message = { role: "user", content: inputValue };
    setMessages((prev) => [...prev, userMessage]);
    setInputValue("");

    // Call server route to get AI response
    const { text } = await callChatAPI(userMessage.content);

    // Add AI response
    const aiMessage: Message = { role: "assistant", content: text };
    setMessages((prev) => [...prev, aiMessage]);
  };

  // Pressing "Enter" triggers handleSend
  const handleKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (event.key === "Enter") {
      event.preventDefault();
      handleSend();
    }
  };


  return (
    <div
      className={`
        fixed top-0 right-0 h-full w-1/3 z-50
        border-l-2 shadow-lg transform transition-transform duration-300
        ${isOpen ? "translate-x-0" : "translate-x-full"}
      `}
    >
      <div className="flex flex-col h-full">
        <div className="flex items-center justify-between p-3 border-b">
          <div className="flex items-center gap-2">
            <label htmlFor="source" className="font-semibold">
              Select Model:
            </label>
            <select
              id="source"
              name="source"
              value={source}
              onChange={handleSelectSource}
              className="border rounded px-2 py-1"
            >
              <option value="GPT">GPT</option>
              <option value="Gemini">Gemini</option>
              <option value="DeepSeek">DeepSeek</option>
              <option value="Claude">Claude</option>
            </select>
          </div>
          <Button variant="outline" onClick={() => onClose()}>
            Close
          </Button>
        </div>

        <div ref={chatContainerRef} className="flex-1 overflow-auto bg-gradient-to-l from-[#fdf7f4] to-[#ffffff] p-4 flex flex-col gap-3">
          {/* ALL CHATBOT UI is here */}
          {messages.length === 0 ? (
            /* Default message when no messages */
            <div className="flex-1 flex flex-col justify-center items-center text-center text-gray-700">
              <h2 className="text-xl font-bold">Chat with your Notes</h2>
              <p className="mt-2">
                Enter a message to start chatting with Notefaratu
              </p>
            </div>
          ) : (
            /* Otherwise, show the chat bubbles */
            messages.map((msg, index) => {
              const isUser = msg.role === "user";
              return (
                <div
                  key={index}
                  className={`
                    max-w-[75%] rounded-md p-3 text-sm
                    ${isUser ? "self-end bg-black text-white" : "self-start bg-gray-100 text-black"}
                  `}
                >
                  {msg.content}
                </div>
              );
            })
          )}
        </div>

        {/* Input bar */}
        <div className="flex items-center border-t p-3">
          <input
            type="text"
            placeholder={`Message ${source}`}
            className="flex-1 border border-gray-300 rounded-md px-3 py-2 text-sm focus:outline-none"
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
            onKeyDown={handleKeyDown}
          />
          <Button onClick={() => callChatAPI(inputValue)} className="ml-2 text-sm">Send</Button>
        </div>
      </div>
    </div>
  );
}
