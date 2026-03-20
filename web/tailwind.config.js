/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './pages/**/*.{js,ts,jsx,tsx,mdx}',
    './components/**/*.{js,ts,jsx,tsx,mdx}',
    './app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {
      colors: {
        obsidian: {
          DEFAULT: '#0A0A0B',
          50: '#FAFAFA',
          100: '#F5F5F5',
          200: '#E5E5E5',
          300: '#D4D4D4',
          400: '#A3A3A3',
          500: '#737373',
          600: '#525252',
          700: '#404040',
          800: '#262626',
          850: '#1A1A1A',
          900: '#111111',
          950: '#0A0A0B',
          975: '#070708',
        },
        aurora: {
          DEFAULT: '#E8B4B4',
          50: '#FDF8F8',
          100: '#FAF0F0',
          200: '#F5E0E0',
          300: '#E8B4B4',
          400: '#D48B8B',
          500: '#C06B6B',
          600: '#A85454',
          700: '#8B4040',
          800: '#6B3030',
          900: '#4A2020',
        },
        glass: {
          DEFAULT: 'rgba(255, 255, 255, 0.03)',
          light: 'rgba(255, 255, 255, 0.05)',
          medium: 'rgba(255, 255, 255, 0.08)',
          heavy: 'rgba(255, 255, 255, 0.12)',
          border: 'rgba(255, 255, 255, 0.06)',
          'border-hover': 'rgba(255, 255, 255, 0.12)',
          'border-active': 'rgba(255, 255, 255, 0.18)',
        },
        surface: {
          DEFAULT: '#0A0A0B',
          raised: '#0F0F10',
          elevated: '#131315',
          hover: '#18181A',
          active: '#1E1E20',
          overlay: 'rgba(0, 0, 0, 0.6)',
        },
        success: '#4ADE80',
        warning: '#FBBF24',
        error: '#F87171',
        info: '#60A5FA',
      },
      fontFamily: {
        serif: ['var(--font-playfair)', 'Playfair Display', 'Georgia', 'serif'],
        sans: ['var(--font-geist-sans)', 'system-ui', '-apple-system', 'BlinkMacSystemFont', 'sans-serif'],
        mono: ['var(--font-jetbrains-mono)', 'JetBrains Mono', 'Fira Code', 'monospace'],
      },
      fontSize: {
        'display-xl': ['clamp(4rem, 12vw, 9rem)', { lineHeight: '0.9', letterSpacing: '-0.04em', fontWeight: '400' }],
        'display-lg': ['clamp(3rem, 8vw, 6rem)', { lineHeight: '0.95', letterSpacing: '-0.03em', fontWeight: '400' }],
        'display-md': ['clamp(2.5rem, 5vw, 4rem)', { lineHeight: '1', letterSpacing: '-0.02em', fontWeight: '400' }],
        'display-sm': ['clamp(2rem, 4vw, 3rem)', { lineHeight: '1.1', letterSpacing: '-0.02em', fontWeight: '400' }],
        'title-xl': ['2rem', { lineHeight: '1.2', letterSpacing: '-0.02em', fontWeight: '500' }],
        'title-lg': ['1.5rem', { lineHeight: '1.3', letterSpacing: '-0.01em', fontWeight: '500' }],
        'title-md': ['1.25rem', { lineHeight: '1.4', letterSpacing: '-0.01em', fontWeight: '500' }],
        'title-sm': ['1.125rem', { lineHeight: '1.45', fontWeight: '500' }],
        'body-xl': ['1.25rem', { lineHeight: '1.7' }],
        'body-lg': ['1.125rem', { lineHeight: '1.7' }],
        'body-md': ['1rem', { lineHeight: '1.75' }],
        'body-sm': ['0.9375rem', { lineHeight: '1.7' }],
        'caption': ['0.875rem', { lineHeight: '1.6' }],
        'caption-sm': ['0.8125rem', { lineHeight: '1.5' }],
      },
      spacing: {
        '4.5': '1.125rem',
        '5.5': '1.375rem',
        '18': '4.5rem',
        '22': '5.5rem',
        '30': '7.5rem',
        '34': '8.5rem',
        '38': '9.5rem',
        'section': '8rem',
        'section-lg': '12rem',
      },
      borderRadius: {
        '2xl': '1rem',
        '2.5xl': '1.25rem',
        '3xl': '1.5rem',
        '4xl': '2rem',
      },
      boxShadow: {
        'soft': '0 2px 16px rgba(0, 0, 0, 0.2)',
        'medium': '0 4px 24px rgba(0, 0, 0, 0.3)',
        'large': '0 8px 40px rgba(0, 0, 0, 0.4)',
        'glass': '0 0 0 1px rgba(255, 255, 255, 0.06), 0 2px 16px rgba(0, 0, 0, 0.2)',
        'glass-hover': '0 0 0 1px rgba(255, 255, 255, 0.1), 0 8px 32px rgba(0, 0, 0, 0.35)',
        'glow-aurora': '0 0 80px rgba(232, 180, 180, 0.15)',
        'glow-aurora-sm': '0 0 40px rgba(232, 180, 180, 0.1)',
        'inner-glow': 'inset 0 1px 1px rgba(255, 255, 255, 0.05)',
      },
      backgroundImage: {
        'aurora-1': 'linear-gradient(135deg, rgba(232, 180, 180, 0.15) 0%, transparent 50%)',
        'aurora-2': 'linear-gradient(225deg, rgba(180, 200, 232, 0.1) 0%, transparent 50%)',
        'aurora-3': 'linear-gradient(315deg, rgba(232, 200, 180, 0.08) 0%, transparent 50%)',
        'mesh-1': 'radial-gradient(ellipse 80% 50% at 20% 40%, rgba(232, 180, 180, 0.08) 0%, transparent 50%)',
        'mesh-2': 'radial-gradient(ellipse 60% 40% at 80% 20%, rgba(180, 200, 232, 0.06) 0%, transparent 50%)',
        'mesh-3': 'radial-gradient(ellipse 50% 60% at 60% 80%, rgba(232, 200, 180, 0.05) 0%, transparent 50%)',
        'gradient-radial': 'radial-gradient(var(--tw-gradient-stops))',
        'gradient-subtle': 'linear-gradient(180deg, rgba(232, 180, 180, 0.03) 0%, transparent 100%)',
        'gradient-glass': 'linear-gradient(180deg, rgba(255, 255, 255, 0.05) 0%, rgba(255, 255, 255, 0.02) 100%)',
        'liquid-gold': 'linear-gradient(135deg, rgba(232, 200, 180, 0.2) 0%, rgba(232, 180, 180, 0.1) 50%, transparent 100%)',
        'liquid-silver': 'linear-gradient(225deg, rgba(180, 200, 232, 0.15) 0%, rgba(200, 200, 220, 0.08) 50%, transparent 100%)',
        'obsidian-depth': 'radial-gradient(ellipse 150% 100% at 20% 100%, #0F0F10 0%, #070708 50%, #050505 100%)',
      },
      animation: {
        'fade-in': 'fade-in 0.8s cubic-bezier(0.16, 1, 0.3, 1) forwards',
        'fade-in-up': 'fade-in-up 0.8s cubic-bezier(0.16, 1, 0.3, 1) forwards',
        'slide-in-up': 'slide-in-up 0.6s cubic-bezier(0.16, 1, 0.3, 1) forwards',
        'scale-in': 'scale-in 0.5s cubic-bezier(0.16, 1, 0.3, 1) forwards',
        'aurora-slow': 'aurora-slow 20s ease-in-out infinite',
        'aurora-medium': 'aurora-medium 15s ease-in-out infinite',
        'aurora-fast': 'aurora-fast 10s ease-in-out infinite',
        'pulse-soft': 'pulse-soft 6s ease-in-out infinite',
        'pulse-glow': 'pulse-glow 4s ease-in-out infinite',
        'shimmer': 'shimmer 2s linear infinite',
        'float': 'float 6s ease-in-out infinite',
      },
      keyframes: {
        'fade-in': {
          from: { opacity: '0' },
          to: { opacity: '1' },
        },
        'fade-in-up': {
          from: { opacity: '0', transform: 'translateY(30px)' },
          to: { opacity: '1', transform: 'translateY(0)' },
        },
        'slide-in-up': {
          from: { opacity: '0', transform: 'translateY(20px)' },
          to: { opacity: '1', transform: 'translateY(0)' },
        },
        'scale-in': {
          from: { opacity: '0', transform: 'scale(0.95)' },
          to: { opacity: '1', transform: 'scale(1)' },
        },
        'aurora-slow': {
          '0%, 100%': { transform: 'translate(0%, 0%) rotate(0deg)' },
          '33%': { transform: 'translate(2%, 2%) rotate(1deg)' },
          '66%': { transform: 'translate(-1%, 1%) rotate(-1deg)' },
        },
        'aurora-medium': {
          '0%, 100%': { transform: 'translate(0%, 0%) rotate(0deg)' },
          '33%': { transform: 'translate(-2%, -1%) rotate(-1deg)' },
          '66%': { transform: 'translate(1%, -2%) rotate(1deg)' },
        },
        'aurora-fast': {
          '0%, 100%': { transform: 'translate(0%, 0%) rotate(0deg)' },
          '33%': { transform: 'translate(1%, -2%) rotate(1deg)' },
          '66%': { transform: 'translate(-2%, 1%) rotate(-1deg)' },
        },
        'pulse-soft': {
          '0%, 100%': { opacity: '0.3' },
          '50%': { opacity: '0.6' },
        },
        'pulse-glow': {
          '0%, 100%': { opacity: '0.4', transform: 'scale(1)' },
          '50%': { opacity: '0.7', transform: 'scale(1.05)' },
        },
        'shimmer': {
          '0%': { backgroundPosition: '-200% 0' },
          '100%': { backgroundPosition: '200% 0' },
        },
        'float': {
          '0%, 100%': { transform: 'translateY(0)' },
          '50%': { transform: 'translateY(-10px)' },
        },
      },
      transitionTimingFunction: {
        'spring': 'cubic-bezier(0.16, 1, 0.3, 1)',
        'smooth': 'cubic-bezier(0.4, 0, 0.2, 1)',
        'expo-out': 'cubic-bezier(0.19, 1, 0.22, 1)',
      },
      transitionDuration: {
        '400': '400ms',
        '600': '600ms',
        '800': '800ms',
      },
    },
  },
  plugins: [
    function({ addUtilities }) {
      addUtilities({
        '.text-balance': {
          'text-wrap': 'balance',
        },
        '.text-gradient': {
          'background': 'linear-gradient(135deg, #FFFFFF 0%, rgba(255, 255, 255, 0.7) 100%)',
          '-webkit-background-clip': 'text',
          '-webkit-text-fill-color': 'transparent',
          'background-clip': 'text',
        },
        '.text-gradient-aurora': {
          'background': 'linear-gradient(135deg, #E8B4B4 0%, #D48B8B 50%, #E8C8B4 100%)',
          '-webkit-background-clip': 'text',
          '-webkit-text-fill-color': 'transparent',
          'background-clip': 'text',
        },
        '.glass': {
          'background': 'rgba(255, 255, 255, 0.03)',
          'backdrop-filter': 'blur(24px) saturate(180%)',
          '-webkit-backdrop-filter': 'blur(24px) saturate(180%)',
        },
        '.glass-light': {
          'background': 'rgba(255, 255, 255, 0.05)',
          'backdrop-filter': 'blur(20px) saturate(150%)',
          '-webkit-backdrop-filter': 'blur(20px) saturate(150%)',
        },
        '.glass-heavy': {
          'background': 'rgba(255, 255, 255, 0.08)',
          'backdrop-filter': 'blur(32px) saturate(200%)',
          '-webkit-backdrop-filter': 'blur(32px) saturate(200%)',
        },
        '.glass-border': {
          'border': '1px solid rgba(255, 255, 255, 0.06)',
        },
        '.glass-border-hover': {
          'border': '1px solid rgba(255, 255, 255, 0.12)',
        },
        '.glass-shine': {
          'box-shadow': `
            inset 0 1px 1px rgba(255, 255, 255, 0.08),
            0 0 0 1px rgba(255, 255, 255, 0.06),
            0 4px 20px rgba(0, 0, 0, 0.2)
          `,
        },
        '.surface-obsidian': {
          'background': 'linear-gradient(180deg, #0F0F10 0%, #0A0A0B 100%)',
        },
        '.surface-raised': {
          'background': 'linear-gradient(180deg, #131315 0%, #0F0F10 100%)',
        },
        '.surface-elevated': {
          'background': 'linear-gradient(180deg, #18181A 0%, #131315 100%)',
        },
        '.aurora-bg': {
          'background': `
            radial-gradient(ellipse 80% 50% at 20% 40%, rgba(232, 180, 180, 0.06) 0%, transparent 50%),
            radial-gradient(ellipse 60% 40% at 80% 20%, rgba(180, 200, 232, 0.04) 0%, transparent 50%),
            radial-gradient(ellipse 50% 60% at 60% 80%, rgba(232, 200, 180, 0.03) 0%, transparent 50%),
            #0A0A0B
          `,
        },
        '.spotlight': {
          'position': 'relative',
          'overflow': 'hidden',
        },
        '.spotlight::before': {
          'content': '""',
          'position': 'absolute',
          'inset': '0',
          'background': 'radial-gradient(600px circle at var(--mouse-x, 50%) var(--mouse-y, 50%), rgba(232, 180, 180, 0.06), transparent 40%)',
          'pointer-events': 'none',
          'opacity': '0',
          'transition': 'opacity 0.3s',
        },
        '.spotlight:hover::before': {
          'opacity': '1',
        },
        '.magnetic': {
          'transition': 'transform 0.3s cubic-bezier(0.16, 1, 0.3, 1)',
        },
        '.liquid-glass': {
          'background': 'linear-gradient(135deg, rgba(255, 255, 255, 0.08) 0%, rgba(255, 255, 255, 0.02) 100%)',
          'backdrop-filter': 'blur(40px) saturate(200%)',
          '-webkit-backdrop-filter': 'blur(40px) saturate(200%)',
          'border': '1px solid rgba(255, 255, 255, 0.08)',
          'box-shadow': `
            inset 0 1px 1px rgba(255, 255, 255, 0.1),
            inset 0 -1px 1px rgba(0, 0, 0, 0.1),
            0 8px 32px rgba(0, 0, 0, 0.4),
            0 0 0 1px rgba(255, 255, 255, 0.04)
          `,
        },
        '.liquid-glass-hover': {
          'background': 'linear-gradient(135deg, rgba(255, 255, 255, 0.12) 0%, rgba(255, 255, 255, 0.04) 100%)',
          'border-color': 'rgba(255, 255, 255, 0.15)',
          'box-shadow': `
            inset 0 1px 1px rgba(255, 255, 255, 0.15),
            inset 0 -1px 1px rgba(0, 0, 0, 0.15),
            0 16px 48px rgba(0, 0, 0, 0.5),
            0 0 0 1px rgba(255, 255, 255, 0.08)
          `,
        },
        '.text-shimmer': {
          'background': 'linear-gradient(90deg, #FFFFFF 0%, #E8B4B4 50%, #FFFFFF 100%)',
          'background-size': '200% 100%',
          '-webkit-background-clip': 'text',
          '-webkit-text-fill-color': 'transparent',
          'background-clip': 'text',
          'animation': 'shimmer 3s ease-in-out infinite',
        },
        '.aurora-glow': {
          'box-shadow': '0 0 100px rgba(232, 180, 180, 0.2), 0 0 200px rgba(232, 180, 180, 0.1)',
        },
        '.premium-border': {
          'position': 'relative',
        },
        '.premium-border::before': {
          'content': '""',
          'position': 'absolute',
          'inset': '0',
          'border-radius': 'inherit',
          'padding': '1px',
          'background': 'linear-gradient(135deg, rgba(255, 255, 255, 0.2) 0%, rgba(255, 255, 255, 0.05) 50%, rgba(232, 180, 180, 0.1) 100%)',
          '-webkit-mask': 'linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0)',
          '-webkit-mask-composite': 'xor',
          'mask-composite': 'exclude',
          'pointer-events': 'none',
        },
      })
    },
  ],
}
