# The Fundamental Theorem of Algebra

The Fundamental Theorem of Algebra asserts that every nonconstant polynomial with complex coefficients has at least one root in $\mathbb{C}$. This is a statement about algebra — polynomials, roots, factorizations — but its standard and most elegant proof is a theorem of complex analysis, following immediately from Liouville's theorem. The theorem is fundamental in the sense that it establishes $\mathbb{C}$ as algebraically closed: every polynomial splits into linear factors over $\mathbb{C}$, in sharp contrast with $\mathbb{R}$ (where $x^2 + 1$ is irreducible) or $\mathbb{Q}$ (where far more polynomials are irreducible).

## Statement

**Theorem (Fundamental Theorem of Algebra).** Let $p(z) = a_n z^n + a_{n-1}z^{n-1} + \cdots + a_1 z + a_0$ be a polynomial of degree $n \geq 1$ with complex coefficients $a_k \in \mathbb{C}$ and $a_n \neq 0$. Then there exists $z_0 \in \mathbb{C}$ such that $p(z_0) = 0$.

## Proof via Liouville's Theorem

**Proof.** Suppose for contradiction that $p(z) \neq 0$ for all $z \in \mathbb{C}$. Define $f(z) = 1/p(z)$. Then $f$ is entire (since $p$ is entire and never zero). We claim $f$ is bounded.

Since $|p(z)| \to \infty$ as $|z| \to \infty$ (because the leading term $a_n z^n$ dominates), there exists $R > 0$ such that $|p(z)| \geq 1$ for all $|z| \geq R$. Hence $|f(z)| = 1/|p(z)| \leq 1$ for $|z| \geq R$.

On the compact set $\{|z| \leq R\}$, the continuous function $|f(z)|$ attains a maximum $M_0 < \infty$ (since $p$ has no zeros, $f$ is continuous and hence bounded on compact sets). Therefore $|f(z)| \leq \max(1, M_0)$ for all $z \in \mathbb{C}$, and $f$ is bounded.

By Liouville's theorem, $f$ is constant. But then $p = 1/f$ is also constant, contradicting the hypothesis $n \geq 1$. $\square$

## Corollary: Complete Factorization

**Corollary.** Every polynomial $p(z)$ of degree $n \geq 1$ factors completely as:
$$p(z) = a_n(z - z_1)(z - z_2)\cdots(z - z_n),$$
where $z_1, \ldots, z_n \in \mathbb{C}$ are the roots (counted with multiplicity).

**Proof.** By the theorem, $p$ has a root $z_1$. Then $p(z) = (z - z_1) q(z)$ for some polynomial $q$ of degree $n - 1$ (by polynomial division). Apply the theorem to $q$ to find $z_2$, and continue by induction. After $n$ steps, we have $p(z) = a_n(z-z_1)\cdots(z-z_n)$, where $a_n$ is the leading coefficient. $\square$

## Multiplicity

**Definition.** A root $z_0$ of $p$ has multiplicity $m$ if $(z - z_0)^m$ divides $p(z)$ but $(z - z_0)^{m+1}$ does not. Equivalently, $p(z_0) = p'(z_0) = \cdots = p^{(m-1)}(z_0) = 0$ but $p^{(m)}(z_0) \neq 0$.

**Counting with multiplicity:** a polynomial of degree $n$ has exactly $n$ roots in $\mathbb{C}$ counting multiplicity.

**Worked example.** The polynomial $p(z) = z^4 - 1 = (z-1)(z+1)(z-i)(z+i)$ has four distinct roots in $\mathbb{C}$. The polynomial $q(z) = (z-1)^2(z+2)$ has degree $3$, with root $1$ of multiplicity $2$ and root $-2$ of multiplicity $1$.

## Algebraic Closedness

**Corollary.** $\mathbb{C}$ is algebraically closed: every polynomial with coefficients in $\mathbb{C}$ has all its roots in $\mathbb{C}$.

This distinguishes $\mathbb{C}$ from $\mathbb{R}$ (where $x^2 + 1$ has no real root), $\mathbb{Q}$ (where $x^2 - 2$ has no rational root), and $\mathbb{R}(t)$ (where much remains outside).

## Real Polynomials: Pairs of Complex Roots

**Corollary.** Every real polynomial of odd degree has at least one real root.

**Proof.** Let $p$ have real coefficients and degree $n$. By the theorem, it has $n$ complex roots. If $z_0 = a + bi$ (with $b \neq 0$) is a root, then so is $\bar{z}_0 = a - bi$, because $p(\bar{z}) = \overline{p(z)} = 0$ (using real coefficients). Complex roots come in conjugate pairs. If $n$ is odd, after pairing up the complex roots, at least one root must be its own conjugate, hence real. $\square$

This proves the real analysis fact that every odd-degree real polynomial has a real root, using complex analytic methods.

## Connection to Contour Integration

The Fundamental Theorem of Algebra can also be proved using the argument principle (a consequence of the residue theorem): if $p$ has no zeros inside a large circle $|z| = R$, then $\frac{1}{2\pi i}\oint_{|z|=R}\frac{p'(z)}{p(z)}\, dz = 0$. But a direct computation shows this integral equals $n$ for a degree-$n$ polynomial, giving a contradiction. This argument-principle proof provides a quantitative refinement: it counts the number of zeros inside any contour.

## Alternative Proof via Topology (for context)

A topological proof uses the winding number. The curve $\Gamma_R : [0, 2\pi] \to \mathbb{C}$, $\Gamma_R(\theta) = p(Re^{i\theta})$, has winding number $n$ around the origin for large $R$ (since $p(Re^{i\theta}) \approx a_n R^n e^{in\theta}$ dominates). For $R = 0$, $\Gamma_0$ is the constant curve $p(0) = a_0$, which has winding number $0$ around the origin if $a_0 \neq 0$. The winding number changes continuously as $R$ increases, so at some $R$ the curve must pass through the origin — i.e., $p$ has a root on $|z| = R$. This argument works whenever $a_0 \neq 0$; if $a_0 = 0$, then $z = 0$ is already a root.
