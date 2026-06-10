# 1.4.1 Fading Memory and the Stone-Weierstrass Theorem for Functionals

## Why "Memory" Needs to Fade

The intuition is simple: a system that remembers everything equally is not really a computational system — it is an archive. Useful computation requires the ability to weight information by its relevance, and in most natural settings, relevance decays with time. An echo fades. A wound heals. A conversation moves on.

More formally, a system that perfectly remembers all past inputs to arbitrary precision cannot be approximated by any finite-state machine, because the infinite past constitutes an infinite amount of information that must be maintained. But if memory fades — if the influence of past inputs on current output decays as those inputs recede into the past — then the effective state space of the system is finite-dimensional, and approximation by finite dynamical systems becomes possible.

This is not merely a technical convenience. It reflects something deep about how useful computation works in the physical world: physical systems always have limited memory due to dissipation, noise, and finite energy. The mathematical idealization of fading memory is a faithful abstraction of this physical reality.

## Formal Definition: Fading Memory

We work in discrete time for clarity. Let $\mathbf{u} = (\ldots, u_{-2}, u_{-1}, u_0)$ denote a bi-infinite input sequence with $u_t \in \mathbb{R}^d$.

**Definition (Fading Memory System):** A causal, time-invariant functional $H$ mapping input sequences to output sequences has the **fading memory property** with weighting sequence $w = (w_0, w_1, w_2, \ldots)$ (with $w_k > 0$ and $w_k \to 0$ as $k \to \infty$) if:

For any $\varepsilon > 0$, there exists $\delta > 0$ such that:

$$\sup_{k \geq 0} w_k \|u_t - v_t\| < \delta \implies |H[\mathbf{u}]_0 - H[\mathbf{v}]_0| < \varepsilon$$

In words: if two input sequences agree approximately on recent inputs (the weighting sequence emphasizes recency) and agree exactly on distant past inputs, then their outputs are approximately equal. The system's output is a **continuous functional** with respect to the weighted norm $\|\mathbf{u}\|_w = \sup_k w_k \|u_{-k}\|$.

The weighting sequence $w$ encodes the rate of memory decay. Exponentially decaying weights $w_k = \rho^k$ for some $\rho \in (0,1)$ give a particularly natural class, corresponding to systems whose memory decays geometrically.

## Why This Is the Right Formalization

The fading memory property is not just technically convenient — it is the natural formalization of several distinct intuitions:

**1. Physical realizability.** Any real physical system is subject to dissipation. Energy and information both decay. A system that perfectly remembered every past input would violate thermodynamic constraints. Fading memory is a mathematical statement of the Second Law of Thermodynamics applied to information processing.

**2. Statistical stationarity.** For a system to be useful as a general-purpose temporal processor, it must be able to reach a steady-state response to stationary inputs regardless of initial conditions. This requires that the influence of initial conditions fade over time — precisely the fading memory condition applied to the initial state rather than distant past inputs.

**3. Generalization.** For learning to work, the system's output must be insensitive to small perturbations in the distant past (which we cannot observe accurately) while remaining sensitive to recent inputs (which we can). The fading memory condition formalizes exactly this sensitivity structure.

**4. Approximability.** As we shall see, fading memory is the key condition that makes a system approximable by finite-dimensional dynamical systems. Systems without fading memory — systems with perfect or growing memory — cannot in general be approximated by finite systems.

## The Functional Approximation Problem

We want to approximate a functional $H$ by a system of the form:

$$\mathbf{x}_{t+1} = f(\mathbf{x}_t, u_t) \qquad \text{(state update)}$$
$$y_t = g(\mathbf{x}_t) \qquad \text{(readout)}$$

where $\mathbf{x}_t \in \mathbb{R}^N$ is a finite-dimensional state vector. The state encodes everything the system knows about the past. The readout maps the current state to the current output.

The question is: for which functionals $H$ does such a finite-dimensional approximation exist? The answer, due to Boyd and Chua [Boyd1985], is: for any functional with the fading memory property.

## The Stone-Weierstrass Theorem: Background

Before stating the Boyd-Chua theorem, let us recall the classical Stone-Weierstrass theorem, which it generalizes.

**Theorem (Stone-Weierstrass, classical):** Let $\mathcal{X}$ be a compact Hausdorff space. Let $\mathcal{A}$ be an algebra of continuous real-valued functions on $\mathcal{X}$ that:
1. Separates points: for any $x \neq y$ in $\mathcal{X}$, there exists $f \in \mathcal{A}$ with $f(x) \neq f(y)$.
2. Contains the constant functions.

Then $\mathcal{A}$ is dense in $C(\mathcal{X})$ with the uniform norm.

This theorem is the reason polynomial approximation works: polynomials form an algebra that separates points on any compact interval, so they can approximate any continuous function uniformly. It is why neural networks work: the class of multilayer networks (for appropriate activations) separates points and forms an approximating class.

The Boyd-Chua theorem extends this logic from functions to functionals — from static maps to temporal processors.

## The Boyd-Chua Approximation Theorem

We state the theorem in the discrete-time setting for clarity.

**Setting:** Let $\mathcal{U}$ be a compact set of input values in $\mathbb{R}^d$. Let $\mathcal{U}^{\mathbb{Z}^-}$ denote the space of all bi-infinite input sequences with values in $\mathcal{U}$, equipped with the weighted norm $\|\mathbf{u}\|_w = \sup_k w_k \|u_{-k}\|$ for a weighting sequence $w$ with $w_k \to 0$.

**Theorem (Boyd and Chua, 1985):** Let $H: \mathcal{U}^{\mathbb{Z}^-} \to \mathbb{R}$ be a causal, time-invariant functional with the fading memory property (with respect to the weighting sequence $w$). Then for any $\varepsilon > 0$, there exists:
- An integer $N \geq 1$ (state dimension)
- A state update function $f: \mathbb{R}^N \times \mathcal{U} \to \mathbb{R}^N$
- A readout function $g: \mathbb{R}^N \to \mathbb{R}$

such that the dynamical system

$$\mathbf{x}_{t+1} = f(\mathbf{x}_t, u_t), \qquad y_t = g(\mathbf{x}_t)$$

approximates $H$ to within $\varepsilon$: for all input sequences $\mathbf{u}$ and for all $t$,

$$|y_t - H[\mathbf{u}]_t| < \varepsilon$$

**Proof Sketch:**

The key insight is that the fading memory condition makes the functional $H$ depend effectively only on a finite portion of the past. Specifically, for any $\varepsilon > 0$, there exists a finite window length $K$ such that $H[\mathbf{u}]_t$ is well-approximated by a function of $(u_t, u_{t-1}, \ldots, u_{t-K})$ alone — the influence of inputs earlier than $t-K$ is bounded by $\varepsilon/2$.

The remaining problem is to approximate a function of $(u_t, \ldots, u_{t-K})$ by a finite-dimensional dynamical system. This can be done by taking the state $\mathbf{x}_t = (u_t, u_{t-1}, \ldots, u_{t-K})^\top \in \mathbb{R}^{(K+1)d}$ (a delay embedding), with the update rule $\mathbf{x}_{t+1} = (\mathbf{u}_{t+1}, \mathbf{x}_{t,1:(K-1)d})$ (shift the window). The readout $g$ is then the function of $K+1$ past inputs that approximates $H$, which exists by Stone-Weierstrass.

The full proof formalizes this sketch using the topology of the weighted function space $(\mathcal{U}^{\mathbb{Z}^-}, \|\cdot\|_w)$ and verifying the density condition. The relevant algebra is the class of polynomial functionals of finitely many past inputs — these separate points (by the fading memory condition) and contain constants, so Stone-Weierstrass applies.

$\square$

## What the Theorem Means for Reservoir Computing

The Boyd-Chua theorem is the theoretical license for reservoir computing. It says:

1. **Any fading-memory system can be approximated by a driven dynamical system.** The dynamical system serves as a state machine that encodes the relevant past.

2. **The approximation is general.** We do not need to know which dynamical system to use for a given $H$. Any dynamical system rich enough to encode the relevant past will work, provided the readout is trained correctly.

3. **Randomness is acceptable.** The theorem requires the existence of *some* dynamical system, but does not specify what it must look like. This opens the door to using randomly chosen systems — which is exactly what reservoir computing does.

4. **The linear readout is the key.** Once the state $\mathbf{x}_t$ faithfully represents the relevant past, the readout function $g$ is what implements the approximation of $H$. If $g$ is sufficiently expressive (e.g., a linear function with enough inputs), it can learn to approximate $H$ from training data.

The reservoir computing paradigm exploits all four of these points:
- Use a randomly generated, fixed dynamical system as the state machine.
- Trust the reservoir's rich state to encode the relevant past.
- Train only the linear readout.
- Enjoy the theoretical guarantee of Boyd-Chua: if the reservoir has fading memory and is sufficiently rich, the trained system approximates any fading-memory functional.

## The Role of Richness

The theorem guarantees approximation, but how good? This depends on the **richness** of the state representation — how much of the relevant past the state actually encodes, and how faithfully.

A delay line — storing $(u_t, u_{t-1}, \ldots, u_{t-N+1})$ as the state — is rich in the sense that it remembers $N$ past inputs exactly. But it has no nonlinearities, so it can only approximate functionals that depend on the past in a linear or quasi-linear way.

A reservoir — a high-dimensional nonlinear dynamical system — does something richer: it **mixes** past inputs through nonlinear interactions. This is not just a computational trick; it is what allows the reservoir state to encode nonlinear functions of the past, making the linear readout sufficient even for tasks that require nonlinear temporal processing.

We will quantify this richness precisely in Chapter 7, using the concept of information processing capacity.

---

## References

- [Boyd1985] Boyd, S. & Chua, L.O. (1985). Fading memory and the problem of approximating nonlinear operators with Volterra series. *IEEE Transactions on Circuits and Systems*, 32(11), 1150–1161. **[The foundational theorem for this section.]**
- [Sandberg1991] Sandberg, I.W. (1991). Approximation theorems for discrete-time systems. *IEEE Transactions on Circuits and Systems*, 38(5), 564–566.
- [Volterra1930] Volterra, V. (1930). *Theory of Functionals and of Integral and Integro-Differential Equations*. Blackie & Son. (Reprinted by Dover, 1959.)
- [Stone1948] Stone, M.H. (1948). The generalized Weierstrass approximation theorem. *Mathematics Magazine*, 21(4), 167–184.
- [Rudin1991] Rudin, W. (1991). *Functional Analysis*, 2nd ed. McGraw-Hill. (For background on the topological setting of the Stone-Weierstrass theorem.)
