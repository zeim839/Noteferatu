import { useEffect, useRef, useState } from 'react'
import { FocusIcon, PlusIcon, MinusIcon } from "lucide-react"
import cytoscape, { ElementDefinition } from 'cytoscape'
import { useRouter } from "next/navigation"
import { Note } from '@/lib/controller/NoteController'
import { Edge } from '@/lib/controller/EdgeController'
import { Button } from "@/components/ui/button"
import { useDB } from '@/components/DatabaseProvider'
import { toast } from "sonner"

// CreateNoteCard asks a user to create a new note. It is shown in place
// of the graph whenever there are no notes available.
const CreateNoteCard = () => {
  const router = useRouter()
  return (
    <div className='w-full h-full flex items-center justify-center pb-8'>
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

// GraphView renders notes in a graph, with hyperlinks between graphs
// serving as edges.
export default function GraphView() {
  const [notes, setNotes] = useState<Note[]>([])
  const [edges, setEdges] = useState<Edge[]>([])
  const cyInstanceRef = useRef<cytoscape.Core | null>(null)
  const cyContainerRef = useRef(null)
  const router = useRouter()
  const db = useDB()

  // Fetch notes and edges from the database controllers.
  const fetchData = async () => {
    if (!db) return
    try {
      setNotes(await db.notes.readAll())
      setEdges(await db.edges.readAll())
    } catch (error) {
      let description = 'an unknown database error has occurred'
      if (error instanceof Error) {
        description = error.message
      }
      toast("Error: Could Not Fetch Notes", {description})
    }
  }

  useEffect(() => { fetchData() }, [db])

  // graphElements transforms nodes and edges into cytoscape elements.
  const graphElements = () => {
    const elements : object[] = notes.map(note => (
      { data: { id: note.id?.toString(), title: note.title } }
    ))
    return elements.concat(edges.map(edge => (
      { data: { source: edge.src.toString(),
        target: edge.dst.toString() } }
    ))) as ElementDefinition[]
  }

  // Render the graph view.
  useEffect(() => {
    if (!cyContainerRef.current || notes.length == 0) return

    // The grid layout makes the graph look more organized
    // when there are no edges between nodes.
    const layout = (edges.length > 0) ? {
      name: 'cose',
      randomize: true,
      animate: false,
      padding: 20,
    } : {
      name: 'grid',
      animate: false,
      avoidOverlap: true,
      avoidOverlapPadding: 50,
      padding: 50,
      condense: true,
    }

    // Create the cytoscape graph object.
    const cy = cytoscape({
      container: cyContainerRef.current,
      elements: graphElements(),
      style: [
        {
          selector: 'node',
          style: {
            'background-color': '#0074D9',
            'text-valign': 'bottom',
            'text-halign': 'center',
            'text-margin-y': 5,
            'font-family': 'Iosevka Comfy, monospace',
            'font-weight': 'lighter',
            'text-wrap': 'wrap',
            'text-max-width': '80px',
            'font-size': 5,
            label: 'data(title)',
            width: 10,
            height: 10,
          },
        },
        {
          selector: 'edge',
          style: {
            'line-color': '#979797',
            'curve-style': 'bezier',
            width: 0.5,
          },
        },
      ],
      layout
    })

    // Redirect to the note page when user clicks/taps on a node.
    cy.on('tap', 'node', (event) => {
      const node = event.target
      router.push(`/note?id=${node.id()}`)
    })

    cyInstanceRef.current = cy

    return () => { cy.destroy() }

  }, [notes, router, edges])

  // Zoom in: multiply the current zoom level by 1.2.
  const zoomIn = () => {
    if (!cyInstanceRef.current) return
    const currentZoom = cyInstanceRef.current.zoom()
    cyInstanceRef.current.zoom({
      level: currentZoom * 1.2,
      renderedPosition: {
        x: cyInstanceRef.current.width() / 2,
        y: cyInstanceRef.current.height() / 2,
      },
    })
  }

  // Zoom out: multiply the current zoom level by 0.8.
  const zoomOut = () => {
    if (!cyInstanceRef.current) return
    const currentZoom = cyInstanceRef.current.zoom()
    cyInstanceRef.current.zoom({
      level: currentZoom * 0.8,
      renderedPosition: {
        x: cyInstanceRef.current.width() / 2,
        y: cyInstanceRef.current.height() / 2,
      },
    })
  }

  // Recenters the graph in the viewport.
  const recenter = () => {
    if (!cyInstanceRef.current) return
    cyInstanceRef.current.center()
  }

  return (
    <div className='w-full h-screen overflow-hidden relative'>
      {
        (notes.length > 0) ? (
          <>
            <div
              ref={cyContainerRef}
              className='w-screen h-screen'
            />
            <div className='absolute bottom-3 right-3 z-10 flex flex-col gap-1'>
              <Button size='icon' onClick={zoomIn}><PlusIcon /></Button>
              <Button size='icon' onClick={zoomOut}><MinusIcon /></Button>
              <Button size='icon' onClick={recenter}><FocusIcon /></Button>
            </div>
          </>
        ) : <CreateNoteCard />
      }
    </div>
  )
}
