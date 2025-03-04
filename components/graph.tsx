"use client"

import { DefaultNode, Graph } from '@visx/network'
import { useRouter } from "next/navigation"

export type NetworkProps = {
  width: number
  height: number
}

interface CustomNode {
  x: number
  y: number
  color?: string
}

interface CustomLink {
  source: CustomNode
  target: CustomNode
  dashed?: boolean
}

const nodes: CustomNode[] = [
  { x: 50, y: 20 },
  { x: 200, y: 250 },
  { x: 300, y: 40, color: '#26deb0' },
]

const links: CustomLink[] = [
  { source: nodes[0], target: nodes[1] },
  { source: nodes[1], target: nodes[2] },
  { source: nodes[2], target: nodes[0], dashed: true },
]

const graph = {
  nodes,
  links,
}

export default function GraphView({ width, height }: NetworkProps) {
  const router = useRouter()
  return width < 10 ? null : (
    <svg width={width} height={height}>
      <Graph<CustomLink, CustomNode>
        graph={graph}
        top={20}
        left={100}
        nodeComponent={({ node: { color } }) =>
          color ? <DefaultNode onClick={() => router.push('/note')} fill={color} /> : <DefaultNode onClick={() => router.push('/note')} />
        }
        linkComponent={({ link: { source, target, dashed } }) => (
          <line
            x1={source.x}
            y1={source.y}
            x2={target.x}
            y2={target.y}
            strokeWidth={2}
            stroke="#999"
            strokeOpacity={0.6}
            strokeDasharray={dashed ? '8,4' : undefined}
          />
        )}
      />
    </svg>
  )
}
