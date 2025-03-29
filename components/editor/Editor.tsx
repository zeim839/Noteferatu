"use client"

import { EditorView, keymap } from "@codemirror/view"
import { useRef, useState, useMemo, useEffect } from "react"
import { EditorState } from "@codemirror/state"
import { basicSetup } from "codemirror"
import { markdown } from "@codemirror/lang-markdown"
import { defaultKeymap, indentWithTab } from "@codemirror/commands"
import { syntaxHighlighting } from "@codemirror/language"
import { markdownHighlightStyle, codeMirrorTheme } from "./theme"
import { placeholder } from '@codemirror/view'
import { useDB } from "@/components/DatabaseProvider"
import { useSearchParams } from 'next/navigation'
import { toast } from "sonner"
import { autocompletion } from "@codemirror/autocomplete"
import { NoteLinkMenu } from "./NoteLinkMenu"
import { edgesField, noteIDField, setNoteIDEffect } from "./State"
import { EdgesPlugin } from "./Edges"
import { useRouter } from "next/navigation"
import { SavingIndicator, SavedIndicator } from "./Autosave"

import NoteTitle from "./NoteTitle"
import Decorations from "./Decorations"

// UUID generates a unique note primary key id.
const UUID = () : number => {
  const buffer = new Uint32Array(2)
  crypto.getRandomValues(buffer)
  return (buffer[0] & 0x001fffff) * 0x100000000 + buffer[1]
}

export default function Editor() {
  const searchParams = useSearchParams()
  const idParam = searchParams.get('id')

  const [showSavedMsg, setShowSavedMsg] = useState<boolean>(false)
  const [isSaving, setIsSaving] = useState<boolean>(false)
  const [text, setText] = useState<string>('')
  const [title, setTitle] = useState<string>('')
  const [noteID,] = useState<number>(
    idParam ? Number(idParam) : UUID()
  )

  const titleRef = useRef<HTMLInputElement>(null)
  const editorRef = useRef(null)
  const editorViewRef = useRef<EditorView | null>(null)

  const router = useRouter()
  const db = useDB()

  const onUpdate = useMemo(() =>
    EditorView.updateListener.of((v) => {
      if (!v.docChanged) return
      setText(v.state.doc.toString())
    }), [])

  const focusEditor = () => {
    if (!editorViewRef.current) return
    const view = editorViewRef.current
    view.focus()
    view.dispatch({
      // Move cursor to the end of the first line
      selection: { anchor: view.state.doc.line(1).to },
    })
  }

  const focusTitle = () => {
    if (!titleRef.current) return
    titleRef.current.focus()
    if (editorViewRef.current) {
      editorViewRef.current.contentDOM.blur()
    }
    const range = document.createRange()
    const sel = window.getSelection()
    range.selectNodeContents(titleRef.current)
    range.collapse(false)
    sel?.removeAllRanges()
    sel?.addRange(range)
  }

  // Initialize editor.
  useEffect(() => {
    let isMounted = true
    let view: EditorView | null = null

    const initEditor = async () => {
      if (!editorRef.current) return

      const allNotes = await db.notes.readAll()
      db.edges.deleteEdgesBySrc(Number(noteID))
      let [content, noteTitle] = ['', '']

      if (idParam) {
        const note = allNotes.find(n => n.id === noteID)
        if (!note) {
          toast('Error: Note Not Found', {
            description: 'The current note no longer exists or could not be found.'
          })
        } else {
          content = note.content
          noteTitle = note.title
        }
      }

      if (!isMounted) return
      setTitle(noteTitle === 'Untitled' ? '' : noteTitle)
      setText(content)
      focusTitle()
      const state = EditorState.create({
        doc: content,
        extensions: [
          basicSetup,
          keymap.of([...defaultKeymap, indentWithTab]),
          markdown(),
          onUpdate,
          syntaxHighlighting(markdownHighlightStyle),
          EditorView.lineWrapping,
          codeMirrorTheme,
          Decorations,
          placeholder('Start typing here...'),
          autocompletion({
            override: [NoteLinkMenu(allNotes)]
          }),
          noteIDField,
          edgesField,
          EdgesPlugin
        ],
      })

      view = new EditorView({
        state,
        parent: editorRef.current,
      })
      editorViewRef.current = view
      view.dispatch({ effects: setNoteIDEffect.of(noteID.toString()) })

      const editorDom = view.dom
      const keyDownHandler = (e: KeyboardEvent) => {
        if (e.key === 'ArrowUp') {
          const pos = view!.state.selection.main.head
          const line = view!.state.doc.lineAt(pos)
          if (line.number === 1) {
            e.preventDefault()
            e.stopPropagation()
            focusTitle()
            return false
          }
        }
      }
      editorDom.addEventListener('keydown', keyDownHandler, true) // ensure custom keydown handler runs first
      // custom event to use nextjs react component only router
      const handleNavigate = (event: CustomEvent) => {
        const { path } = event.detail
        router.push(path)
      }
      document.addEventListener('navigate', handleNavigate as EventListener)
      return () => {
        editorDom.removeEventListener('keydown', keyDownHandler, true)
        document.removeEventListener('navigate', handleNavigate as EventListener)
      }
    }

    initEditor()
    return () => {
      isMounted = false
      if (view) {
        const edges = view.state.field(edgesField)
        if (edges && db) {
          edges.forEach(edge => {
            db.notes.read(edge.dst)
              .then((note) => {
                if (note) db.edges.create(edge)
              })
          })
        }
        view.destroy()
      }
    }
  }, [])

  // Autosave.
  useEffect(() => {
    if (text === '' && title === '') return
    setIsSaving(true)
    const timeout = setTimeout(async () => {
      try {
        await db.notes.create({
          id      : noteID,
          title   : title === '' ? "Untitled" : title,
          content : text,
          atime   : Math.floor(Date.now() / 1000),
          mtime   : Math.floor(Date.now() / 1000)
        })
      } catch {
        toast('Error: Could not save note', {
          description: 'A database error prevented the note from being saved.'
        })
      }
      setIsSaving(false)
      setShowSavedMsg(true)
      setTimeout(() => setShowSavedMsg(false), 2000)
    }, 500)
    return () => clearTimeout(timeout)
  }, [text, title])

  return (
    <div className='pt-12 h-screen overflow-hidden relative max-w-[800px] w-full m-auto flex flex-col'>
      <div ref={editorRef} className='w-full h-full overflow-auto'>
        <NoteTitle
          ref={titleRef}
          className="text-4xl font-medium p-3 outline-none break-words overflow-hidden my-4 max-w-[800px] relative"
          placeholder="Untitled"
          value={title}
          onEdit={setTitle}
          onExit={focusEditor}
        />
      </div>
      <div className='absolute bottom-3 left-3 text-green-900 flex flex-row items-center'>
        { (isSaving) ? <SavingIndicator/> : showSavedMsg ? <SavedIndicator /> : null}
      </div>
    </div>
  )
}
