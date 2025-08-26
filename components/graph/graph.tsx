import * as React from "react"

import { Network } from "vis-network/esnext"
import { DataSet } from "vis-data/esnext"

function Graph() {
  const graphRef = React.useRef<HTMLDivElement>(null)
  const networkRef = React.useRef<Network | null>(null)

  React.useEffect(() => {
    if (!graphRef.current) {
      return
    }

    const nodes = new DataSet([
      { id: 1, label: "Node 1" },
      { id: 2, label: "Node 2" },
      { id: 3, label: "Node 3" },
      { id: 4, label: "Node 4" },
      { id: 5, label: "Node 5" }
    ])

    const edges = new DataSet([
      { id: 1, from: 1, to: 3 },
      { id: 2, from: 1, to: 2 },
      { id: 3, from: 2, to: 4 },
      { id: 4, from: 2, to: 5 },
    ])

    const options = {
      autoResize: false,
      physics: {
        enabled: true,
        stabilization: { iterations: 100 }
      },
      interaction: {
        zoomView: true,
        dragView: true
      },
      layout: {
        randomSeed: 777,
      },
      nodes: {
        shape: 'dot',
        color: {
          background: '#7A89A5',
          border: '#7A89A5',
          highlight: {
            background: '#7A89A5',
            border: '#7A89A5',
          },
        },
        font: {
          face: 'Iosevka Comfy',
          size: 13,
        },
        size: 13
      },
      edges: {
        smooth: false,
      }
    }

    networkRef.current = new Network(graphRef.current, {
      nodes: nodes,
      edges: edges
    }, options)

    // Destroy canvas, stop physics on cleanup.
    return () => {
      if (networkRef.current) {
        networkRef.current.setOptions({ physics: { enabled: false } })
        networkRef.current.destroy()
        networkRef.current = null
      }
    }
  }, [])

  return <div className="w-full h-full" ref={graphRef} />
}

export { Graph }
