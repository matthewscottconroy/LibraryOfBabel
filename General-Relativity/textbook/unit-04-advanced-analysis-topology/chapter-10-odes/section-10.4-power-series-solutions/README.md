# Section 10.4: Power Series Solutions and Special Functions

---

## Section Introduction

Many ODEs arising in physics — the Legendre equation, the Bessel equation, the hypergeometric equation, the Hermite equation — cannot be solved in terms of elementary functions. But they can be solved by **power series**: assume a solution of the form $y = \sum_{n=0}^\infty a_n x^n$, substitute into the ODE, and determine the coefficients by matching powers of x.

This method, the **Frobenius method**, produces series solutions near **regular singular points** — points where the ODE has a singularity, but of a mild enough kind that power series (or slightly generalized series) still work. The solutions so produced are the **special functions of mathematical physics**: Legendre polynomials, spherical harmonics, Bessel functions, and their cousins. These functions appear throughout physics because they are the natural eigenfunctions of the angular parts of differential operators in spherical and cylindrical symmetry.

In GR, the perturbation theory of black holes leads to Regge-Wheeler and Teukolsky equations whose solutions are these same special functions, extended to more complicated potentials. The structure of the quasi-normal mode spectrum depends on the analytic properties of these solutions near the horizon and at infinity.

---

## 10.4.1 Power Series Solutions at Ordinary Points

**Definition**: A point x₀ is an **ordinary point** of y'' + P(x)y' + Q(x)y = 0 if P and Q are analytic (have convergent power series) near x₀. A point where P or Q is singular is a **singular point**.

**Theorem**: Near an ordinary point x₀, every solution is analytic. There exist two linearly independent power series solutions $y = \sum_{n=0}^\infty a_n (x - x_0)^n$, each converging in the largest disk around x₀ free of singularities of P and Q.

**Method**: Substitute $y = \sum a_n x^n$ (setting x₀ = 0 for simplicity), compute y' and y'', substitute into the ODE, collect powers of x, and set each coefficient to zero. This gives a **recurrence relation** for the aₙ.

**Example** (Airy equation): y'' = xy. Substituting $y = \sum a_n x^n$:
- $y'' = \sum_{n=2}^\infty n(n-1) a_n x^{n-2} = \sum_{m=0}^\infty (m+2)(m+1) a_{m+2} x^m$
- $xy = \sum_{n=0}^\infty a_n x^{n+1} = \sum_{m=1}^\infty a_{m-1} x^m$

Setting coefficients equal:
- m = 0: 2·1·a₂ = 0, so a₂ = 0.
- m ≥ 1: (m+2)(m+1) a_{m+2} = a_{m-1}.

Recurrence: $a_{m+2} = a_{m-1}/[(m+2)(m+1)]$ — each coefficient is determined by the one three steps back.

Two independent solutions (choosing a₀ = 1, a₁ = 0 and a₀ = 0, a₁ = 1) give the **Airy functions** Ai(x) and Bi(x). These appear in quantum mechanics (wave functions near a turning point), in gravitational lensing (diffraction near caustics), and in the asymptotic analysis of oscillatory integrals.

---

## 10.4.2 The Frobenius Method and Regular Singular Points

**Definition**: A singular point x₀ of y'' + P(x)y' + Q(x)y = 0 is a **regular singular point** if (x−x₀)P(x) and (x−x₀)²Q(x) are analytic near x₀. Otherwise it is an **irregular singular point**.

At a regular singular point, solutions may not be analytic but have the form:

$$y(x) = (x - x_0)^r \sum_{n=0}^\infty a_n (x - x_0)^n = \sum_{n=0}^\infty a_n (x - x_0)^{n+r}$$

where r is determined by the **indicial equation**.

**Indicial equation**: Writing (x−x₀)P(x) = p₀ + p₁(x−x₀) + ... and (x−x₀)²Q(x) = q₀ + q₁(x−x₀) + ..., the indicial equation is:

$$r(r-1) + p_0 r + q_0 = 0$$

The two roots r₁ ≥ r₂ of this quadratic give two solutions:
- If r₁ − r₂ ∉ ℤ (non-integer difference): two Frobenius series solutions.
- If r₁ = r₂ (repeated root): second solution involves y₁ ln(x−x₀).
- If r₁ − r₂ ∈ ℤ⁺: second solution may involve y₁ ln(x−x₀) plus a Frobenius series.

**The Fuchsian class**: Linear ODEs with only regular singular points (and the point at infinity treated carefully) are called **Fuchsian equations**. The hypergeometric equation is the canonical Fuchsian equation with three regular singular points (at 0, 1, and ∞), and nearly all special functions of mathematical physics arise as its special cases.

---

## 10.4.3 Legendre's Equation and Spherical Harmonics

**Legendre's equation** arises when solving Laplace's equation ∇²f = 0 in spherical coordinates. Separation of variables f = R(r)Y(θ,φ) gives the angular part Y satisfying:

$$\frac{1}{\sin\theta}\frac{\partial}{\partial\theta}\left(\sin\theta\frac{\partial Y}{\partial\theta}\right) + \frac{1}{\sin^2\theta}\frac{\partial^2 Y}{\partial\phi^2} = -\ell(\ell+1) Y$$

Further separating Y = Θ(θ)Φ(φ) and setting u = cos θ: the Θ equation becomes:

$$\frac{d}{du}\left[(1-u^2)\frac{d\Theta}{du}\right] + \left[\ell(\ell+1) - \frac{m^2}{1-u^2}\right]\Theta = 0$$

This is the **associated Legendre equation**. For m = 0: the **Legendre equation** $(1-u^2)y'' - 2uy' + \ell(\ell+1)y = 0$.

Singular points at u = ±1 (θ = 0, π — the poles). Regular singular points; indicial equation gives r = 0 or r = 1 at each pole. For solutions to be finite at both poles, ℓ must be a non-negative integer.

**Legendre polynomials** P_ℓ(u) for ℓ = 0, 1, 2, ...:
- P₀ = 1
- P₁ = u = cos θ
- P₂ = (3u² − 1)/2 = (3cos²θ − 1)/2
- P₃ = (5u³ − 3u)/2

Rodrigues' formula: $P_\ell(u) = \frac{1}{2^\ell \ell!} \frac{d^\ell}{du^\ell}(u^2 - 1)^\ell$.

**Orthogonality**: $\int_{-1}^1 P_\ell(u) P_{\ell'}(u) du = \frac{2}{2\ell+1} \delta_{\ell\ell'}$.

**Spherical harmonics**: $Y^\ell_m(\theta, \phi) = N_{\ell m} P_\ell^m(\cos\theta) e^{im\phi}$ where N_{ℓm} is a normalization constant. The Y^ℓ_m form a complete orthonormal basis for L²(S²) — every square-integrable function on the sphere expands in spherical harmonics.

**GR applications**:
- Multipole expansion of the gravitational field of a body: V(r,θ) = Σ_{ℓ} (A_ℓ/r^{ℓ+1}) P_ℓ(cos θ).
- Perturbations of black holes expand in spherical harmonics: the quasi-normal modes are labeled by (ℓ, m).
- Gravitational waves are predominantly ℓ = 2 (quadrupole) radiation.
- The CMB temperature fluctuations are expanded in spherical harmonics; the C_ℓ power spectrum encodes cosmological information.

---

## 10.4.4 Bessel's Equation

**Bessel's equation** arises from Laplace's equation or the wave equation in cylindrical coordinates. For a function f = R(r)Φ(φ)Z(z) in cylindrical coordinates, separation gives:

$$r^2 R'' + r R' + (k^2 r^2 - n^2) R = 0$$

With the substitution x = kr, this becomes: $x^2 y'' + xy' + (x^2 - n^2)y = 0$.

Regular singular point at x = 0. Indicial equation: r² − n² = 0, roots r = ±n.

**Bessel functions of the first kind**: For integer n,

$$J_n(x) = \sum_{k=0}^\infty \frac{(-1)^k}{k!\,(k+n)!} \left(\frac{x}{2}\right)^{2k+n}$$

**Bessel functions of the second kind** Y_n(x): the second linearly independent solution, singular at x = 0.

**Asymptotic behavior**: For large x, $J_n(x) \approx \sqrt{2/(\pi x)} \cos(x - n\pi/2 - \pi/4)$ — oscillatory with algebraically decaying amplitude. For small x: $J_n(x) \approx x^n/(2^n n!)$.

**GR application**: Gravitational waves from a source in cylindrical symmetry (like cosmic strings) are described by Bessel functions. The Green's function for the wave equation in 3D (used to compute gravitational wave emission) involves Bessel functions via Fourier-Bessel transforms.

---

## 10.4.5 Connection to Quasi-Normal Modes

The Regge-Wheeler equation for perturbations of a Schwarzschild black hole is:

$$\frac{d^2\Psi}{dr_*^2} + \left[\omega^2 - V_\ell(r)\right]\Psi = 0$$

where $r_* = r + r_s \ln|r/r_s - 1|$ is the tortoise coordinate and V_ℓ(r) is the Regge-Wheeler potential. Near the horizon (r → r_s, r* → −∞), V_ℓ → 0 and the equation becomes $\ddot\Psi + \omega^2 \Psi = 0$ — plane waves. Near infinity (r* → +∞), again V_ℓ → 0.

**Quasi-normal modes** are solutions with purely ingoing boundary conditions at the horizon and purely outgoing at infinity. These are analogues of the resonance conditions for Sturm-Liouville problems — they select discrete complex frequencies ω_n whose real parts give oscillation frequencies and imaginary parts give damping rates. The computation requires the full analytic theory of the Frobenius method applied to the singular points of the Regge-Wheeler equation.

[Chandrasekhar, S. (1983). *The Mathematical Theory of Black Holes.* Oxford University Press. §§4.1–4.4 on Regge-Wheeler and Zerilli equations.]

---

## References

- Arfken, G.B., Weber, H.J., and Harris, F.E. (2013). *Mathematical Methods for Physicists*, 7th ed. Academic Press. [Chapters 7–8 on Sturm-Liouville theory and special functions; Chapter 11 on Bessel functions; Chapter 15 on Legendre polynomials.]
- Abramowitz, M. and Stegun, I.A. (1965). *Handbook of Mathematical Functions.* National Bureau of Standards. [The complete reference for special functions: definitions, recurrence relations, asymptotic expansions, and tables. Freely available online.]
- Chandrasekhar, S. (1983). *The Mathematical Theory of Black Holes.* Oxford University Press. [The most complete treatment of perturbation theory for black holes; derives the Regge-Wheeler and Teukolsky equations and analyzes them by Frobenius methods.]
- Bessel, F.W. (1824). "Untersuchung des Theils der planetarischen Störungen, welcher aus der Bewegung der Sonne entsteht." *Abhandlungen der Berliner Akademie*, 1–52. [The original paper introducing Bessel functions, in the context of planetary perturbations.]
- Legendre, A.-M. (1782). "Recherches sur l'attraction des sphéroïdes homogènes." *Mémoires de Mathématique et de Physique, présentés à l'Académie Royale des Sciences*, 10, 411–435. [Introduces the Legendre polynomials to solve the gravitational potential of an oblate spheroid.]
