# Chapter 6 Important Researchers

---

## Nicole d'Oresme (c. 1320–1382)

**Nationality**: French  
**Key work**: First proof of divergence of the harmonic series (ca. 1350)

Oresme was a medieval philosopher, theologian, and natural scientist at the University of Paris. In his geometric and algebraic work, he proved that the harmonic series 1 + 1/2 + 1/3 + ⋯ diverges — more than 300 years before the formal development of calculus — using the grouping argument we still teach today.

His proof is a model of clarity: group the terms as 1 + 1/2 + (1/3 + 1/4) + (1/5 + ⋯ + 1/8) + ⋯, observe that each group exceeds 1/2, and conclude the sum grows without bound. This requires no algebra, only careful reasoning about inequalities.

Oresme also developed a graphical representation of variables as areas — a precursor to the concept of a function's graph — and was among the first to apply mathematics systematically to motion. His influence on Galileo and Newton was indirect but real.

---

## Leonhard Euler (1707–1783)

*(See Chapter 5 for full biography.)*

For Chapter 6, Euler's central contributions are:
- **The Basel problem** (1734): proved Σ_{n=1}^∞ 1/n² = π²/6, a startling connection between a simple arithmetic series and the circle constant. This was one of the most celebrated results of the 18th century.
- **Euler's formula** e^{iθ} = cos θ + i sin θ (1748): derived from power series in *Introductio in analysin infinitorum*. This unification of exponential and trigonometric functions is the foundation of Fourier analysis and complex methods in physics.
- **Euler's identity** e^{iπ} + 1 = 0.
- Systematic development of power series methods, including the first comprehensive treatment of Taylor/Maclaurin series.

---

## Augustin-Louis Cauchy (1789–1857)

*(See Chapter 4 for full biography.)*

For Chapter 6: Cauchy gave the first rigorous definitions of convergence for sequences and series in the *Cours d'analyse* (1821). His **Cauchy criterion** for sequences — converges iff Cauchy — is both a theorem (in ℝ) and a definition (in general metric spaces). He also proved the ratio test and developed the theory of power series convergence, including the Cauchy-Hadamard radius of convergence formula.

---

## Bernhard Riemann (1826–1866)

*(See Chapter 5 for full biography.)*

For Chapter 6: Riemann's 1854 Habilitation lecture on Fourier series is the source of the Riemann integral (Chapter 5) and of the **Riemann rearrangement theorem** — the stunning result that conditionally convergent series can be rearranged to converge to any value. The rearrangement theorem is a direct consequence of Riemann's analysis of which functions have Fourier series representations.

---

## Karl Weierstrass (1815–1897)

*(See Chapter 4 for full biography.)*

For Chapter 6: The **Weierstrass M-test** for uniform convergence (1861), the concept of uniform convergence itself (developed in the late 1850s–1860s), and the **Weierstrass approximation theorem** (1885) — that every continuous function on [a,b] can be uniformly approximated by polynomials — are Weierstrass's contributions to this chapter.

Uniform convergence was first clearly recognized by Weierstrass as the correct condition for interchanging limits with integrals and derivatives. Earlier mathematicians (including Cauchy) had made errors by assuming pointwise convergence sufficed. Weierstrass's precise formulation set the standard for 20th-century analysis.

---

## Peter Gustav Lejeune Dirichlet (1805–1859)

**Nationality**: German (Prussian)  
**Key work**: Fourier series convergence theorem (1829); Dirichlet series; proof of Dirichlet's theorem on primes in arithmetic progressions

Dirichlet gave the first rigorous theorem on the convergence of Fourier series: if f is piecewise smooth (or satisfies the "Dirichlet conditions"), then its Fourier series converges to f(x) at points of continuity, and to the average (f(x⁺) + f(x⁻))/2 at jump discontinuities.

This result motivated the development of uniform convergence (to understand what "convergence" of the Fourier series means) and eventually led Riemann and others to ask: for which functions do Fourier series converge? This question drove the development of measure theory, Lebesgue integration, and functional analysis.

In number theory, Dirichlet proved that every arithmetic progression aₙ = a + nd (with gcd(a,d) = 1) contains infinitely many primes. His proof used **Dirichlet series** Σ f(n) n^{-s} — the ancestors of the Riemann zeta function — and complex analysis. This was the first use of analytic methods in number theory, inaugurating analytic number theory.

---

## Colin Maclaurin (1698–1746)

**Nationality**: Scottish  
**Key work**: *Treatise of Fluxions* (1742); Maclaurin series; Euler-Maclaurin formula

Maclaurin gave the first systematic account of Newton's calculus using the series representation (the "Maclaurin series" — Taylor series about the origin). His *Treatise* was the first fully rigorous exposition of Newton's methods, written partly to defend Newton against Berkeley's philosophical attacks on the infinitesimals.

The **Euler-Maclaurin formula** (developed independently by Euler and Maclaurin in 1735) connects discrete sums to integrals: Σ_{k=1}^n f(k) ≈ ∫₁ⁿ f(x) dx + correction terms involving Bernoulli numbers. This formula has applications in computing series sums, in the Casimir effect in quantum field theory, and in zeta function regularization.
