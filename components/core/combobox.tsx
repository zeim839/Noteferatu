"use client"

import * as React from "react"
import { Slot } from "@radix-ui/react-slot"

import {
  Command,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/core/command"

import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/core/popover"

interface ComboboxProps extends React.ComponentProps<"div"> {
  open?: boolean
  onOpenChange?: (open: boolean) => void
}

function Combobox({
  children,
  open: openProp,
  onOpenChange: onOpenChangeProp,
}: ComboboxProps) {
  const [internalOpen, setInternalOpen] = React.useState(false)
  const isControlled = openProp !== undefined
  const open = isControlled ? openProp : internalOpen
  const handleOpenChange = (value: boolean) => {
    if (onOpenChangeProp) {
      onOpenChangeProp(value)
    }
    if (!isControlled) {
      setInternalOpen(value)
    }
  }

  // Deconstruct combobox subcomponents.
  const slots = React.useMemo(() => {
    let trigger: React.ReactNode | null = null
    let groups: Array<React.ReactNode> | null = null
    let emptyBody: React.ReactNode | null = null
    React.Children.forEach(children, (child) => {
      if (!React.isValidElement(child)) return
      if (child.type === Combobox.Trigger) {
        trigger = child
        return
      }
      if (child.type === Combobox.Group) {
        if (groups === null) {
          groups = []
        }
        groups.push(child)
        return
      }
      if (child.type === Combobox.EmptyBody) {
        emptyBody = child
        return
      }
    })
    return { trigger, groups, emptyBody }
  }, [children])

  return (
    <Popover open={open} onOpenChange={handleOpenChange}>
      <PopoverTrigger asChild>{slots.trigger}</PopoverTrigger>
      <PopoverContent className="w-[200px] p-0">
        {slots.groups !== null ? (
          <Command>
            <CommandInput placeholder="Search..." className="h-9" />
            <CommandList>
              {slots.groups}
            </CommandList>
          </Command>
        ) : (
          slots.emptyBody
        )}
      </PopoverContent>
    </Popover>
  )
}

function ComboboxSlot({ ...props }: React.ComponentProps<"div">) {
  return <Slot {...props} />
}

function ComboboxEmptyBody({ ...props }: React.ComponentProps<"div">) {
  return <Slot {...props} />
}

export type ComboboxValue = {
  value: string
  label: string
}

// Combobox Subcomponents.
Combobox.Trigger = ComboboxSlot
Combobox.EmptyBody = ComboboxEmptyBody
Combobox.Group = CommandGroup
Combobox.Item = CommandItem

export { Combobox }
