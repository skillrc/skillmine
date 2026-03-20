'use client'

import { useRef, ReactNode } from 'react'

interface SpotlightCardProps {
  children: ReactNode
  className?: string
  style?: React.CSSProperties
}

export default function SpotlightCard({ children, className = '', style }: SpotlightCardProps) {
  const ref = useRef<HTMLDivElement>(null)

  const handleMouseMove = (e: React.MouseEvent<HTMLDivElement>) => {
    if (!ref.current) return
    
    const rect = ref.current.getBoundingClientRect()
    const x = e.clientX - rect.left
    const y = e.clientY - rect.top
    
    ref.current.style.setProperty('--mouse-x', `${x}px`)
    ref.current.style.setProperty('--mouse-y', `${y}px`)
  }

  return (
    <div
      ref={ref}
      className={`glass-card glass-card-spotlight ${className}`}
      onMouseMove={handleMouseMove}
      style={style}
    >
      {children}
    </div>
  )
}
