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
        coral: {
          DEFAULT: '#FF6B5B',
          50: '#FFF5F3',
          100: '#FFE8E5',
          200: '#FFD4CF',
          300: '#FFB8AF',
          400: '#FF9589',
          500: '#FF6B5B',
          600: '#E85545',
          700: '#D14739',
          800: '#B23A2E',
          900: '#8F3027',
        },
        gray: {
          0: '#FFFFFF',
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
          950: '#0A0A0A',
        },
        surface: {
          DEFAULT: '#0F0F10',
          raised: '#18181A',
          elevated: '#202023',
          hover: '#27272A',
          active: '#323236',
        },
        success: '#22C55E',
        warning: '#F59E0B',
        error: '#EF4444',
        info: '#3B82F6',
        border: {
          DEFAULT: 'rgba(255, 255, 255, 0.08)',
          hover: 'rgba(255, 255, 255, 0.12)',
          active: 'rgba(255, 255, 255, 0.16)',
        },
      },
      fontFamily: {
        sans: ['var(--font-geist-sans)', 'system-ui', '-apple-system', 'BlinkMacSystemFont', 'sans-serif'],
        mono: ['var(--font-jetbrains-mono)', 'JetBrains Mono', 'Fira Code', 'monospace'],
      },
      fontSize: {
        'display-xl': ['5rem', { lineHeight: '1', letterSpacing: '-0.04em', fontWeight: '600' }],
        'display-lg': ['4rem', { lineHeight: '1.05', letterSpacing: '-0.03em', fontWeight: '600' }],
        'display-md': ['3rem', { lineHeight: '1.1', letterSpacing: '-0.02em', fontWeight: '600' }],
        'display-sm': ['2.25rem', { lineHeight: '1.15', letterSpacing: '-0.02em', fontWeight: '600' }],
        'title-xl': ['1.75rem', { lineHeight: '1.3', letterSpacing: '-0.01em', fontWeight: '600' }],
        'title-lg': ['1.5rem', { lineHeight: '1.35', letterSpacing: '-0.01em', fontWeight: '600' }],
        'title-md': ['1.25rem', { lineHeight: '1.4', letterSpacing: '-0.01em', fontWeight: '600' }],
        'title-sm': ['1.125rem', { lineHeight: '1.45', fontWeight: '600' }],
        'body-lg': ['1.125rem', { lineHeight: '1.75' }],
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
      },
      borderRadius: {
        '2xl': '1rem',
        '2.5xl': '1.25rem',
        '3xl': '1.5rem',
        '4xl': '2rem',
      },
      boxShadow: {
        'soft': '0 2px 16px rgba(0, 0, 0, 0.12)',
        'medium': '0 4px 24px rgba(0, 0, 0, 0.16)',
        'large': '0 8px 40px rgba(0, 0, 0, 0.24)',
        'card': '0 0 0 1px rgba(255, 255, 255, 0.06), 0 2px 16px rgba(0, 0, 0, 0.12)',
        'card-hover': '0 0 0 1px rgba(255, 255, 255, 0.1), 0 8px 32px rgba(0, 0, 0, 0.24)',
        'glow-coral': '0 0 60px rgba(255, 107, 91, 0.3)',
        'glow-coral-sm': '0 0 30px rgba(255, 107, 91, 0.2)',
      },
      backgroundImage: {
        'gradient-radial': 'radial-gradient(var(--tw-gradient-stops))',
        'gradient-subtle': 'linear-gradient(180deg, rgba(255, 107, 91, 0.03) 0%, transparent 100%)',
        'gradient-card': 'linear-gradient(180deg, rgba(255, 255, 255, 0.03) 0%, rgba(255, 255, 255, 0) 100%)',
      },
      animation: {
        'fade-in': 'fade-in 0.6s cubic-bezier(0.16, 1, 0.3, 1) forwards',
        'fade-in-up': 'fade-in-up 0.6s cubic-bezier(0.16, 1, 0.3, 1) forwards',
        'slide-in-up': 'slide-in-up 0.5s cubic-bezier(0.16, 1, 0.3, 1) forwards',
        'scale-in': 'scale-in 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards',
        'pulse-soft': 'pulse-soft 4s ease-in-out infinite',
      },
      keyframes: {
        'fade-in': {
          from: { opacity: '0' },
          to: { opacity: '1' },
        },
        'fade-in-up': {
          from: { opacity: '0', transform: 'translateY(20px)' },
          to: { opacity: '1', transform: 'translateY(0)' },
        },
        'slide-in-up': {
          from: { opacity: '0', transform: 'translateY(16px)' },
          to: { opacity: '1', transform: 'translateY(0)' },
        },
        'scale-in': {
          from: { opacity: '0', transform: 'scale(0.96)' },
          to: { opacity: '1', transform: 'scale(1)' },
        },
        'pulse-soft': {
          '0%, 100%': { opacity: '0.4' },
          '50%': { opacity: '0.7' },
        },
      },
      transitionTimingFunction: {
        'spring': 'cubic-bezier(0.16, 1, 0.3, 1)',
        'smooth': 'cubic-bezier(0.4, 0, 0.2, 1)',
      },
      transitionDuration: {
        '400': '400ms',
      },
    },
  },
  plugins: [
    function({ addUtilities }) {
      addUtilities({
        '.text-balance': {
          'text-wrap': 'balance',
        },
        '.glass': {
          'background': 'rgba(24, 24, 26, 0.6)',
          'backdrop-filter': 'blur(20px)',
          '-webkit-backdrop-filter': 'blur(20px)',
        },
        '.surface': {
          'background': 'linear-gradient(180deg, #18181A 0%, #141416 100%)',
        },
        '.surface-elevated': {
          'background': 'linear-gradient(180deg, #202023 0%, #1A1A1C 100%)',
        },
        '.border-subtle': {
          'border': '1px solid rgba(255, 255, 255, 0.06)',
        },
        '.border-hover': {
          'border': '1px solid rgba(255, 255, 255, 0.12)',
        },
        '.gradient-text': {
          'background': 'linear-gradient(135deg, #FFFFFF 0%, rgba(255, 255, 255, 0.6) 100%)',
          '-webkit-background-clip': 'text',
          '-webkit-text-fill-color': 'transparent',
          'background-clip': 'text',
        },
        '.gradient-text-coral': {
          'background': 'linear-gradient(135deg, #FF6B5B 0%, #FF9589 100%)',
          '-webkit-background-clip': 'text',
          '-webkit-text-fill-color': 'transparent',
          'background-clip': 'text',
        },
      })
    },
  ],
}
