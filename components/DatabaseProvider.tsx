"use client"

import NoteController from '@/lib/controller/NoteController'
import EdgeController from '@/lib/controller/EdgeController'
import KeyController from '@/lib/controller/KeyController'
import ChatHistoryController from '@/lib/controller/ChatHistoryController'

import { appLocalDataDir, join } from '@tauri-apps/api/path'
import { toast } from "sonner"

import {
  createContext,
  useContext,
  useState,
  ReactNode,
  useEffect
} from 'react'

export type DatabaseContextType = {
  notes   : NoteController,
  edges   : EdgeController,
  keys    : KeyController,
  history : ChatHistoryController,
}

// DatabaseContext exposes initialized database controllers.
export const DatabaseContext = createContext<DatabaseContextType | null>(null)

// DatabaseProvider exposes initialized database controllers into the DOM.
export function DatabaseProvider({ children }: { children: ReactNode }) {
  const [database, setDatabase] = useState<DatabaseContextType | null>(null)
  const initDatabase = async () => {
    try {
      const dbPath = await join(await appLocalDataDir(), 'db.sqlite')
      setDatabase({
        notes   : new NoteController(dbPath),
        edges   : new EdgeController(dbPath),
        keys    : new KeyController(dbPath),
        history : new ChatHistoryController(dbPath)
      })
    } catch (error) {
      let description = 'an unknown database error has occurred'
      if (error instanceof Error) {
        description = error.message
      }
      toast("Error: Failed to Initialize Database", {description})
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
