import * as React from "react"
import { Node } from "@/lib/markdown"

interface RootProps extends React.ComponentProps<"div"> {
  isEditable?: boolean
  node: Node
}

function Root({ node, isEditable = false } : RootProps) {
  if (node.type === "root") {
    return (
      <>
        {node.children.map((child, index) => (
          <Root key={index} node={child} isEditable={isEditable} />
        ))}
      </>
    )
  }

  if (node.type === "blockquote") {
    return null
  }

  if (node.type === "footnoteDefinition") {
    return null
  }

  if (node.type === "list") {
    return null
  }

  if (node.type === "break") {
    return null
  }

  if (node.type === "inlineCode") {
    return null
  }

  if (node.type === "inlineMath") {
    return null
  }

  if (node.type === "delete") {
    return null
  }

  if (node.type === "emphasis") {
    return null
  }

  if (node.type === "footnoteReference") {
    return null
  }

  if (node.type === "html") {
    return null
  }

  if (node.type === "image") {
    return null
  }

  if (node.type === "imageReference") {
    return null
  }

  if (node.type === "link") {
    return null
  }

  if (node.type === "linkReference") {
    return null
  }

  if (node.type === "strong") {
    return null
  }

  if (node.type === "text") {
    return (<p>{node.value}</p>)
  }

  if (node.type === "code") {
    return null
  }

  if (node.type === "math") {
    return null
  }

  if (node.type === "heading") {
    return (
      <div className="text-2xl">
        {node.children.map((child, index) => (
          <Root key={index} node={child} isEditable={isEditable} />
        ))}
      </div>
    )
  }

  if (node.type === "table") {
    return null
  }

  if (node.type === "thematicBreak") {
    return null
  }

  if (node.type === "tableRow") {
    return null
  }

  if (node.type === "tableCell") {
    return null
  }

  if (node.type === "listItem") {
    return null
  }

  if (node.type === "definition") {
    return null
  }

  if (node.type === "paragraph") {
    return (
      <>
        {node.children.forEach((child) => (
          <Root node={child} isEditable={isEditable} />
        ))}
      </>
    )
  }
}

export { Root }
