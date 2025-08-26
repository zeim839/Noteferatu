import * as React from "react"

export type ViewType = 'document' | 'graph'

// Defines a common context for the markdown editor.
export type EditorContextType = {
  view: ViewType
  setView: (view: ViewType) => void
}

// Implements EditorContextType as a React Context.
export const EditorContext = React.createContext<EditorContextType | null>(null)

// Exposes EditorContext.
export function EditorProvider({ children }: { children: React.ReactNode }) {
  const [view, setView] = React.useState<ViewType>('document')
  const context: EditorContextType = {
    view, setView
  }
  return (
    <EditorContext.Provider value={context}>
      { children }
    </EditorContext.Provider>
  )
}

export function useEditorContext() {
  const context = React.useContext(EditorContext)
  if (!context) {
    throw new Error("useEditorContext must be called within EditorProvider")
  }
  return context
}
