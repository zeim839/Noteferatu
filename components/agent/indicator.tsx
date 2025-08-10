import * as React from "react"

function MessageLoadingIndicator() {
  const [loadingText, setLoadingText] = React.useState<string>(".")
  React.useEffect(() => {
    const intervalId = setInterval(() => {
      setLoadingText(prev => (prev.length >= 3 ? "." : prev + "."));
    }, 300)
    return () => clearInterval(intervalId)
  }, [])
  return (
    <p className="text-sm my-4 mx-2"> { loadingText } </p>
  )
}

export { MessageLoadingIndicator }
