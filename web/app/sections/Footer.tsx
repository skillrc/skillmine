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
      className="relative overflow-hidden border-t border-white/5"
    >
      <div className="absolute inset-0 pointer-events-none">
        <div 
          className="absolute -bottom-1/2 left-1/2 -translate-x-1/2 w-[800px] h-[800px] rounded-full animate-aurora-slow"
          style={{
            background: 'radial-gradient(circle, rgba(232, 180, 180, 0.03) 0%, transparent 60%)',
            filter: 'blur(100px)',
          }}
        />
      </div>

      <div className="container relative z-10 py-20 lg:py-24">
        <div 
          className={`grid md:grid-cols-2 lg:grid-cols-5 gap-12 lg:gap-8 transition-all duration-1000 ${
            isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'
          }`}
        >
          <div className="lg:col-span-2">
            <div className="flex items-center gap-3 mb-6">
              <div className="w-10 h-10 rounded-xl liquid-glass-premium flex items-center justify-center">
                <svg className="w-5 h-5 text-aurora-300" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M21 7.5l-9-5.25L3 7.5m18 0l-9 5.25m9-5.25v9l-9 5.25M3 7.5l9 5.25M3 7.5v9l9 5.25m0-9v9" />
                </svg>
              </div>
              <span className="font-serif text-2xl text-gradient">Skillmine</span>
            </div>
            
            <p className="text-white/40 text-sm leading-relaxed max-w-sm mb-8">
              Public alpha for the closed-loop lifecycle of AI coding assistant skills, from local creation to runtime sync and diagnostics.
            </p>
            
            <div className="flex gap-3">
              <a 
                href="https://github.com/skillrc/skillmine" 
                target="_blank" 
                rel="noopener noreferrer"
                className="w-12 h-12 rounded-2xl liquid-glass-premium flex items-center justify-center text-white/40 hover:text-aurora-300 transition-all duration-300 hover:scale-110"
                aria-label="GitHub"
              >
                <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                </svg>
              </a>
              
              <a 
                href="https://skillmine-app.vercel.app/"
                target="_blank"
                rel="noopener noreferrer"
                className="w-12 h-12 rounded-2xl liquid-glass-premium flex items-center justify-center text-white/40 hover:text-aurora-300 transition-all duration-300 hover:scale-110"
                aria-label="Live Site"
              >
                <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M13.5 6H5.25A2.25 2.25 0 003 8.25v10.5A2.25 2.25 0 005.25 21h10.5A2.25 2.25 0 0018 18.75V10.5m-10.5 6L21 3m0 0h-5.25M21 3v5.25" />
                </svg>
              </a>
            </div>
          </div>
          
          {footerLinks.map((group, idx) => (
            <div 
              key={idx}
              className={`transition-all duration-1000 ${
                isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'
              }`}
              style={{ transitionDelay: isVisible ? `${100 + idx * 100}ms` : '0ms' }}
            >
              <h4 className="font-serif text-white mb-5 text-sm">
                {group.title}
              </h4>
              
              <ul className="space-y-4">
                {group.items.map((item, itemIdx) => (
                  <li key={itemIdx}>
                    <a 
                      href={item.href}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="group inline-flex items-center gap-2 text-white/40 hover:text-aurora-300 transition-all duration-300 text-sm"
                    >
                      <span className="relative">
                        {item.label}
                        <span className="absolute -bottom-0.5 left-0 w-0 h-px bg-aurora-300 transition-all duration-300 group-hover:w-full"></span>
                      </span>
                      
                      <svg 
                        className="w-3 h-3 opacity-0 -translate-x-2 group-hover:opacity-100 group-hover:translate-x-0 transition-all duration-300" 
                        fill="none" 
                        viewBox="0 0 24 24" 
                        stroke="currentColor"
                      >
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4.5 19.5l15-15m0 0H8.25m11.25 0v11.25" />
                      </svg>
                    </a>
                  </li>
                ))}
              </ul>
            </div>
          ))}
        </div>
        
        <div 
          className={`mt-16 pt-8 border-t border-white/5 transition-all duration-1000 delay-300 ${
            isVisible ? 'opacity-100' : 'opacity-0'
          }`}
        >
          <div className="flex flex-col md:flex-row justify-between items-center gap-4">
            <div className="flex flex-wrap items-center gap-4 text-white/30 text-xs">
              <span className="px-3 py-1.5 rounded-full liquid-glass-premium">MIT License</span>
              <span className="px-3 py-1.5 rounded-full liquid-glass-premium">Built with Rust</span>
              <span className="px-3 py-1.5 rounded-full liquid-glass-premium">Public Alpha</span>
            </div>
            
            <p className="text-white/30 text-xs">
              © {new Date().getFullYear()} Skillmine
            </p>
          </div>
        </div>
      </div>
    </footer>
  )
}
