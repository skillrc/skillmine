'use client'

import { useEffect, useRef, useState } from 'react'
import { lifecycleSteps } from '../lib/lifecycle'

export default function HowItWorks() {
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
      id="how-it-works" 
      ref={sectionRef}
      className="section relative overflow-hidden"
    >
      <div className="absolute inset-0 pointer-events-none">
        <div 
          className="absolute top-0 left-1/4 w-[700px] h-[700px] rounded-full animate-aurora-slow"
          style={{
            background: 'radial-gradient(circle, rgba(232, 180, 180, 0.05) 0%, transparent 60%)',
            filter: 'blur(100px)',
          }}
        />
        <div 
          className="absolute bottom-1/4 right-0 w-[500px] h-[500px] rounded-full animate-aurora-medium"
          style={{
            background: 'radial-gradient(circle, rgba(180, 200, 232, 0.04) 0%, transparent 60%)',
            filter: 'blur(80px)',
          }}
        />
        
        <div 
          className="absolute top-1/3 right-1/4 w-48 h-48 glass-orb opacity-30 animate-float"
        />
        <div 
          className="absolute bottom-1/3 left-1/4 w-32 h-32 glass-orb opacity-20 animate-float-delayed"
        />
      </div>

      <div className="container relative z-10">
        <div className="max-w-3xl mx-auto text-center mb-20">
          <div 
            className={`transition-all duration-1000 ${
              isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'
            }`}
          >
            <span className="section-label">
              <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z" />
              </svg>
              How It Works
            </span>
          </div>
          
          <h2 
            className={`font-serif text-[clamp(2.5rem,5vw,4rem)] leading-[1.1] tracking-[-0.02em] text-white mb-6 transition-all duration-1000 delay-100 ${
              isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'
            }`}
          >
            The closed-loop{' '}
            <span className="text-gradient-aurora italic">lifecycle</span>
          </h2>
          
          <p 
            className={`text-lg text-white/40 leading-relaxed max-w-xl mx-auto transition-all duration-1000 delay-200 ${
              isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'
            }`}
          >
            Create, add or register, install, sync, and doctor. One continuous workflow from authoring to runtime health.
          </p>
        </div>

        <div className="relative max-w-4xl mx-auto">
          <div 
            className="absolute left-8 lg:left-12 top-20 bottom-20 w-px hidden md:block"
            style={{
              background: 'linear-gradient(to bottom, rgba(232, 180, 180, 0.3) 0%, rgba(232, 180, 180, 0.1) 50%, transparent 100%)',
            }}
          />

          <div className="space-y-8">
            {lifecycleSteps.map((step, idx) => (
              <div 
                key={idx} 
                className={`relative transition-all duration-1000 ${
                  isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-12'
                }`}
                style={{ transitionDelay: isVisible ? `${300 + idx * 200}ms` : '0ms' }}
              >
                <div className={`flex gap-6 lg:gap-10 ${idx % 2 === 1 ? 'lg:flex-row-reverse' : ''}`}>
                  
                  <div className="flex-shrink-0 hidden md:flex flex-col items-center">
                    <div 
                      className="w-16 h-16 lg:w-20 lg:h-20 rounded-2xl liquid-glass-premium premium-border flex items-center justify-center relative z-10"
                    >
                      <span className="font-serif text-2xl lg:text-3xl text-gradient-aurora">{step.number}</span>
                    </div>
                  </div>

                  
                  <div 
                    className={`flex-1 p-8 lg:p-10 rounded-3xl liquid-glass-premium premium-border group hover:scale-[1.02] transition-all duration-500 ${
                      idx % 2 === 1 ? 'lg:text-right' : ''
                    }`}
                  >
                    <div className={`flex items-center gap-4 mb-4 ${idx % 2 === 1 ? 'lg:flex-row-reverse' : ''}`}>
                      <div className="md:hidden w-12 h-12 rounded-xl liquid-glass-premium flex items-center justify-center">
                        <span className="font-serif text-xl text-gradient-aurora">{step.number}</span>
                      </div>
                      
                      <h3 className="font-serif text-2xl text-white group-hover:text-gradient-aurora transition-all duration-500">
                        {step.title}
                      </h3>
                    </div>
                    
                    <p className={`text-white/40 mb-6 leading-relaxed text-lg ${idx % 2 === 1 ? 'lg:ml-auto' : ''}`}>
                      {step.description}
                    </p>

                    <div 
                      className={`inline-flex items-center gap-3 px-5 py-3 rounded-xl bg-black/30 border border-white/5 font-mono text-sm group-hover:border-aurora-300/30 transition-all duration-500 ${idx % 2 === 1 ? 'lg:ml-auto' : ''}`}
                    >
                      <span className="text-aurora-300 font-semibold">$</span>
                      <span className="text-white/80">{step.code}</span>
                      
                      <button 
                        className="ml-4 opacity-0 group-hover:opacity-100 transition-all duration-300 text-white/30 hover:text-aurora-300"
                        onClick={() => navigator.clipboard.writeText(step.code)}
                        title="Copy to clipboard"
                      >
                        <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                        </svg>
                      </button>
                    </div>
                  </div>
                </div>
              </div>
            ))}
          </div>

          <div 
            className={`mt-16 text-center transition-all duration-1000 ${
              isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'
            }`}
            style={{ transitionDelay: isVisible ? '1400ms' : '0ms' }}
          >
            <div className="inline-flex items-center gap-4 px-6 py-4 rounded-2xl liquid-glass-premium"
            >
              <div className="flex -space-x-2">
                {['create', 'add', 'install', 'sync', 'doctor'].map((cmd, i) => (
                  <div 
                    key={cmd}
                    className="w-10 h-10 rounded-full bg-gradient-to-br from-aurora-300/20 to-aurora-500/10 border border-aurora-300/20 flex items-center justify-center text-xs font-mono text-aurora-300"
                  >
                    {i + 1}
                  </div>
                ))}
              </div>
              <span className="text-white/60 text-sm">Complete lifecycle in one tool</span>
            </div>
          </div>
        </div>
      </div>
    </section>
  )
}
