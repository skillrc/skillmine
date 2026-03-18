import Hero from './sections/Hero'
import Problem from './sections/Problem'
import HowItWorks from './sections/HowItWorks'
import WhySkillmine from './sections/WhySkillmine'
import CTA from './sections/CTA'
import Footer from './sections/Footer'

export default function Home() {
  return (
    <main>
      <Hero />
      <Problem />
      <HowItWorks />
      <WhySkillmine />
      <CTA />
      <Footer />
    </main>
  )
}
