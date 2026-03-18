const problems = [
  {
    icon: (
      <svg className="w-8 h-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z" />
      </svg>
    ),
    title: 'Skill Chaos',
    description: 'AI skills scattered across different assistants, versions, and locations. No single source of truth for your workflow enhancements.',
  },
  {
    icon: (
      <svg className="w-8 h-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M3.75 13.5l10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75z" />
      </svg>
    ),
    title: 'Version Drift',
    description: 'Skills get updated silently. Your carefully tuned prompts break overnight. No way to lock to known-good versions or track changes.',
  },
  {
    icon: (
      <svg className="w-8 h-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />
      </svg>
    ),
    title: 'Manual Sync Hell',
    description: 'Copying files between Claude, OpenCode, and Cursor. Updating one assistant means repeating work three times.',
  },
  {
    icon: (
      <svg className="w-8 h-8" fill="none" viewBox="0 0 24 24" stroke="currentColor"
      >
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M12 6v6h4.5m4.5 0a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
    ),
    title: 'No Visibility',
    description: 'No way to see what skills are installed, which are outdated, or diagnose why something broke. Blind debugging.',
  },
]

export default function Problem() {
  return (
    <section className="section bg-obsidian">
      <div className="container">
        <div className="text-center mb-16">
          <p className="text-brand-orange font-semibold mb-4">THE PROBLEM</p>
          <h2 className="text-4xl lg:text-5xl font-bold mb-6">
            Managing AI skills is <span className="gradient-text">a mess</span>
          </h2>
          <p className="text-xl text-secondary max-w-2xl mx-auto">
            You have powerful AI assistants, but no good way to create, organize, version,
            and sync the skills that make them useful.
          </p>
        </div>
        
        <div className="grid md:grid-cols-2 gap-6">
          {problems.map((problem, idx) => (
            <div
              key={idx}
              className="p-8 rounded-2xl border border-white/10 bg-surface card-hover"
            >
              <div className="text-brand-orange mb-4">{problem.icon}</div>
              <h3 className="text-xl font-semibold mb-2">{problem.title}</h3>
              <p className="text-secondary">{problem.description}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}
