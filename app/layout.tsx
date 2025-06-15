"use client"

import "./globals.css"
import { Window, WindowProvider } from "@/components/window/window"

export default function RootLayout({children}: Readonly<{children: React.ReactNode}>) {
  return (
    <html lang="en">
      <body>
        <WindowProvider>
          <Window>
            {children}
          </Window>
        </WindowProvider>
      </body>
    </html>
  )
}
