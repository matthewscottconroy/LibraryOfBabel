# Chapter 2: Exercises

---

## Part A: Conceptual Exercises

**Exercise 1.** *(Fixed points and the cobweb)*

Consider the map $f(x) = \mu x(1 - x^2/3)$ for $x \in \mathbb{R}$ and $\mu > 0$.

(a) Find all fixed points as functions of $\mu$.

(b) For each fixed point, determine for which values of $\mu$ it is stable.

(c) Sketch the cobweb diagram for $\mu = 1.5$ starting from $x_0 = 0.5$. Describe the long-term behavior.

(d) At what value of $\mu$ does the origin change stability? What kind of bifurcation is this?

**Sketch of solution:** The fixed points are $x^* = 0$ (always) and $x^* = \pm\sqrt{3(1 - 1/\mu)}$ (real when $\mu > 1$). The derivative at the origin is $f'(0) = \mu$; stability requires $|\mu| < 1$, so the origin is stable for $\mu < 1$ and loses stability at $\mu = 1$. At the nonzero fixed points, $f'(x^*) = \mu(1 - x^{*2}) = \mu(1 - 3(1-1/\mu)) = 3 - 2\mu$; stability requires $|3 - 2\mu| < 1$, i.e., $1 < \mu < 2$. At $\mu = 1$, the origin undergoes a **pitchfork bifurcation**: two new stable fixed points are born as the origin loses stability.

---

**Exercise 2.** *(Trace-determinant classification)*

For each of the following Jacobians, identify the fixed-point type (node, spiral, saddle, center) and stability:

(a) $J = \begin{pmatrix} -3 & 1 \\ -2 & -1 \end{pmatrix}$

(b) $J = \begin{pmatrix} 0 & -4 \\ 1 & 0 \end{pmatrix}$

(c) $J = \begin{pmatrix} 2 & -5 \\ 1 & -2 \end{pmatrix}$

(d) $J = \begin{pmatrix} -1 & 3 \\ -3 & -1 \end{pmatrix}$

**Sketch of solutions:**

(a) $\tau = -4$, $\Delta = 3 + 2 = 5 > 0$. Discriminant: $16 - 20 = -4 < 0$. Complex eigenvalues, $\text{Re}(\lambda) = \tau/2 = -2 < 0$. **Stable spiral.**

(b) $\tau = 0$, $\Delta = 4 > 0$. Eigenvalues purely imaginary: $\lambda = \pm 2i$. **Center.** (Neutral, linearization inconclusive for nonlinear stability.)

(c) $\tau = 0$, $\Delta = -4 + 5 = 1 > 0$. Discriminant: $0 - 4 < 0$. Complex eigenvalues, $\text{Re}(\lambda) = 0$. **Center** again. But note $\tau = 0$ and $\Delta = 1 > 0$.

(d) $\tau = -2$, $\Delta = 1 + 9 = 10 > 0$. Discriminant: $4 - 40 = -36 < 0$. Complex eigenvalues, $\text{Re}(\lambda) = -1 < 0$. **Stable spiral.**

---

**Exercise 3.** *(Volume contraction and attractors)*

A dynamical system $\dot{\mathbf{x}} = f(\mathbf{x})$ on $\mathbb{R}^3$ has $\nabla \cdot f = -c$ for a constant $c > 0$.

(a) Show that any initial volume element $V(0)$ shrinks as $V(t) = V(0) e^{-ct}$.

(b) Explain why this implies the system must have an attractor of zero volume (Lebesgue measure zero).

(c) Can a zero-volume attractor still have positive topological dimension? Give an example.

(d) What is the relationship between $c$ and the sum of Lyapunov exponents for this system?

**Sketch of solution:** (a) Liouville's theorem states $\dot{V}/V = \nabla \cdot f$; solving: $V(t) = V(0)e^{-ct}$. (b) The attractor is the intersection of all forward-time images of a bounded absorbing set; each image has smaller volume. The limit has zero volume. (c) Yes: the Lorenz attractor has dimension $\approx 2.06 > 0$ but zero 3D volume. (d) By the sum formula, $\sum_k \lambda_k = \langle \nabla \cdot f \rangle = -c$.

---

**Exercise 4.** *(Sensitive dependence — quantitative)*

The maximum Lyapunov exponent of a chaotic system is $\lambda_{\max} = 0.5 \text{ time}^{-1}$.

(a) If the initial measurement uncertainty is $\varepsilon = 10^{-6}$ and the tolerable prediction error is $\delta = 1$, compute the predictability horizon.

(b) How much does the predictability horizon increase if you reduce measurement uncertainty to $\varepsilon = 10^{-12}$?

(c) If you improve your model so that the effective $\lambda_{\max}$ decreases to $0.1$, what is the new predictability horizon for $\varepsilon = 10^{-6}$?

(d) Which improvement gives more forecast lead time: a factor of $10^6$ better measurements, or a factor of 5 better model?

**Sketch of solution:** (a) $T = \ln(10^6)/0.5 = 6\ln(10)/0.5 \approx 27.6$ time units. (b) New horizon: $\ln(10^{12})/0.5 = 12\ln(10)/0.5 \approx 55.2$ — an increase of $27.6$ units. (c) $\ln(10^6)/0.1 = 138$ units. (d) Better model (5× reduction in $\lambda$) gives $\ln(10^6)/0.1 = 138$ vs. $\ln(10^{12})/0.5 = 55.2$. The better model wins by a factor of $\sim 2.5$.

---

**Exercise 5.** *(Nullclines and fixed points)*

Consider the system:

$$\dot{x} = x(3 - x - 2y), \qquad \dot{y} = y(2 - y - x)$$

(a) Find all fixed points. (Hint: there are four.)

(b) Sketch the nullclines in the first quadrant ($x \geq 0$, $y \geq 0$).

(c) Compute the Jacobian at each fixed point and classify it.

(d) This is a two-species competition model. Based on your stability analysis, describe the biological outcome for initial conditions in the interior of the first quadrant.

**Sketch of solution:** The $x$-nullclines are $x = 0$ and $3 - x - 2y = 0$ (i.e., $x + 2y = 3$). The $y$-nullclines are $y = 0$ and $2 - y - x = 0$ (i.e., $x + y = 2$). Fixed points: $(0,0)$, $(3,0)$, $(0,2)$, and the intersection of $x + 2y = 3$ and $x + y = 2$: solving, $y = 1$, $x = 1$, giving $(1,1)$. Stability analysis of $(1,1)$: Jacobian has $\tau = -1 - 1 = -2$ (from diagonal terms $-x$ and $-y$ at the interior fixed point, evaluated via chain rule), and positive determinant, giving a stable node. The interior fixed point is the coexistence equilibrium and it is stable — both species persist.

---

**Exercise 6.** *(The logistic map and the Feigenbaum constant)*

Using the bifurcation values $r_1 = 3$, $r_2 \approx 3.449$, $r_3 \approx 3.544$, $r_4 \approx 3.5644$:

(a) Compute the ratios $(r_2 - r_1)/(r_3 - r_2)$, $(r_3 - r_2)/(r_4 - r_3)$.

(b) Compare to the Feigenbaum constant $\delta \approx 4.6692$.

(c) Extrapolate to estimate $r_5$ (the parameter at which period-16 orbit appears) and $r_\infty$.

(d) Explain in words why this universal ratio exists, referencing the concept of a renormalization group fixed point.

**Sketch of solution:** (a) $(3.449 - 3)/(3.544 - 3.449) = 0.449/0.095 \approx 4.73$; $(3.544 - 3.449)/(3.5644 - 3.544) = 0.095/0.0204 \approx 4.66$. (b) Converging toward $4.6692$. (c) $r_5 \approx r_4 + (r_4 - r_3)/\delta \approx 3.5644 + 0.0204/4.6692 \approx 3.5688$. $r_\infty = r_4 + (r_4 - r_3)/(\delta - 1) \approx 3.5644 + 0.0204/3.6692 \approx 3.5699$. (d) The Feigenbaum constant arises because period-doubling is a fixed point of a functional renormalization group transformation on unimodal maps; the universal constant is an eigenvalue of the linearization of this transformation.

---

**Exercise 7.** *(Schur-Cohn stability)*

For the discrete-time map $x_{t+1} = f(x_t, y_t)$, $y_{t+1} = g(x_t, y_t)$ with Jacobian at a fixed point having trace $\tau$ and determinant $\Delta$:

(a) Derive the three Schur-Cohn conditions from first principles, by requiring that $p(1) > 0$, $p(-1) > 0$, and $\Delta < 1$ where $p(\lambda) = \lambda^2 - \tau\lambda + \Delta$.

(b) Sketch the stability region in the $(\tau, \Delta)$ plane.

(c) Show that for real eigenvalues, the conditions reduce to $-1 < \lambda_k < 1$ for each $k$.

(d) A reservoir weight matrix has spectral radius $\rho = 0.99$. Is the autonomous fixed point necessarily stable? What additional information do you need?

**Sketch of solution:** (a) $p(1) = 1 - \tau + \Delta > 0 \Rightarrow \tau < 1 + \Delta$. $p(-1) = 1 + \tau + \Delta > 0 \Rightarrow \tau > -(1 + \Delta)$. Leading coefficient positive already. Plus $|\Delta| < 1$ from $|\lambda_1 \lambda_2| < 1$ when both are inside unit circle. (d) Not necessarily: the spectral radius gives stability only for linear maps (or after linearization). The nonlinear reservoir has a Jacobian that depends on the operating point; the spectral radius of $W^{\text{res}}$ is only an approximation near the zero state. You also need to know the gain of the nonlinearity ($\tanh'$) at the operating point.

---

**Exercise 8.** *(Lyapunov exponents and dimensions)*

A 4-dimensional dissipative system has Lyapunov exponents $\lambda_1 = 2.1$, $\lambda_2 = 0.3$, $\lambda_3 = 0$, $\lambda_4 = -8.7$.

(a) Verify that the sum is consistent with dissipation ($\sum \lambda_k < 0$).

(b) Compute the Kaplan-Yorke dimension.

(c) What is the minimum embedding dimension for reconstructing this attractor via Takens' theorem?

(d) What is the predictability horizon if $\varepsilon = 10^{-4}$ and $\delta = 1$?

**Sketch of solution:** (a) $2.1 + 0.3 + 0 + (-8.7) = -6.3 < 0$. ✓ (b) $j = 3$ since $2.1 + 0.3 + 0 = 2.4 > 0$ but $2.4 + (-8.7) < 0$. $d_{KY} = 3 + 2.4/8.7 \approx 3.28$. (c) Takens requires $m \geq 2d + 1 \approx 2(3.28) + 1 \approx 7.56$, so $m = 8$ dimensions minimum. (d) $T = \ln(10^4)/2.1 = 4\ln(10)/2.1 \approx 4.38$ time units.

---

## Part B: Thought Experiments

**Thought Experiment 1.** *(The determinism paradox)*

The Lorenz system is fully deterministic: given the initial state exactly, the future is exactly determined. Yet it is practically unpredictable beyond a few Lyapunov times. Consider the following claim:

> "True randomness and deterministic chaos are, for all practical purposes, identical."

Do you agree? Develop your answer in three parts: (i) In what senses are they *equivalent* from an information-theoretic perspective? (ii) In what senses are they *different*? (iii) How does this affect the design of reservoir computers for chaotic time series prediction?

---

**Thought Experiment 2.** *(Dimensionality and memory)*

A reservoir has $N = 100$ neurons. Suppose the echo function $\phi$ maps input histories to reservoir states, and that $\phi$ is smooth and injective on input histories of length $L$.

(a) What is the maximum $L$ for which we might expect $\phi$ to be injective, based on dimensional considerations? (Hint: input histories of length $L$ form an $L$-dimensional space.)

(b) If the input history space has effective dimension $d_{\text{task}}$, how does the required reservoir size scale with $d_{\text{task}}$?

(c) A chaotic system's attractor has Kaplan-Yorke dimension $d_{KY} = 2.06$ (Lorenz). Estimate the required reservoir size to reconstruct its attractor with a single reservoir.

---

**Thought Experiment 3.** *(Edge of stability)*

The "edge of chaos" hypothesis in neuroscience [Langton1990] holds that neural systems at the boundary between ordered and chaotic dynamics are maximally complex and computationally powerful.

(a) What does "edge of chaos" mean precisely in terms of Lyapunov exponents and spectral radius for a reservoir?

(b) Give two advantages of operating near the edge. Give two disadvantages.

(c) Is "edge of stability" a better term than "edge of chaos" for reservoir computing? Why or why not?

---

**Thought Experiment 4.** *(Conservation laws and computation)*

The undamped Lotka-Volterra system conserves the quantity $H = \delta x - \gamma \ln x + \beta y - \alpha \ln y$. The trajectories are closed curves (centers).

(a) Would a reservoir built from undamped Lotka-Volterra units have the echo state property? Why or why not?

(b) What would happen if you drove such a reservoir with an external input?

(c) More generally: why must a useful reservoir be *dissipative* rather than conservative? What does dissipation provide that conservation cannot?

---

## Part C: Lab Exercises

**Lab Exercise 1: The Lorenz System — Sensitive Dependence**

Write a Python script that:

(a) Integrates the Lorenz system (canonical parameters) from two initial conditions differing by $10^{-8}$ in the $x$-coordinate.

(b) Plots the separation $\|\mathbf{x}^{(1)}(t) - \mathbf{x}^{(2)}(t)\|$ on a semilog scale.

(c) Fits a straight line to the initial linear growth phase to estimate $\lambda_{\max}$.

(d) Marks the "predictability horizon" (where separation reaches 1% of attractor diameter).

**Starter code:**

```python
import numpy as np
import matplotlib.pyplot as plt
from scipy.stats import linregress

def lorenz(state, sigma=10., rho=28., beta=8/3):
    x, y, z = state
    return np.array([sigma*(y-x), x*(rho-z)-y, x*y-beta*z])

def rk4(f, s, h):
    k1=f(s); k2=f(s+h/2*k1); k3=f(s+h/2*k2); k4=f(s+h*k3)
    return s + h/6*(k1+2*k2+2*k3+k4)

dt = 0.01
T = 40
N = int(T/dt)

x0a = np.array([1., 1., 1.])
x0b = x0a + np.array([1e-8, 0., 0.])

# TODO: integrate both trajectories and compute separation
# TODO: fit exponential to early growth phase
# TODO: estimate attractor diameter (max pairwise distance over long run)
# TODO: mark predictability horizon on plot
```

**Expected result:** $\lambda_{\max} \approx 0.9$, predictability horizon $\approx 25$ time units.

---

**Lab Exercise 2: Bifurcation Diagram of the Logistic Map**

Write a Python script that generates the bifurcation diagram of the logistic map $x_{t+1} = rx_t(1-x_t)$ for $r \in [2.5, 4.0]$.

**Requirements:**
- For each $r$, discard the first 500 iterates (transient) and plot the next 200 iterates.
- Use a scatter plot with small dots ($\alpha = 0.1$) and a fine grid of $r$ values ($\geq 1000$).
- Mark the onset of chaos ($r \approx 3.5699$) with a vertical dashed line.
- Annotate the first three period-doubling bifurcations.
- In a second panel, plot the maximum Lyapunov exponent $\lambda(r)$ as a function of $r$.

**Starter code:**

```python
import numpy as np
import matplotlib.pyplot as plt

def logistic_bifurcation(r_min=2.5, r_max=4.0, num_r=2000,
                          warmup=500, plot_iters=200):
    r_values = np.linspace(r_min, r_max, num_r)
    fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(12, 8), sharex=True)
    
    lyapunov = np.zeros(num_r)
    
    for i, r in enumerate(r_values):
        x = 0.5
        # Warmup
        for _ in range(warmup):
            x = r * x * (1 - x)
        # Collect trajectory and compute Lyapunov exponent
        xs = []
        lyap_sum = 0.
        for _ in range(plot_iters):
            xs.append(x)
            # Accumulate log|f'(x)| = log|r(1-2x)|
            deriv = abs(r * (1 - 2*x))
            if deriv > 0:
                lyap_sum += np.log(deriv)
            x = r * x * (1 - x)
        
        ax1.plot([r]*len(xs), xs, ',k', alpha=0.1, markersize=0.5)
        lyapunov[i] = lyap_sum / plot_iters
    
    ax2.plot(r_values, lyapunov, 'b-', lw=0.8)
    ax2.axhline(y=0, color='r', linestyle='--', lw=1)
    ax2.set_ylabel('Lyapunov exponent $\\lambda$')
    ax2.set_xlabel('$r$')
    ax1.set_ylabel('$x$')
    ax1.set_title('Logistic map bifurcation diagram')
    plt.tight_layout()
    plt.savefig('logistic_bifurcation.png', dpi=200)
    plt.show()

logistic_bifurcation()
```

**Questions to answer from your plot:**
1. At what values of $r$ do you observe period-2, period-4, period-8 orbits?
2. Can you identify any windows of periodic behavior within the chaotic region?
3. Where does $\lambda$ cross zero, and how does this relate to the onset of chaos?

---

**Lab Exercise 3: Echo State Property Demonstration**

Write a Python script demonstrating the echo state property for a small reservoir.

**Setup:** Use a reservoir of $N = 50$ neurons with $W^{\text{res}}$ drawn from a sparse Gaussian distribution (density 0.1) and rescaled to have spectral radius $\rho$.

**Protocol:**
1. Drive the reservoir with a random input sequence of length $T = 500$.
2. Start from $K = 20$ different random initial states.
3. Plot all $K$ reservoir state trajectories for a single neuron.
4. For each $\rho \in \{0.5, 0.9, 0.99, 1.1\}$, compute and plot the spread (max-min) of the reservoir state across initial conditions over time.

**Expected behavior:** For $\rho < 1$, all trajectories converge to the same driven trajectory (echo state property). For $\rho = 1.1$, they diverge.

```python
import numpy as np
import matplotlib.pyplot as plt

def make_reservoir(N, density, spectral_radius, seed=42):
    rng = np.random.default_rng(seed)
    W = rng.standard_normal((N, N))
    mask = rng.uniform(0, 1, (N, N)) < density
    W = W * mask
    # Rescale to desired spectral radius
    current_rho = np.max(np.abs(np.linalg.eigvals(W)))
    if current_rho > 0:
        W = W * (spectral_radius / current_rho)
    return W

def run_reservoir(W, W_in, u_seq, x0):
    """Run reservoir from initial state x0 on input sequence u_seq."""
    N = W.shape[0]
    T = len(u_seq)
    X = np.zeros((T, N))
    x = x0.copy()
    for t, u in enumerate(u_seq):
        x = np.tanh(W @ x + W_in * u)
        X[t] = x
    return X

N = 50
T = 500
K = 20  # number of initial conditions
rho_values = [0.5, 0.9, 0.99, 1.1]

rng = np.random.default_rng(0)
u_seq = rng.standard_normal(T)  # random input

fig, axes = plt.subplots(2, 2, figsize=(12, 8))

for ax, rho in zip(axes.flat, rho_values):
    W = make_reservoir(N, 0.1, rho)
    W_in = rng.standard_normal(N) * 0.1
    
    spreads = []
    for t in range(T):
        states_t = []
        for k in range(K):
            x0 = rng.standard_normal(N) * 0.1
            # Run from x0 to time t
            X = run_reservoir(W, W_in, u_seq[:t+1], x0)
            states_t.append(X[-1, 0])  # first neuron
        spreads.append(np.max(states_t) - np.min(states_t))
    
    ax.semilogy(spreads)
    ax.set_title(f'$\\rho = {rho}$')
    ax.set_xlabel('Time step')
    ax.set_ylabel('Spread across initial conditions')

plt.suptitle('Echo State Property: convergence of initial conditions')
plt.tight_layout()
plt.savefig('echo_state_demo.png', dpi=150)
plt.show()
```

---

## Part D: Programming Projects

**Project 1: Lorenz System Parameter Sweep**

Implement a systematic investigation of how the behavior of the Lorenz system changes as $\rho$ varies from 0 to 50 (with $\sigma = 10$, $\beta = 8/3$ fixed).

**Deliverables:**
1. For each $\rho$ in a fine grid, classify the long-term behavior: fixed point, limit cycle, or chaos (using the maximum Lyapunov exponent).
2. Generate a "phase diagram" in $\rho$-space showing the transitions.
3. For $\rho < 1$: show convergence to the origin.
4. For $1 < \rho < 13.926$: show convergence to $C_\pm$.
5. For $13.926 < \rho < 24.06$: "transient chaos" — identify this regime.
6. For $\rho > 24.06$: strange attractor.
7. Produce 3D phase-space plots for representative $\rho$ values in each regime.

**Extension:** Repeat for $\sigma \in [1, 20]$ with $\rho = 28$ fixed. At what $\sigma$ does the attractor change qualitatively?

---

**Project 2: Reservoir Spectral Radius and Lyapunov Exponent**

Investigate the relationship between the spectral radius of a reservoir's weight matrix and the maximum Lyapunov exponent of the driven reservoir.

**Setup:** Use a reservoir of $N = 100$ neurons driven by a sinusoidal input $u_t = \sin(0.1 t)$.

**Protocol:**
1. For each spectral radius $\rho \in [0.1, 1.5]$, estimate the maximum Lyapunov exponent of the driven reservoir using the QR method (adapted from Section 4.3).
2. Plot $\lambda_{\max}$ vs. $\rho$.
3. Identify the transition from $\lambda_{\max} < 0$ (echo state property) to $\lambda_{\max} > 0$ (chaos).
4. Repeat for three different input amplitudes ($A = 0.1, 0.5, 2.0$ in $u_t = A\sin(0.1t)$). How does input amplitude shift the transition?

**Expected finding:** The transition from echo state to chaos occurs at a spectral radius $\rho^* < 1$ (for large inputs) or $\rho^* > 1$ (for small inputs). The echo state property is not simply determined by the spectral radius of $W^{\text{res}}$ alone — the input amplitude and the gain of the nonlinearity matter.

**Extension:** Instead of sinusoidal input, use the $x$-component of the Lorenz system as input. Compare the transition point.
