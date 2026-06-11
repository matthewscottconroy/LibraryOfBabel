# Unit Overview: Foundations

## Why Foundations Are Indispensable

Differential equations are, at the most fundamental level, statements about limits. The expression $dy/dt = f(t, y)$ says: at every point $(t, y)$ in the plane, the rate of change of $y$ with respect to $t$ equals $f(t, y)$. Whether such a statement is meaningful — whether it has solutions, whether those solutions are unique, whether they depend continuously on initial data, whether a numerical approximation to them converges — reduces, ultimately, to questions whose proper setting is real analysis and linear algebra.

Students who have completed calculus but not a proof-based analysis course often carry a collection of tacit assumptions: that continuous functions are differentiable somewhere, that interchanging a limit and an integral is always permissible, that a sequence of solutions to a differential equation converges to a solution of the limiting equation. Each of these assumptions fails in important cases. This unit exists to replace tacit assumptions with theorems whose hypotheses are explicit, so that later in the course, when a theorem fails, you know exactly why.

There is also a structural reason to begin here. Differential equations organize themselves into an algebraic hierarchy: the solution set of a homogeneous linear ODE is a vector space, an $n$th-order ODE is equivalent to a first-order system in $n$ dimensions, and the spectral theory of differential operators (which underlies Sturm-Liouville theory, Fourier analysis, and quantum mechanics) is simply the eigenvalue theory of linear operators on function spaces. None of this structure is visible without the language of linear algebra. The student who sees only solution formulas, without the vector space framework, is missing the skeleton that gives the body its shape.

This unit therefore builds two great pillars: real analysis, which supplies the tools for proving existence, uniqueness, and convergence results; and linear algebra, which supplies the structural language for understanding solution spaces, transformations, and the behavior of systems.

## Central Theorems

### Real Analysis Essentials

**Theorem (Completeness of $\mathbb{R}$).** Every Cauchy sequence in $\mathbb{R}$ converges to a real number. Equivalently: every nonempty subset of $\mathbb{R}$ that is bounded above has a least upper bound (supremum) in $\mathbb{R}$.

This is the axiom that distinguishes the real numbers from the rationals. The sequence $3, 3.1, 3.14, 3.141, \ldots$ of rational approximations to $\pi$ is Cauchy in $\mathbb{Q}$, but its limit is not in $\mathbb{Q}$. Completeness is what guarantees convergence of Picard iterates (in the proof of the existence-uniqueness theorem for ODEs), convergence of power series inside their disk of convergence, and a dozen other foundational facts.

**Theorem (Intermediate Value Theorem).** If $f : [a, b] \to \mathbb{R}$ is continuous and $f(a) < c < f(b)$, there exists $x_0 \in (a, b)$ with $f(x_0) = c$.

The proof uses completeness of $\mathbb{R}$: let $s = \sup\{x \in [a,b] : f(x) \leq c\}$; completeness guarantees $s$ exists; continuity of $f$ forces $f(s) = c$. The IVT is used in existence proofs: it guarantees that a differential equation built from a continuous right-hand side has a solution on some interval.

**Theorem (Uniform Continuity on Compact Sets).** Every continuous function on a closed bounded interval $[a,b]$ is uniformly continuous there.

Uniform continuity — the requirement that $\delta$ depend only on $\epsilon$ and not on the point $x$ — is precisely what is needed to interchange limits and integrals. It appears in the Arzelà-Ascoli theorem, in the justification of term-by-term integration of uniformly convergent series, and in the proof that the Picard iteration converges uniformly.

**Theorem (Mean Value Theorem).** If $f$ is continuous on $[a,b]$ and differentiable on $(a,b)$, there exists $c \in (a,b)$ with $f'(c) = (f(b) - f(a))/(b - a)$.

**Corollary (Lipschitz Bound).** If $|f'(x)| \leq L$ for all $x \in (a,b)$, then $|f(x) - f(y)| \leq L|x - y|$ for all $x, y \in [a,b]$.

This corollary is the key. Lipschitz continuity is the hypothesis of the Picard-Lindelöf theorem; without it, existence may hold but uniqueness fails (as the example $y' = y^{1/2}$, $y(0) = 0$ demonstrates).

**Theorem (Taylor's Theorem with Remainder).** If $f$ is $(n+1)$-times continuously differentiable on $[a, a+h]$, then
$$f(a+h) = \sum_{k=0}^n \frac{f^{(k)}(a)}{k!} h^k + \frac{f^{(n+1)}(c)}{(n+1)!} h^{n+1}$$
for some $c$ between $a$ and $a+h$.

The Lagrange remainder term $R_n = f^{(n+1)}(c)/(n+1)! \cdot h^{n+1}$ is essential when computing bounds on the error of truncated power series solutions to ODEs (as in the Frobenius method) and when analyzing the truncation error of numerical methods (Euler, Runge-Kutta).

**Theorem (Fundamental Theorem of Calculus).** If $f$ is Riemann integrable on $[a,b]$ and $F(x) = \int_a^x f(t)\,dt$, then $F$ is continuous on $[a,b]$. If $f$ is also continuous at $x_0 \in (a,b)$, then $F'(x_0) = f(x_0)$.

Conversely: if $G$ is differentiable on $[a,b]$ with $G' = f$ integrable, then $\int_a^b f = G(b) - G(a)$.

The FTC is how one converts an ODE $y' = f(x)$ into an integral equation $y(x) = y_0 + \int_{x_0}^x f(t)\,dt$, a reformulation that is both more amenable to analysis and more general (admissible for $f$ merely integrable, not differentiable).

### Linear Algebra Foundations

**Theorem (Existence and Uniqueness for Linear Systems).** The system $A\mathbf{x} = \mathbf{b}$ has a unique solution if and only if $\det(A) \neq 0$. When $A$ is $n \times n$ and invertible, the unique solution is $\mathbf{x} = A^{-1}\mathbf{b}$.

**Theorem (Spectral Theorem for Symmetric Matrices).** Every real symmetric matrix $A = A^T$ has $n$ real eigenvalues (counted with multiplicity) and $n$ mutually orthogonal eigenvectors. In particular, $A = Q\Lambda Q^T$ where $Q$ is orthogonal and $\Lambda$ is diagonal.

This theorem is the finite-dimensional prototype for the spectral theory of self-adjoint differential operators. The Sturm-Liouville theorem — that a self-adjoint second-order ODE operator has a complete orthonormal set of eigenfunctions with real eigenvalues — is the infinite-dimensional version, and its proof follows the same lines.

**Theorem (Cayley-Hamilton).** Every square matrix satisfies its own characteristic polynomial: if $p(\lambda) = \det(\lambda I - A)$, then $p(A) = 0$.

Cayley-Hamilton is used in computing the matrix exponential $e^{At}$ without explicitly diagonalizing $A$.

**Theorem (Jordan Normal Form).** Every complex square matrix is similar to a matrix in Jordan form: block-diagonal with Jordan blocks $J_k(\lambda) = \lambda I + N$ where $N$ is the nilpotent shift. Two matrices are similar if and only if they have the same Jordan form.

Jordan form is essential when $A$ has repeated eigenvalues, which is the generic situation for degenerate equilibria of ODE systems.

## How the Sections Build

The two sub-units — Real Analysis Essentials and Linear Algebra Foundations — are designed to be studied in parallel or sequentially, since each supports the other.

**Real Analysis Essentials** (Unit 01) proceeds: Logic and Proof $\to$ Real Numbers and Completeness $\to$ Sequences and Limits $\to$ Series and Power Series $\to$ Continuity $\to$ Differentiation $\to$ Riemann Integration. Each step is logically prior to the next. Convergence of sequences is needed before convergence of series can be discussed; continuity is needed before differentiation; and the Mean Value Theorem is the key tool in bounding approximation errors throughout.

**Linear Algebra Foundations** (Unit 02) proceeds: Vector Spaces and Subspaces $\to$ Linear Maps and Matrices $\to$ Determinants and Inverses $\to$ Eigenvalues and Eigenvectors $\to$ Diagonalization and Jordan Form $\to$ Inner Products and the Spectral Theorem. The progression is from concrete (matrices, column operations) to abstract (vector spaces, linear maps) to spectral (eigenvalues and their geometric meaning).

## Worked Examples of Key Techniques

### Example 1: Proving Convergence of a Sequence via Completeness

Let $a_1 = 1$ and $a_{n+1} = (a_n + 2/a_n)/2$ (Newton's method for $\sqrt{2}$). To show this converges:

1. **Bounded below:** $a_n > 0$ for all $n$ (induction, since $a_{n+1} = (a_n + 2/a_n)/2 > 0$).
2. **Decreasing:** $a_{n+1} - a_n = (2/a_n - a_n)/2 = (2 - a_n^2)/(2a_n)$. We claim $a_n^2 \geq 2$ for $n \geq 2$: by AM-GM, $a_{n+1}^2 = (a_n + 2/a_n)^2/4 \geq (2\sqrt{a_n \cdot 2/a_n})^2/4 = 2$. So $a_{n+1} \leq a_n$ for $n \geq 2$.
3. **Convergence:** A decreasing sequence bounded below converges (Monotone Convergence Theorem, proved from completeness). Let $L = \lim a_n$. Taking limits in $a_{n+1} = (a_n + 2/a_n)/2$: $L = (L + 2/L)/2$, so $L^2 = 2$, $L = \sqrt{2}$.

This pattern — establishing monotonicity and a bound, then identifying the limit from the recursion — is the template for proving convergence of Picard iterates.

### Example 2: Eigenvalue Analysis for a $2 \times 2$ System

Consider the matrix $A = \begin{pmatrix} 3 & 1 \\ 1 & 3 \end{pmatrix}$.

Characteristic polynomial: $\det(A - \lambda I) = (3-\lambda)^2 - 1 = \lambda^2 - 6\lambda + 8 = (\lambda-2)(\lambda-4)$.

Eigenvalues: $\lambda_1 = 2$, $\lambda_2 = 4$.

Eigenvector for $\lambda_1 = 2$: $(A - 2I)\mathbf{v} = \begin{pmatrix}1&1\\1&1\end{pmatrix}\mathbf{v} = 0$, so $v_1 + v_2 = 0$, giving $\mathbf{v}_1 = \begin{pmatrix}1\\-1\end{pmatrix}$.

Eigenvector for $\lambda_2 = 4$: $(A - 4I)\mathbf{v} = \begin{pmatrix}-1&1\\1&-1\end{pmatrix}\mathbf{v} = 0$, so $v_1 = v_2$, giving $\mathbf{v}_2 = \begin{pmatrix}1\\1\end{pmatrix}$.

The system $\mathbf{x}' = A\mathbf{x}$ has general solution $\mathbf{x}(t) = c_1\begin{pmatrix}1\\-1\end{pmatrix}e^{2t} + c_2\begin{pmatrix}1\\1\end{pmatrix}e^{4t}$.

Note that $A$ is symmetric, its eigenvalues are real, and its eigenvectors are orthogonal ($(1,-1)\cdot(1,1) = 0$), confirming the Spectral Theorem.

### Example 3: Power Series and Radius of Convergence

The series $\sum_{n=0}^\infty a_n (x - x_0)^n$ converges absolutely for $|x - x_0| < R$ where
$$R = \frac{1}{\limsup_{n\to\infty} |a_n|^{1/n}} \quad \text{(Hadamard's formula)}.$$

For $a_n = 1/n!$: $|a_n|^{1/n} = (1/n!)^{1/n} \to 0$ since $n!$ grows faster than any exponential. So $R = \infty$: the exponential series $e^x = \sum x^n/n!$ converges everywhere.

For $a_n = n!$: $|a_n|^{1/n} = (n!)^{1/n} \to \infty$ (by Stirling: $n! \approx (n/e)^n\sqrt{2\pi n}$, so $(n!)^{1/n} \approx n/e \to \infty$). So $R = 0$: the series diverges for all $x \neq 0$.

In the Frobenius method for solving ODEs, the radius of convergence of the series solution equals the distance from the expansion point to the nearest singularity of the coefficient functions (in the complex plane). Knowing this from the outset tells you precisely the domain of validity of a power series solution.

## Historical Notes

The foundational issues underlying real analysis were not resolved until the nineteenth century, long after differential equations had been in active use for two hundred years. Newton and Leibniz operated with infinitesimals — infinitely small quantities — without a rigorous definition of what they were. This worked in practice, but it created paradoxes (Berkeley's "ghosts of departed quantities") and left the subject on uncertain logical ground.

**Augustin-Louis Cauchy (1789–1857)** was the first mathematician to place the calculus on rigorous foundations. In his *Cours d'analyse* (1821) he defined the limit of a sequence precisely, defined continuity in terms of limits, and used these definitions to prove the Intermediate Value Theorem and the Mean Value Theorem. Cauchy also introduced the notion that would become the Cauchy sequence, and his work on convergence criteria for series is still standard.

**Karl Weierstrass (1815–1897)** carried Cauchy's program to completion with his $\epsilon$-$\delta$ formalism. His most dramatic contribution was the construction, in 1872, of a continuous function that is nowhere differentiable — a function that could not exist in a world where "continuous" implied "differentiable," as most nineteenth-century mathematicians had assumed. This example forced a clean separation between continuity and differentiability and sharpened the hypotheses required for theorems in ODE theory.

**Richard Dedekind (1831–1916)** and **Georg Cantor (1845–1918)** made the completeness of the real numbers precise. Dedekind's construction (Dedekind cuts) and Cantor's construction (equivalence classes of Cauchy sequences) both produce the real number system axiomatically from the rationals, placing the least-upper-bound property on a firm logical foundation.

The linear algebra needed for differential equations has a different history. **Leibniz (1646–1716)** introduced the determinant around 1693. **Carl Friedrich Gauss (1777–1855)** systematized the elimination method for linear systems (Gaussian elimination) in connection with least-squares problems in geodesy and astronomy. **Cayley (1821–1895)** introduced the formal algebra of matrices in 1858 and stated the Cayley-Hamilton theorem. **Weierstrass** and **Jordan (1838–1922)** independently arrived at the Jordan normal form in the 1860s–70s, which resolved the question of how to handle repeated eigenvalues. **David Hilbert (1862–1943)** abstracted the inner product from geometry to analysis, creating the Hilbert space framework that underlies both spectral theory and quantum mechanics.

## Connections to Other Units

**Downstream in this course:**
- Picard-Lindelöf (Unit 03) uses the Banach fixed-point theorem, which is a direct application of completeness and Cauchy sequences.
- Series solutions (Unit 03, Unit 04 of the Fourier module) use power series convergence theory from the analysis unit.
- The matrix exponential and eigenvalue methods (Unit 03) are direct applications of eigenvalue theory and Jordan form.
- Fourier analysis (Unit 04) uses inner product spaces, orthogonality, and $L^2$ convergence — the infinite-dimensional generalization of the inner product spaces here.
- Sturm-Liouville theory (Unit 03, Unit 05) is the spectral theory of self-adjoint differential operators, directly analogous to the Spectral Theorem for symmetric matrices.
- Sobolev spaces (Unit 08) are completions of function spaces in Sobolev norms, using the completeness ideas of this unit.

**Within this unit:**
The two sub-units reinforce each other: the real analysis develops the convergence tools needed to make linear algebra over infinite-dimensional spaces rigorous, while linear algebra provides the structural language that makes the ODE theory meaningful as algebra and not just as a collection of formulas.

## Key Theorems at a Glance

1. **Completeness of $\mathbb{R}$:** Every Cauchy sequence converges; every nonempty bounded-above set has a supremum.
2. **Bolzano-Weierstrass:** Every bounded sequence in $\mathbb{R}^n$ has a convergent subsequence.
3. **Intermediate Value Theorem:** A continuous function on $[a,b]$ achieves every value between $f(a)$ and $f(b)$.
4. **Extreme Value Theorem:** A continuous function on a closed bounded interval attains its maximum and minimum.
5. **Mean Value Theorem:** $f(b) - f(a) = f'(c)(b-a)$ for some interior $c$; corollary gives Lipschitz bounds.
6. **Taylor's Theorem with Lagrange Remainder:** Quantitative bounds on polynomial approximation.
7. **Uniform Convergence and Term-by-Term Differentiation/Integration:** If $f_n \to f$ uniformly and each $f_n$ is integrable (resp. differentiable with continuous derivative), then $\int f = \lim \int f_n$ (resp. under additional hypothesis, $(\lim f_n)' = \lim f_n'$).
8. **Arzelà-Ascoli Theorem:** A sequence of equicontinuous, uniformly bounded functions on a compact interval has a uniformly convergent subsequence. (Used in the Peano existence theorem.)
9. **Spectral Theorem for Symmetric Matrices:** $A = A^T$ real $\Rightarrow$ $A$ is orthogonally diagonalizable with real eigenvalues.
10. **Jordan Normal Form:** Every complex square matrix is similar to a unique Jordan form; this form determines the behavior of the matrix exponential and hence of linear ODE systems with repeated eigenvalues.
