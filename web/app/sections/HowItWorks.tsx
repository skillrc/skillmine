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
      className="section bg-surface-raised relative overflow-hidden"
    >
      <div className="container relative z-10">
        <div className="max-w-3xl mx-auto text-center mb-16">
          <div 
            className={`transition-all duration-700 ${
              isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-6'
            }`}
          >
            <span className="section-label">How It Works</span>
          </div>
          
          <h2 
            className={`section-title mb-6 transition-all duration-700 delay-100 ${
              isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-6'
            }`}
          >
            The closed-loop{' '}
            <span className="gradient-text-coral">lifecycle</span>
          </h2>
          
          <p 
            className={`section-description mx-auto transition-all duration-700 delay-200 ${
              isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-6'
            }`}
          >
            Create, add or register, install, sync, and doctor. One continuous workflow from authoring to runtime health.
          </p>
        </div>

        <div className="max-w-3xl mx-auto">
          {lifecycleSteps.map((step, idx) => (
            <div 
              key={idx} 
              className={`relative transition-all duration-700 ${
                isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-8'
              }`}
              style={{ transitionDelay: isVisible ? `${300 + idx * 150}ms` : '0ms' }}
            >
              {idx !== lifecycleSteps.length - 1 && (
                <div className="absolute left-6 top-16 w-px h-[calc(100%-32px)] hidden md:block bg-gradient-to-b from-coral-500/50 to-transparent" />
              )}

              <div className="flex gap-6 pb-12 last:pb-0">
                <div className="flex-shrink-0">
                  <div className="w-12 h-12 rounded-xl bg-coral-500/10 border border-coral-500/20 flex items-center justify-center">
                    <span className="text-sm font-semibold text-coral-500">{step.number}</span>
                  </div>
                </div>

                <div className="flex-1 pt-1">
                  <h3 className="text-xl font-semibold mb-2 text-white">
                    {step.title}
                  </h3>
                  <p className="text-gray-400 mb-5 leading-relaxed">
                    {step.description}
                  </p>

                  <div className="code-block group cursor-pointer hover:border-coral-500/30 transition-colors duration-300">
                    <div className="flex items-center gap-2">
                      <span className="terminal-prompt font-semibold">$</span>
                      <span className="terminal-command">{step.code}</span>
                      <button 
                        className="ml-auto opacity-0 group-hover:opacity-100 transition-opacity duration-200 text-gray-500 hover:text-white"
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
