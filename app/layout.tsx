import type { Metadata } from "next"
import { DatabaseProvider } from "@/components/DatabaseProvider"
import Navigation from "@/components/Navigation"
import "./globals.css"

export const metadata: Metadata = {
  title: "NoteFeratu",
  description: "NoteFeratu is a plain-text personal knowledge management system with LLM capabilities",
}

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body>
        <DatabaseProvider>
          <Navigation>
            {children}
          </Navigation>
        </DatabaseProvider>
      </body>
    </html>
  )
}
