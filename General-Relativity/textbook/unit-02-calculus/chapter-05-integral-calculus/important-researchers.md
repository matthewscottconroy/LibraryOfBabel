# Chapter 5 Important Researchers

---

## Archimedes of Syracuse (c. 287–212 BCE)

**Nationality**: Greek (Syracuse, Sicily)  
**Key work**: Method of exhaustion; computation of areas and volumes

Archimedes was the greatest mathematician of antiquity and, arguably, of any era before Newton. He computed the area of a parabolic segment by "exhausting" it with inscribed triangles — a limiting process that prefigures Riemann sums by 2,000 years. He also computed the volume of a sphere, the area of a sphere's surface, the center of gravity of various figures, and the volume of a paraboloid — all using limiting arguments.

His *Method*, rediscovered in 1906 on a palimpsest in Constantinople, reveals that he thought of surfaces and volumes as composed of infinitely many infinitesimal cross-sections — an essentially correct intuition, but one he could not make rigorous. His *Quadrature of the Parabola* contains a rigorous proof (by contradiction and exhaustion) that the area of a parabolic segment is 4/3 the area of the inscribed triangle — the same result calculus gives immediately.

Archimedes did not have calculus. But he had the ideas. The 1,900 years between Archimedes and Newton produced no mathematical progress on integration of comparable depth.

---

## Isaac Newton (1643–1727) and Gottfried Wilhelm Leibniz (1646–1716)

*(See Chapter 4 profiles for full biographies.)*

The Fundamental Theorem of Calculus — the great synthesis of differentiation and integration — was discovered by both Newton and Leibniz independently. Newton saw it as a result about the relationship between his method of fluxions and his method of quadratures. Leibniz saw it through his notation: if F' = f, then ∫f = F — the integral symbol ∫ is an elongated S for "summa" (sum), and the derivative undoes the summation.

---

## Bernhard Riemann (1826–1866)

**Nationality**: German  
**Key work**: Riemann integral (1854), Riemann curvature tensor (1854), Riemann hypothesis (1859)

Riemann was one of the most original mathematicians in history and one of the most relevant to this book — the geometric framework of GR is his. His Habilitation lecture of 1854, "*Über die Hypothesen, welche der Geometrie zu Grunde liegen*" ("On the Hypotheses Which Lie at the Foundations of Geometry"), introduced the concept of a Riemannian manifold and the intrinsic curvature tensor that bears his name.

In the same year, a separate paper introduced what we call the Riemann integral — defined to settle questions about Fourier series representation of functions. The paper revealed a precise condition for integrability (the oscillation condition, equivalent to Lebesgue's measure-zero discontinuity condition) and gave examples of integrable functions with infinitely many discontinuities.

Riemann died of tuberculosis at 39, in Italy where he had gone seeking recovery. Despite the brevity of his career, his published papers span topics from number theory (the Riemann hypothesis, perhaps the most famous unsolved problem in mathematics) to complex analysis (Riemann surfaces, Cauchy-Riemann equations) to differential geometry (Riemannian manifolds) to physics (the Riemann integral, shock waves, electromagnetic theory). His geometric ideas, dormant for 60 years, became the foundation of Einstein's general relativity in 1915.

---

## Henri Lebesgue (1875–1941)

**Nationality**: French  
**Key work**: Lebesgue measure and integral (doctoral thesis, 1902)

Lebesgue's integral is more powerful than Riemann's. Its key advantage: the Lebesgue integral interacts well with limits. If fₙ → f pointwise and the fₙ are uniformly bounded, then ∫fₙ → ∫f (dominated convergence theorem). This fails for Riemann integrals. The consequence is that Lebesgue integration works seamlessly with function spaces and functional analysis, which is the foundation of quantum mechanics and quantum field theory.

Lebesgue also developed measure theory — a precise notion of the "size" of arbitrary subsets of ℝ. Measure theory is the foundation of modern probability theory (Kolmogorov, 1933) and of the Lebesgue integral.

The Lebesgue integral is developed in Chapter 11. For a first course in calculus, Riemann's approach suffices; but Lebesgue's is ultimately indispensable for advanced analysis and physics.

---

## Leonhard Euler (1707–1783)

**Nationality**: Swiss (worked in Basel, Berlin, Saint Petersburg)  
**Key work**: Systematic development of integration techniques; Gamma function (1729–1730)

Euler was the most prolific mathematician in history (his collected works fill over 80 volumes). He systematized integral calculus, discovering many of the techniques in Section 5.3: trigonometric substitutions, partial fractions, reduction formulas. He introduced the Gamma function Γ(s) in 1729–1730 as an extension of the factorial to non-integer arguments, and showed Γ(n) = (n−1)!.

Euler's approach was formal and computationally brilliant, but not always rigorous by modern standards. He would routinely interchange limits and integrals, sum divergent series, and perform other operations that require careful justification. He was almost always right. When he was wrong, he was wrong in interesting ways that drove the development of rigorous analysis.

---

## Andrei Nikolaevich Kolmogorov (1903–1987)

**Nationality**: Soviet/Russian  
**Key work**: Foundations of probability theory (*Grundbegriffe der Wahrscheinlichkeitsrechnung*, 1933); turbulence theory; algorithmic complexity

Kolmogorov put probability theory on a rigorous axiomatic foundation using Lebesgue measure theory. A probability space is a measure space with total measure 1; a random variable is a measurable function; expected values are integrals. This framework made modern statistics, information theory, and stochastic processes mathematically rigorous.

In the context of this book, Kolmogorov's work is relevant to: stochastic approaches to quantum field theory (the path integral is formally a measure on a function space), the theory of turbulence (relevant to astrophysical fluid dynamics), and the connection between measure theory and the Lebesgue integration of Chapter 11.
