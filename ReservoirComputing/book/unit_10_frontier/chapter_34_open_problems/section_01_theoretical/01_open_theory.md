# Section 34.1: Theoretical Open Problems

## 34.1.1 Problem 1: Tight Approximation Rate Bounds for Reservoir Computing

**What is known.** Chapter 26 established the following: any fading-memory functional can be approximated by polynomial functionals (Boyd-Chua theorem), and polynomial functionals of degree $s$ depending on $N$ past inputs achieve error $O(N^{-\alpha s / d_{\text{eff}}})$ for functionals in the Sobolev-type class $\mathcal{F}(w, s, R)$ with weight growth $w_k \geq Ck^\alpha$. These are *upper bounds* on the approximation error for polynomial functionals.

For *random reservoir functionals* specifically (the output of a reservoir with random weights and tanh activation), some results are known [Gonon2020, Gonon2021]: the approximation error scales as $O(N^{-s/(2d)})$ for certain classes of functionals, and generalization bounds have been proved for ridge regression readouts.

**What is not known.** 
1. **Tight bounds for random reservoirs**: The upper bounds for random reservoirs are not known to be tight. Are there functionals in $\mathcal{F}(w, s, R)$ for which random reservoirs cannot beat the polynomial upper bound? Or do the random features produced by tanh reservoirs give faster rates?

2. **Approximation vs. estimation tradeoff**: The optimal tradeoff between approximation error (decreasing in $N$) and estimation error (increasing in $N$ for fixed $T$) has not been fully characterized. Theorem 26.5.3 gives a bound, but whether the ridge regression estimator achieves the optimal tradeoff is not known.

3. **Beyond linear functionals**: The known results focus on linear and polynomial functionals of the reservoir state. For nonlinear readouts (polynomial, kernel, or neural network readouts), almost nothing is known about approximation rates.

**What a resolution would require.** A resolution would require:
- A *matching lower bound*: a class of functionals for which approximation by any random reservoir of size $N$ requires error $\geq cN^{-\beta}$ for some $\beta$.
- A proof technique for lower bounds on random reservoir approximation. This would likely require new tools from information theory (data processing inequality applied to random projections) or convex geometry (packing arguments in function spaces).
- An optimal estimator (not just any estimator) with provably matching upper and lower bounds.

**Why this matters.** Without tight bounds, we cannot determine whether larger reservoirs will help for a given task, how to optimally allocate resources between reservoir size and training set size, or whether there are architectural improvements that fundamentally break the $N^{-\alpha s / d_{\text{eff}}}$ barrier.

## 34.1.2 Problem 2: Optimal Reservoir Design — When Is Random Suboptimal?

**What is known.** Random reservoirs work well on many tasks, and the Boyd-Chua theorem guarantees that random reservoirs can approximate any fading-memory functional (given enough units). The HiPPO initialization (Chapter 30) provides a principled alternative to random initialization for linear reservoirs, and empirical results show that HiPPO significantly outperforms random initialization on tasks requiring long-range temporal dependencies.

**What is not known.** A theory of *optimal reservoir design* — characterizing which reservoir architectures minimize the required $N$ to achieve a given approximation error for a given task class. Specifically:

1. **For what task classes is random initialization suboptimal?** HiPPO outperforms random on tasks with long-range dependencies, but by how much, and is HiPPO itself optimal? Or does an even better initialization exist?

2. **What is the optimal reservoir for a known target functional?** If we know that the target functional is a Volterra series of order $s$ with kernels supported on a window of size $K$, what is the optimal reservoir architecture?

3. **Structured vs. random tradeoff**: When does the structure of the target functional allow a structured reservoir to dramatically outperform a random one? What invariances or symmetries of the task should be built into the reservoir?

**What a resolution would require.** A theory of optimal reservoir design would require:
- A precise optimization problem: "minimize $N$ subject to approximation error $\leq \varepsilon$ for all $H$ in class $\mathcal{F}$."
- A tractable parameterization of reservoir architectures.
- Connection to classical optimal experiment design and information geometry.
- Likely a minimax theorem: the optimal reservoir for worst-case approximation of $\mathcal{F}$ achieves a specific rate, and this rate is achieved by a specific architecture.

**Why this matters.** Optimal reservoir design would allow practitioners to design reservoirs specifically tailored to their task class, potentially achieving dramatic improvements over random initialization for structured tasks.

## 34.1.3 Problem 3: Online Learning Convergence in Reservoir Computing

**What is known.** The standard reservoir computing training procedure — collect $T$ time steps, solve the linear regression once offline — is well-analyzed theoretically (Chapters 26-27). Online learning algorithms for reservoir readouts (recursive least squares, online gradient descent, FORCE learning [SussilloMaass2009]) are used in practice but are much less well-analyzed.

FORCE learning is a continuous-time online learning rule for reservoir readouts that uses recursive least squares with a "running matrix inverse":
$$\dot{W}_{\text{out}} = -P(t) r(t) e(t)^\top, \quad \dot{P}(t) = -P(t)r(t)r(t)^\top P(t) / (1 + r(t)^\top P(t) r(t)),$$
where $e(t) = W_{\text{out}} r(t) - y^{\text{target}}(t)$ is the error and $P(t) = (\int_0^t r(s)r(s)^\top ds + \lambda I)^{-1}$ is the running regularized inverse correlation matrix. Empirically, FORCE learning converges quickly and stably for many tasks, but theoretical guarantees are sparse.

**What is not known.** 
1. **Convergence guarantee for FORCE**: Does FORCE converge for all fading-memory targets $y^{\text{target}}$? At what rate? The only known results are under strong stationarity assumptions on the reservoir states.

2. **Regret bounds for online RC**: What is the regret of online reservoir readout learning relative to the best offline readout? Online learning theory [CesaBianchi2006] provides regret bounds for convex losses, but the temporal correlations in reservoir states violate the independence assumptions of standard online learning theory.

3. **Stability of online RC**: Can the online readout update destabilize the reservoir dynamics? For reservoirs with fixed weights, the readout is just a linear mapping and cannot affect the reservoir state — so stability is not an issue. But for reservoirs with online weight updates (e.g., Hebbian synaptic plasticity in the reservoir), online learning can drive the system unstable.

**What a resolution would require.**
- A convergence proof for FORCE learning, likely using a Lyapunov function approach applied to the combined system of reservoir dynamics + FORCE update dynamics.
- Regret bounds for online reservoir readout learning under mixing input processes.
- A characterization of when online readout updates (combined with fixed-weight reservoir dynamics) are stable.

## 34.1.4 Problem 4: The ESP-Task Performance Gap

**What is known.** The echo state property (ESP) requires that the reservoir state is uniquely determined by the input history. The standard sufficient condition is $\rho(W) < 1$. Empirically, reservoir performance often peaks at spectral radii close to 1 (near the ESP boundary), and some researchers have reported good performance from reservoirs with spectral radius slightly above 1 (no strict ESP).

**The paradox.** If a reservoir does not have the ESP, its state is not uniquely determined by the input history — different initial conditions produce different states, and hence different outputs. How can such a system work at all for computation?

**What is not known.** 
1. **Necessary conditions for practical performance**: What is the weakest condition on reservoir dynamics that is necessary for reliable temporal computation?

2. **The "practical ESP"**: Systems without strict ESP may have a "practical ESP" — they converge approximately but not exactly to a unique state. What does "approximately" mean precisely? How much deviation from strict ESP can be tolerated without degrading performance?

3. **Why do mildly chaotic reservoirs sometimes work?** In the mildly chaotic regime ($\lambda_{\max} > 0$ but close to 0), the reservoir does not have the ESP, but the chaotic dynamics may explore a richer state space than a contracting reservoir. Under what conditions does the exploration benefit outweigh the ESP cost?

**What a resolution would require.**
- A precise quantitative statement of the performance-ESP tradeoff: $\text{NMSE}(\text{task}) \leq f(\lambda_{\max}, \text{task parameters})$.
- A distinction between tasks that require strict ESP (e.g., tasks with no tolerance for initial condition dependence) and tasks that do not (e.g., tasks where the input eventually "dominates" any initial condition even without ESP).
- A theory of "approximate ESP" — quantifying how much history must be observed before the initial condition dependence becomes negligible.

## 34.1.5 Problem 5: Understanding FORCE Learning Convergence

**What is known.** FORCE learning [SussilloMaass2009] was introduced to train reservoir readout weights online, in a way that mimics the plasticity rules thought to operate in cerebellar Purkinje cells. It converges empirically on a wide range of tasks, including generating complex oscillatory patterns. The convergence is remarkably fast (often within a few target signal cycles) and robust.

The FORCE update is essentially recursive least squares (RLS), which is well-analyzed for stationary i.i.d. observations. The convergence rate for RLS with i.i.d. observations is $O(1/t)$ for the parameter estimation error.

**What is not known.**
1. **Convergence under temporal correlations**: For reservoir states (which are strongly temporally correlated), the i.i.d. analysis does not apply. Does FORCE still converge at rate $O(1/t)$? At a slower rate? 

2. **Global vs. local convergence**: RLS is known to converge globally (from any initial weights) for i.i.d. observations when the observations span the feature space. For correlated observations (reservoir states), does FORCE converge globally or only locally (near a good solution)?

3. **The role of the running inverse matrix $P(t)$**: The FORCE algorithm requires maintaining $P(t) \in \mathbb{R}^{N \times N}$ and updating it at each step, at cost $O(N^2)$. For large reservoirs, this is computationally prohibitive. Are there approximations to $P(t)$ (e.g., diagonal, low-rank) that maintain convergence while reducing computational cost?

4. **FORCE in continuous time**: The continuous-time version of FORCE (standard in neuroscience applications) has a different analysis from the discrete-time version. The continuous-time analysis is essentially open.

**What a resolution would require.**
- A convergence proof for discrete-time FORCE under $\phi$-mixing reservoir states, using the mixing ergodic theorem (Chapter 29) and RLS convergence theory.
- Lower bounds on convergence rates under the same mixing conditions.
- Practical approximations to $P(t)$ with convergence guarantees.
