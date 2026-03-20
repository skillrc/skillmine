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
      className="section bg-deep-slate relative overflow-hidden"
    >
      <div className="absolute inset-0 grid-pattern opacity-20" />
      
      <div className="container relative z-10">
        <div className="text-center mb-16">
          <div 
            className={`transition-all duration-700 ${
              isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-6'
            }`}
          >
            <span className="section-label">How It Works</span>
          </div>
          
          <h2 
            className={`text-4xl lg:text-5xl font-bold mb-6 tracking-[-0.02em] transition-all duration-700 delay-100 ${
              isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-6'
            }`}
          >
            The closed-loop{' '}
            <span className="gradient-text">lifecycle</span>
          </h2>
          
          <p 
            className={`text-lg lg:text-xl text-text-secondary max-w-2xl mx-auto leading-relaxed transition-all duration-700 delay-200 ${
              isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-6'
            }`}
          >
            Create, add or register, install, sync, and doctor. One continuous workflow from authoring to runtime health.
          </p>
        </div>

        <div className="max-w-4xl mx-auto">
          {lifecycleSteps.map((step, idx) => (
            <div 
              key={idx} 
              className={`relative transition-all duration-700 ${
                isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'
              }`}
              style={{ transitionDelay: isVisible ? `${300 + idx * 150}ms` : '0ms' }}
            >
              {/* Connector line */}
              {idx !== lifecycleSteps.length - 1 && (
                <div className="absolute left-8 top-20 w-px h-[calc(100%-40px)] hidden md:block">
                  <div className="w-full h-full bg-gradient-to-b from-brand-orange/50 via-brand-orange/20 to-transparent" />
                </div>
              )}

              <div className="flex flex-col md:flex-row gap-6 md:gap-8 pb-12 last:pb-0">
                <div className="flex-shrink-0">
                  <div className="relative">
                    <div className="w-16 h-16 rounded-2xl bg-gradient-to-br from-brand-orange/20 to-brand-orange/5 border border-brand-orange/30 flex items-center justify-center relative z-10 shadow-glow-orange">
                      <span className="text-2xl font-bold text-brand-orange">{step.number}</span>
                    </div>
                    <div className="absolute inset-0 rounded-2xl bg-brand-orange/20 blur-xl" />
                  </div>
                </div>

                <div className="flex-1 pt-1">
                  <h3 className="text-2xl font-semibold mb-3 text-text-primary">
                    {step.title}
                  </h3>
                  <p className="text-text-secondary mb-5 leading-relaxed">
                    {step.description}
                  </p>

                  <div className="code-block p-4 font-mono text-sm group cursor-pointer hover:border-brand-orange/30 transition-colors duration-300">
                    <div className="flex items-center gap-2">
                      <span className="text-brand-orange font-semibold">$</span>
                      <span className="text-cyan-bright">{step.code}</span>
                      <button 
                        className="ml-auto opacity-0 group-hover:opacity-100 transition-opacity duration-200 text-text-muted hover:text-text-primary"
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
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}
