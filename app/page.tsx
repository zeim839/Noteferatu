"use client"

import { useEffect } from "react"
import { useRouter } from "next/navigation"
import Database from "@/lib/Database"

export default function Home() {
  const router = useRouter()
  useEffect(() => {
    const db = new Database('db.sqlite')
    db.connect()
  }, [])
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
