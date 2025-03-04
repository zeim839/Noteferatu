"use client"

import { useEffect } from "react"
import Database from "@/lib/Database"
import Graph from "@/components/graph"

export default function Home() {
  useEffect(() => {
    const db = new Database('db.sqlite')
    db.connect()
  }, [])
  return (
    <div className="w-full h-full flex items-center justify-center">
      <Graph width={500} height={500} />
    </div>
  )
}
