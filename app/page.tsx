"use client"

import { useRouter } from "next/navigation"
import ChatOverlay from "@/components/ui/chat"
import { useState } from "react"

export default function Home() {
  const router = useRouter()
  const [isOpen, setIsOpen] = useState(false);
  const [source, setSource] = useState("GPT");

  const toggleChat = () => {
    setIsOpen(!isOpen);
  }

  const handleSourceChange = (newSource: string) => {
    setSource(newSource);
    };

  return (
    <div className="w-full h-full flex items-center justify-center">
      <div className="flex gap-1 flex-col items-center justify-center"
        onClick={() => router.push("/note") }>
        <div className="rounded-full w-12 h-12 bg-[#4C8EDA]" />
        <p>Click me!</p>
      </div>
      {/* Chat Overlay */}
      <ChatOverlay
        isOpen={isOpen}
        source={source}
        onClose={toggleChat}
        onSourceChange={handleSourceChange}
      />
    </div>
  )
}
