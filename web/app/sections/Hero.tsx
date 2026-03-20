'use client'

import { useEffect, useState } from 'react'
import Terminal from '../../components/Terminal'

export default function Hero() {
  const [isVisible, setIsVisible] = useState(false)

  useEffect(() => {
    setIsVisible(true)
  }, [])

  const features = [
    { label: 'Rust-powered', delay: 'stagger-4' },
    { label: 'Git-native', delay: 'stagger-5' },
    { label: 'Content-addressable', delay: 'stagger-6' },
  ]

  return (
    <section className="relative min-h-screen flex items-center justify-center overflow-hidden gradient-hero">
      {/* Background Elements */}
      <div className="absolute inset-0 grid-pattern opacity-30" />
      
      {/* Floating gradient orbs */}
      <div className="absolute top-1/4 left-1/4 w-96 h-96 bg-brand-orange/10 rounded-full blur-[120px] animate-pulse-glow" />
      <div className="absolute bottom-1/4 right-1/4 w-80 h-80 bg-cyan-bright/5 rounded-full blur-[100px] animate-pulse-glow" style={{ animationDelay: '1s' }} />
      
      <div className="container relative z-10">
        <div className="grid lg:grid-cols-2 gap-16 lg:gap-20 items-center">
          {/* Left Content */}
          <div className="space-y-8">
            {/* Badge */}
            <div 
              className={`inline-flex items-center gap-2 px-4 py-2 rounded-full glass glass-border transition-all duration-700 ${
                isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-4'
              }`}
            >
              <span className="text-xl">⛏</span>
              <span className="text-sm font-medium text-text-secondary">Public alpha for the closed-loop skill lifecycle</span>
            </div>
            
            {/* Headline */}
            <div className="space-y-4">
              <h1 
                className={`text-5xl sm:text-6xl lg:text-7xl font-bold leading-[1.1] tracking-[-0.03em] transition-all duration-700 delay-100 ${
                  isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-6'
                }`}
              >
                <span className="gradient-text">Skillmine</span>
              </h1>
              <p 
                className={`text-2xl sm:text-3xl lg:text-4xl text-text-secondary font-normal leading-[1.3] tracking-[-0.02em] transition-all duration-700 delay-200 ${
                  isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-6'
                }`}
              >
                Create, manage, and sync AI skills
              </p>
            </div>
            
            {/* Description */}
            <p 
              className={`text-lg text-text-secondary max-w-lg leading-relaxed transition-all duration-700 delay-300 ${
                isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-6'
              }`}
            >
              The native create-to-doctor workflow for Claude Code and OpenCode. Build local skills, register them declaratively, install deterministically, sync supported targets, and diagnose drift.
            </p>
            
            {/* CTA Buttons */}
            <div 
              className={`flex flex-wrap gap-4 transition-all duration-700 delay-400 ${
                isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-6'
              }`}
            >
              <a href="#install" className="btn-primary inline-flex items-center">
                <span>Get Started</span>
                <svg className="w-4 h-4 ml-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 8l4 4m0 0l-4 4m4-4H3" />
                </svg>
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
                <span>View on GitHub</span>
              </a>
            </div>
            
            {/* Feature Pills */}
            <div 
              className={`flex flex-wrap items-center gap-4 pt-4 transition-all duration-700 delay-500 ${
                isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-6'
              }`}
            >
              {features.map((feature, idx) => (
                <div 
                  key={idx} 
                  className={`flex items-center gap-2 text-sm text-text-secondary ${feature.delay}`}
                >
                  <svg className="w-5 h-5 text-brand-orange" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                  </svg>
                  <span>{feature.label}</span>
                </div>
              ))}
            </div>
          </div>
          
          {/* Right Content - Terminal */}
          <div 
            className={`relative transition-all duration-1000 delay-300 ${
              isVisible ? 'opacity-100 translate-x-0' : 'opacity-0 translate-x-12'
            }`}
          >
            {/* Glow effect behind terminal */}
            <div className="absolute -inset-8 bg-gradient-to-r from-brand-orange/20 via-cyan-bright/10 to-brand-orange/20 rounded-3xl blur-3xl opacity-60 animate-pulse-glow" />
            
            {/* Terminal */}
            <div className="relative">
              <Terminal />
            </div>
          </div>
        </div>
      </div>
      
      {/* Scroll indicator */}
      <div className="absolute bottom-8 left-1/2 -translate-x-1/2 animate-bounce">
        <a href="#problem" className="flex flex-col items-center gap-2 text-text-muted hover:text-text-secondary transition-colors">
          <span className="text-xs font-medium uppercase tracking-wider">Scroll</span>
          <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 14l-7 7m0 0l-7-7m7 7V3" />
          </svg>
        </a>
      </div>
    </section>
  )
}
