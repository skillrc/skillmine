'use client'

import { useEffect, useState, useRef } from 'react'
import Terminal from '../../components/Terminal'

export default function Hero() {
  const [isVisible, setIsVisible] = useState(false)
  const [mousePos, setMousePos] = useState({ x: 50, y: 50 })
  const containerRef = useRef<HTMLElement>(null)

  useEffect(() => {
    setIsVisible(true)
  }, [])

  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      if (containerRef.current) {
        const rect = containerRef.current.getBoundingClientRect()
        const x = ((e.clientX - rect.left) / rect.width) * 100
        const y = ((e.clientY - rect.top) / rect.height) * 100
        setMousePos({ x, y })
      }
    }

    window.addEventListener('mousemove', handleMouseMove, { passive: true })
    return () => window.removeEventListener('mousemove', handleMouseMove)
  }, [])

  const commands = [
    { cmd: 'init', desc: 'Initialize' },
    { cmd: 'create', desc: 'Create' },
    { cmd: 'add', desc: 'Register' },
    { cmd: 'install', desc: 'Install' },
    { cmd: 'sync', desc: 'Sync' },
    { cmd: 'doctor', desc: 'Diagnose' },
  ]

  return (
    <section 
      ref={containerRef}
      className="relative min-h-screen flex items-center overflow-hidden obsidian-depth"
      style={{ '--mouse-x': `${mousePos.x}%`, '--mouse-y': `${mousePos.y}%` } as React.CSSProperties}
    >
      <div className="absolute inset-0 pointer-events-none overflow-hidden">
        <div 
          className="absolute -top-1/4 -left-1/4 w-[1200px] h-[1200px] rounded-full"
          style={{
            background: 'radial-gradient(circle, rgba(232, 180, 180, 0.12) 0%, transparent 60%)',
            filter: 'blur(120px)',
            transform: `translate(${(mousePos.x - 50) * 0.02}%, ${(mousePos.y - 50) * 0.02}%)`,
            transition: 'transform 0.3s ease-out',
          }}
        />
        <div 
          className="absolute top-1/3 -right-1/4 w-[900px] h-[900px] rounded-full animate-aurora-medium"
          style={{
            background: 'radial-gradient(circle, rgba(180, 200, 232, 0.08) 0%, transparent 60%)',
            filter: 'blur(100px)',
          }}
        />
        <div 
          className="absolute -bottom-1/4 left-1/4 w-[1000px] h-[1000px] rounded-full animate-aurora-fast"
          style={{
            background: 'radial-gradient(circle, rgba(232, 200, 180, 0.06) 0%, transparent 60%)',
            filter: 'blur(110px)',
          }}
        />
        
        <div 
          className="absolute top-1/4 right-1/4 w-64 h-64 glass-orb opacity-60"
          style={{
            transform: `translate(${(mousePos.x - 50) * -0.03}%, ${(mousePos.y - 50) * -0.03}%)`,
            transition: 'transform 0.5s ease-out',
          }}
        />
        <div 
          className="absolute bottom-1/3 left-1/5 w-48 h-48 glass-orb opacity-40"
          style={{
            animationDelay: '-5s',
            transform: `translate(${(mousePos.x - 50) * 0.02}%, ${(mousePos.y - 50) * 0.02}%)`,
            transition: 'transform 0.4s ease-out',
          }}
        />
      </div>

      <div 
        className="absolute inset-0 pointer-events-none"
        style={{
          background: 'radial-gradient(ellipse 100% 60% at 50% -20%, rgba(232, 180, 180, 0.08), transparent 60%)',
        }}
      />

      <div className="container relative z-10 py-24 lg:py-32">
        <div className="grid lg:grid-cols-12 gap-12 lg:gap-8 items-start">
          
          <div className="lg:col-span-7 space-y-8 lg:pt-12">
            
            <div 
              className={`inline-flex items-center gap-3 px-5 py-2.5 rounded-full liquid-glass-premium transition-all duration-1000 ${
                isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-4'
              }`}
            >
              <span className="flex h-2.5 w-2.5 relative">
                <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-aurora-400 opacity-75"></span>
                <span className="relative inline-flex rounded-full h-2.5 w-2.5 bg-aurora-300"></span>
              </span>
              <span className="text-sm font-medium tracking-wide text-white/80">Public Alpha</span>
              <span className="text-xs text-white/40 px-2 py-0.5 rounded-full bg-white/5">v0.1</span>
            </div>

            <div className="space-y-2">
              <h1 
                className={`font-serif text-[clamp(3.5rem,12vw,10rem)] leading-[0.85] tracking-[-0.04em] font-medium transition-all duration-1000 delay-100 ${
                  isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'
                }`}
              >
                <span className="text-white">Skill</span>
                <span className="text-gradient-aurora italic">mine</span>
              </h1>
              
              <p 
                className={`font-serif text-[clamp(1.5rem,4vw,2.5rem)] leading-[1.2] tracking-[-0.02em] text-white/50 transition-all duration-1000 delay-200 ${
                  isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'
                }`}
              >
                The native lifecycle for
                <br />
                <span className="text-white/70">AI assistant skills</span>
              </p>
            </div>


            <p 
              className={`text-lg lg:text-xl text-white/40 max-w-lg leading-relaxed font-light transition-all duration-1000 delay-300 ${
                isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'
              }`}
            >
              Create, register, install, sync, and diagnose skills across 
              <span className="text-white/60"> Claude Code</span> and 
              <span className="text-white/60"> OpenCode</span>. 
              Built in Rust. Powered by Git.
            </p>


            <div 
              className={`flex flex-wrap items-center gap-4 pt-4 transition-all duration-1000 delay-400 ${
                isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'
              }`}
            >
              <a 
                href="#install" 
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
                href="https://github.com/skillrc/skillmine" 
                target="_blank" 
                rel="noopener noreferrer" 
                className="inline-flex items-center gap-3 px-6 py-4 rounded-2xl liquid-glass-premium text-white font-medium text-base transition-all duration-300 hover:scale-[1.02]"
              >
                <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                </svg>
                <span>GitHub</span>
              </a>
            </div>


            <div 
              className={`pt-8 transition-all duration-1000 delay-500 ${
                isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'
              }`}
            >
              <div className="flex flex-wrap gap-3">
                {commands.map((item, idx) => (
                  <div 
                    key={item.cmd}
                    className="group relative px-4 py-2.5 rounded-xl liquid-glass-premium cursor-default overflow-hidden"
                    style={{ animationDelay: `${idx * 100}ms` }}
                  >
                    <div className="absolute inset-0 bg-gradient-to-r from-aurora-300/0 via-aurora-300/10 to-aurora-300/0 translate-x-[-100%] group-hover:translate-x-[100%] transition-transform duration-700" />
                    <div className="relative flex items-center gap-2">
                      <span className="text-aurora-300 font-mono text-sm font-semibold">$</span>
                      <span className="text-white/90 font-mono text-sm">{item.cmd}</span>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>


          <div className="lg:col-span-5 lg:col-start-8 lg:pt-8">
            <div 
              className={`relative transition-all duration-1200 delay-300 ${
                isVisible ? 'opacity-100 translate-x-0' : 'opacity-0 translate-x-12'
              }`}
            >

              <div 
                className="absolute -inset-10 rounded-[3rem] animate-pulse-glow"
                style={{
                  background: 'radial-gradient(circle, rgba(232, 180, 180, 0.2) 0%, transparent 70%)',
                  filter: 'blur(60px)',
                }}
              />
              

              <div className="absolute -top-8 -right-8 w-32 h-32 glass-orb opacity-50" />
              <div className="absolute -bottom-6 -left-6 w-24 h-24 glass-orb opacity-30" style={{ animationDelay: '-8s' }} />
              

              <div className="relative">
                <Terminal />
              </div>


              <div className="mt-6 grid grid-cols-3 gap-3">
                {[
                  { label: 'Commands', value: '12+' },
                  { label: 'Targets', value: '2' },
                  { label: 'Language', value: 'Rust' },
                ].map((stat, idx) => (
                  <div 
                    key={stat.label}
                    className="px-4 py-3 rounded-xl liquid-glass-premium text-center"
                    style={{ animationDelay: `${600 + idx * 100}ms` }}
                  >
                    <div className="text-lg font-semibold text-white">{stat.value}</div>
                    <div className="text-xs text-white/40 uppercase tracking-wider">{stat.label}</div>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </div>
      </div>


      <div className="absolute bottom-8 left-1/2 -translate-x-1/2">
        <a 
          href="#problem" 
          className="group flex flex-col items-center gap-2 text-white/30 hover:text-white/60 transition-colors duration-300"
        >
          <span className="text-xs font-medium uppercase tracking-[0.2em]">Explore</span>
          <svg 
            className="w-5 h-5 animate-bounce" 
            fill="none" 
            viewBox="0 0 24 24" 
            stroke="currentColor"
          >
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M19 14l-7 7m0 0l-7-7m7 7V3" />
          </svg>
        </a>
      </div>
    </section>
  )
}
