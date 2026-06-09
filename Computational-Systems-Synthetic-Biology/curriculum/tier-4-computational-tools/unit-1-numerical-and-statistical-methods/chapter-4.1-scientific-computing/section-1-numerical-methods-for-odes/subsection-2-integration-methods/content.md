# Integration Methods for ODEs

Imagine you are tracking the mRNA and protein concentrations in the repressilator oscillator — three genes locked in a repression ring, each suppressing the next. The true trajectory of these six concentrations traces a limit cycle through six-dimensional space. Your job as a numerical integrator is to follow that trajectory faithfully, stepping forward in time without accumulating so much error that your path diverges from the true one. How you take those steps — how big they are, how you estimate where the trajectory is going, whether you adapt to the local curvature — determines whether you succeed.

The family of numerical integration methods for ODEs spans a wide spectrum of accuracy, stability, and computational cost. For biological models, three methods dominate practical use: Euler (instructional baseline), Runge-Kutta 4 (fixed-step workhorse), and Dormand-Prince RK45 (the adaptive-step default). Understanding how each works reveals why the adaptive methods are almost always preferred.

## Euler's Method: The Pedagogical Baseline

Given $\dot{u} = f(u, t)$ with $u(t_0) = u_0$, Euler's method advances the solution by one step of size $h$:

$$u_{n+1} = u_n + h \cdot f(u_n, t_n)$$

This is a **first-order** method: the global error after $T/h$ steps scales as $O(h)$. Halving $h$ halves the error. The method uses only one function evaluation per step, but the accuracy is poor.

**Derivation:** Truncate the Taylor expansion of $u(t_n + h)$ after the first derivative term:
$$u(t_n + h) = u(t_n) + h\dot{u}(t_n) + \frac{h^2}{2}\ddot{u}(t_n) + \cdots$$

The local truncation error is $O(h^2)$ per step, giving $O(h)$ globally.

**Why Euler fails in practice:** For the gene expression model $\dot{P} = \alpha - \delta P$ with $\delta = 1$ and $h = 2.5$, the stability condition requires $h < 2/\delta = 2$. With $h = 2.5$, the numerical solution oscillates and diverges, while the true solution decays smoothly to steady state $\alpha/\delta$.

```python
import numpy as np
import matplotlib.pyplot as plt

def euler_integrate(f, u0, t_span, h):
    """Forward Euler integration."""
    t0, tf = t_span
    t = np.arange(t0, tf + h, h)
    u = np.zeros((len(t), len(u0)))
    u[0] = u0
    for i in range(len(t) - 1):
        u[i+1] = u[i] + h * np.array(f(t[i], u[i]))
    return t, u

# Gene expression with fast degradation
def gene_expr(t, u, alpha=1.0, delta=1.0):
    P = u[0]
    return [alpha - delta * P]

u0 = [0.0]
t_span = (0, 10)

# Stable Euler (h=0.1 < 2/delta=2)
t_stable, u_stable = euler_integrate(gene_expr, u0, t_span, h=0.1)

# Unstable Euler (h=2.5 > 2/delta=2)
t_unstable, u_unstable = euler_integrate(gene_expr, u0, t_span, h=2.5)

# True solution
t_exact = np.linspace(0, 10, 1000)
u_exact = 1.0 * (1 - np.exp(-1.0 * t_exact))

fig, axes = plt.subplots(1, 2, figsize=(10, 4))
axes[0].plot(t_exact, u_exact, 'k-', label='Exact')
axes[0].plot(t_stable, u_stable[:, 0], 'C0--', label='Euler h=0.1')
axes[0].set_title('Stable: h=0.1')
axes[0].legend()

axes[1].plot(t_exact, u_exact, 'k-', label='Exact')
axes[1].plot(t_unstable, u_unstable[:, 0], 'C1--o', label='Euler h=2.5')
axes[1].set_title('Unstable: h=2.5')
axes[1].set_ylim(-5, 5)
axes[1].legend()

plt.tight_layout()
plt.savefig("euler_stability.pdf")
```

## Runge-Kutta 4: Four Evaluations, Fourth-Order Accuracy

The **classical Runge-Kutta method of order 4 (RK4)** achieves fourth-order accuracy by evaluating $f$ at four trial points within each step:

$$k_1 = f(t_n, u_n)$$
$$k_2 = f\!\left(t_n + \tfrac{h}{2},\; u_n + \tfrac{h}{2}k_1\right)$$
$$k_3 = f\!\left(t_n + \tfrac{h}{2},\; u_n + \tfrac{h}{2}k_2\right)$$
$$k_4 = f(t_n + h,\; u_n + hk_3)$$

$$u_{n+1} = u_n + \frac{h}{6}(k_1 + 2k_2 + 2k_3 + k_4)$$

Global error scales as $O(h^4)$: halving $h$ reduces error by a factor of 16. With a fixed step size of $h = 0.1$, RK4 is roughly $10^4$ times more accurate than Euler at the same step size (comparing $h^1$ vs $h^4$).

The drawback of classical RK4 is its **fixed step size**. Biology is rarely uniform: a signaling pulse lasting 10 ms embedded in a 6-hour simulation requires either a tiny step everywhere (expensive) or adaptive logic.

## Dormand-Prince: Adaptive Step Size via Embedded Pairs

The **Dormand-Prince method** (RKDP, implemented in SciPy as `method='RK45'`) solves the fixed step problem through an **embedded pair**: a single set of six function evaluations simultaneously produces a fourth-order and a fifth-order estimate of $u_{n+1}$. The difference between these estimates is the **local error estimate**:

$$\hat{e}_n = u_{n+1}^{(5)} - u_{n+1}^{(4)}$$

If $\|\hat{e}_n\|$ exceeds the requested tolerance, the step is rejected and repeated with a smaller $h$. If the error is much smaller than needed, the next step uses a larger $h$. The step size controller update rule is:

$$h_{\text{new}} = h_{\text{old}} \cdot \left(\frac{\text{tol}}{\|\hat{e}\|}\right)^{1/5}$$

This is the **FSAL property** (First Same As Last): $k_6$ of the current step equals $k_1$ of the next step, giving seven-stage efficiency from six unique evaluations per accepted step.

```python
from scipy.integrate import solve_ivp
import numpy as np

def repressilator(t, u, alpha=100, alpha0=1e-4, n=2, beta=1.0):
    m1, m2, m3, p1, p2, p3 = u
    dm1 = -m1 + alpha / (1 + p3**n) + alpha0
    dm2 = -m2 + alpha / (1 + p1**n) + alpha0
    dm3 = -m3 + alpha / (1 + p2**n) + alpha0
    dp1 = -beta * (p1 - m1)
    dp2 = -beta * (p2 - m2)
    dp3 = -beta * (p3 - m3)
    return [dm1, dm2, dm3, dp1, dp2, dp3]

u0 = [0.1, 0.2, 0.3, 0.1, 0.2, 0.3]

# Adaptive RK45 — step size chosen automatically
sol = solve_ivp(
    repressilator,
    t_span=(0, 200),
    y0=u0,
    method='RK45',
    rtol=1e-8,
    atol=1e-10,
    dense_output=True  # enables sol.sol(t) for continuous interpolation
)

# dense_output: evaluate at arbitrary t between accepted steps
t_fine = np.linspace(0, 200, 5000)
u_fine = sol.sol(t_fine)  # no additional RHS evaluations needed

print(f"Steps accepted: {sol.t.shape[0]}")
print(f"RHS evaluations: {sol.nfev}")
print(f"Step sizes ranged from {np.diff(sol.t).min():.4f} to {np.diff(sol.t).max():.4f}")
```

## Comparing Methods Side by Side

| Method | Order | Evaluations/step | Step size | Best for |
|--------|-------|-----------------|-----------|----------|
| Euler | 1 | 1 | Fixed | Learning only |
| RK4 | 4 | 4 | Fixed | Simple non-stiff, known step |
| RK45 (RKDP) | 4(5) | 6 | Adaptive | Non-stiff, general use |
| RK23 | 2(3) | 3 | Adaptive | Low accuracy needed, fast |
| DOP853 | 8 | 13 | Adaptive | High accuracy needed |

For biological ODE systems that are smooth and non-stiff, `solve_ivp(method='RK45')` is the default choice. For stiff systems — discussed in the next section — an entirely different class of methods is required.

## Why This Matters

Choosing an inappropriate integrator wastes compute at best and silently produces wrong answers at worst. The intuition that "more steps = more accurate" is only true within a given method family. A single step of RK45 is more accurate than 10,000 steps of Euler for the same problem. Adaptive step control is not a luxury — it is the only principled way to get the requested accuracy without manual tuning.
