"use client"

import Titlebar from "@/components/Titlebar"
import "./globals.css"

export default function RootLayout({children}: Readonly<{children: React.ReactNode}>) {
  return (
    <html lang="en">
      <body>
        <Titlebar />
        {children}
      </body>
    </html>
  )
}
