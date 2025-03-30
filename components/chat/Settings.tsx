'use client'

import { useState } from "react"
import { Button } from "@/components/ui/button"
import {
    Dialog,
    DialogTrigger,
    DialogContent,
    DialogHeader,
    DialogTitle,
    DialogFooter
} from "@/components/ui/dialog"
import { SettingsIcon } from "lucide-react"

const Settings = () => {
    const [open, setOpen] = useState(false)
    const [apiKey, setApiKey] = useState("")

    const handleSubmit = () => {
        // console.log("here is the open open router key: ", apiKey)
        setOpen(false)
    }

    return (
        <Dialog open={open} onOpenChange={setOpen}>
            <DialogTrigger asChild>
                <Button size='icon'>
                    <SettingsIcon />
                </Button>
            </DialogTrigger>
            <DialogContent className="w-full !p-4">
                <DialogHeader>
                    <DialogTitle>Enter your OpenRouter API Key</DialogTitle>
                </DialogHeader>
                <input
                id="api-key"
                placeholder="sk-..."
                value={apiKey}
                onChange={(e) => setApiKey(e.target.value)}
                className="col-span-3 px-3 py-2 border rounded-md w-full"
                />
                <DialogFooter className="mr-[35px]">
                    <Button onClick={handleSubmit}>Submit</Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    )
}

export default Settings
