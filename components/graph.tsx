import React, { useEffect, useRef, useState } from 'react'
import cytoscape from 'cytoscape'
import { useRouter } from "next/navigation"
import NoteController, { Note } from '@/lib/controller/NoteController'
import { appLocalDataDir } from '@tauri-apps/api/path'
import { Button } from "@/components/ui/button"
import path from "path"

import {
  FocusIcon,
  PlusIcon,
  MinusIcon,
} from "lucide-react"

const CreateNoteCard = () => {
  const router = useRouter()
  return (
    <div className='w-full h-full flex items-center justify-center pb-32'>
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

export default function GraphView() {
  const [notes, setNotes] = useState<Note[]>([])
  const cyInstanceRef = useRef<cytoscape.Core | null>(null)
  const cyContainerRef = useRef(null)
  const router = useRouter()

  const fetchNotes = async () => {
    const dbDir = await appLocalDataDir()
    const controller = new NoteController(path.join(dbDir, 'db.sqlite'))
    setNotes(await controller.readAll())
  }

  useEffect(() => { fetchNotes() }, [])

  useEffect(() => {
    if (!cyContainerRef.current || notes.length == 0) return
    const cy = cytoscape({
      container: cyContainerRef.current,
      elements: [
        { data: { id: 'a', title: 'Skibidi Toilet Lore' } },
        { data: { id: 'b', title: 'Quandale Dingle' } },
        { data: { id: 'c', title: 'TikTok Rizz party' } },
        { data: { id: 'd', title: 'Open Source Club' } },
        { data: { id: 'e', title: 'WICSE' } },
        { data: { id: 'f', title: 'Bytes of Love' } },
        { data: { id: 'ab', source: 'a', target: 'b' } },
        { data: { id: 'de', source: 'd', target: 'e' } },
        { data: { id: 'df', source: 'd', target: 'f' } },
        { data: { id: 'ce', source: 'c', target: 'e' } },
        { data: { id: 'dc', source: 'd', target: 'c' } },
      ],
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
      layout: {
        name: 'cose',
        randomize: true,
        animate: false,
        padding: 20,
      },
    })

    cy.on('tap', 'node', (event) => {
      const node = event.target
      router.push(`/note?id=${node.id()}`)
    })

    cyInstanceRef.current = cy

    return () => { cy.destroy() }
  }, [notes, router])

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
    <div className='w-screen h-[calc(100vh-70px)] overflow-hidden relative'>
      {
        (notes.length > 0) ? (
          <>
            <div
              ref={cyContainerRef}
              className='w-screen h-[calc(100vh-70px)]'
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
