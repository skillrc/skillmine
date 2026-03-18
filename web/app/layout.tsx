import './globals.css'

export const metadata = {
  title: 'Skillmine - Public Alpha Skill Lifecycle for AI Skills',
  description: 'Public alpha for create, register, install, sync, and doctor workflows across Claude Code and OpenCode.',
  keywords: ['skillmine', 'AI skills', 'skill lifecycle', 'claude code', 'opencode', 'AI coding assistant', 'public alpha'],
  metadataBase: new URL('https://skillmine-app.vercel.app'),
  authors: [{ name: 'Skillmine' }],
  openGraph: {
    title: 'Skillmine - Public Alpha Skill Lifecycle for AI Skills',
    description: 'Public alpha for create, register, install, sync, and doctor AI coding assistant skills.',
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
