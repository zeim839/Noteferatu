// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-nocheck
import { Button } from "@/components/ui/button"
import { AlignJustify, MessageSquare, Settings } from 'lucide-react'

import {
  Command,
  CommandInput,
  CommandList,
  CommandEmpty
} from "@/components/ui/command"

export default function Home() {
  return (
    <div className='w-full flex flex-row p-3 justify-between'>
      <div className='flex flex-row gap-2'>
        <Button size='icon'><AlignJustify /></Button>
        <Command>
          <CommandInput placeholder='Search Notes' />
          <CommandList>
            <CommandEmpty>No results found.</CommandEmpty>
          </CommandList>
        </Command>
      </div>
      <div className='flex flex-row gap-1'>
        <Button size='icon'><MessageSquare /></Button>
        <Button size='icon'><Settings /></Button>
      </div>
    </div>
  )
}
