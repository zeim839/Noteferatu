"use client"

import { DefaultNode, Graph } from '@visx/network'
import { Text } from "@visx/text"
import { useRouter } from "next/navigation"

export type NetworkProps = {
  width  : number
  height : number
}

interface CustomNode {
  x          : number
  y          : number
  title      : string
  isSelected : boolean
}

interface CustomLink {
  source : CustomNode
  target : CustomNode
}

const nodes: CustomNode[] = [
  { x: 70, y: 20, title: 'Roman Republic', isSelected: false },
  { x: 200, y: 250, title: 'Second Punic War', isSelected: false },
  { x: 300, y: 40, title: 'Hamilcar Barca', isSelected: true },
]

const links: CustomLink[] = [
  { source: nodes[0], target: nodes[1] },
  { source: nodes[1], target: nodes[2] },
  { source: nodes[2], target: nodes[0] },
]

const graph = {
  nodes,
  links,
}

const Node = ({node} : { node: CustomNode }) => {
  const router = useRouter()
  return (
    <>
      { (node.isSelected) ? (<DefaultNode r={20} fill='#CE2E8C' />) : null }
      <DefaultNode r={ (node.isSelected) ? 16 : 20 }
        fill='#4C8EDA' onClick={() => router.push('/note')} />
      <Text dy={40} verticalAnchor="middle" textAnchor="middle">{node.title}</Text>
    </>
  )
}

export default function GraphView({ width, height }: NetworkProps) {
  return (
    <svg width={width} height={height}>
      <Graph<CustomLink, CustomNode>
        graph={graph}
        nodeComponent={Node}
        linkComponent={({ link: { source, target } }) => (
          <line
            x1={source.x}
            y1={source.y}
            x2={target.x}
            y2={target.y}
            strokeWidth={1}
            stroke="#979797"
            strokeOpacity={0.6}
          />
        )}
      />
    </svg>
  )
}
