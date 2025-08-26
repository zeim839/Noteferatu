import * as React from "react"

import { TabRecord } from "./tabs"
import { Header } from "./header"

import { readFromFile } from "@/lib/helsync"
import { Node } from "@/lib/markdown"
import { Editor } from "@/components/editor/editor"
import { EditorProvider } from "@/components/editor/context"

function BufferNode({onSplit, onClose}: {
  onSplit: (orientation: "vertical" | "horizontal" | null) => void,
  onClose?: () => void
}) {
  const [active, setActive] = React.useState<number>(0)
  const [node, setNode] = React.useState<Node | null>(null)
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


  // Load demo file data into the buffer.
  React.useEffect(() => {
    readFromFile("0").then((node) => {
      console.log(node)
      setNode(node)
    })
  }, [])

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
      <EditorProvider>
        { (node !== null) ? <Editor node={node} /> : null }
      </EditorProvider>
    </div>
  )
}

export { BufferNode }
