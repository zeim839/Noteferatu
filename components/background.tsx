"use client"

import { createContext, useContext, useState, ReactNode } from 'react'

type EditorBackgroundContextType = {
  setEditorMode : (active: boolean) => void
  isEditorMode  : boolean
}

const EditorBackgroundContext = createContext<EditorBackgroundContextType>({
  setEditorMode: () => {},
  isEditorMode: false
})

// EditorBackgroundProvider is used to switch between transparent
// and opaque window backgrounds. Note titles need opaque background,
// the graph view needs transparent background.
export const EditorBackgroundProvider = ({ children }: {children: ReactNode}) => {
  const [isEditorMode, setEditorMode] = useState<boolean>(false)
  return (
    <EditorBackgroundContext.Provider value={{ isEditorMode, setEditorMode }}>
      {children}
    </EditorBackgroundContext.Provider>
  )
}

// Exposes background context within an EditorBackgroundProvider.
export const useEditorBackground = (): EditorBackgroundContextType => {
  const context = useContext(EditorBackgroundContext)
  if (!context) {
    throw new Error('useEditorBackground must be used within an EditorBackgroundProvider')
  }
  return context
}
