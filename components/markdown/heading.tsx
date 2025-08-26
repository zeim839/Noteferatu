import { Heading as HeadingType } from "@/lib/markdown"

function Heading({ node }: { node: HeadingType }) {
  return (
    <div
      data-depth={node.depth}
      className="data-[depth=1]:text-2xl data-[depth=2]:text-2xl"
    >
    </div>
  )
}

export { Heading }
