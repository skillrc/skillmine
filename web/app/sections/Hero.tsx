import Terminal from '../../components/Terminal'

export default function Hero() {
  return (
    <section className="relative min-h-screen flex items-center justify-center overflow-hidden gradient-hero">
      <div className="absolute inset-0 grid-pattern opacity-30"></div>
      
      <div className="container relative z-10">
        <div className="grid lg:grid-cols-2 gap-16 items-center">
          <div className="space-y-8">
            <div className="inline-flex items-center gap-2 px-4 py-2 rounded-full border border-white/10 bg-white/5">
              <span className="text-2xl">⛏</span>
              <span className="text-sm font-medium text-secondary">Public alpha for the closed-loop skill lifecycle</span>
            </div>
            
            <h1 className="text-5xl lg:text-7xl font-bold leading-tight">
              <span className="gradient-text">Skillmine</span>
              <br />
              <span className="text-3xl lg:text-5xl text-secondary font-normal">
                Create, manage, and sync AI skills
              </span>
            </h1>
            
            <p className="text-xl text-secondary max-w-lg">
              The native create-to-doctor workflow for Claude Code and OpenCode. Build local skills, register them declaratively, install deterministically, sync supported targets, and diagnose drift.
            </p>
            
            <div className="flex flex-wrap gap-4">
              <a href="#install" className="btn-primary inline-block">
                Get Started
              </a>
              <a href="https://github.com/skillrc/skillmine" target="_blank" rel="noopener noreferrer" className="btn-secondary inline-block">
                View on GitHub
              </a>
            </div>
            
            <div className="flex items-center gap-6 text-sm text-muted pt-4">
              <div className="flex items-center gap-2">
                <svg className="w-5 h-5 text-brand-orange" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                </svg>
                <span>Rust-powered</span>
              </div>
              <div className="flex items-center gap-2">
                <svg className="w-5 h-5 text-brand-orange" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                </svg>
                <span>Git-native</span>
              </div>
              <div className="flex items-center gap-2">
                <svg className="w-5 h-5 text-brand-orange" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                </svg>
                <span>Content-addressable</span>
              </div>
            </div>
          </div>
          
          <div className="relative">
            <div className="absolute -inset-4 bg-gradient-to-r from-brand-orange/20 to-cyan-bright/20 rounded-3xl blur-3xl opacity-50"></div>
            <Terminal />
          </div>
        </div>
      </div>
      
      <div className="absolute bottom-8 left-1/2 -translate-x-1/2 animate-bounce">
        <svg className="w-6 h-6 text-muted" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 14l-7 7m0 0l-7-7m7 7V3" />
        </svg>
      </div>
    </section>
  )
}
