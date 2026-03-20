'use client'

import { useEffect, useRef, useState } from 'react'

export default function CTA() {
  const sectionRef = useRef<HTMLElement>(null)
  const [isVisible, setIsVisible] = useState(false)

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          setIsVisible(true)
          observer.unobserve(entry.target)
        }
      },
      { threshold: 0.2 }
    )

    if (sectionRef.current) {
      observer.observe(sectionRef.current)
    }

    return () => observer.disconnect()
  }, [])

  return (
    <section 
      id="install" 
      ref={sectionRef}
      className="section bg-deep-slate relative overflow-hidden"
    >
      <div className="absolute inset-0 gradient-hero-intense" />
      
      <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[800px] h-[600px] bg-brand-orange/10 rounded-full blur-[150px] animate-pulse-glow" />
      
      <div className="container relative z-10">
        <div className="max-w-3xl mx-auto text-center">
          <h2 
            className={`text-4xl lg:text-5xl font-bold mb-6 tracking-[-0.02em] transition-all duration-700 ${
              isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-6'
            }`}
          >
            Ready to mine some{' '}
            <span className="gradient-text">skills</span>?
          </h2>
          
          <p 
            className={`text-lg lg:text-xl text-text-secondary mb-10 transition-all duration-700 delay-100 ${
              isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-6'
            }`}
          >
            Install Skillmine and run the full loop from{' '}
            <code className="px-2 py-0.5 rounded bg-brand-orange/10 text-brand-orange text-sm">create</code>
            {' '}to{' '}
            <code className="px-2 py-0.5 rounded bg-brand-orange/10 text-brand-orange text-sm">doctor</code>
            {' '}with one CLI.
          </p>

          <div 
            className={`mb-8 rounded-2xl glass glass-border p-6 text-left transition-all duration-700 delay-200 ${
              isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-6'
            }`}
          >
            <div className="flex items-center gap-2 mb-3">
              <span className="flex h-2 w-2">
                <span className="animate-ping absolute inline-flex h-2 w-2 rounded-full bg-brand-orange opacity-75"></span>
                <span className="relative inline-flex rounded-full h-2 w-2 bg-brand-orange"></span>
              </span>
              <span className="font-semibold text-text-primary">Public alpha status</span>
            </div>
            <div className="text-text-secondary text-sm space-y-1">
              <p>Supported runtime targets: Claude Code and OpenCode.</p>
              <p>Known limitations: no Cursor target in this alpha, and the website is documentation-only.</p>
            </div>
          </div>
          
          <div 
            className={`code-block p-6 text-left mb-10 max-w-2xl mx-auto transition-all duration-700 delay-300 ${
              isVisible ? 'opacity-100 translate-y-0 scale-100' : 'opacity-0 translate-y-6 scale-95'
            }`}
          >
            <div className="flex items-center justify-between mb-4 pb-4 border-b border-white/5">
              <span className="text-text-muted text-sm">Install via cargo</span>
              <button 
                className="text-text-muted hover:text-text-primary transition-colors"
                onClick={() => navigator.clipboard.writeText('cargo install skillmine')}
                title="Copy to clipboard"
              >
                <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                </svg>
              </button>
            </div>
            <div className="font-mono">
              <span className="text-brand-orange font-semibold">$</span>{' '}
              <span className="text-cyan-bright">cargo install skillmine</span>
            </div>
          </div>
          
          <div 
            className={`flex flex-wrap justify-center gap-4 transition-all duration-700 delay-400 ${
              isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-6'
            }`}
          >
            <a href="#how-it-works" className="btn-primary">
              <span>Get Started</span>
              <svg className="w-4 h-4 inline-block ml-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 8l4 4m0 0l-4 4m4-4H3" />
              </svg>
            </a>
            
            <a 
              href="https://skillmine-app.vercel.app/"
              target="_blank"
              rel="noopener noreferrer"
              className="btn-secondary"
            >
              <span>Live Site</span>
            </a>
            
            <a 
              href="https://github.com/skillrc/skillmine" 
              target="_blank" 
              rel="noopener noreferrer"
              className="btn-secondary inline-flex items-center"
            >
              <svg className="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 24 24">
                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
              </svg>
              <span>GitHub</span>
            </a>
          </div>
        </div>
      </div>
    </section>
  )
}
