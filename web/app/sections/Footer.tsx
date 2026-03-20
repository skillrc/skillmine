'use client'

import { useEffect, useRef, useState } from 'react'

const footerLinks = [
  {
    title: 'Product',
    items: [
      { label: 'README', href: 'https://github.com/skillrc/skillmine#readme' },
      { label: 'Issues', href: 'https://github.com/skillrc/skillmine/issues' },
      { label: 'Live Site', href: 'https://skillmine-app.vercel.app/' },
    ],
  },
  {
    title: 'Community',
    items: [
      { label: 'GitHub', href: 'https://github.com/skillrc/skillmine' },
      { label: 'Bug Backlog', href: 'https://github.com/skillrc/skillmine/blob/main/docs/bugs.md' },
      { label: 'Public Alpha', href: 'https://github.com/skillrc/skillmine#readme' },
    ],
  },
  {
    title: 'Legal',
    items: [
      { label: 'License', href: 'https://github.com/skillrc/skillmine/blob/main/LICENSE' },
      { label: 'Repository', href: 'https://github.com/skillrc/skillmine' },
      { label: 'Alpha Scope', href: 'https://github.com/skillrc/skillmine#readme' },
    ],
  },
]

export default function Footer() {
  const footerRef = useRef<HTMLElement>(null)
  const [isVisible, setIsVisible] = useState(false)

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          setIsVisible(true)
          observer.unobserve(entry.target)
        }
      },
      { threshold: 0.1 }
    )

    if (footerRef.current) {
      observer.observe(footerRef.current)
    }

    return () => observer.disconnect()
  }, [])

  return (
    <footer 
      ref={footerRef}
      className="bg-obsidian border-t border-white/5"
    >
      <div className="container py-16 lg:py-20">
        <div 
          className={`grid md:grid-cols-2 lg:grid-cols-4 gap-12 transition-all duration-700 ${
            isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-6'
          }`}
        >
          <div className="lg:col-span-1">
            <div className="flex items-center gap-3 mb-5">
              <span className="text-3xl">⛏</span>
              <span className="text-2xl font-bold gradient-text">Skillmine</span>
            </div>
            <p className="text-text-secondary text-sm leading-relaxed max-w-xs mb-6">
              Public alpha for the closed-loop lifecycle of AI coding assistant skills, from local creation to runtime sync and diagnostics.
            </p>
            
            <div className="flex gap-3">
              <a 
                href="https://github.com/skillrc/skillmine" 
                target="_blank" 
                rel="noopener noreferrer"
                className="w-10 h-10 rounded-xl glass glass-border flex items-center justify-center text-text-muted hover:text-text-primary hover:border-brand-orange/30 transition-all duration-200"
                aria-label="GitHub"
              >
                <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                </svg>
              </a>
            </div>
          </div>
          
          {footerLinks.map((group, idx) => (
            <div 
              key={idx}
              className={`transition-all duration-700 ${
                isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-6'
              }`}
              style={{ transitionDelay: isVisible ? `${100 + idx * 100}ms` : '0ms' }}
            >
              <h4 className="font-semibold text-text-primary mb-4 text-sm uppercase tracking-wider">
                {group.title}
              </h4>
              <ul className="space-y-3">
                {group.items.map((item, itemIdx) => (
                  <li key={itemIdx}>
                    <a 
                      href={item.href}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-text-secondary hover:text-brand-orange transition-colors duration-200 text-sm"
                    >
                      {item.label}
                    </a>
                  </li>
                ))}
              </ul>
            </div>
          ))}
        </div>
        
        <div 
          className={`border-t border-white/5 mt-12 pt-8 transition-all duration-700 delay-300 ${
            isVisible ? 'opacity-100' : 'opacity-0'
          }`}
        >
          <div className="flex flex-col md:flex-row justify-between items-center gap-4">
            <p className="text-text-muted text-xs text-center md:text-left">
              MIT License · Built with Rust · Public alpha supports Claude Code and OpenCode
            </p>
            <p className="text-text-muted text-xs">
              © {new Date().getFullYear()} Skillmine
            </p>
          </div>
        </div>
      </div>
    </footer>
  )
}
