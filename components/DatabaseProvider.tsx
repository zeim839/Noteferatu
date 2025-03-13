"use client"

import NoteController from '@/lib/controller/NoteController'
import { appLocalDataDir, join } from '@tauri-apps/api/path'

import {
  createContext,
  useContext,
  useState,
  ReactNode,
  useEffect
} from 'react'

type DatabaseContextType = {
  notes: NoteController
}

// DatabaseContext exposes initialized database controllers.
export const DatabaseContext = createContext<DatabaseContextType | null>(null)

// DatabaseProvider exposes initialized database controllers into the DOM.
export function DatabaseProvider({ children }: { children: ReactNode }) {
  const [database, setDatabase] = useState<DatabaseContextType | null>(null)
  const initDatabase = async () => {
    try {
      const dbPath = await join(await appLocalDataDir(), 'db.sqlite')
      const notesController = new NoteController(dbPath)
      setDatabase({notes: notesController})
    } catch (error) {
      // TODO: build a notification feature for errors.
      console.error("Failed to initialize database:", error)
    }
  }

  useEffect(() => { initDatabase() }, [])
  if (!database) {
    return null
  }

  return (
    <DatabaseContext.Provider value={database}>
      {children}
    </DatabaseContext.Provider>
  )
}

// useDB fetches a DatabaseContext and exposes database controllers to
// react components.
export function useDB() {
  const context = useContext(DatabaseContext)
  if (!context) {
    throw new Error('useDB must be used within a DatabaseContextProvider')
  }
  return context
}
