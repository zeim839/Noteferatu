"use client"

import { useEffect } from "react"
import Graph from "@/components/graph"
import { appLocalDataDir } from '@tauri-apps/api/path'
import NoteController from "@/lib/controller/NoteController"
import path from "path"

export default function Home() {
  useEffect(() => {
    appLocalDataDir().then(async (dir: string) => {
      const notes = new NoteController(path.join(dir, 'db.sqlite'))
      console.log(await notes.readAll())
    })
  }, [])
  return (
    <div className="w-full h-full flex items-center justify-center">
      <Graph />
    </div>
  )
}
