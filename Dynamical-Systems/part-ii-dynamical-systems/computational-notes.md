# Computational Notes — Part II: Dynamical Systems

These notes connect the dynamical systems theory of Part II to computational experiments. The emphasis is on three themes: visualization of phase portraits and attractors, numerical computation of invariant objects (Lyapunov exponents, invariant measures, Markov partitions), and symbolic representation of dynamics.

---

## 1. Phase Portrait Visualization

### Nullcline Analysis

For a 2D system $\dot{x} = f(x,y)$, $\dot{y} = g(x,y)$, the *nullclines* are the curves $f(x,y) = 0$ (where $\dot{x} = 0$) and $g(x,y) = 0$ (where $\dot{y} = 0$). Equilibria are intersections of nullclines.

```
# Pseudocode for phase portrait
x_range = linspace(-5, 5, 50)
y_range = linspace(-5, 5, 50)
X, Y = meshgrid(x_range, y_range)
dX = f(X, Y)
dY = g(X, Y)
# Normalize for display
speed = sqrt(dX**2 + dY**2)
dX_norm = dX / (speed + 1e-10)
dY_norm = dY / (speed + 1e-10)
streamplot(x_range, y_range, dX_norm, dY_norm)
contour(X, Y, f(X,Y), levels=[0], colors='blue')   # x-nullcline
contour(X, Y, g(X,Y), levels=[0], colors='red')    # y-nullcline
```

**Key systems to visualize**:
- van der Pol oscillator: $\dot{x} = y$, $\dot{y} = \mu(1-x^2)y - x$. Observe the limit cycle for $\mu = 1$ and the "relaxation oscillation" for $\mu = 10$.
- Lotka-Volterra: $\dot{x} = ax - bxy$, $\dot{y} = cxy - dy$. Observe the center (conservative orbits) and the role of the Hamiltonian $H = -c\ln x - a\ln y + bx + dy$.
- The Lorenz system projected onto $(x, z)$ plane: observe the "butterfly" attractor.

---

## 2. Computing Lyapunov Exponents

### Method 1: Direct Divergence (Single Exponent)

Start two orbits at distance $\varepsilon$ and measure their separation over time:

```
def max_lyapunov(f, x0, T, dt=0.01, epsilon=1e-8):
    x = x0
    y = x0 + epsilon * random_unit_vector()
    log_sum = 0.0
    for t in range(T):
        x = rk4(f, x, dt)
        y = rk4(f, y, dt)
        d = norm(y - x)
        log_sum += log(d / epsilon)
        y = x + epsilon * (y - x) / d  # renormalize
    return log_sum / (T * dt)
```

The renormalization step prevents overflow when orbits diverge. The result is $\lambda_{\max}$.

**Expected values**:
- Lorenz (standard params): $\lambda_{\max} \approx 0.906$
- Hénon ($a=1.4$, $b=0.3$): $\lambda_{\max} \approx 0.419$
- Logistic map ($\mu = 4$): $\lambda = \log 2 \approx 0.693$ (exact)
- Van der Pol limit cycle: $\lambda_{\max} = 0$ (periodic orbit)

### Method 2: QR Algorithm (Full Spectrum)

```
def lyapunov_spectrum(f, Df, x0, T, dt=0.01):
    n = len(x0)
    x = x0
    J = identity(n)  # tangent frame
    exponents = zeros(n)
    for t in range(T):
        x_new = rk4(f, x, dt)
        # Propagate tangent frame via variational equations
        # dJ/dt = Df(x) @ J
        J_new = rk4(lambda J: Df(x) @ J, J, dt)
        # QR orthogonalization
        Q, R = qr(J_new)
        exponents += log(abs(diag(R)))
        J = Q
        x = x_new
    return exponents / (T * dt)
```

**The full Lyapunov spectrum** for the Lorenz system: $(\lambda_1, \lambda_2, \lambda_3) \approx (0.906, 0, -14.572)$.
- $\lambda_1 > 0$: chaotic (exponential separation)
- $\lambda_2 = 0$: neutral (along the flow direction)
- $\lambda_3 < 0$: dissipative (volumes contract)
- Sum $= \lambda_1 + \lambda_2 + \lambda_3 = \text{tr}(Df) = -\sigma - 1 - \beta \approx -13.67$

---

## 3. Ergodic Theory: Computing Invariant Measures

### Birkhoff Averaging

The simplest computation in ergodic theory: approximate $\int \varphi \, d\mu$ by the time average $\frac{1}{N}\sum_{k=0}^{N-1} \varphi(f^k(x))$ for a single long orbit.

```
def time_average(f, phi, x0, N=100000):
    x = x0
    total = 0.0
    for k in range(N):
        total += phi(x)
        x = f(x)
    return total / N
```

**Experiment**: For the logistic map $f_4(x) = 4x(1-x)$ and $\varphi(x) = x$:
- Time average ≈ $1/2$ (the mean of the arcsine distribution)
- For $\varphi(x) = x^2$: time average ≈ $3/8$
- For $\varphi(x) = \mathbf{1}_{[0,1/2]}(x)$: time average ≈ $1/2$ (equal time in left and right halves)

### Ulam's Method for Invariant Density

Discretize the phase space into $n$ cells and build the Markov matrix:
$$P_{ij} = \frac{\text{Lebesgue}(f^{-1}(I_j) \cap I_i)}{\text{Lebesgue}(I_i)}$$

The stationary distribution $\pi P = \pi$ approximates the invariant density.

```
def ulam_matrix(f, n=100, num_samples=10000):
    cells = linspace(0, 1, n+1)
    P = zeros((n, n))
    for i in range(n):
        # Sample from cell i
        for s in range(num_samples):
            x = uniform(cells[i], cells[i+1])
            y = f(x)
            j = searchsorted(cells, y) - 1
            if 0 <= j < n:
                P[j, i] += 1  # y lands in cell j
    # Normalize columns
    P /= P.sum(axis=0, keepdims=True)
    return P
```

For the logistic map $f_4$: the left eigenvector of $P$ approximates $\frac{1}{\pi\sqrt{x(1-x)}}$ (the arcsine density). Verify by plotting.

---

## 4. Symbolic Dynamics: Coding a Hyperbolic Map

### Itinerary Coding

For the doubling map $D(x) = 2x \pmod 1$ with partition $\{[0,1/2), [1/2,1)\}$:
```
def itinerary(f, x, n, partition_boundary=0.5):
    """Return binary symbolic sequence of length n"""
    code = []
    for k in range(n):
        code.append(0 if x < partition_boundary else 1)
        x = f(x)
    return code
```

**Key experiment**: Verify that the itinerary completely determines $x$ for the doubling map:
$$x = \sum_{k=0}^\infty s_k \cdot 2^{-(k+1)}$$
where $s_k \in \{0,1\}$ is the itinerary. The itinerary is just the binary expansion of $x$.

### Subshift of Finite Type (SFT)

The golden mean shift: forbidden words $= \{11\}$. Transition matrix:
$$A = \begin{pmatrix} 1 & 1 \\ 1 & 0 \end{pmatrix}$$

```
def count_allowed_words(A, n):
    """Count allowed words of length n in SFT"""
    k = len(A)
    v = ones(k)  # start from all states
    for i in range(n-1):
        v = A @ v
    return int(sum(v))
```

Topological entropy $= \log \lambda_{\text{PF}}(A) = \log \frac{1+\sqrt{5}}{2} \approx 0.481$ (log of the golden ratio).

Verify: the number of allowed words of length $n$ is the $n$-th Fibonacci number $F_n$, and $\frac{1}{n}\log F_n \to \log \phi$ where $\phi = (1+\sqrt{5})/2$.

---

## 5. Bifurcation Diagrams

### Period-Doubling Cascade for the Logistic Map

The bifurcation diagram is the most iconic image of nonlinear dynamics:

```
def bifurcation_diagram(mu_range, n_transient=1000, n_plot=200):
    for mu in mu_range:
        x = 0.5  # initial condition
        # Transient: let the system settle
        for k in range(n_transient):
            x = mu * x * (1 - x)
        # Record attractor
        for k in range(n_plot):
            x = mu * x * (1 - x)
            plot(mu, x, '.', color='black', markersize=0.1)
```

**Features to observe and verify computationally**:
- Period-2 orbit for $\mu \in (3, 3.449...)$: the attractor consists of 2 points
- Period-4 orbit for $\mu \in (3.449..., 3.544...)$: 4 points
- The Feigenbaum ratio: $\delta = (\mu_n - \mu_{n-1})/(\mu_{n+1} - \mu_n) \approx 4.669$
- At $\mu = 4$: the attractor fills $[0,1]$ with arcsine density

**Computing bifurcation values numerically**:
```
def period_n_orbit(f, x0, n, mu):
    """Find fixed points of f^n via Newton's method"""
    # f_n = f composed n times
    # Solve f_n(x) = x
    # Use Newton: x <- x - (f_n(x) - x) / (Df_n(x) - 1)
```

---

## 6. The Lorenz Attractor: SRB Measure and Fractal Dimension

### Box-Counting Dimension

```
def box_counting_dimension(points, epsilon_range):
    dimensions = []
    for epsilon in epsilon_range:
        # Count boxes of size epsilon that contain at least one point
        # Discretize to grid
        grid = set()
        for p in points:
            box = tuple(floor(p / epsilon).astype(int))
            grid.add(box)
        N = len(grid)
        dimensions.append((log(1/epsilon), log(N)))
    # Slope of log N vs log(1/epsilon) = box-counting dimension
    return linregress(dimensions)
```

**Expected results**:
- Lorenz attractor: $d_B \approx 2.06$
- Hénon attractor ($a=1.4$, $b=0.3$): $d_B \approx 1.26$
- Cantor middle-third set: $d_B = \log 2/\log 3 \approx 0.631$ (analytically exact)

### Kaplan-Yorke Conjecture

Given Lyapunov exponents $\lambda_1 \geq \lambda_2 \geq \cdots \geq \lambda_n$ (computed by QR method), the Kaplan-Yorke (Lyapunov) dimension is:
$$d_{KY} = j + \frac{\lambda_1 + \cdots + \lambda_j}{|\lambda_{j+1}|}$$
where $j$ is the largest index with $\lambda_1 + \cdots + \lambda_j \geq 0$.

For the Lorenz attractor: $j = 2$ (since $\lambda_1 > 0$, $\lambda_2 = 0$, $\lambda_1 + \lambda_2 > 0 > \lambda_1 + \lambda_2 + \lambda_3$):
$$d_{KY} = 2 + \frac{\lambda_1 + \lambda_2}{|\lambda_3|} \approx 2 + \frac{0.906}{14.572} \approx 2.062.$$

---

## 7. Synchronization and the Kuramoto Model

### Numerical Integration of Kuramoto

The Kuramoto model for $N$ coupled oscillators:
$$\dot{\theta}_i = \omega_i + \frac{K}{N}\sum_{j=1}^N \sin(\theta_j - \theta_i), \quad i = 1, \ldots, N.$$

```
def kuramoto_step(theta, omega, K, dt):
    N = len(theta)
    dtheta = omega.copy()
    for i in range(N):
        for j in range(N):
            dtheta[i] += (K/N) * sin(theta[j] - theta[i])
    return theta + dt * dtheta

def order_parameter(theta):
    """Complex order parameter r * e^{i*psi}"""
    return abs(mean(exp(1j * theta)))
```

**Phase transition**: The order parameter $r = |\frac{1}{N}\sum_j e^{i\theta_j}|$ transitions from $r \approx 0$ (incoherent) to $r \approx 1$ (synchronized) at the critical coupling
$$K_c = \frac{2}{\pi g(\omega_0)}$$
where $g(\omega)$ is the distribution of natural frequencies $\omega_i$ evaluated at the mean $\omega_0$.

For Gaussian $\omega_i \sim \mathcal{N}(0, \sigma^2)$: $K_c = 2\sigma\sqrt{2/\pi}$.

**Experiment**: Run the Kuramoto model with $N = 100$ oscillators, $\omega_i \sim \mathcal{N}(0,1)$, and vary $K$ from $0$ to $4$. Plot $r(K)$ and observe the phase transition near $K_c \approx 1.6$.

---

## 8. Connection to the Quiz App

The quiz app generates questions from the chapter content. Computational chapters add questions like:
- "What is the expected Lyapunov exponent of the logistic map $f_4$ at Lebesgue-a.e. initial condition? Explain the computation."
- "For the Kuramoto model with 100 oscillators and Gaussian frequency distribution, describe what happens to the order parameter as $K$ increases through the critical coupling."
- "Describe the QR method for computing the full Lyapunov spectrum of a flow. Why is orthogonalization necessary at each step?"
- "Compute the topological entropy of the golden mean shift using the Perron-Frobenius eigenvalue of the transition matrix."

These questions test both theoretical understanding and computational fluency — the combination that distinguishes a practitioner of dynamical systems.
