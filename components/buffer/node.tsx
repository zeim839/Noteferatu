import * as React from "react"
import { TabRecord } from "./tabs"
import { Header } from "./header"
import { MDMode } from "@/components/mdmode/mdmode"

function BufferNode({onSplit, onClose}: {
  onSplit: (orientation: "vertical" | "horizontal" | null) => void,
  onClose?: () => void
}) {
  const [active, setActive] = React.useState<number>(0)
  const [tabs, setTabs] = React.useState<Array<TabRecord>>([
    {
      prev: null,
      next: null,
      name: "Introduction",
      type: "",
      path: "",
    },
    {
      prev: null,
      next: null,
      name: "Markdown",
      type: "",
      path: "",
    },
    {
      prev: null,
      next: null,
      name: "Introduction",
      type: "",
      path: "",
    },
    {
      prev: null,
      next: null,
      name: "Markdown",
      type: "",
      path: "",
    },

  ])

  const handleCloseTab = (i: number) => {
    // If this is the last tab, close the entire buffer.
    if (tabs.length === 1) {
      onClose?.()
      return
    }
    const newTabs = tabs.filter((_, index) => index !== i)
    setTabs(newTabs)
    if (active >= newTabs.length) {
      setActive(newTabs.length - 1)
    } else if (active >= i) {
      setActive(Math.max(active - 1, 0))
    }
  }

  return (
    <div className="flex flex-col h-full w-full">
      <Header
        tabs={tabs}
        onSplit={onSplit}
        active={active}
        setActive={setActive}
        onCloseTab={handleCloseTab}
        onCloseBuffer={() => onClose?.()}
      />
      <MDMode />
    </div>
  )
}

export { BufferNode }
