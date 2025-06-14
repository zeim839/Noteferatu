import { useState, useEffect } from 'react'
import { getCurrentWindow } from "@tauri-apps/api/window"

export default function Titlebar() {
  const [isMaximized, setIsMaximized] = useState(false)

  useEffect(() => {
    const setupWindow = async () => {
      const window = getCurrentWindow()
      setIsMaximized(await window.isMaximized())

      const unlisten = await window.onResized(() => {
        window.isMaximized().then(setIsMaximized)
      })

      return unlisten
    }
    setupWindow()
  }, [])

  const minimize = async () => {
    const window = getCurrentWindow()
    await window.minimize()
  }

  const toggleMaximize = async () => {
    const window = getCurrentWindow()
    if (isMaximized) {
      await window.unmaximize()
    } else {
      await window.maximize()
    }
    setIsMaximized(!isMaximized)
  }

  const close = async () => {
    const window = getCurrentWindow()
    await window.close()
  }

  return (
    <div className="title-bar" data-tauri-drag-region>
      <div className="title-bar-title">
        NoteFeratu
      </div>
      <div className="title-bar-controls">
        <button onClick={minimize}>−</button>
        <button onClick={toggleMaximize}>
          {isMaximized ? '❐' : '□'}
        </button>
        <button onClick={close} className="close-btn">×</button>
      </div>
    </div>
  )
}
