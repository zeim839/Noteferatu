import type { Metadata } from "next"
import { LayoutProvider, Layout } from "@/components/layout"
import { EditorBackgroundProvider } from "@/components/background"
import "./globals.css"
import { DatabaseProvider } from "@/components/DatabaseProvider"

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
        <EditorBackgroundProvider>
          <DatabaseProvider>
            <LayoutProvider>
              <Layout>
                {children}
              </Layout>
            </LayoutProvider>
          </DatabaseProvider>
        </EditorBackgroundProvider>
      </body>
    </html>
  )
}
