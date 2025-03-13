import RecentsCard from "./RecentsCard"
import React, { useState, useEffect, useMemo, useLayoutEffect} from "react"
import { appLocalDataDir } from '@tauri-apps/api/path'
import { Button } from "@/components/ui/button"
import { PlusIcon } from "lucide-react"
import { useRouter } from "next/navigation"
import NoteController from "@/lib/controller/NoteController"
import path from "path"

type NoteData = {
  id      : number
  title   : string
  content : string
  atime   : number
}

async function getRecents(count: number): Promise<NoteData[] | null> {
  const appDataDir = await appLocalDataDir()
  const controller = new NoteController(path.join(appDataDir, 'db.sqlite'))
  const notes = await controller.getRecents(count)
  return notes as NoteData[]
}

function useWindowSize() {
  const [size, setSize] = useState([0, 0])
  useLayoutEffect(() => {
    function updateSize() {
      setSize([window.innerWidth, window.innerHeight])
    }
    window.addEventListener('resize', updateSize)
    updateSize()
    return () => window.removeEventListener('resize', updateSize)
  }, [])
  return size
}

export default function Recents() {
  const [cardCount, setCardCount] = useState<number>(0)
  const [recentsData, setRecentsData] = useState<NoteData[] | null>([])
  const [, height] = useWindowSize()
  const [isLoading, setIsLoading] = useState(true)
  const router = useRouter()
  const divHeight = 77

  useMemo(() => {
    setCardCount(Math.round((height - 60)/(divHeight+8)))
  }, [height])

  useEffect(() => {
    async function fetchData() {
      try {
        const data = await getRecents(cardCount)
        setRecentsData(data)
      } catch {
        setRecentsData(null)
      } finally {
        setIsLoading(false)
      }
    }
    fetchData()
  }, [cardCount])

  if (isLoading) {
    return null
  }

  if (recentsData && recentsData.length > 0) {
    const recentsCardsList = recentsData.slice(0, cardCount).map((note, i) => (
      <div key={i} className="opacity-0 animate-fade-in" style={{ animationDelay: `${i * 0.06}s` }}>
        <RecentsCard
          title={note.title}
          desc={note.content}
          atime={note.atime}
        />
      </div>
    ))
    return (
      <div className="pt-12 h-full">
        {recentsCardsList}
      </div>
    )
  }

  if (recentsData === null) {
    return (
      <div className="flex h-full items-center justify-center">
        <p className="text-xl font-bold text-red-700">
          Unable to connect to Database
        </p>
      </div>
    )
  }

  return (
    <div className="h-full flex flex-col justify-center items-center text-center text-gray-700">
      <div>
        <p className='mb-2'>Create a note to get started</p>
        <Button onClick={() => router.push('/note')} className='w-full'>
          Create
          <PlusIcon />
        </Button>
      </div>
    </div>
  )
}
