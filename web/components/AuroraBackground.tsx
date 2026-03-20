'use client'

import { ReactNode } from 'react'

interface AuroraBackgroundProps {
  children: ReactNode
  className?: string
}

export default function AuroraBackground({ children, className = '' }: AuroraBackgroundProps) {
  return (
    <div className={`relative overflow-hidden ${className}`}>
      <div className="absolute inset-0 pointer-events-none">
        <div 
          className="absolute top-0 left-1/4 w-[800px] h-[800px] rounded-full animate-aurora-slow"
          style={{
            background: 'radial-gradient(circle, rgba(232, 180, 180, 0.08) 0%, transparent 60%)',
            filter: 'blur(80px)',
          }}
        />
        <div 
          className="absolute top-1/3 right-1/4 w-[600px] h-[600px] rounded-full animate-aurora-medium"
          style={{
            background: 'radial-gradient(circle, rgba(180, 200, 232, 0.05) 0%, transparent 60%)',
            filter: 'blur(60px)',
          }}
        />
        <div 
          className="absolute bottom-0 left-1/3 w-[700px] h-[700px] rounded-full animate-aurora-fast"
          style={{
            background: 'radial-gradient(circle, rgba(232, 200, 180, 0.04) 0%, transparent 60%)',
            filter: 'blur(70px)',
          }}
        />
      </div>
      {children}
    </div>
  )
}
