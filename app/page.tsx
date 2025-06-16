"use client"

import { Window } from "@/components/window/window"
import { VBuffer } from "@/components/window/vbuffer"

export default function Home() {
  return (
    <Window>
      <Window.LeftSidebarBody>
        <p> Hello, world! </p>
      </Window.LeftSidebarBody>
      <Window.Content>
        <VBuffer />
      </Window.Content>
      <Window.RightSidebarBody>
        <p> Goodbye, moon! </p>
      </Window.RightSidebarBody>
    </Window>
  )
}
