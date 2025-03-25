// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-nocheck

import { useState, useRef, useEffect } from "react"
import { useRouter, usePathname } from "next/navigation"
import { AlignJustifyIcon, HouseIcon } from "lucide-react"
import { Button } from "@/components/ui/button"
import Recents from "@/components/recents/Recents"
import { NavigationState } from "./NavigationState"
import { cn } from "@/lib/utils"

import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command"

// LeftNavigation consists of the 'recents' hamburger menu and search bar.
const LeftNavigation = ({ state } : { state: NavigationState }) => {
  const [searchValue, setSearchValue] = useState<string>("")
  const [isOpen, setIsOpen] = useState<boolean>(false)
  const commandRef = useRef<HTMLDivElement>(null)
  const inputRef = useRef<HTMLInputElement>(null)
  const isNotePage = usePathname() === '/note'
  const router = useRouter()

  // Pressing the meta/ctrl + K keybinding opens the Command menu.
  // It automatically focuses on the CommandInput.
  const handleKeyDown = (e: KeyboardEvent) => {
    if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
      e.preventDefault()
      setIsOpen(!isOpen)
      setTimeout(() => inputRef.current?.focus(), 0)
    }
  }

  useEffect(() => {
    document.addEventListener('keydown', handleKeyDown)
    return () => document.removeEventListener('keydown', handleKeyDown)
  }, [isOpen])

  // Clicking outside of the CommandInput closes the CommandList.
  const handleClickOutside = (event: MouseEvent) => {
    if (!commandRef.current ||
      commandRef.current.contains(event.target as Node)) {
      return
    }
    setIsOpen(false)
  }

  useEffect(() => {
    document.addEventListener("mousedown", handleClickOutside)
    return () => {
      document.removeEventListener("mousedown", handleClickOutside)
    }
  }, [])

  // Call an action whenever a Command menu item is selected.
  // Automatically clears the CommandInput value, closes the CommandList,
  // and erases the input value.
  const handleSelect = (action: () => void) => {
    action()
    setSearchValue("")
    inputRef.current?.blur()
    setIsOpen(false)
  }

  return (
    <div className='flex flex-row gap-1 z-20 fixed left-2 top-2'>
      { /* Show a 'home' button when on the note page */ }
      { (isNotePage) ? (
        <Button size='icon' onClick={() => router.push('/')}>
          <HouseIcon />
        </Button>
      ) : null
      }
      { /* Toggles the left sidebar */ }
      <Button
        onClick={() => state.setLeftOpen(!state.isLeftOpen)}
        size='icon'
      >
        <AlignJustifyIcon />
      </Button>
      { /* Shrink the search bar when the 'home' button is shown */ }
      <Command
        className={`${isNotePage ? 'w-[263px]' : 'w-[305px]'}`}
        ref={commandRef}>
        <CommandInput
          placeholder="Search Notes"
          value={searchValue}
          onValueChange={(v: string) => setSearchValue(v)}
          onClick={() => setIsOpen(true)}
          ref={inputRef}
        />
        <CommandList className={cn(!isOpen && "hidden")}>
          <CommandEmpty>No results found.</CommandEmpty>
          <CommandGroup heading="Suggestions">
            <CommandItem
              onSelect={() => handleSelect(() => state.setLeftOpen(!state.isLeftOpen))}>
              <span>Recent Notes</span>
            </CommandItem>
            <CommandItem
              onSelect={() => handleSelect(() => router.push('/note'))}>
              <span>Create Note</span>
            </CommandItem>
            { (isNotePage) ? (
              <CommandItem
                onSelect={() => handleSelect(() => router.push('/'))}>
                <span>Graph View</span>
              </CommandItem>
            ): null
            }
            <CommandItem
              onSelect={() => handleSelect(() => state.setRightOpen(!state.isRightOpen))}>
              <span>Chat</span>
            </CommandItem>
            <CommandItem>
              <span>Settings</span>
            </CommandItem>
          </CommandGroup>
        </CommandList>
      </Command>
    </div>
  )
}

// LeftSidebar shows recently accessed notes or search results.
const LeftSidebar = ({isOpen}: { isOpen: boolean }) => {
  // Use fixed positioning for the GraphView to prevent shifting
  // nodes and edges.
  const fixedPos = usePathname() === '/' ? 'fixed left-0' : ''
  return (
    <div className={cn(fixedPos, "min-w-[366px] w-[366px] h-screen bg-[rgba(245,245,245,0.75)] p-2 border border-r-gray-300 z-10 transition-transform duration-300 ease-in-out transform",
    isOpen ? "translate-x-0" : "-translate-x-full")}>
      <Recents />
    </div>
  )
}

export { LeftNavigation, LeftSidebar }
