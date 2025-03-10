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
import { useEditorBackground } from "./background"
import { placeholder } from '@codemirror/view'

export default function Editor() {
  const [/*text*/, setText] = useState<string>('')
  const titleRef = useRef<HTMLInputElement>(null)
  const editorRef = useRef(null)
  const { setEditorMode } = useEditorBackground();
  const onUpdate = useMemo(() =>
    EditorView.updateListener.of((v) => {
      setText(v.state.doc.toString())
    }), [])

  useEffect(() => {
    if (!editorRef.current) return
    setEditorMode(true)

    // Set keyboard caret to the end of the title text.
    if (titleRef.current) {
      const element = titleRef.current
      const range = document.createRange()
      range.selectNodeContents(element)
      range.collapse(false)

      const selection = window.getSelection()
      selection?.removeAllRanges()
      selection?.addRange(range)
    }

    const state = EditorState.create({
      doc: '# Heading 1\nLorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.\n\nLorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.\n## Heading 2\nLorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.\n### Heading 3\nLorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.',
      extensions: [
        basicSetup,
        keymap.of([...defaultKeymap, indentWithTab]),
        markdown(),
        onUpdate,
        syntaxHighlighting(markdownHighlightStyle),
        EditorView.lineWrapping,
        codeMirrorTheme,
        Decorations,
        placeholder('Start typing here...')
      ],
    });

    const view = new EditorView({
      state,
      parent: editorRef.current,
    });

    return () => {
      view.destroy()
      setEditorMode(false)
    }
  }, [onUpdate, setEditorMode])

  return (
    <div className='h-[calc(100vh-80px)] overflow-hidden relative max-w-[800px] w-full m-auto flex flex-col'>
      <div ref={editorRef} className='w-full h-full overflow-auto'>
        <div
          ref={titleRef}
          className="text-4xl font-medium p-3 outline-none break-words overflow-hidden my-4 max-w-[800px]"
          contentEditable
          suppressContentEditableWarning={true}
        >
          Untitled
        </div>
      </div>
    </div>
  )
}
