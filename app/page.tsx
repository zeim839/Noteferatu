"use client"

import { useRouter } from "next/navigation"

export default function Home() {
  const router = useRouter()
  return (
    <div className="w-full h-full flex items-center justify-center">
      <div className="flex gap-1 flex-col items-center justify-center"
        onClick={() => router.push("/note") }>
        <div className="rounded-full w-12 h-12 bg-[#4C8EDA]" />
        <p>Click me!</p>
      </div>
    </div>
  )
}
