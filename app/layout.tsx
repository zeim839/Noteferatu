import type { Metadata } from "next"
import { LayoutProvider, Layout } from "@/components/layout"
import "./globals.css"
import { EditorBackgroundProvider } from "@/components/editor/background";

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
        <LayoutProvider>
          <Layout>
            {children}
          </Layout>
        </LayoutProvider>
      </EditorBackgroundProvider>
      </body>
    </html>
  )
}
