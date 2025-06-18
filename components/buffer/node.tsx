import * as React from "react"
import { TabRecord } from "./tabs"
import { Header } from "./header"

function BufferNode({onSplit}: {
  onSplit: (orientation: "vertical" | "horizontal" | null) => void
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

  return (
    <div className="flex flex-col h-full w-full">
      <Header
        tabs={tabs}
        onSplit={onSplit}
        active={active}
        setActive={setActive}
      />
      <div className="w-full h-full bg-[#EFF1F5]" />
    </div>
  )
}

export { BufferNode }
