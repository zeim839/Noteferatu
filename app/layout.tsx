"use client"

import "./globals.css"
import { Window } from "@/components/window/window"

export default function RootLayout({children}: Readonly<{children: React.ReactNode}>) {
  return (
    <html lang="en">
      <body>
        <Window>{children}</Window>
      </body>
    </html>
  )
}
