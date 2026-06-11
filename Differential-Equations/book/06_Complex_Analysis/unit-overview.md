# Unit Overview: Complex Analysis

## The Unreasonable Power of Analyticity

Complex analysis is a subject of singular beauty. Its starting point — extending calculus to complex-valued functions of a complex variable — is natural enough. But the consequences of this extension are astonishing. A function that is once differentiable in the complex sense turns out to be infinitely differentiable, representable as a convergent power series, and subject to global constraints with no real-variable analogue. The entire behavior of an analytic function throughout a domain is determined by its values on any disk, or even on any curve, inside that domain — a rigidity utterly unlike anything in real analysis.

This rigidity is not a limitation; it is a source of power. Because analytic functions are so constrained, they can be computed, evaluated, and estimated with extraordinary precision. Contour integrals can be evaluated by residues, turning integrals that would require pages of real-variable calculation into one-line applications of the residue theorem. Conformal mappings transform complicated domains into simple ones, allowing boundary value problems for Laplace's equation to be transplanted into geometries where they are trivially solved. The Riemann mapping theorem asserts that any simply connected domain (other than all of $\mathbb{C}$) looks the same to complex analysis as the unit disk — a striking uniformity theorem.

For the study of differential equations, complex analysis is indispensable in several ways. The Cauchy integral formula gives a representation for analytic functions that is the complex analogue of Green's function representations. The theory of singularities — poles, essential singularities, branch points — classifies the behavior of solutions to ODEs at singular points and gives the starting point for the Frobenius method. The Laplace and Fourier transforms, which solve PDEs, are both contour integrals in disguise, and their inversion formulas require deforming contours in the complex plane by Cauchy's theorem.

## Cauchy's Theorem and Its Consequences

**Theorem (Cauchy's Theorem).** Let $f$ be analytic (holomorphic) on a simply connected open domain $\Omega \subset \mathbb{C}$. Then for any closed curve $C$ in $\Omega$:
$$\oint_C f(z)\,dz = 0.$$

This is the complex analogue of the statement that a conservative vector field has zero circulation. The Cauchy-Riemann equations $u_x = v_y$, $u_y = -v_x$ (where $f = u + iv$) translate complex differentiability into a condition on the real Jacobian: it must be a rotation-scaling matrix. Green's theorem applied to this condition yields Cauchy's theorem.

**Theorem (Cauchy Integral Formula).** If $f$ is analytic on and inside a simple closed curve $C$ traversed counterclockwise, and $z_0$ is inside $C$:
$$f(z_0) = \frac{1}{2\pi i}\oint_C \frac{f(z)}{z - z_0}\,dz.$$

And for derivatives of all orders:
$$f^{(n)}(z_0) = \frac{n!}{2\pi i}\oint_C \frac{f(z)}{(z-z_0)^{n+1}}\,dz.$$

The Cauchy integral formula is one of the deepest theorems in all of analysis. It says that the values of an analytic function in the interior of a region are completely determined by its values on the boundary — a maximum-principle type statement, here valid in exact rather than approximate form. Moreover, all derivatives of $f$ exist and are themselves analytic, which is the key step in proving that analyticity (once-differentiable) is equivalent to being a convergent power series.

**Theorem (Liouville's Theorem).** Every bounded entire function (analytic on all of $\mathbb{C}$) is constant.

**Proof:** If $f$ is entire and $|f| \leq M$, then by the Cauchy estimate $|f'(z_0)| \leq M/R$ for any $R > 0$. Taking $R \to \infty$ gives $f'(z_0) = 0$ for all $z_0$, hence $f$ is constant.

**Corollary (Fundamental Theorem of Algebra).** Every nonconstant polynomial has a root in $\mathbb{C}$.

**Proof:** If $p(z)$ has no root, then $1/p(z)$ is entire. For large $|z|$, $|p(z)| \to \infty$, so $1/p$ is bounded. By Liouville, $1/p$ is constant, contradiction.

## Taylor and Laurent Series; Singularities

**Theorem.** Every analytic function $f$ on an open set $U$ is equal to its Taylor series at each point: $f(z) = \sum_{n=0}^\infty a_n(z-z_0)^n$ on any disk $B_R(z_0) \subset U$, where $a_n = f^{(n)}(z_0)/n! = \frac{1}{2\pi i}\oint \frac{f(z)}{(z-z_0)^{n+1}}\,dz$.

On an annulus $r < |z - z_0| < R$, an analytic function has a Laurent series:
$$f(z) = \sum_{n=-\infty}^\infty c_n(z-z_0)^n.$$

The principal part $\sum_{n=-\infty}^{-1} c_n(z-z_0)^n$ determines the nature of the singularity at $z_0$:
- **Removable singularity:** all $c_n = 0$ for $n < 0$; $f$ extends analytically to $z_0$.
- **Pole of order $m$:** $c_{-m} \neq 0$ and $c_n = 0$ for $n < -m$; $|f(z)| \to \infty$ as $z\to z_0$.
- **Essential singularity:** infinitely many nonzero $c_n$ for $n < 0$; by the Casorati-Weierstrass theorem, $f$ takes values arbitrarily close to every complex number in every punctured neighborhood of $z_0$.

## The Residue Theorem

**Definition.** The residue of $f$ at an isolated singularity $z_0$ is the Laurent coefficient $c_{-1}$:
$$\text{Res}(f; z_0) = \frac{1}{2\pi i}\oint_{|z-z_0|=\epsilon} f(z)\,dz.$$

For a simple pole: $\text{Res}(f;z_0) = \lim_{z\to z_0}(z-z_0)f(z)$.
For a pole of order $m$: $\text{Res}(f;z_0) = \frac{1}{(m-1)!}\lim_{z\to z_0}\frac{d^{m-1}}{dz^{m-1}}[(z-z_0)^m f(z)]$.

**Theorem (Residue Theorem).** If $f$ is analytic on and inside a simple closed curve $C$ except at isolated singularities $z_1, \ldots, z_k$ inside $C$:
$$\oint_C f(z)\,dz = 2\pi i \sum_{j=1}^k \text{Res}(f; z_j).$$

The residue theorem converts contour integrals into algebra. Its applications include:

**Real integrals of rational functions:** $\int_{-\infty}^\infty R(x)\,dx$ for rational $R$ with no real poles, computed by closing the contour in the upper half-plane.

**Trigonometric integrals:** $\int_0^{2\pi} R(\cos\theta, \sin\theta)\,d\theta$ converted to $\oint_{|z|=1} R\!\left(\frac{z+z^{-1}}{2}, \frac{z-z^{-1}}{2i}\right)\frac{dz}{iz}$.

**Summing series:** $\sum_{n=-\infty}^\infty f(n) = -\sum_{\text{non-integer poles}} \text{Res}(\pi\cot(\pi z)f(z))$.

## Analytic Continuation

**Theorem (Identity Theorem).** If $f, g$ are analytic on a connected open set $\Omega$ and $f = g$ on any sequence with a limit point in $\Omega$, then $f = g$ on all of $\Omega$.

This means an analytic function cannot be "partially altered": if you know it on any open set or any converging sequence, you know it everywhere.

**Analytic Continuation.** If $f$ is analytic on $\Omega$ and $\tilde{f}$ is analytic on a larger domain $\tilde{\Omega} \supset \Omega$ with $\tilde{f} = f$ on $\Omega$, then $\tilde{f}$ is the unique analytic continuation of $f$. Analytic continuation provides the means to extend a function defined by one formula (e.g., the Euler Gamma function defined by a convergent integral for $\text{Re}(z) > 0$) to a larger domain.

The Riemann zeta function $\zeta(s) = \sum_{n=1}^\infty n^{-s}$ (convergent for $\text{Re}(s) > 1$) has an analytic continuation to all of $\mathbb{C}$ except for a simple pole at $s=1$. The location of its zeros in the critical strip $0 < \text{Re}(s) < 1$ — the Riemann Hypothesis — is the most famous unsolved problem in mathematics.

## Conformal Mappings

A map $f : \Omega \to \Omega'$ is conformal if it is analytic with $f'(z) \neq 0$: it preserves angles and local orientation. The key property is that if $u$ is harmonic in $\Omega'$, then $u \circ f$ is harmonic in $\Omega$ (harmonic functions are preserved under conformal maps). This allows the Dirichlet problem $\Delta u = 0$ on $\Omega$ to be transplanted to a simpler domain.

**Möbius Transformations.** Maps of the form $f(z) = (az+b)/(cz+d)$ with $ad-bc \neq 0$ form the group $PSL(2,\mathbb{C})$. They are the only bijective conformal maps of the Riemann sphere $\mathbb{C}\cup\{\infty\}$ to itself. Every Möbius transformation maps circles/lines to circles/lines and preserves the cross-ratio. Möbius transformations map the upper half-plane to the disk (and vice versa), which is the conformal equivalence that allows Dirichlet problems to be interchanged between these domains.

**Schwarz-Christoffel Formula.** A conformal map from the upper half-plane $\mathbb{H}$ to a polygon with vertices $w_1, \ldots, w_n$ and interior angles $\alpha_k\pi$ is given by
$$f(z) = C_1 \int^z \prod_{k=1}^n (\zeta - x_k)^{\alpha_k - 1}\,d\zeta + C_2$$
where $x_1 < \cdots < x_{n-1}$ are real prevertices (with $x_n = \infty$ if desired). This formula is used in engineering to compute electrostatic fields in polygonal geometries.

**Theorem (Riemann Mapping Theorem).** Let $\Omega$ be a simply connected proper open subset of $\mathbb{C}$. Then there exists a bijective conformal map $f : \Omega \to \mathbb{D}$ (the unit disk), unique up to the choice of $f(z_0)$ and $\arg f'(z_0)$ for any fixed $z_0 \in \Omega$.

This theorem is one of the great existence theorems in mathematics. Its proof proceeds by compactness (Montel's theorem: a family of uniformly bounded analytic functions is normal, i.e., has a uniformly convergent subsequence) combined with an extremal argument.

## Worked Examples

### Example 1: Residue Computation

Compute $I = \int_{-\infty}^\infty \frac{x^2}{(x^2+1)(x^2+4)}\,dx$.

Close the contour with a semicircle in the upper half-plane. Poles in the upper half-plane: $z = i$ (from $z^2+1=0$) and $z = 2i$ (from $z^2+4=0$).

At $z = i$: $\text{Res}\!\left(\frac{z^2}{(z^2+1)(z^2+4)};i\right) = \lim_{z\to i}\frac{(z-i)z^2}{(z-i)(z+i)(z^2+4)} = \frac{-1}{(2i)(3)} = \frac{-1}{6i} = \frac{i}{6}$.

At $z = 2i$: $\text{Res} = \lim_{z\to 2i}\frac{(z-2i)z^2}{(z^2+1)(z-2i)(z+2i)} = \frac{-4}{(-3)(4i)} = \frac{1}{3i} = \frac{-i}{3}$.

$I = 2\pi i\left(\frac{i}{6} + \frac{-i}{3}\right) = 2\pi i\cdot\frac{-i}{6} = 2\pi i\cdot\frac{-i}{6} = \frac{\pi}{3}$.

### Example 2: Conformal Mapping for a Boundary Value Problem

Find a harmonic function $u$ on the upper half-plane $y > 0$ with $u(x,0) = 1$ for $|x| < 1$ and $u(x,0) = 0$ for $|x| > 1$.

Use the Möbius transformation $w = (1+z)/(1-z)$ mapping the unit disk to the right half-plane (not directly helpful). Better: use the map $w = \arcsin(z)/\pi$ (normalized) mapping the upper half-plane to the strip. 

Alternatively, directly: the Poisson integral formula for the half-plane gives
$$u(x,y) = \frac{y}{\pi}\int_{-\infty}^\infty \frac{f(t)}{(x-t)^2+y^2}\,dt = \frac{1}{\pi}\int_{-1}^1 \frac{y}{(x-t)^2+y^2}\,dt.$$
Evaluating: $u(x,y) = \frac{1}{\pi}\left[\arctan\!\frac{1-x}{y} + \arctan\!\frac{1+x}{y}\right]$.

### Example 3: Analytic Continuation

The function $f(z) = \sum_{n=0}^\infty z^n = 1/(1-z)$ is defined by its power series only for $|z| < 1$. The formula $1/(1-z)$ is its analytic continuation to all of $\mathbb{C}\setminus\{1\}$. Expanding around a new center $z_0 = -1$: $1/(1-z) = 1/(2+w)$ where $w = z+1$, giving $\sum_{n=0}^\infty (-1)^n w^n / 2^{n+1}$, convergent for $|z+1| < 2$.

## Historical Notes

**Leonhard Euler (1707–1783)** established Euler's formula $e^{i\theta} = \cos\theta + i\sin\theta$ and worked with complex numbers in the study of real integrals, though without a systematic theory.

**Carl Friedrich Gauss (1777–1855)** understood the geometric interpretation of complex numbers (the complex plane, now called the Argand plane or Gaussian plane) and gave the first complete proof of the Fundamental Theorem of Algebra in his doctoral dissertation (1799). Gauss worked with analytic functions and recognized the central role of the Cauchy-Riemann equations, though he never published these ideas systematically.

**Augustin-Louis Cauchy (1789–1857)** created complex analysis as a systematic theory. He proved Cauchy's theorem and the Cauchy integral formula (1814, 1825), introduced the concept of a residue, and proved the residue theorem. Cauchy's approach was computational and powerful; he used complex integration to evaluate real integrals that had resisted all real-variable methods.

**Bernhard Riemann (1826–1866)** gave complex analysis its modern conceptual depth in his doctoral dissertation (1851). He introduced the idea of a Riemann surface to handle multi-valued functions, proved the Riemann mapping theorem (existence of conformal maps), and defined what we now call the Cauchy-Riemann equations as the fundamental condition. Riemann's geometric viewpoint — seeing conformal maps as the natural class of structure-preserving maps for complex analysis — transformed the subject.

**Karl Weierstrass (1815–1897)** developed complex analysis from a different starting point: power series. For Weierstrass, an analytic function was by definition one that could be locally expanded in a convergent power series. The Weierstrass M-test, the concept of uniform convergence, and the theory of entire functions as products over their zeros (Weierstrass factorization theorem) are his contributions.

**Henri Poincaré (1854–1912)** made foundational contributions to automorphic forms and uniformization theory, extending the Riemann mapping theorem to multiply connected domains and to more general Riemann surfaces.

## Connections to Other Units

**Prerequisites:**
- Unit 00 (Foundations): power series convergence (the basic tool for analytic functions), complex numbers.
- Unit 01 (Multivariable Calculus): partial derivatives (Cauchy-Riemann equations), the Jacobian.
- Unit 02 (Vector Calculus): Green's theorem is used to prove Cauchy's theorem; line integrals over curves in $\mathbb{C}$.

**Downstream:**
- Unit 05 (PDEs): harmonic functions are the real parts of analytic functions; the theory of Laplace's equation on domains with complex geometry uses conformal mapping (the Schwarz reflection principle, the Poisson kernel derived via the Cauchy formula, Green's functions for the disk and half-plane).
- Unit 04 (Fourier Analysis): the Paley-Wiener theorem characterizes the Fourier transforms of square-integrable functions with compact support as entire functions of exponential type. The inversion of the Laplace transform uses a contour integral (Bromwich integral) whose evaluation uses residues.
- Unit 08 (Advanced Topics): the theory of distributions and the analytic continuation of the Fourier transform generalize the ideas here. The Riemann zeta function and $L$-functions in number theory are analytic continuations of Dirichlet series.

## Key Theorems at a Glance

1. **Cauchy-Riemann Equations:** $f = u + iv$ is holomorphic at $z_0$ iff $u_x = v_y$ and $u_y = -v_x$ hold at $z_0$. Holomorphic iff the Jacobian matrix is a rotation-scaling.
2. **Cauchy's Theorem:** $\oint_C f\,dz = 0$ for analytic $f$ on a simply connected domain and any closed curve $C$ in it.
3. **Cauchy Integral Formula:** $f^{(n)}(z_0) = (n!/2\pi i)\oint f(z)/(z-z_0)^{n+1}\,dz$; values and derivatives determined by boundary values.
4. **Analyticity Equals Power Series:** A function is holomorphic on $\Omega$ iff it equals a convergent power series in every disk in $\Omega$.
5. **Liouville's Theorem:** Bounded entire function is constant; implies the Fundamental Theorem of Algebra.
6. **Laurent Expansion:** Analytic function on an annulus has a Laurent series; the principal part classifies isolated singularities.
7. **Residue Theorem:** $\oint_C f\,dz = 2\pi i \sum \text{Res}(f;z_j)$; most powerful computational tool in complex analysis.
8. **Identity Theorem:** Analytic functions agreeing on any sequence with a limit point must agree everywhere in a connected domain.
9. **Riemann Mapping Theorem:** Every simply connected proper open subset of $\mathbb{C}$ is conformally equivalent to the unit disk.
10. **Schwarz-Christoffel Formula:** Explicit conformal map from upper half-plane to polygon; used for boundary value problems in polygonal domains.
