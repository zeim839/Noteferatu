import { useState, useEffect, useRef } from "react"

export type windowSize = {
  width: number,
  height: number
}

function useWindowSize(callback: (current: windowSize, prev: windowSize) => void) {
  const [windowSize, setWindowSize] = useState<windowSize>({
    width: (typeof window !== "undefined") ? window.innerWidth : 0,
    height: (typeof window !== "undefined") ? window.innerHeight : 0,
  })
  const prevSizeRef = useRef(windowSize)
  useEffect(() => {
    const handleResize = () => {
      const newSize = {
        width: window.innerWidth,
        height: window.innerHeight,
      }

      const prevSize = prevSizeRef.current

      // Call callback with both values
      if (callback) {
        callback(newSize, prevSize)
      }

      prevSizeRef.current = newSize
      setWindowSize(newSize)
    }
    window.addEventListener('resize', handleResize)
    return () => window.removeEventListener('resize', handleResize)
  }, [callback])

  return { current: windowSize, previous: prevSizeRef.current }
}

export { useWindowSize }
