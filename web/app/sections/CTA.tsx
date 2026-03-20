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
      className="section relative overflow-hidden"
    >
      <div className="absolute inset-0 pointer-events-none">
        <div 
          className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[1200px] h-[800px] rounded-full animate-pulse-glow"
          style={{
            background: 'radial-gradient(circle, rgba(232, 180, 180, 0.12) 0%, transparent 60%)',
            filter: 'blur(120px)',
          }}
        />
        
        <div 
          className="absolute top-1/4 right-1/4 w-[600px] h-[600px] rounded-full animate-aurora-medium"
          style={{
            background: 'radial-gradient(circle, rgba(180, 200, 232, 0.06) 0%, transparent 60%)',
            filter: 'blur(80px)',
          }}
        />
        
        <div 
          className="absolute bottom-1/4 left-1/4 w-[500px] h-[500px] rounded-full animate-aurora-fast"
          style={{
            background: 'radial-gradient(circle, rgba(232, 200, 180, 0.04) 0%, transparent 60%)',
            filter: 'blur(60px)',
          }}
        />
        
        <div 
          className="absolute top-1/3 left-1/3 w-64 h-64 glass-orb opacity-40 animate-float"
        />
        <div 
          className="absolute bottom-1/3 right-1/3 w-48 h-48 glass-orb opacity-30 animate-float-delayed"
        />
      </div>

      <div className="container relative z-10">
        <div className="max-w-4xl mx-auto">
          <div 
            className={`text-center mb-16 transition-all duration-1000 ${
              isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'
            }`}
          >
            <h2 
              className={`font-serif text-[clamp(3rem,7vw,5rem)] leading-[1] tracking-[-0.03em] text-white mb-8 transition-all duration-1000 delay-100 ${
                isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'
              }`}
            >
              Ready to mine
              <br />
              some{' '}
              <span className="text-gradient-aurora italic">skills</span>?
            </h2>
            
            <p 
              className={`text-xl text-white/40 max-w-2xl mx-auto leading-relaxed transition-all duration-1000 delay-200 ${
                isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'
              }`}
            >
              Install Skillmine and run the full loop from{' '}
              <code className="px-2 py-1 rounded-lg bg-aurora-500/10 border border-aurora-300/20 text-aurora-300 text-sm font-mono">create</code>
              {' '}to{' '}
              <code className="px-2 py-1 rounded-lg bg-aurora-500/10 border border-aurora-300/20 text-aurora-300 text-sm font-mono">doctor</code>
              {' '}with one CLI.
            </p>
          </div>

          <div 
            className={`mb-12 transition-all duration-1000 delay-300 ${
              isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'
            }`}
          >
            <div className="relative p-10 lg:p-12 rounded-[2.5rem] liquid-glass-premium premium-border"
            >
              <div 
                className="absolute -inset-4 rounded-[3rem] animate-pulse-glow -z-10"
                style={{
                  background: 'radial-gradient(circle, rgba(232, 180, 180, 0.08) 0%, transparent 70%)',
                  filter: 'blur(40px)',
                }}
              />
              
              <div className="flex items-center gap-3 mb-8">
                <span className="flex h-3 w-3 relative">
                  <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-aurora-400 opacity-75"></span>
                  <span className="relative inline-flex rounded-full h-3 w-3 bg-aurora-300"></span>
                </span>
                <span className="font-semibold text-white">Public Alpha Status</span>
              </div>

              <div className="grid md:grid-cols-2 gap-8">
                <div>
                  <div className="mb-4">
                    <span className="text-xs uppercase tracking-widest text-white/40">Install via Cargo</span>
                  </div>
                  
                  <div className="group relative">
                    <div className="flex items-center gap-4 px-6 py-5 rounded-2xl bg-black/40 border border-white/10 font-mono text-base"
                    >
                      <span className="text-aurora-300 font-bold">$</span>
                      <span className="text-white/90">cargo install skillmine</span>
                      
                      <button 
                        className="ml-auto opacity-0 group-hover:opacity-100 transition-all duration-300 text-white/30 hover:text-aurora-300 p-2 rounded-lg hover:bg-white/5"
                        onClick={() => navigator.clipboard.writeText('cargo install skillmine')}
                        title="Copy to clipboard"
                      >
                        <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                        </svg>
                      </button>
                    </div>
                  </div>
                </div>

                <div className="space-y-4">
                  <div className="flex items-start gap-3">
                    <div className="w-2 h-2 rounded-full bg-aurora-300 mt-2 flex-shrink-0"></div>
                    <div>
                      <div className="text-white/80 text-sm font-medium mb-1">Supported Runtime Targets</div>
                      <div className="text-white/40 text-sm">Claude Code and OpenCode</div>
                    </div>
                  </div>
                  
                  <div className="flex items-start gap-3">
                    <div className="w-2 h-2 rounded-full bg-aurora-300/60 mt-2 flex-shrink-0"></div>
                    <div>
                      <div className="text-white/80 text-sm font-medium mb-1">Known Limitations</div>
                      <div className="text-white/40 text-sm">No Cursor target in this alpha. Website is documentation-only.</div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
          
          <div 
            className={`flex flex-wrap justify-center gap-4 transition-all duration-1000 delay-400 ${
              isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'
            }`}
          >
            <a 
              href="#how-it-works" 
              className="group inline-flex items-center gap-3 px-8 py-4 rounded-2xl bg-gradient-to-r from-aurora-300 to-aurora-400 text-obsidian-950 font-semibold text-base transition-all duration-300 hover:shadow-[0_0_40px_rgba(232,180,180,0.3)] hover:scale-[1.02]"
            >
              <span>Get Started</span>
              <svg 
                className="w-5 h-5 transition-transform duration-300 group-hover:translate-x-1" 
                fill="none" 
                viewBox="0 0 24 24" 
                stroke="currentColor"
              >
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 8l4 4m0 0l-4 4m4-4H3" />
              </svg>
            </a>
            
            <a 
              href="https://skillmine-app.vercel.app/"
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center gap-3 px-8 py-4 rounded-2xl liquid-glass-premium text-white font-medium text-base transition-all duration-300 hover:scale-[1.02]"
            >
              <span>Live Site</span>
            </a>
            
            <a 
              href="https://github.com/skillrc/skillmine" 
              target="_blank" 
              rel="noopener noreferrer"
              className="inline-flex items-center gap-3 px-8 py-4 rounded-2xl liquid-glass-premium text-white font-medium text-base transition-all duration-300 hover:scale-[1.02]"
            >
              <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
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
