"use client"

import Editor from "@/components/editor/Editor"
import { useLayout } from "@/components/layout"
import { useEffect } from "react"

export default function Page() {
  const { setBackButton } = useLayout()
  useEffect(() => { setBackButton(true) }, [])
  return <Editor />
}