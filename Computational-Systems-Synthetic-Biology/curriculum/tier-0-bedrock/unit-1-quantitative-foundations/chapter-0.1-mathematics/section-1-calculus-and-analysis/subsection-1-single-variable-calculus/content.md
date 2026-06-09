# Single-Variable Calculus

Here is the central question that calculus was invented to answer: *how do things change?*

Not change in the vague, everyday sense — but change precisely, quantitatively, instantaneously. When a cell doubles its volume, when a morphogen gradient steepens across a tissue, when a repressor concentration drops after a pulse of inducer — all of these are stories about quantities that change over time or space. Single-variable calculus is the grammar that lets us write those stories down, and then *reason* about them.

You almost certainly encountered calculus before. But if you learned it as a collection of rules to memorize — power rule here, integration by parts there — it probably felt like a toolkit without a purpose. The purpose is this: every dynamical system you will build in this curriculum, from a simple gene expression ODE to a whole-cell model, is written in the language developed in this section. Understanding *why* these techniques work, not just *how* to apply them, is what separates a modeler who can interpret results from one who merely generates them.

## What Is a Limit, and Why Should You Care?

Mathematics, like biology, is built on approximation. We rarely have exact, closed-form answers to interesting questions — but we can often describe what happens as we *approach* an answer. That notion of approach is formalized as the **limit**.

The limit of $f(x)$ as $x$ approaches $a$ is the value that $f(x)$ gets arbitrarily close to, without $x$ ever needing to actually reach $a$. Formally:

$$\lim_{x \to a} f(x) = L \iff \forall \varepsilon > 0,\ \exists \delta > 0 \text{ such that } 0 < |x - a| < \delta \Rightarrow |f(x) - L| < \varepsilon$$

The Greek-letter formalism is intimidating at first glance, but the idea is disarmingly simple: I can make $f(x)$ as close to $L$ as you like — within any tolerance $\varepsilon$ you name — by staying sufficiently close to $a$, within distance $\delta$. That's all it says.

Why does this matter for biology? A function is **continuous** at $a$ if $\lim_{x \to a} f(x) = f(a)$ — the limit equals the actual value. Continuity encodes a physical intuition: small causes produce small effects. In biological models, continuity is not just a mathematical nicety. It is the difference between a model where a tiny increase in inducer concentration smoothly ramps up gene expression, and one where some infinitesimal perturbation triggers a catastrophic state switch. Both are biologically real phenomena — the first describes linear signaling, the second describes a bistable switch — and the mathematics of limits gives you the vocabulary to distinguish them precisely.

## The Derivative: Instantaneous Rate of Change

Suppose you are watching the concentration of a transcription factor $[X](t)$ in real time. At any given moment, you might ask: *how fast is it changing right now?* Not on average over the last minute, but at this exact instant.

That question has a precise answer. The **derivative** of $f$ at $x$ is:

$$f'(x) = \lim_{h \to 0} \frac{f(x+h) - f(x)}{h}$$

The fraction $[f(x+h) - f(x)]/h$ is the average rate of change over an interval of width $h$. As $h$ shrinks toward zero, this average converges to the *instantaneous* rate of change. The limit is doing the essential work: it extracts the exact slope of the curve at a single point from the slope of a chord that keeps getting shorter.

Biologically, the derivative is everywhere. If $N(t)$ is the number of cells in a colony, $dN/dt$ is the growth rate. If $[P](t)$ is protein concentration, $d[P]/dt$ is the net synthesis minus degradation rate. If $V$ is membrane potential, $dV/dt$ is how fast the neuron is depolarizing. The derivative converts a static snapshot of a quantity into a description of its *dynamics*.

**The rules you must internalize** — and I mean internalize, not merely look up — are:

- **Power rule**: $\dfrac{d}{dx} x^n = n x^{n-1}$
- **Product rule**: $\dfrac{d}{dx}[f \cdot g] = f'g + fg'$
- **Quotient rule**: $\dfrac{d}{dx}\left[\dfrac{f}{g}\right] = \dfrac{f'g - fg'}{g^2}$
- **Chain rule**: $\dfrac{d}{dx} f(g(x)) = f'(g(x)) \cdot g'(x)$

The **chain rule** deserves special emphasis because biological systems are almost never described by single functions in isolation — they are *cascades*, where the output of one process feeds into the next. If protein activity $A$ depends on phosphorylation state $[P_{ph}]$, and phosphorylation depends on upstream kinase activity $K$, then $dA/dK = (dA/d[P_{ph}])(d[P_{ph}]/dK)$. Sensitivity analysis of signaling networks is, at its core, chains of derivatives multiplied together. When you see a Jacobian matrix in Chapter 2.1, you will recognize it as a structured collection of chain-rule computations.

**Implicit differentiation** handles situations where you can't easily solve for one variable in terms of another — which is common in enzyme kinetics. If the relationship between substrate concentration $[S]$ and reaction rate $v$ is given implicitly by $F([S], v) = 0$, then:

$$\frac{dv}{d[S]} = -\frac{\partial F/\partial [S]}{\partial F/\partial v}$$

The Michaelis-Menten equation $v = V_{\max}[S]/(K_m + [S])$ can be derived this way from the quasi-steady-state assumption on the enzyme-substrate complex.

## Integration: Accumulation and the Fundamental Theorem

If the derivative asks "how fast is this changing?", the integral asks "how much has accumulated?" The **definite integral** $\int_a^b f(x)\, dx$ is the signed area between the curve $f(x)$ and the $x$-axis from $a$ to $b$. For biology, think of it as total protein produced between times $a$ and $b$, or the total dose of a drug absorbed over an interval, or the total mRNA degraded during a pulse.

What makes calculus remarkable — truly remarkable, and historically it surprised even Newton and Leibniz — is the **Fundamental Theorem of Calculus**, which reveals that differentiation and integration are inverse operations:

$$\frac{d}{dx} \int_a^x f(t)\, dt = f(x)$$

Differentiate an accumulation and you recover the rate. Integrate a rate and you recover the accumulation. This deep symmetry underlies the reason that solving ODEs and computing integrals are essentially the same problem.

**The two techniques you will use constantly:**

**Substitution** converts a complicated integral into a simpler one by changing variables. If $u = g(x)$, then $\int f(g(x)) g'(x)\, dx = \int f(u)\, du$. You will use this every time you integrate an exponential with a non-trivial exponent, which in practice means almost always.

**Integration by parts** handles products: $\int u\, dv = uv - \int v\, du$. The functions $te^{-t}$, $e^{-\lambda t}\sin(\omega t)$, and $t^n e^{-t}$ — all of which appear in probability distributions and impulse-response functions — yield readily to this technique.

**Worked example — exponential decay.** Consider a protein that degrades at rate proportional to its own concentration:

$$\frac{d[P]}{dt} = -\delta [P]$$

Separation of variables gives $d[P]/[P] = -\delta\, dt$. Integrating both sides:

$$\int_{[P]_0}^{[P](t)} \frac{d[P']}{[P']} = -\delta \int_0^t dt' \implies \ln\frac{[P](t)}{[P]_0} = -\delta t \implies [P](t) = [P]_0 e^{-\delta t}$$

The protein half-life is $t_{1/2} = \ln(2)/\delta$. Every circuit designer needs this calculation cold: given a measured half-life, what degradation rate do you plug into your ODE? Given a tagged protein with half-life 20 minutes, $\delta = \ln 2 / 20 \approx 0.035\ \text{min}^{-1}$. This single calculation, done in your head, calibrates the timescale of your model.

## Taylor Series: The Art of Principled Approximation

Here is a question that will recur throughout this curriculum: given a complicated nonlinear function, how do you analyze it without drowning in algebra? The answer, almost invariably, is to *approximate it locally with a simpler function*.

The **Taylor series** expands any smooth function $f$ around a point $a$ as a polynomial:

$$f(x) = f(a) + f'(a)(x-a) + \frac{f''(a)}{2!}(x-a)^2 + \frac{f'''(a)}{3!}(x-a)^3 + \cdots$$

For most practical purposes, the **linear approximation** — the first two terms — is sufficient:

$$f(x) \approx f(a) + f'(a)(x - a)$$

This is the tangent line to $f$ at $a$. It seems almost too simple. But it is precisely this approximation that lets us analyze the stability of nonlinear ODE systems. When you have a steady state $x^*$ and you want to know whether small perturbations grow or decay, you linearize the system around $x^*$: replace every nonlinear term with its first-order Taylor expansion. The resulting linear system can be analyzed exactly using eigenvalues. You will do this calculation in Chapter 2.1, but the mathematical foundation is right here: the Taylor series.

The exponential function $e^x = 1 + x + x^2/2! + x^3/3! + \cdots$ converges for all $x \in \mathbb{R}$, making it uniquely useful. The logarithm, the trigonometric functions, the Hill function — all have Taylor expansions that reveal their behavior near particular operating points.

## A Note on Convergence

Not all series converge, and knowing when an approximation is valid is as important as knowing how to compute it. The **ratio test** provides a simple check:

$$\lim_{n \to \infty} \left|\frac{a_{n+1}}{a_n}\right| < 1 \implies \text{the series } \sum a_n \text{ converges}$$

For the Taylor series of $\ln(1+x)$, this test reveals that the series only converges for $|x| < 1$ — a fact that matters when you use log-linearization in parameter estimation. The radius of convergence tells you the domain over which your approximation is trustworthy.

## The Bigger Picture

Everything in this section — limits, derivatives, integrals, Taylor series — is in service of a single idea: *the mathematics of continuous change*. Biological processes unfold continuously in time, concentrations vary smoothly across space, probabilities accumulate as experiments proceed. Calculus is what lets us translate verbal descriptions ("the gene turns on when the activator exceeds a threshold") into precise quantitative models that can be simulated, analyzed, and tested against data.

When you encounter an ODE model of gene regulation in Chapter 2.3, every term will be a derivative. When you fit that model to experimental data in Chapter 4.1, the fitting procedure will minimize a cost function using derivatives. When you analyze the stability of a metabolic network in Chapter 2.2, you will compute a Jacobian — a matrix of partial derivatives. The thread running through all of it begins here.

```python
import numpy as np
import matplotlib.pyplot as plt
from scipy.integrate import solve_ivp

# Exponential decay: dP/dt = -delta * P
# Demonstrates that integration and the analytical solution agree.

delta = 0.1       # degradation rate (min⁻¹)
P0    = 100.0     # initial concentration (nM)

t_eval = np.linspace(0, 60, 300)
sol = solve_ivp(
    lambda t, P: [-delta * P[0]],
    (0, 60), [P0],
    t_eval=t_eval, dense_output=True
)

t_half = np.log(2) / delta
print(f"Protein half-life: {t_half:.1f} min")
print(f"Concentration at t½: {sol.y[0][np.argmin(np.abs(sol.t - t_half))]:.1f} nM  (expect {P0/2:.1f})")

fig, ax = plt.subplots()
ax.plot(sol.t, sol.y[0], label='Numerical (solve_ivp)')
ax.plot(sol.t, P0 * np.exp(-delta * sol.t), '--', label='Analytical: $P_0 e^{-\delta t}$')
ax.axvline(t_half, color='gray', linestyle=':')
ax.axhline(P0/2,   color='gray', linestyle=':')
ax.set_xlabel('Time (min)')
ax.set_ylabel('[P] (nM)')
ax.set_title('Protein Degradation — Numerical vs. Analytical')
ax.legend()
plt.tight_layout()
```
