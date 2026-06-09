# Functions of a Complex Variable

A function of a complex variable is a rule that assigns to each complex number $z$ in some domain $D \subseteq \mathbb{C}$ a complex number $f(z) \in \mathbb{C}$. The formal definition is identical to that of a real-valued function, but the two-dimensional nature of $\mathbb{C}$ endows these functions with a geometric richness that has no real-variable analogue. This section establishes the basic vocabulary — domain, range, image, preimage — and introduces the decomposition of a complex function into its real and imaginary parts, which is the bridge between complex analysis and the multivariable calculus you already know.

## Definition and Notation

**Definition.** Let $D$ be a nonempty subset of $\mathbb{C}$. A function $f : D \to \mathbb{C}$ assigns to each $z \in D$ a unique complex number $f(z)$. The set $D$ is called the domain of $f$. The range or image of $f$ is $f(D) = \{f(z) : z \in D\}$.

Since every complex number has a real and an imaginary part, we can write
$$f(z) = f(x + iy) = u(x, y) + i\, v(x, y),$$
where $u : D' \to \mathbb{R}$ and $v : D' \to \mathbb{R}$ are real-valued functions on the corresponding subset $D' \subseteq \mathbb{R}^2$. Here $u = \mathrm{Re}(f)$ is the real part and $v = \mathrm{Im}(f)$ is the imaginary part of $f$.

**Worked example.** Let $f(z) = z^2$. If $z = x + iy$, then
$$f(z) = (x + iy)^2 = x^2 - y^2 + 2ixy.$$
So $u(x,y) = x^2 - y^2$ and $v(x,y) = 2xy$.

**Worked example.** Let $f(z) = \bar{z}$. Then $f(x + iy) = x - iy$, so $u(x,y) = x$ and $v(x,y) = -y$.

## Standard Classes of Complex Functions

**Polynomials.** Any polynomial $p(z) = a_n z^n + a_{n-1}z^{n-1} + \cdots + a_0$ with coefficients $a_k \in \mathbb{C}$ defines a function $p : \mathbb{C} \to \mathbb{C}$.

**Rational functions.** A rational function is a quotient $R(z) = p(z)/q(z)$ of two polynomials. It is defined everywhere $q(z) \neq 0$.

**Transcendental functions.** The exponential function $e^z$, the trigonometric functions $\sin z$ and $\cos z$, the logarithm $\log z$, and power functions $z^\alpha$ are defined via power series or via the exponential, and their properties are developed carefully in Unit 02.

## Visualizing Complex Functions

Because $f$ maps $\mathbb{C}$ to $\mathbb{C}$, its graph lives in $\mathbb{C} \times \mathbb{C} \cong \mathbb{R}^4$, which cannot be visualized directly. Several strategies are used instead:

1. **Domain coloring:** color each point $z$ in the domain by the argument (hue) and modulus (brightness) of $f(z)$.
2. **Mapping diagrams:** draw the $z$-plane (input) and $w$-plane (output) side by side, and indicate how curves or regions in the $z$-plane map to the $w$-plane.
3. **Real and imaginary part surfaces:** plot $u(x,y)$ and $v(x,y)$ separately as surfaces over $\mathbb{R}^2$.

The mapping diagram approach is the most informative for understanding how analytic functions distort geometry. For instance, $f(z) = z^2$ maps horizontal lines $y = c$ to parabolas opening to the left, and vertical lines $x = c$ to parabolas opening to the right. The two families of parabolas are orthogonal, a fact that will be explained by the angle-preserving property of analytic functions in Unit 04.

## The Real-Part Decomposition: Examples

**Worked example.** Find $u$ and $v$ for $f(z) = 1/z$, $z \neq 0$.

Using the formula $z^{-1} = \bar{z}/|z|^2$:
$$\frac{1}{x + iy} = \frac{x - iy}{x^2 + y^2}.$$
So $u(x,y) = \dfrac{x}{x^2 + y^2}$ and $v(x,y) = \dfrac{-y}{x^2 + y^2}$.

**Worked example.** Find $u$ and $v$ for $f(z) = e^z$.

Using $e^{x+iy} = e^x e^{iy} = e^x(\cos y + i\sin y)$:
$$u(x,y) = e^x \cos y, \qquad v(x,y) = e^x \sin y.$$

These examples illustrate a general fact: the components $u$ and $v$ of a "nice" complex function are not arbitrary pairs of real functions — they are linked by the Cauchy-Riemann equations, as will be shown in Unit 02.

## Preimages and Level Sets

The preimage of a set $S \subseteq \mathbb{C}$ under $f$ is $f^{-1}(S) = \{z \in D : f(z) \in S\}$. Level curves (or level sets) of $u$ and $v$ play an important role in visualizing $f$. For an analytic function, the level curves $\{u = c\}$ and $\{v = d\}$ are orthogonal families in the $z$-plane (wherever $f'(z) \neq 0$), which is the geometric content of the Cauchy-Riemann equations.

**Worked example.** For $f(z) = z^2$, the level curves of $u = x^2 - y^2 = c$ are hyperbolas with asymptotes along the coordinate axes, and the level curves of $v = 2xy = d$ are rectangular hyperbolas perpendicular to the first family. The two families are indeed orthogonal in $\mathbb{R}^2$.

## Domains and Conventions

In complex analysis, the natural setting for studying a function is a domain: a nonempty, open, connected subset of $\mathbb{C}$. Openness ensures that limits and derivatives can be defined without boundary complications. Connectedness ensures that analytic functions on domains satisfy the identity principle: if two analytic functions agree on a set with an accumulation point in a domain, they agree everywhere in the domain.

Common domains:
- The complex plane $\mathbb{C}$ itself.
- The punctured plane $\mathbb{C} \setminus \{0\}$.
- The upper half-plane $H = \{z : \mathrm{Im}(z) > 0\}$.
- The unit disk $\mathbb{D} = \{z : |z| < 1\}$.
- Annuli $\{z : r < |z| < R\}$ for $0 \leq r < R \leq \infty$.

## Bounded and Entire Functions

A function $f : D \to \mathbb{C}$ is bounded on $D$ if there exists $M > 0$ such that $|f(z)| \leq M$ for all $z \in D$. A function defined and analytic on all of $\mathbb{C}$ is called an entire function. Polynomials, $e^z$, $\sin z$, and $\cos z$ are all entire. Liouville's theorem (Unit 03) states that every bounded entire function is constant — a dramatic illustration of how rigid complex analyticity is compared to real differentiability.

## Connection to Real Analysis

Every complex function $f = u + iv$ can be viewed as a mapping $F : D' \to \mathbb{R}^2$, $F(x,y) = (u(x,y), v(x,y))$, from a subset of $\mathbb{R}^2$ to $\mathbb{R}^2$. The real analysis of such mappings — continuity, differentiability, the Jacobian, the inverse function theorem — all apply, but complex analyticity imposes additional constraints that make the theory far richer. In particular, the Jacobian of an analytic function at a point has the special form $\begin{pmatrix} u_x & -v_x \\ v_x & u_x \end{pmatrix}$ (by the Cauchy-Riemann equations), which is a scalar multiple of a rotation matrix, encoding the angle-preserving property of conformal maps.
