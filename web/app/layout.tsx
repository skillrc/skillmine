import type { Metadata } from 'next'
import { GeistSans } from 'geist/font/sans'
import { JetBrains_Mono } from 'next/font/google'
import './globals.css'

const jetbrainsMono = JetBrains_Mono({
  subsets: ['latin'],
  variable: '--font-jetbrains-mono',
  display: 'swap',
})

export const metadata: Metadata = {
  title: 'Skillmine - Public Alpha Skill Lifecycle for AI Skills',
  description: 'Public alpha for create, register, install, sync, and doctor workflows across Claude Code and OpenCode.',
  keywords: ['skillmine', 'AI skills', 'skill lifecycle', 'claude code', 'opencode', 'AI coding assistant', 'public alpha'],
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
    <html lang="en" className={`${GeistSans.variable} ${jetbrainsMono.variable}`}>
      <body className="antialiased">
        {children}
      </body>
    </html>
  )
}
