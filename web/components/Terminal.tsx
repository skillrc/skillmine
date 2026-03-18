export default function Terminal() {
  const commands = [
    { prompt: '$', cmd: 'skillmine create my-skill', output: 'Created ./my-skill with SKILL.toml and starter files' },
    { prompt: '$', cmd: 'skillmine add ./my-skill', output: 'Registered my-skill in managed configuration' },
    { prompt: '$', cmd: 'skillmine install', output: 'Installing 3 skills...\n  ✓ Hello-World\n  ✓ git-release\n  ✓ python-testing' },
    { prompt: '$', cmd: 'skillmine sync --target=claude', output: 'Synced 3 skills to ~/.claude/skills/ (public alpha target)' },
    { prompt: '$', cmd: 'skillmine doctor', output: '✓ Configuration valid\n✓ Lifecycle healthy\n✓ No drift detected' },
  ]

  return (
    <div className="terminal animate-slide-up">
      <div className="terminal-header px-4 py-3 flex items-center gap-2">
        <div className="flex gap-2">
          <div className="w-3 h-3 rounded-full bg-red-500/80"></div>
          <div className="w-3 h-3 rounded-full bg-yellow-500/80"></div>
          <div className="w-3 h-3 rounded-full bg-green-500/80"></div>
        </div>
        <div className="flex-1 text-center">
          <span className="text-xs text-muted font-mono">skillmine — bash</span>
        </div>
        <div className="w-16"></div>
      </div>
      
      <div className="p-6 font-mono text-sm space-y-4 overflow-x-auto">
        {commands.map((item, idx) => (
          <div key={idx} className="space-y-1">
            <div className="flex items-start gap-2">
              <span className="text-brand-orange">{item.prompt}</span>
              <span className="text-cyan-bright">{item.cmd}</span>
            </div>
            <div className="pl-6 text-text-secondary whitespace-pre-line">
              {item.output}
            </div>
          </div>
        ))}
        <div className="flex items-center gap-2">
          <span className="text-brand-orange">$</span>
          <span className="w-2 h-4 bg-brand-orange animate-pulse"></span>
        </div>
      </div>
    </div>
  )
}
