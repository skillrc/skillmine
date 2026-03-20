'use client'

import { useEffect, useRef, useState } from 'react'

const features = [
  {
    title: 'Native Skill Creation',
    description: 'Start the lifecycle with `skillmine create` and keep creation, registration, install, sync, and diagnostics in one tool.',
    icon: (
      <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
        <path strokeLinecap="round" strokeLinejoin="round" d="M12 6v12m6-6H6" />
      </svg>
    ),
    size: 'large',
  },
  {
    title: 'Declarative Configuration',
    description: 'Define your desired state in skills.toml. Git-trackable, reviewable, shareable.',
    icon: (
      <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
        <path strokeLinecap="round" strokeLinejoin="round" d="M9 12h3.75M9 15h3.75M9 18h3.75m3 .75H18a2.25 2.25 0 002.25-2.25V6.108c0-1.135-.845-2.098-1.976-2.192a48.424 48.424 0 00-1.123-.08m-5.801 0c-.065.21-.1.433-.1.664 0 .414.336.75.75.75h4.5a.75.75 0 00.75-.75 2.25 2.25 0 00-.1-.664m-5.8 0A2.251 2.251 0 0113.5 2.25H15c1.012 0 1.867.668 2.15 1.586m-5.8 0c-.376.023-.75.05-1.124.08C9.095 4.01 8.25 4.973 8.25 6.108V8.25m0 0H4.875c-.621 0-1.125.504-1.125 1.125v11.25c0-.621.504-1.125 1.125-1.125h9.75c-.621 0-1.125-.504-1.125-1.125V9.375c0-.621-.504-1.125-1.125-1.125H8.25zM6.75 12h.008v.008H6.75V12zm0 3h.008v.008H6.75V15zm0 3h.008v.008H6.75V18z" />
      </svg>
    ),
    size: 'small',
  },
  {
    title: 'Deterministic State',
    description: 'Content-addressable storage ensures identical inputs produce identical outputs. Reproducible everywhere.',
    icon: (
      <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
        <path strokeLinecap="round" strokeLinejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />
      </svg>
    ),
    size: 'small',
  },
  {
    title: 'Alpha Runtime Support',
    description: 'Sync skills to Claude Code and OpenCode in the current public alpha. One config, supported targets kept explicit.',
    icon: (
      <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
        <path strokeLinecap="round" strokeLinejoin="round" d="M7.5 14.25v2.25m3-4.5v4.5m3-6.75v6.75m3-9v9M6 20.25h12A2.25 2.25 0 0020.25 18V6A2.25 2.25 0 0018 3.75H6A2.25 2.25 0 003.75 6v12A2.25 2.25 0 006 20.25z" />
      </svg>
    ),
    size: 'medium',
  },
  {
    title: 'Version Locking',
    description: 'Lock files pin exact versions. Update when you choose, not when upstream changes.',
    icon: (
      <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
        <path strokeLinecap="round" strokeLinejoin="round" d="M16.5 10.5V6.75a4.5 4.5 0 10-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 002.25-2.25v-6.75a2.25 2.25 0 00-2.25-2.25H6.75a2.25 2.25 0 00-2.25 2.25v6.75a2.25 2.25 0 002.25 2.25z" />
      </svg>
    ),
    size: 'small',
  },
  {
    title: 'Drift Detection',
    description: 'skillmine doctor validates state and detects drift. Know when things change.',
    icon: (
      <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
        <path strokeLinecap="round" strokeLinejoin="round" d="M10.5 6a7.5 7.5 0 107.5 7.5h-7.5V6z" />
        <path strokeLinecap="round" strokeLinejoin="round" d="M13.5 10.5H21A7.5 7.5 0 0013.5 3v7.5z" />
      </svg>
    ),
    size: 'small',
  },
  {
    title: 'Git-Native Workflow',
    description: 'Skills are just Git repos. Branch, tag, fork, and PR your AI workflows like code.',
    icon: (
      <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth={1.5}>
        <path strokeLinecap="round" strokeLinejoin="round" d="M17.25 6.75L22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3l-4.5 16.5" />
      </svg>
    ),
    size: 'medium',
  },
]

export default function WhySkillmine() {
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
      { threshold: 0.1 }
    )

    if (sectionRef.current) {
      observer.observe(sectionRef.current)
    }

    return () => observer.disconnect()
  }, [])

  return (
    <section 
      ref={sectionRef}
      className="section relative overflow-hidden obsidian-depth"
    >
      <div className="absolute inset-0 pointer-events-none">
        <div 
          className="absolute top-0 right-0 w-[900px] h-[900px] rounded-full animate-aurora-slow"
          style={{
            background: 'radial-gradient(circle, rgba(232, 180, 180, 0.06) 0%, transparent 60%)',
            filter: 'blur(120px)',
          }}
        />
        
        <div 
          className="absolute bottom-0 left-0 w-[700px] h-[700px] rounded-full animate-aurora-medium"
          style={{
            background: 'radial-gradient(circle, rgba(180, 200, 232, 0.04) 0%, transparent 60%)',
            filter: 'blur(100px)',
          }}
        />
        
        <div 
          className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[500px] h-[500px] rounded-full animate-aurora-fast"
          style={{
            background: 'radial-gradient(circle, rgba(232, 200, 180, 0.03) 0%, transparent 60%)',
            filter: 'blur(80px)',
          }}
        />
        
        <div 
          className="absolute top-1/4 left-1/4 w-40 h-40 glass-orb opacity-25 animate-float"
        />
        <div 
          className="absolute bottom-1/3 right-1/3 w-32 h-32 glass-orb opacity-20 animate-float-delayed"
        />
      </div>

      <div className="container relative z-10">
        <div className="grid lg:grid-cols-2 gap-16 lg:gap-24 items-center mb-20">
          <div>
            <div 
              className={`transition-all duration-1000 ${
                isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'
              }`}
            >
              <span className="section-label">
                <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z" />
                </svg>
                Why Skillmine
              </span>
            </div>
            
            <h2 
              className={`font-serif text-[clamp(2.5rem,5vw,4rem)] leading-[1.1] tracking-[-0.02em] text-white mb-6 transition-all duration-1000 delay-100 ${
                isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'
              }`}
            >
              Built for{' '}
              <span className="text-gradient-aurora italic">AI-native</span> workflows
            </h2>
            
            <p 
              className={`text-lg text-white/40 leading-relaxed max-w-lg transition-all duration-1000 delay-200 ${
                isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'
              }`}
            >
              Not just downstream install and sync. A public alpha lifecycle for creating, managing, shipping, and validating AI skills.
            </p>
          </div>

          <div 
            className={`hidden lg:block transition-all duration-1000 delay-300 ${
              isVisible ? 'opacity-100 translate-x-0' : 'opacity-0 translate-x-12'
            }`}
          >
            <div className="relative">
              <div 
                className="absolute -inset-8 rounded-[3rem] animate-pulse-glow"
                style={{
                  background: 'radial-gradient(circle, rgba(232, 180, 180, 0.1) 0%, transparent 70%)',
                  filter: 'blur(40px)',
                }}
              />
              
              <div className="relative p-8 rounded-3xl liquid-glass-premium premium-border">
                <div className="grid grid-cols-3 gap-4 mb-6">
                  {[
                    { value: '12+', label: 'Commands' },
                    { value: '2', label: 'Targets' },
                    { value: 'Rust', label: 'Built with' },
                  ].map((stat, idx) => (
                    <div key={idx} className="text-center">
                      <div className="font-serif text-2xl text-white mb-1">{stat.value}</div>
                      <div className="text-xs text-white/40 uppercase tracking-wider">{stat.label}</div>
                    </div>
                  ))}
                </div>
                
                <div className="space-y-2">
                  {['skillmine create', 'skillmine install', 'skillmine doctor'].map((cmd, idx) => (
                    <div 
                      key={cmd}
                      className="flex items-center gap-3 px-4 py-3 rounded-xl bg-white/[0.02] border border-white/5"
                    >
                      <span className="text-aurora-300 font-mono text-sm">$</span>
                      <span className="text-white/60 font-mono text-sm">{cmd}</span>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          </div>
        </div>

        <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
          {features.map((feature, idx) => (
            <div
              key={idx}
              className={`group transition-all duration-1000 ${
                feature.size === 'large' ? 'md:col-span-2 lg:col-span-2 lg:row-span-2' : ''
              } ${isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-12'}`}
              style={{ transitionDelay: isVisible ? `${400 + idx * 100}ms` : '0ms' }}
            >
              <div 
                className={`h-full p-8 rounded-3xl liquid-glass-premium premium-border transition-all duration-500 group-hover:scale-[1.02] ${
                  feature.size === 'large' ? 'flex flex-col justify-between min-h-[320px]' : ''
                }`}
              >
                <div>
                  <div 
                    className={`flex items-center justify-center w-12 h-12 rounded-2xl bg-gradient-to-br from-aurora-300/20 to-aurora-500/10 border border-aurora-300/20 text-aurora-300 mb-6 transition-all duration-500 group-hover:scale-110 group-hover:shadow-[0_0_30px_rgba(232,180,180,0.2)] ${
                      feature.size === 'large' ? 'w-16 h-16' : ''
                    }`}
                  >
                    {feature.icon}
                  </div>
                  
                  <h3 
                    className={`font-serif text-white mb-3 group-hover:text-gradient-aurora transition-all duration-500 ${
                      feature.size === 'large' ? 'text-2xl lg:text-3xl' : 'text-lg'
                    }`}
                  >
                    {feature.title}
                  </h3>                  
                  <p className={`text-white/40 leading-relaxed ${
                    feature.size === 'large' ? 'text-lg max-w-md' : 'text-sm'
                  }`}>
                    {feature.description}
                  </p>
                </div>

                {feature.size === 'large' && (
                  <div className="mt-8 flex items-center gap-4">
                    <div className="flex -space-x-2">
                      {['Claude', 'OpenCode'].map((target, i) => (
                        <div 
                          key={target}
                          className="w-10 h-10 rounded-full bg-gradient-to-br from-white/10 to-white/5 border border-white/10 flex items-center justify-center text-xs font-medium text-white/80"
                        >
                          {target[0]}
                        </div>
                      ))}
                    </div>
                    <span className="text-sm text-white/40">Supported targets</span>
                  </div>
                )}
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}
