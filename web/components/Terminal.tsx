'use client'

import { useEffect, useState } from 'react'

interface Command {
  prompt: string
  cmd: string
  output: string
}

const commands: Command[] = [
  { prompt: '$', cmd: 'skillmine create my-skill', output: 'Created ./my-skill with SKILL.toml and starter files' },
  { prompt: '$', cmd: 'skillmine add ./my-skill', output: 'Registered my-skill in managed configuration' },
  { prompt: '$', cmd: 'skillmine install', output: 'Installing 3 skills...\n  ✓ Hello-World\n  ✓ git-release\n  ✓ python-testing' },
  { prompt: '$', cmd: 'skillmine sync --target=claude', output: 'Synced 3 skills to ~/.claude/skills/ (public alpha target)' },
  { prompt: '$', cmd: 'skillmine doctor', output: '✓ Configuration valid\n✓ Lifecycle healthy\n✓ No drift detected' },
]

function TypewriterText({ text, delay = 0, onComplete }: { text: string; delay?: number; onComplete?: () => void }) {
  const [displayText, setDisplayText] = useState('')
  const [started, setStarted] = useState(false)

  useEffect(() => {
    const startTimeout = setTimeout(() => setStarted(true), delay)
    return () => clearTimeout(startTimeout)
  }, [delay])

  useEffect(() => {
    if (!started) return

    let index = 0
    const interval = setInterval(() => {
      if (index <= text.length) {
        setDisplayText(text.slice(0, index))
        index++
      } else {
        clearInterval(interval)
        onComplete?.()
      }
    }, 30)

    return () => clearInterval(interval)
  }, [started, text, onComplete])

  return <span>{displayText}</span>
}

export default function Terminal() {
  const [visibleLines, setVisibleLines] = useState(0)
  const [typingLines, setTypingLines] = useState<number[]>([])

  useEffect(() => {
    let lineIndex = 0
    const showInterval = setInterval(() => {
      if (lineIndex < commands.length) {
        setVisibleLines(prev => prev + 1)
        setTypingLines(prev => [...prev, lineIndex])
        lineIndex++
      } else {
        clearInterval(showInterval)
      }
    }, 800)

    return () => clearInterval(showInterval)
  }, [])

  return (
    <div className="terminal">
      <div className="terminal-header">
        <div className="flex gap-2">
          <div className="terminal-dot terminal-dot-red"></div>
          <div className="terminal-dot terminal-dot-yellow"></div>
          <div className="terminal-dot terminal-dot-green"></div>
        </div>
        <div className="flex-1 text-center">
          <span className="text-xs text-gray-500 font-mono">skillmine</span>
        </div>
        <div className="w-16"></div>
      </div>
      
      <div className="terminal-body space-y-4 overflow-x-auto">
        {commands.map((item, idx) => (
          <div 
            key={idx} 
            className={`space-y-2 transition-all duration-500 ${
              idx < visibleLines ? 'opacity-100' : 'opacity-0'
            }`}
          >
            <div className="flex items-start gap-2">
              <span className="terminal-prompt font-semibold">{item.prompt}</span>
              <span className="terminal-command">
                {typingLines.includes(idx) ? (
                  <TypewriterText 
                    text={item.cmd} 
                    delay={idx === 0 ? 500 : 0}
                  />
                ) : (
                  item.cmd
                )}
              </span>
            </div>
            <div 
              className={`pl-6 terminal-output whitespace-pre-line transition-all duration-500 ${
                idx < visibleLines ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-2'
              }`}
              style={{ transitionDelay: typingLines.includes(idx) ? `${item.cmd.length * 30 + 200}ms` : '0ms' }}
            >
              {item.output.split('\n').map((line, lineIdx) => (
                <div key={lineIdx} className={line.startsWith('✓') ? 'terminal-success' : ''}>
                  {line}
                </div>
              ))}
            </div>
          </div>
        ))}
        
        <div className="flex items-center gap-2 pt-2">
          <span className="terminal-prompt font-semibold">$</span>
          <span className="w-2 h-4 bg-coral-500 animate-pulse"></span>
        </div>
      </div>
    </div>
  )
}
