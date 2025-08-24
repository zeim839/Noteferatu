import * as React from "react"

// Defines a common context for the markdown mode editor.
export type MDModeContextType = {
  isBookmarked: boolean,
  setIsBookmarked: (isBookmarked: boolean) => void,
}

// Implements MDModeContext as a React Context.
export const MDModeContext = React.createContext<MDModeContextType | null>(null)

// Exposes MDModeContext.
export function MDModeProvider({ children }: { children: React.ReactNode }) {
  const [isBookmarked, setIsBookmarked] = React.useState<boolean>(false)

  // Construct context.
  const context: MDModeContextType = {
    isBookmarked,
    setIsBookmarked,
  }

  return (
    <MDModeContext.Provider value={context}>
      { children }
    </MDModeContext.Provider>
  )
}

export function useMDModeContext() {
  const context = React.useContext(MDModeContext)
  if (!context) {
    throw new Error("useMDModeContext must be called within MDModeProvider")
  }
  return context
}
