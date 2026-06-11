# Solutions: Foundations

## Problem 1: Proving Convergence by the Definition

**Problem.** Show directly from the definition that $\lim_{n\to\infty} n/(n+1) = 1$.

**Solution.** We must show: for every $\epsilon > 0$, there exists $N$ such that $n > N$ implies $|n/(n+1) - 1| < \epsilon$.

Compute: $\left|\frac{n}{n+1} - 1\right| = \left|\frac{n - (n+1)}{n+1}\right| = \frac{1}{n+1}$.

We need $1/(n+1) < \epsilon$, i.e., $n+1 > 1/\epsilon$, i.e., $n > 1/\epsilon - 1$.

So choose $N = \lceil 1/\epsilon \rceil$ (the smallest integer $\geq 1/\epsilon$). Then for $n > N$, $n > 1/\epsilon - 1$, so $n+1 > 1/\epsilon$, so $|n/(n+1) - 1| = 1/(n+1) < \epsilon$.

**Common mistake.** Writing "$1/(n+1) < \epsilon$ when $n$ is large enough" without specifying how large. The explicit choice $N = \lceil 1/\epsilon \rceil$ is required for a rigorous proof.

---

## Problem 2: Absolute Convergence and the Ratio Test

**Problem.** Determine whether $\sum_{n=1}^\infty n^2/3^n$ converges.

**Solution.** Apply the Ratio Test: $\left|\frac{a_{n+1}}{a_n}\right| = \frac{(n+1)^2/3^{n+1}}{n^2/3^n} = \frac{(n+1)^2}{3n^2} = \frac{1}{3}\left(1 + \frac{1}{n}\right)^2 \to \frac{1}{3} < 1$.

Since the ratio limit is $1/3 < 1$, the series converges absolutely.

**Common mistake.** Computing the ratio but forgetting to take the limit (just checking one value of $n$ is not enough), or concluding incorrectly from a ratio of 1.

**Remark.** One can compute the sum: $\sum_{n=1}^\infty n^2 x^n = x(1+x)/(1-x)^3$ for $|x| < 1$ (derived by differentiating the geometric series twice). At $x = 1/3$: sum $= (1/3)(4/3)/(2/3)^3 = (4/9)/(8/27) = (4/9)(27/8) = 3/2$.

---

## Problem 3: Radius of Convergence

**Problem.** Find the radius of convergence of $\sum_{n=0}^\infty \frac{(-1)^n}{n+1}(x-2)^n$.

**Solution.** Using the Ratio Test: $\left|\frac{a_{n+1}(x-2)^{n+1}}{a_n(x-2)^n}\right| = \frac{n+1}{n+2}|x-2| \to |x-2|$.

The series converges absolutely when $|x-2| < 1$ and diverges when $|x-2| > 1$. The radius of convergence is $R = 1$; the interval of convergence is centered at $x = 2$.

**Endpoints.** At $x = 3$ ($x - 2 = 1$): $\sum(-1)^n/(n+1)$ converges conditionally (alternating series). At $x = 1$ ($x-2 = -1$): $\sum 1/(n+1) = \sum 1/(n+1)$ diverges (harmonic series).

**Common mistake.** Forgetting to check endpoints separately from the interior; the Ratio Test is inconclusive on the boundary.

---

## Problem 4: Uniform Continuity

**Problem.** Prove that $f(x) = x^2$ is uniformly continuous on $[0, 3]$ but not on $[0, \infty)$.

**Solution.** **On $[0,3]$:** We need $\delta$ depending only on $\epsilon$, not on the specific points.

$|x^2 - y^2| = |x+y||x-y| \leq 6|x-y|$ for $x, y \in [0,3]$ (since $|x+y| \leq 6$).

Given $\epsilon > 0$, choose $\delta = \epsilon/6$. Then $|x-y| < \delta$ implies $|x^2-y^2| \leq 6|x-y| < 6\delta = \epsilon$.

**On $[0,\infty)$:** Take $x_n = n + 1/n$ and $y_n = n$. Then $|x_n - y_n| = 1/n \to 0$, but $|x_n^2 - y_n^2| = |(n+1/n)^2 - n^2| = |2 + 1/n^2| \to 2 \neq 0$. So $f$ is not uniformly continuous on $[0,\infty)$.

---

## Problem 5: The Lipschitz Condition and ODE Existence

**Problem.** Does the IVP $y' = |y|^{1/2}$, $y(0) = 0$ have a unique solution?

**Solution.** Check the Lipschitz condition in $y$ near $y = 0$: $\big||y_1|^{1/2} - |y_2|^{1/2}\big|$. For $y_1 = h > 0$, $y_2 = 0$: $h^{1/2}/h = h^{-1/2} \to \infty$ as $h \to 0$. The function $f(y) = |y|^{1/2}$ is not Lipschitz at $y = 0$.

By Peano's theorem, continuity guarantees existence; but the Picard-Lindelöf uniqueness hypothesis fails. And indeed, uniqueness fails:
- $y = 0$ (identically zero) is a solution.
- $y = (x/2)^2$ for $x \geq 0$ (and $0$ for $x \leq 0$) is another solution.
- More generally, $y = \begin{cases}0 & x \leq c \\ (x-c)^2/4 & x > c\end{cases}$ solves the IVP for any $c \geq 0$.

There are infinitely many solutions.

**Common mistake.** Concluding that "the equation has a solution $y=0$" and stopping. The question of uniqueness is separate from existence.

---

## Problem 6: Eigenvalues and the Solution Space

**Problem.** Find the eigenvalues and eigenvectors of $A = \begin{pmatrix}2&1\\1&2\end{pmatrix}$ and write the general solution to $\mathbf{x}' = A\mathbf{x}$.

**Solution.** Characteristic polynomial: $\det(A - \lambda I) = (2-\lambda)^2 - 1 = \lambda^2 - 4\lambda + 3 = (\lambda-1)(\lambda-3) = 0$.

$\lambda_1 = 1$: $(A-I)\mathbf{v} = \begin{pmatrix}1&1\\1&1\end{pmatrix}\mathbf{v} = 0 \Rightarrow v_1 + v_2 = 0$, so $\mathbf{v}_1 = \begin{pmatrix}1\\-1\end{pmatrix}$.

$\lambda_2 = 3$: $(A-3I)\mathbf{v} = \begin{pmatrix}-1&1\\1&-1\end{pmatrix}\mathbf{v} = 0 \Rightarrow v_1 = v_2$, so $\mathbf{v}_2 = \begin{pmatrix}1\\1\end{pmatrix}$.

General solution: $\mathbf{x}(t) = c_1\begin{pmatrix}1\\-1\end{pmatrix}e^t + c_2\begin{pmatrix}1\\1\end{pmatrix}e^{3t}$.

Since both eigenvalues are positive, the origin is an unstable node; all solutions grow without bound.

---

## Problem 7: Power Series Differentiation

**Problem.** Find the power series for $f(x) = \arctan(x)$ about $x = 0$ and give its radius of convergence.

**Solution.** We know $f'(x) = 1/(1+x^2)$. Expand: $\frac{1}{1+x^2} = \sum_{n=0}^\infty (-1)^n x^{2n}$ for $|x| < 1$.

Integrating term by term (valid within the radius of convergence):
$$\arctan(x) = \int_0^x \frac{dt}{1+t^2} = \sum_{n=0}^\infty \frac{(-1)^n x^{2n+1}}{2n+1} = x - \frac{x^3}{3} + \frac{x^5}{5} - \cdots$$

Radius of convergence: $R = 1$ (inherits from geometric series). By Abel's theorem, the series actually converges at $x = \pm 1$ as well (the series is alternating at $|x|=1$), giving the Leibniz formula $\pi/4 = 1 - 1/3 + 1/5 - \cdots$.
