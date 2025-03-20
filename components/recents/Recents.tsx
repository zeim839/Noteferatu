import RecentsCard from "./RecentsCard"
import { useState, useEffect, useMemo, useLayoutEffect } from "react"
import { Button } from "@/components/ui/button"
import { PlusIcon } from "lucide-react"
import { useRouter } from "next/navigation"
import { Note } from "@/lib/controller/NoteController"
import { useDB } from "@/components/DatabaseProvider"
import { toast } from "sonner"

// useWindowSize tracks the window dimensions. It is used to calculate
// the number of RecentCards to show in the recents sidebar.
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

// Recents displays the most recently accessed notes. It only displays
// enough notes to fill the window height.
export default function Recents() {
  const [cardCount, setCardCount] = useState<number>(0)
  const [recentsData, setRecentsData] = useState<Note[] | null>([])
  const [, height] = useWindowSize()
  const [isLoading, setIsLoading] = useState(true)

  const db = useDB()
  const router = useRouter()

  // Height of a RecentsCard container.
  const divHeight = 77

  useMemo(() => {
    setCardCount(Math.round((height - 60)/(divHeight+8)))
  }, [height])

  // fetchData reads all notes.
  const fetchData = async () => {
    try { setRecentsData(await db.notes.readAll()) }
    catch (error) {
      let description = 'An unknown error has occurred'
      if (error instanceof Error) {
        description = error.message
      }
      toast('Error: Could Not Fetch Recent Notes', { description })
      setRecentsData(null)
    } finally { setIsLoading(false) }
  }

  useEffect(() => {
    if (!db) return
    fetchData()
  }, [cardCount, db])

  // Stall when notes are still loading.
  if (isLoading) {
    return null
  }

  // When notes are loaded/available, render them to the DOM.
  if (recentsData && recentsData.length > 0) {
    const recentsCardsList = recentsData.slice(0, cardCount).map((note, i) => (
      <div key={i}
        onClick={() => router.push(`/note?id=${note.id}`) }
        className="opacity-0 animate-fade-in"
        style={{ animationDelay: `${i * 0.06}s` }}>
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

  // Show an error message if a non-empty array is returned.
  if (recentsData === null) {
    return (
      <div className="flex h-full items-center justify-center">
        <p className="text-xl font-bold text-red-700">
          Unable to connect to Database
        </p>
      </div>
    )
  }

  // Show a placeholder asking the user to create a new note when there
  // are no notes available.
  return (
    <div className="h-full flex flex-col justify-center items-center text-center text-gray-700">
      <div className='z-40'>
        <p className='mb-2'>Create a note to get started</p>
        <Button onClick={() => router.push('/note')} className='w-full'>
          Create
          <PlusIcon />
        </Button>
      </div>
    </div>
  )
}
