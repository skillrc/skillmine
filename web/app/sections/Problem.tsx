'use client'

import { useEffect, useRef, useState } from 'react'

const problems = [
  {
    icon: (
      <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
        <path strokeLinecap="round" strokeLinejoin="round" d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z" />
      </svg>
    ),
    title: 'Skill Chaos',
    description: 'AI skills scattered across different assistants, versions, and locations. No single source of truth for your workflow enhancements.',
  },
  {
    icon: (
      <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
        <path strokeLinecap="round" strokeLinejoin="round" d="M3.75 13.5l10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75z" />
      </svg>
    ),
    title: 'Version Drift',
    description: 'Skills get updated silently. Your carefully tuned prompts break overnight. No way to lock to known-good versions or track changes.',
  },
  {
    icon: (
      <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
        <path strokeLinecap="round" strokeLinejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />
      </svg>
    ),
    title: 'Manual Sync Hell',
    description: 'Copying files between Claude Code and OpenCode. Updating one assistant still means repeating runtime setup by hand.',
  },
  {
    icon: (
      <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
        <path strokeLinecap="round" strokeLinejoin="round" d="M12 6v6h4.5m4.5 0a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
    ),
    title: 'No Visibility',
    description: 'No way to see what skills are installed, which are outdated, or diagnose why something broke. Blind debugging.',
  },
]

export default function Problem() {
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
      { threshold: 0.15 }
    )

    if (sectionRef.current) {
      observer.observe(sectionRef.current)
    }

    return () => observer.disconnect()
  }, [])

  return (
    <section 
      id="problem"
      ref={sectionRef}
      className="section relative overflow-hidden obsidian-depth"
    >
      <div className="absolute inset-0 pointer-events-none">
        <div 
          className="absolute -top-1/4 -right-1/4 w-[800px] h-[800px] rounded-full animate-aurora-slow"
          style={{
            background: 'radial-gradient(circle, rgba(232, 180, 180, 0.06) 0%, transparent 60%)',
            filter: 'blur(100px)',
          }}
        />
        <div 
          className="absolute bottom-0 left-0 w-[600px] h-[600px] rounded-full animate-aurora-medium"
          style={{
            background: 'radial-gradient(circle, rgba(180, 200, 232, 0.04) 0%, transparent 60%)',
            filter: 'blur(80px)',
          }}
        />
        <div 
          className="absolute top-1/2 left-1/3 w-[500px] h-[500px] rounded-full animate-aurora-fast"
          style={{
            background: 'radial-gradient(circle, rgba(232, 200, 180, 0.03) 0%, transparent 60%)',
            filter: 'blur(60px)',
          }}
        />
      </div>

      <div className="container relative z-10">
        <div className="grid lg:grid-cols-12 gap-12 lg:gap-16 items-start">
          <div className="lg:col-span-5 lg:sticky lg:top-32">
            <div 
              className={`transition-all duration-1000 ${
                isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'
              }`}
            >
              <span className="section-label">
                <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                </svg>
                The Problem
              </span>
            </div>
            
            <h2 
              className={`font-serif text-[clamp(2.5rem,5vw,4rem)] leading-[1.1] tracking-[-0.02em] text-white mb-6 transition-all duration-1000 delay-100 ${
                isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'
              }`}
            >
              Managing AI skills is{' '}
              <span className="text-gradient-aurora italic">a mess</span>
            </h2>
            
            <p 
              className={`text-lg text-white/40 leading-relaxed max-w-md transition-all duration-1000 delay-200 ${
                isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'
              }`}
            >
              You have powerful AI assistants, but no good way to create, organize, version, and sync the skills that make them useful.
            </p>
          </div>

          <div className="lg:col-span-7">
            <div className="grid gap-6">
              {problems.map((problem, idx) => (
                <div
                  key={idx}
                  className={`group relative transition-all duration-1000 ${
                    isVisible ? 'opacity-100 translate-x-0' : 'opacity-0 translate-x-12'
                  }`}
                  style={{ transitionDelay: isVisible ? `${300 + idx * 150}ms` : '0ms' }}
                >
                  <div 
                    className={`relative p-8 rounded-3xl liquid-glass-premium premium-border ${
                      idx % 2 === 0 ? 'lg:ml-0 lg:mr-12' : 'lg:ml-12 lg:mr-0'
                    }`}
                  >
                    <div className="flex items-start gap-6">
                      <div className="flex-shrink-0">
                        <div className="w-14 h-14 rounded-2xl bg-gradient-to-br from-aurora-300/20 to-aurora-500/10 border border-aurora-300/20 flex items-center justify-center text-aurora-300 transition-all duration-500 group-hover:scale-110 group-hover:shadow-[0_0_30px_rgba(232,180,180,0.2)]">
                          {problem.icon}
                        </div>
                      </div>
                      
                      <div className="flex-1 min-w-0">
                        <h3 className="font-serif text-xl mb-3 text-white group-hover:text-gradient-aurora transition-all duration-500">
                          {problem.title}
                        </h3>
                        <p className="text-white/40 leading-relaxed text-base">
                          {problem.description}
                        </p>
                      </div>
                    </div>

                    <div className="absolute top-4 right-4 opacity-0 group-hover:opacity-100 transition-opacity duration-500">
                      <div className="w-8 h-8 rounded-full bg-white/5 flex items-center justify-center">
                        <svg className="w-4 h-4 text-white/30" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M4.5 19.5l15-15m0 0H8.25m11.25 0v11.25" />
                        </svg>
                      </div>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>
    </section>
  )
}
