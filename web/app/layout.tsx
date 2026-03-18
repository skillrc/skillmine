import './globals.css'

export const metadata = {
  title: 'Skillmine - Closed-Loop Lifecycle for AI Skills',
  description: 'Create, register, install, sync, and doctor AI coding assistant skills with deterministic state across Claude Code, OpenCode, and Cursor.',
  keywords: ['skillmine', 'AI skills', 'skill lifecycle', 'claude code', 'opencode', 'cursor', 'AI coding assistant'],
  authors: [{ name: 'Skillmine' }],
  openGraph: {
    title: 'Skillmine - Closed-Loop Lifecycle for AI Skills',
    description: 'Create, register, install, sync, and doctor AI coding assistant skills.',
    type: 'website',
  },
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className="antialiased">
        {children}
      </body>
    </html>
  )
}
