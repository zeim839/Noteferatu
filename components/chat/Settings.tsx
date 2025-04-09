'use client'

import { useState, useEffect, Dispatch, SetStateAction } from "react"
import { Button } from "@/components/ui/button"
import { useDB } from "@/components/DatabaseProvider"
import { toast } from "sonner"
import { Keys } from "@/lib/controller/KeyController"

import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog"

type SettingsProps = {
  onOpenChange: Dispatch<SetStateAction<boolean>>
  open: boolean,
}

// Settings displays a modal asking the user to configure their
// OpenRouter API key.
const Settings = ({open, onOpenChange} : SettingsProps) => {
  const [apiKey, setApiKey] = useState("")
  const db = useDB()

  // Load a preview of the key from the database.
  useEffect(() => {
    const fetchKey = async () => {
      let keys : Keys[] = []
      try {keys = await db.keys.readAll()} catch {
        toast('Error: Failed to Load API Key', {
          description: 'The database failed to load a valid API key'
        })
      }
      if (keys.length === 0) {
        setApiKey("")
        return
      }
      setApiKey(keys[0].key_hash)
    }
    fetchKey()
  }, [])

  // We only store one key, so on submit, delete everything and
  // append the new key.
  const handleSubmit = async () => {
    try {
      await db.keys.deleteAll()
      if (apiKey === "") {
        onOpenChange(false)
        return
      }
      await db.keys.create({
        key_hash   : apiKey,
        created_at : Math.floor(Date.now() / 1000)
      })
    } catch {
      toast('Error: Could not save key', {
        description: 'A database error prevented the key from being saved'
      })
    }
    onOpenChange(false)
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="w-full !p-4">
        <DialogHeader className="col-span-2">
          <DialogTitle>Enter your OpenRouter API Key</DialogTitle>
        </DialogHeader>
        <input
          id="api-key"
          placeholder="sk-..."
          type="password"
          value={apiKey}
          onChange={(e) => setApiKey(e.target.value)}
          className="col-span-3 px-3 py-2 border rounded-sm w-full"
        />
        <Button className="col-span-3" onClick={handleSubmit}>
          Submit
        </Button>
      </DialogContent>
    </Dialog>
  )
}

export default Settings
