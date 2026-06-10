# 15.3.1 When NVAR Beats ESN and When It Doesn't

## The Core Tradeoff

The comparison between NVAR and ESN is a special case of the polynomial-vs.-smooth-kernel tradeoff in supervised learning. Neither dominates the other in general. The choice depends on:

1. **Dimension of the target system**: low-dimensional systems favor NVAR; high-dimensional favor ESN.
2. **Memory depth required**: short-memory tasks favor NVAR; long-memory tasks favor ESN.
3. **Nature of the dynamics**: polynomial dynamics favor NVAR; smooth but non-polynomial dynamics favor ESN.

## Case 1: Low-Dimensional Chaos — NVAR Wins

The Lorenz system has intrinsic dimension ~2.05 and quadratic dynamics. At short integration steps ($\Delta t = 0.025$), the one-step map is very close to a degree-2 polynomial of the current state. NVAR with $k=1$, $d=2$ captures this almost exactly.

More generally, any system governed by polynomial differential equations of degree $p$ discretized at sufficiently small $\Delta t$ will have one-step maps well-approximated by polynomials of degree $p$ in the current state. NVAR exactly represents such maps, while ESN approximates them through a basis of reservoir states — less efficiently if the reservoir's implicit kernel is not matched to the polynomial structure.

**Quantitative example.** Gauthier et al. [Gauthier2021] compare NVAR and ESN on the Lorenz system:
- NVAR ($d=2$, $k=1$, $D=27$): VPT $\approx 5.0$ Lyapunov times
- ESN ($N=500$, optimally tuned): VPT $\approx 4.9$–$5.5$ Lyapunov times (varies with seed)
- ESN ($N=27$, same feature count as NVAR): VPT $\approx 2.0$–$3.0$ Lyapunov times

The NVAR achieves ESN-with-500-neurons performance with the computational cost of ESN-with-27-neurons. The advantage is structural: polynomial features are the right basis for this task.

## Case 2: High-Dimensional Systems — ESN Wins

Consider the Kuramoto-Sivashinsky (KS) equation:

$$\partial_t u + u\partial_x u + \partial_{xx} u + \partial_{xxxx} u = 0$$

discretized on a periodic domain $[0, L]$ with $L = 22$ (the canonical "turbulent" regime) into $n = 64$ spatial grid points. The observable $\mathbf{u}_t \in \mathbb{R}^{64}$ has 64 dimensions and the dynamics are approximately 10-dimensional [Pathak2018]. The attractor has many interacting spatial modes.

**NVAR feature count.** With $k=1$, $d=2$, $n=64$:

$$D = 128 + \binom{128+1}{2} = 128 + 8256 = 8384 \text{ features}$$

This is tractable. With $k=2$:

$$D = 192 + \binom{193}{2} = 192 + 18528 = 18720 \text{ features}$$

And with $d=3$, $k=1$:

$$D = 128 + 8256 + \binom{128+2}{3} \approx 128 + 8256 + 364736 = 373120 \text{ features}$$

The feature count explodes. More critically, **the optimal Volterra kernels for the KS equation are not low-degree polynomials** — the spatial mixing and nonlinear coupling produce effective long-range correlations that cannot be captured by degree-2 or degree-3 polynomials in a two-step window.

The ESN, by contrast, can maintain a rich internal representation of the current state of the 64-dimensional system through its $N \gg 64$ neurons, using its fading memory to integrate the relevant history over many time steps. Pathak et al. [Pathak2018] achieve valid prediction times of 8 Lyapunov times on the KS equation with $N = 2400$ reservoir neurons — a result that NVAR with any tractable feature count cannot match.

## Case 3: Long Memory Tasks — ESN Wins

NVAR requires a finite history window $k_{max}$. Tasks requiring memory of more than $k_{max}$ steps cannot be solved by NVAR without increasing $k_{max}$ (and hence the feature count). For tasks like NARMA-30 (which requires approximately 30 steps of memory) with $n=1$, $k_{max}=30$, $d=2$:

$$D = 31 + \binom{32}{2} = 31 + 496 = 527 \text{ features}$$

This is manageable, but requires knowing in advance that 30 steps of memory are needed. If the memory requirement is unknown (as in many real-world tasks), NVAR requires a hyperparameter search over $k_{max}$ that can be expensive.

The ESN integrates memory continuously through its leaky state, automatically adapting to tasks with varying memory depths through the choice of $\alpha$ and $\rho_{target}$ — typically with far fewer hyperparameter sweeps.

## Case 4: Online Learning — NVAR Has Advantages

NVAR has no warmup period and no transient state. At test time, prediction begins at step $k_{max}$ — the moment enough history is available. This is an advantage for tasks with short sequences, tasks where each sequence is treated independently (no carry-over of state), and online settings where the model must be reinitialized frequently.

ESNs require a warmup period (typically 100–1000 steps) to allow the initial transient to decay before meaningful predictions can be made. For short sequences (length $< 500$), this warmup cost can be significant.

## Decision Rule: Choosing NVAR vs. ESN

As a practical guide:

**Choose NVAR when:**
- Input dimension $n \leq 10$ and memory requirement $k_{max} \leq 10$ (feature count is tractable)
- The target dynamics are governed by polynomial ODEs with small degree
- The integration step $\Delta t$ is small relative to the system's fastest timescale
- Sequences are short and warmup is prohibitive

**Choose ESN when:**
- Input dimension $n > 20$ or memory requirement $> 30$ steps
- The dynamics are smooth but not polynomial
- The memory structure of the task is unknown and must be learned
- You need calibrated uncertainty estimates (which require the ESN kernel structure)

**Consider hybrid approaches when:**
- The target has both short-memory polynomial components and long-memory smooth components
- You want both efficient feature extraction (NVAR) and long-range integration (ESN)

---

## References

- [Gauthier2021] Gauthier, D.J., Bollt, E., Griffith, A., & Barbosa, W.A.S. (2021). Next generation reservoir computing. *Nature Communications*, 12, 5564.
- [Pathak2018] Pathak, J., Hunt, B., Girvan, M., Lu, Z., & Ott, E. (2018). Model-free prediction of large spatiotemporally chaotic systems from data: A reservoir computing approach. *Physical Review Letters*, 120(2), 024102.
- [Bollt2021] Bollt, E. (2021). On explaining the surprising success of reservoir computing forecaster of chaos? The universal machine learning dynamical system with contractive maps. *Chaos*, 31(1), 013108.
