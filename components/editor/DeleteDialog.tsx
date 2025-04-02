import { Button } from "@/components/ui/button"
import { Trash2Icon } from "lucide-react"
import { useRouter } from "next/navigation"
import { useDB } from "@/components/DatabaseProvider"

import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"

export function DeleteDialog({ noteID }: { noteID: number }) {
  const router = useRouter()
  const db = useDB()

  const handleDelete = async () => {
    await db.notes.delete(noteID)
    router.push("/")
  }

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="destructive" className="px-2">
          <Trash2Icon/>
        </Button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle className="text-red-400">Confirm Deletion</DialogTitle>
          <DialogDescription className="text-black">
            Are you sure you want to permanently remove this item?
            This action is irreversible.
          </DialogDescription>
        </DialogHeader>
        <DialogFooter className="flex justify-center">
          <Button variant="destructive" onClick={handleDelete}>
            Delete
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
