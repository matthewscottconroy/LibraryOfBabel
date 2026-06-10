# Section 6.3: Power Series and Radius of Convergence

---

## Section Introduction

A **power series** centered at a is a series of the form $\sum_{n=0}^\infty c_n (x-a)^n$. For fixed x, this is a series of constants; for varying x, it defines a function. Power series are the most important class of series in analysis because: they converge on an interval around the center; within that interval, they define smooth functions; and every analytic function is locally a power series. The functions eˣ, sin x, cos x, ln(1+x), and (1+x)^α are all power series, and their series representations are both a definition and a computational tool.

---

## 6.3.1 The Radius of Convergence

**Theorem** (Radius of Convergence): For any power series $\sum c_n (x-a)^n$, there exists R ∈ [0, ∞] (the **radius of convergence**) such that:
- The series converges absolutely for |x − a| < R.
- The series diverges for |x − a| > R.
- At |x − a| = R, anything can happen (must be checked case by case).

**Formula** (Cauchy-Hadamard): $\frac{1}{R} = \limsup_{n \to \infty} |c_n|^{1/n}$.

If this lim sup is 0, R = ∞ (convergent everywhere); if it is ∞, R = 0 (convergent only at x = a).

*Proof sketch*: The root test applied to cₙ(x−a)ⁿ gives root = |x−a| · (lim sup |cₙ|^{1/n}) = |x−a|/R. If this is < 1 (i.e., |x−a| < R), the series converges; if > 1, it diverges. □

**The ratio test formula**: If the limit lim |c_{n+1}/cₙ| = L exists, then R = 1/L.

---

## 6.3.2 Operations on Power Series

Within the radius of convergence, power series can be differentiated and integrated term by term, and the resulting series has the same radius of convergence.

**Theorem**: If $f(x) = \sum_{n=0}^\infty c_n (x-a)^n$ for |x−a| < R, then:

$$f'(x) = \sum_{n=1}^\infty n c_n (x-a)^{n-1}, \quad \int f(x) \, dx = C + \sum_{n=0}^\infty \frac{c_n}{n+1} (x-a)^{n+1}$$

both with radius of convergence R.

This is the most powerful property of power series — it makes them as easy to differentiate and integrate as polynomials.

**Proof** (differentiation): The rigorous proof uses uniform convergence (Section 6.4). The key: for any |x−a| < r < R, the series of derivatives converges uniformly on the disk of radius r, which justifies term-by-term differentiation. □

---

## 6.3.3 Taylor Series

**Theorem**: If $f(x) = \sum_{n=0}^\infty c_n (x-a)^n$ on |x−a| < R, then cₙ = f⁽ⁿ⁾(a)/n!. That is, the series *is* the Taylor series of f.

*Proof*: Differentiate term by term n times and evaluate at x = a: f⁽ⁿ⁾(a) = n! cₙ. □

**Converse (Taylor's theorem)**: Given a smooth function f, its Taylor series $\sum f⁽ⁿ⁾(a)/n! \cdot (x−a)^n$ may or may not converge to f. It converges to f iff the remainder Rₙ(x) → 0 as n → ∞.

For the standard elementary functions, Taylor series converge everywhere (eˣ, sin x, cos x have R = ∞) or on a known interval.

**Key expansions** (about a = 0):

$$e^x = \sum_{n=0}^\infty \frac{x^n}{n!} = 1 + x + \frac{x^2}{2!} + \frac{x^3}{3!} + \cdots \quad (R = \infty)$$

$$\sin x = \sum_{n=0}^\infty \frac{(-1)^n x^{2n+1}}{(2n+1)!} = x - \frac{x^3}{6} + \frac{x^5}{120} - \cdots \quad (R = \infty)$$

$$\cos x = \sum_{n=0}^\infty \frac{(-1)^n x^{2n}}{(2n)!} = 1 - \frac{x^2}{2} + \frac{x^4}{24} - \cdots \quad (R = \infty)$$

$$\ln(1+x) = \sum_{n=1}^\infty \frac{(-1)^{n-1} x^n}{n} = x - \frac{x^2}{2} + \frac{x^3}{3} - \cdots \quad (R = 1)$$

$$(1+x)^\alpha = \sum_{n=0}^\infty \binom{\alpha}{n} x^n \quad (R = 1 \text{ for } \alpha \notin \mathbb{N}_0)$$

where $\binom{\alpha}{n} = \frac{\alpha(\alpha-1)\cdots(\alpha-n+1)}{n!}$ is the generalized binomial coefficient.

---

## 6.3.4 Euler's Formula

Perhaps the most beautiful formula in mathematics:

$$e^{i\theta} = \cos\theta + i\sin\theta$$

*Proof*: Substitute iθ into the power series for eˣ:

$$e^{i\theta} = \sum_{n=0}^\infty \frac{(i\theta)^n}{n!} = 1 + i\theta - \frac{\theta^2}{2!} - i\frac{\theta^3}{3!} + \frac{\theta^4}{4!} + \cdots$$

Collecting real and imaginary parts gives the cosine and sine series. □

Setting θ = π: $e^{i\pi} = -1$, or $e^{i\pi} + 1 = 0$ — Euler's identity, connecting e, i, π, 1, and 0 in one equation.

Euler's formula is far more than an aesthetic curiosity. It establishes that the complex exponential is periodic (period 2π), that complex multiplication corresponds to rotation in ℂ, and that all oscillatory behavior can be encoded as real parts of complex exponentials. In GR, the polarization of gravitational waves is described using complex spin-2 quantities; the Newman-Penrose formalism uses complex null tetrads; and analytic continuation (extending real functions to complex domains) is used in the derivation of Hawking radiation.

---

## 6.3.5 Applications in Physics

**Weak-field gravity**: The Schwarzschild metric involves (1 − 2GM/rc²)^{-1}. For r >> 2GM/c², this expands as $1 + 2GM/rc^2 + (2GM/rc^2)^2 + \cdots$ using the geometric series. The leading term gives Newtonian gravity; higher terms give post-Newtonian corrections. [Will, C.M. (1993). *Theory and Experiment in Gravitational Physics*. Chapter 4.]

**Gravitational wave polarization**: A general gravitational wave is $h_{μν} = \text{Re}[H_{μν} e^{ik_\alpha x^\alpha}]$, where the complex exponential encodes both amplitude and phase. The real and imaginary parts are the two polarization modes. Euler's formula is the tool.

**Dirichlet series and prime numbers**: The Riemann zeta function ζ(s) = $\sum_{n=1}^\infty n^{-s}$ is a power series in e^{-s ln n}. Its properties — particularly the Riemann hypothesis about its zeros — are connected to the distribution of prime numbers. In string theory, the Casimir energy calculation involves sums $\sum n = -1/12$ (by zeta function regularization): this formal identity is given precise meaning via analytic continuation of ζ(s) to s = −1.

---

## References

- Euler, L. (1748). *Introductio in analysin infinitorum*. [Contains Euler's formula and systematic treatment of infinite series and products.]
- Hadamard, J. (1892). "Essai sur l'étude des fonctions données par leur développement de Taylor." *Journal de mathématiques pures et appliquées*, 4, 101–186. [The Cauchy-Hadamard formula for the radius of convergence.]
- Rudin, W. (1976). *Principles of Mathematical Analysis*, 3rd ed. McGraw-Hill. [Chapter 3, power series section.]
- Will, C.M. (1993). *Theory and Experiment in Gravitational Physics*. Cambridge University Press. [Post-Newtonian expansions using power series in small parameters.]
