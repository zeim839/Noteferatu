"use client"

import { EditorView, keymap } from "@codemirror/view"
import { useRef, useState, useMemo, useEffect } from "react"
import { EditorState } from "@codemirror/state"
import { basicSetup } from "codemirror"
import { markdown } from "@codemirror/lang-markdown"
import { defaultKeymap, indentWithTab } from "@codemirror/commands"
import { syntaxHighlighting } from "@codemirror/language"
import { markdownHighlightStyle, codeMirrorTheme } from "./theme"
import Decorations from "./Decorations"
import { placeholder } from '@codemirror/view'
import NoteTitle from "./NoteTitle"
import { useDB } from "@/components/DatabaseProvider"
import { useSearchParams } from 'next/navigation'
import { toast } from "sonner"
import { autocompletion } from "@codemirror/autocomplete"
import { NoteLinkMenu } from "./NoteLinkMenu"

export default function Editor() {
  const [/*text*/, setText] = useState<string>('')
  const db = useDB()
  const searchParams = useSearchParams()
  const noteID = searchParams.get('id')
  const [title, setTitle] = useState('')
  const titleRef = useRef<HTMLInputElement>(null)
  const editorRef = useRef(null)
  const editorViewRef = useRef<EditorView | null>(null)
  const onUpdate = useMemo(() =>
    EditorView.updateListener.of((v) => {
      setText(v.state.doc.toString())
    }),
  [])

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

  useEffect(() => {
    let isMounted = true
    let view: EditorView | null = null

    const initEditor = async () => {
      if (!editorRef.current) return

      const allNotes = await db.notes.readAll()
      let content = ''
      let noteTitle = ''
      if (noteID) {
        const note = allNotes.find(n => n.id === Number(noteID))
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
      setTitle(noteTitle)
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
          })
        ],
      })

      view = new EditorView({
        state,
        parent: editorRef.current,
      })
      editorViewRef.current = view

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
      return () => {
        editorDom.removeEventListener('keydown', keyDownHandler, true)
      }
    }

    initEditor()
    return () => {
      isMounted = false
      if (view) {
        view.destroy()
      }
    }
  }, [onUpdate, noteID, db.notes])

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
    </div>
  )
}
