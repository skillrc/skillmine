import { lifecycleSteps } from '../lib/lifecycle'

export default function HowItWorks() {
  return (
    <section id="how-it-works" className="section bg-slate-deep relative">
      <div className="absolute inset-0 grid-pattern opacity-20"></div>

      <div className="container relative z-10">
        <div className="text-center mb-16">
          <p className="text-brand-orange font-semibold mb-4">HOW IT WORKS</p>
          <h2 className="text-4xl lg:text-5xl font-bold mb-6">
            The closed-loop <span className="gradient-text">lifecycle</span>
          </h2>
          <p className="text-xl text-secondary max-w-2xl mx-auto">
            Create, add or register, install, sync, and doctor. One continuous workflow from authoring to runtime health.
          </p>
        </div>

        <div className="max-w-4xl mx-auto">
          {lifecycleSteps.map((step, idx) => (
            <div key={idx} className="relative">
              {idx !== lifecycleSteps.length - 1 && (
                <div className="absolute left-8 top-20 w-px h-24 bg-gradient-to-b from-brand-orange to-transparent hidden md:block"></div>
              )}

              <div className="flex flex-col md:flex-row gap-8 mb-16 last:mb-0">
                <div className="flex-shrink-0">
                  <div className="w-16 h-16 rounded-2xl bg-brand-orange/10 border border-brand-orange/30 flex items-center justify-center">
                    <span className="text-2xl font-bold text-brand-orange">{step.number}</span>
                  </div>
                </div>

                <div className="flex-1">
                  <h3 className="text-2xl font-semibold mb-2">{step.title}</h3>
                  <p className="text-secondary mb-4">{step.description}</p>

                  <div className="code-block p-4 font-mono text-sm">
                    <span className="text-brand-orange">$</span>{' '}
                    <span className="text-cyan-bright">{step.code}</span>
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
