# Computational Notes — Part I: Mathematical Foundations

These notes connect the mathematical foundations of Part I to concrete computational experiments. Every theoretical result described here has a computational manifestation that helps build intuition. The experiments below are described at the pseudocode/algorithm level; the quiz app and associated Rust demos implement several of them.

---

## 1. The Contraction Mapping Theorem in Computation

### What It Implements

The Banach Fixed Point Theorem (Chapter 1) is not merely an existence result — it is an algorithm: to find the fixed point of a contraction $f: X \to X$, iterate starting from any point.

```
fixed_point_iteration(f, x0, tolerance):
    x = x0
    while |f(x) - x| > tolerance:
        x = f(x)
    return x
```

**Convergence rate**: If $f$ has Lipschitz constant $k < 1$, then after $n$ iterations,
$$|x_n - x^*| \leq k^n \cdot \frac{d(x_1, x_0)}{1 - k}.$$

**Experiment**: Implement Newton's method for finding roots of $p(x) = x^2 - 2$. Newton's method is fixed-point iteration for $g(x) = x - p(x)/p'(x) = (x + 2/x)/2$. Verify the convergence rate: the error squares at each step (quadratic convergence), which corresponds to a contraction with constant $k \to 0$ near the fixed point.

**Connection to dynamics**: Fixed points of $g$ are exactly the equilibria of the ODE $\dot{x} = g(x) - x$. Stable equilibria (asymptotically stable fixed points) are exactly the attractive fixed points of $g$ — those for which $|g'(x^*)| < 1$.

---

## 2. Numerical Integration and Smooth Dependence

### The Runge-Kutta Method

For the ODE $\dot{x} = f(x)$ with step size $h$, the 4th-order Runge-Kutta method is:
$$k_1 = f(x_n), \quad k_2 = f(x_n + hk_1/2), \quad k_3 = f(x_n + hk_2/2), \quad k_4 = f(x_n + hk_3)$$
$$x_{n+1} = x_n + \frac{h}{6}(k_1 + 2k_2 + 2k_3 + k_4).$$

This produces a $\delta$-pseudo-orbit (see Chapter 9) with $\delta = O(h^4)$ per step, $O(h^4 T)$ total error over time interval $[0, T]$.

**Experiment**: Integrate the Lorenz system and observe the sensitive dependence on initial conditions. Start two trajectories at distance $10^{-10}$ apart. Plot $\log \|x_1(t) - x_2(t)\|$ as a function of $t$. The slope is the maximal Lyapunov exponent ($\approx 0.9$ for the standard Lorenz parameters).

```python
# Pseudocode
def lorenz(x, sigma=10, rho=28, beta=8/3):
    return [sigma*(x[1]-x[0]), x[0]*(rho-x[2])-x[1], x[0]*x[1]-beta*x[2]]

x1 = [1.0, 0.0, 0.0]
x2 = [1.0 + 1e-10, 0.0, 0.0]
errors = []
for t in range(1000):
    x1 = rk4_step(lorenz, x1, dt=0.01)
    x2 = rk4_step(lorenz, x2, dt=0.01)
    errors.append(log(norm(x1 - x2)))
# slope of errors list ≈ Lyapunov exponent
```

### Smooth Dependence on Parameters

The variational equations $\dot{J} = Df(\Phi_t(x_0)) \cdot J$ (Section 4.1.3) can be integrated *alongside* the main ODE to compute the Jacobian $D_{x_0}\Phi_t$ — the sensitivity matrix. This is the foundation of:
- **Lyapunov exponent computation**: accumulate $\log \|J(t)\|$ over time.
- **Sensitivity analysis**: $J$ tells you how much the final state changes with the initial condition.
- **Adjoint methods**: reverse-mode integration of $J^T$ for gradient computation.

---

## 3. Measure Theory and Monte Carlo Integration

### Lebesgue Integration vs. Riemann Integration

The theoretical superiority of Lebesgue integration (dominated convergence theorem, $L^p$ completeness) manifests computationally in Monte Carlo methods.

**Monte Carlo integration**: For $\int_\Omega f(x) \, d\mu(x)$, generate i.i.d. samples $X_1, \ldots, X_n \sim \mu$ and estimate $\hat{I}_n = \frac{1}{n}\sum_{k=1}^n f(X_k)$. By the strong law of large numbers (Theorem 2.6.1), $\hat{I}_n \to \int f \, d\mu$ a.s. The error rate is $O(1/\sqrt{n})$ — independent of the dimension of $\Omega$.

**Experiment**: Estimate $\pi$ by computing the fraction of uniform points in $[0,1]^2$ that lie inside the unit disk.

```
count = 0
for i in range(N):
    x, y = uniform(0,1), uniform(0,1)
    if x^2 + y^2 < 1: count += 1
pi_estimate = 4 * count / N
# Error ~ 1/sqrt(N) by CLT
```

**Ulam's Method**: For computing invariant measures of a map $f: [0,1] \to [0,1]$, discretize $[0,1]$ into $n$ cells $I_1, \ldots, I_n$ and compute the transition matrix $P_{ij} = \mu(I_j \cap f^{-1}(I_i)) / \mu(I_i)$. The left Perron eigenvector of $P$ approximates the invariant density. This implements the *Frobenius-Perron operator* (transfer operator, Chapter 9) discretized on a finite basis.

---

## 4. Topology: Computing Betti Numbers and Euler Characteristics

### Persistent Homology

Topological data analysis (TDA) computes the Betti numbers $\beta_k =$ rank $H_k(X; \mathbb{Z})$ of a space from a point cloud or a sequence of simplicial complexes. For dynamical systems, this provides quantitative information about the topology of attractors.

**Algorithm (Vietoris-Rips complex)**: Given a set of points $\{x_i\}$ from an attractor at scale $\varepsilon$:
1. Build a simplicial complex: add a $k$-simplex $\{x_{i_0}, \ldots, x_{i_k}\}$ whenever $\max_{a,b} d(x_{i_a}, x_{i_b}) \leq \varepsilon$.
2. Compute the boundary matrices $\partial_k$ and the homology $H_k = \ker \partial_k / \text{im} \partial_{k+1}$.
3. Vary $\varepsilon$ and track when topological features (connected components, loops, voids) are born and die — the *persistence diagram*.

**Euler characteristic**: $\chi(X) = \sum_k (-1)^k \beta_k$. By the Poincaré-Hopf theorem (Chapter 3), $\chi(M)$ equals the sum of indices of the fixed points of any generic vector field on $M$. This connects topology to dynamics: for the sphere $S^2$ (Euler characteristic 2), any vector field must have at least two zeros (with total index 2) — you can't comb a hairy ball.

---

## 5. Linear Algebra: SVD and Lyapunov Exponents

### Singular Value Decomposition in Dynamics

The SVD $A = U\Sigma V^T$ decomposes a linear map into rotation-stretch-rotation. In the context of the flow $D\Phi_t(x_0)$:
- The singular values $\sigma_1 \geq \sigma_2 \geq \cdots \geq \sigma_n$ are the lengths of the semi-axes of the image ellipsoid.
- The logarithms $\frac{1}{t}\log \sigma_i$ approximate the Lyapunov exponents.

**Algorithm (QR method for Lyapunov exponents)**:
```
Initialize J = Identity matrix
exponents = zeros(n)
for k in range(T):
    J = Df(x) @ J  # propagate tangent vectors
    Q, R = QR(J)   # orthogonalize
    exponents += log(diag(abs(R)))
    J = Q          # restart with orthonormal frame
    x = f(x)       # advance trajectory
exponents /= T  # divide by time
```

The QR decomposition maintains orthogonality of the tangent frame, preventing one direction from dominating. The diagonal of $R$ gives the instantaneous expansion rates; accumulated over time, they converge to the Lyapunov exponents.

### Power Method for Perron Root

For computing the topological entropy of an SFT with transition matrix $A$:
```
v = random_vector()
for k in range(T):
    v = A @ v
    v = v / norm(v)
# log(norm(A @ v)) converges to log(lambda_PF)
```

The power method converges to the Perron eigenvector at rate $(\lambda_2/\lambda_1)^T$, where $\lambda_1, \lambda_2$ are the two largest eigenvalues. For primitive matrices (some power of $A$ has all positive entries), $\lambda_1 > \lambda_2$ and convergence is guaranteed.

---

## 6. The Arzelà-Ascoli Theorem and Function Space Approximation

### Equicontinuity in Numerical Analysis

The Arzelà-Ascoli theorem (Chapter 1) characterizes compact subsets of $C(K)$ (continuous functions on a compact space $K$) as the equicontinuous, bounded families. This is the theoretical foundation of:

**Finite element methods**: The solution space of a PDE (a subset of $H^1$ or $C^2$) is compact under appropriate conditions. Approximating the solution by piecewise-linear functions on a mesh is justified by this compactness.

**Convergence of numerical ODE solvers**: The family of approximate solutions to $\dot{x} = f(x)$ (for all step sizes $h > 0$) is equicontinuous (by the Lipschitz condition on $f$). Arzelà-Ascoli guarantees that a convergent subsequence exists; Picard-Lindelöf ensures the limit is the true solution.

---

## 7. Connection to the Quiz App

The quiz app (`quiz-app/`) uses the README files and chapter files from this part to generate questions. The chapter files describe the mathematical content; the quiz generates both conceptual questions ("State the Banach Fixed Point Theorem") and computational questions ("Compute the first 3 iterates of $x \mapsto \cos x$ starting from $x_0 = 1$").

The `subject.toml` file maps chapters to quiz phases, starting with ODE Fundamentals and Geometric Methods from Part I. Adding computational notes like these to the chapter files will enrich the question pool with questions about algorithms and numerical methods.

**Example quiz questions generated from these computational notes**:
- "The QR method for computing Lyapunov exponents uses orthogonalization at each step. Why is this necessary? What goes wrong without it?"
- "Ulam's method approximates the Frobenius-Perron operator on a finite mesh. What does the left Perron eigenvector of the resulting matrix approximate?"
- "State the error bound for fixed-point iteration with contraction constant $k < 1$. If $k = 0.9$ and the initial error is $1$, how many iterations are needed to reach accuracy $10^{-6}$?"
