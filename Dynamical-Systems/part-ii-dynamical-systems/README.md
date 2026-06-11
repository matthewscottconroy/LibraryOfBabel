# Part II — Dynamical Systems

> *"The flow is a group homomorphism from time into diffeomorphisms. Dynamical systems theory asks: what does this group look like, asymptotically?"*

---

## Overview

Part II is the first of the two main pillars of the curriculum. Its subject is the mathematical theory of how states change over time: continuously (flows) or in discrete steps (maps), deterministically (ODEs) or probabilistically (random dynamical systems, covered in Chapter 15), in finite dimensions (Chapters 6–14) or infinite dimensions (Chapter 15).

The organizing question of Part II is: **what can we know about the long-run behavior of a dynamical system, and how do we measure it?** This question is answered at three levels of structure, corresponding to the three main branches of the subject:

1. **Topological dynamics** (Chapter 6): uses only the topology of the phase space — no metric, no measure. The key concepts are orbits, $\omega$-limit sets, minimality, transitivity, and topological conjugacy. Results at this level apply to all continuous maps on compact metric spaces.

2. **Measurable/ergodic theory** (Chapters 7–8): equips the phase space with an invariant measure. The key result — the Birkhoff ergodic theorem — says that time averages converge to space averages. This is the precise mathematical statement underlying the use of statistical mechanics to describe deterministic systems.

3. **Differentiable/geometric dynamics** (Chapters 9–14): uses the smooth structure of the phase space. The key concepts are hyperbolicity, Lyapunov exponents, stable and unstable manifolds, and bifurcations. Results at this level give the richest, most detailed picture of orbit structure but require stronger assumptions on the system.

The progression through these three levels mirrors the historical development of the subject: Poincaré's qualitative theory (topology), Birkhoff's ergodic theorem (measure), Smale's hyperbolic theory (geometry). It also mirrors the principle of *using exactly as much structure as you have*: if you only know the system is continuous, you can prove topological theorems; if you know it preserves a measure, you get ergodic theorems; if you know it is smooth and hyperbolic, you get the richest results.

---

## Prerequisites

The reader should have completed Part I, with emphasis on: metric spaces and compactness (Chapter 1), measure theory and $L^p$ spaces (Chapter 2), smooth manifolds and differential forms (Chapter 3), ODE flows and the stable manifold theorem (Chapter 4), and spectral theory (Chapter 5). For the later chapters (12–15), some comfort with complex analysis and functional analysis is helpful.

**What you gain from this part:**
- A complete working knowledge of topological dynamics: orbits, recurrence, conjugacy, transitivity, and minimality.
- The ergodic theorem and its applications: time averages, Poincaré recurrence, mixing, and the Koopman operator.
- The theory of Lyapunov stability and Lyapunov exponents — the key to understanding chaos quantitatively.
- A thorough grounding in hyperbolic dynamics: horseshoes, Anosov systems, Markov partitions, SRB measures.
- The theory of bifurcations: how complexity is born as parameters vary.
- Precise definitions of chaos and its measurement: sensitive dependence, strange attractors, fractal dimensions.
- Symbolic dynamics as the bridge between continuous systems and combinatorics/information theory.
- Complex dynamics as one of the most visually and mathematically rich areas of the field.
- Hamiltonian and KAM theory: the deep structure of conservative systems.
- Infinite-dimensional and stochastic dynamics: dynamics of PDEs and systems with noise.

---

## Chapter Descriptions

### Chapter 6 — Topological Dynamics

Topological dynamics is the study of continuous maps $f: X \to X$ on compact metric spaces, focusing on qualitative properties that depend only on the topology. The right notion of equivalence is *topological conjugacy*: two systems $(X, f)$ and $(Y, g)$ are conjugate if there is a homeomorphism $h: X \to Y$ with $h \circ f = g \circ h$ — conjugate systems are "the same" for purposes of topological dynamics.

The chapter introduces the key concepts: the orbit $\mathcal{O}(x) = \{x, f(x), f^2(x), \ldots\}$; the $\omega$-limit set $\omega(x) = \bigcap_{n \geq 0} \overline{\{f^k(x) : k \geq n\}}$ (the set of accumulation points of the orbit); recurrence (return to neighborhoods); minimality (every orbit is dense); and topological transitivity (existence of a dense orbit). A fundamental result is that $\omega$-limit sets are closed, invariant, and nonempty for any bounded orbit, providing the correct framework for discussing "what a typical orbit does eventually."

The Devaney definition of chaos synthesizes these concepts: a system is *Devaney chaotic* if it is (1) topologically transitive, (2) has dense periodic orbits, and (3) has sensitive dependence on initial conditions. The remarkable theorem — that conditions (1) and (2) together imply (3) — shows that sensitive dependence is not an independent property but a *consequence* of the topological interplay between density of periodics and transitivity.

### Chapter 7 — Ergodic Theory

Ergodic theory is the measure-theoretic study of dynamical systems — the branch concerned with the statistical properties of orbits over long times. Its central theorem, proved by Birkhoff in 1931, is the ergodic theorem: for a measure-preserving transformation $T$ and integrable function $f$,
$$\frac{1}{N}\sum_{n=0}^{N-1} f(T^n x) \to \bar{f}(x) \quad \text{a.e. and in } L^1.$$
The limit $\bar{f}$ is the *time average* of $f$ along the orbit of $x$. When the system is *ergodic* (meaning $T$-invariant sets have measure 0 or 1), $\bar{f}$ is the constant $\int f \, d\mu$ — the space average. The physical interpretation is profound: for ergodic systems, the time average of any observable equals its space average, regardless of the starting point (a.e.). This is the mathematical foundation of statistical mechanics.

The chapter develops the full hierarchy of statistical behavior: ergodicity (time averages converge to space averages), weak mixing (correlations $\langle f \cdot g \circ T^n \rangle \to \langle f \rangle \langle g \rangle$ on average), strong mixing (correlations decay pointwise), and Bernoullicity (the strongest randomness, equivalent to being isomorphic to an i.i.d. process). The Koopman operator $U_T f = f \circ T$, a unitary operator on $L^2$, gives a spectral-theoretic approach to these concepts: ergodicity is equivalent to $1$ being a simple eigenvalue; weak mixing to absence of eigenvalues other than $1$; and so on. Ornstein's theorem (the entropy of a Bernoulli shift classifies it up to isomorphism) closes the theory elegantly.

### Chapter 8 — Stability Theory and Lyapunov Methods

Stability theory asks: when do small perturbations remain small? The Lyapunov function approach — finding an energy-like function that decreases along orbits — provides a powerful and flexible tool for establishing stability without solving the equations explicitly. The chapter develops the full theory: Lyapunov stability, asymptotic stability, the LaSalle invariance principle, and instability (Chetaev's theorem). The fundamental result that connects linearization to nonlinear behavior — the Hartman-Grobman theorem for the local picture near a hyperbolic fixed point — has already appeared in Chapter 4; here the focus is on the global and quantitative aspects.

Lyapunov exponents are the key to understanding chaos quantitatively. The largest Lyapunov exponent $\lambda_{\max}$ measures the average rate at which nearby orbits diverge: two orbits that start at distance $\epsilon$ will be at distance $\approx \epsilon e^{\lambda_{\max} t}$ after time $t$, for $t$ in the "Lyapunov regime." The Oseledec multiplicative ergodic theorem — the deepest result in this chapter — guarantees the existence of Lyapunov exponents almost everywhere for ergodic systems: for $\mu$-a.e. initial condition, the limits
$$\lambda_i = \lim_{t \to \infty} \frac{1}{t} \log \|Df^t(x) v_i\|$$
exist for a set of $n$ directions $v_i$ (the Oseledec splitting). Positive Lyapunov exponents are the quantitative signature of chaos.

### Chapter 9 — Hyperbolic Dynamics

Hyperbolic systems are the richest and best-understood class of chaotic systems. The theory, developed in the 1960s–70s by Smale, Anosov, Sinai, Ruelle, and Bowen, is now essentially complete — and studying it is both intrinsically beautiful and a necessary preparation for understanding the frontiers (partial hyperbolicity, non-uniformly hyperbolic systems) in Part VI.

The Smale horseshoe is the prototype: a simple stretch-and-fold operation on a square produces a dynamics on an invariant Cantor set that is topologically conjugate to the full two-sided shift on two symbols. This construction shows that complicated symbolic dynamics (all possible binary sequences as orbits) can arise from a simple geometric operation. Anosov diffeomorphisms generalize this to the whole manifold: the tangent bundle splits uniformly into expanding ($E^u$) and contracting ($E^s$) directions, giving the most regular form of hyperbolic behavior.

Markov partitions — finite coverings of the phase space by "rectangles" that respect the stable/unstable foliation — allow hyperbolic systems to be *coded* by symbolic sequences. This is the key bridge to symbolic dynamics (Chapter 12) and hence to information theory (Chapter 24). The topological entropy equals the logarithm of the Perron-Frobenius eigenvalue of the transition matrix. SRB measures (Sinai-Ruelle-Bowen) are the "physical" invariant measures — the ones seen by Lebesgue-typical initial conditions — and they satisfy Pesin's formula: entropy equals the sum of positive Lyapunov exponents.

### Chapter 10 — Bifurcation Theory

Bifurcation theory studies how the qualitative behavior of a dynamical system changes as parameters vary. A *bifurcation* is a value of the parameter at which the phase portrait changes qualitatively — where fixed points are created or destroyed, periodic orbits are born, or the stability of existing structures changes. This is the mathematics of phase transitions, and it is directly relevant to applications in biology, climate, and physics.

The local theory classifies bifurcations by their *normal forms* — the simplest polynomial vector fields to which the original can be reduced by smooth coordinate changes near the bifurcation. The saddle-node bifurcation ($\dot{x} = \mu - x^2$) is codimension 1: one parameter is enough to "unfold" the bifurcation, and the system is stable against small perturbations. The Hopf bifurcation — birth of a periodic orbit from an equilibrium — is the most important local bifurcation for flows, and the first Lyapunov coefficient $\ell_1$ determines whether the bifurcating periodic orbit is stable or unstable.

Feigenbaum's period-doubling cascade is one of the great achievements of modern dynamics: the ratio $\delta = 4.6692\ldots$ is *universal* across all one-parameter families of unimodal maps (families with a single quadratic maximum). This universality is explained by renormalization: the period-doubling cascade is governed by a fixed point of the renormalization operator on the space of unimodal maps, and the Feigenbaum constant is the unstable eigenvalue of the linearized renormalization operator at this fixed point.

### Chapter 11 — Chaos Theory

Chaos is deterministic complexity: systems with a perfectly defined future that is nonetheless impossible to predict beyond a finite horizon. Chapter 11 makes this precise and gives the tools to measure and characterize chaos quantitatively.

Three definitions of chaos are compared: Devaney's topological definition (transitivity + dense periodics), Li-Yorke's definition (existence of an uncountable scrambled set), and the positive-entropy definition. The Lorenz system — the archetype of chaotic dynamics, derived from a Galerkin truncation of the Navier-Stokes equations — is studied in detail, including Tucker's 2002 computer-assisted proof that the Lorenz attractor is a genuine robust chaotic attractor.

Fractal geometry provides the tool for measuring the "size" of strange attractors: Hausdorff dimension quantifies how much volume the attractor occupies in phase space, and for the Lorenz attractor the dimension is approximately 2.06, slightly above the plane. The Kaplan-Yorke conjecture relates this dimension to the Lyapunov exponents. Multifractal analysis goes further: the $f(\alpha)$ spectrum describes how the local dimension varies across the attractor, and is connected via a Legendre transform to the Rényi dimension spectrum — a bridge to information theory.

### Chapter 12 — Symbolic Dynamics

Symbolic dynamics is the study of shifts on spaces of sequences, and the coding of continuous systems by symbolic sequences. A *subshift* is a closed shift-invariant subset $\Sigma \subseteq \mathcal{A}^{\mathbb{Z}}$ of bi-infinite sequences over a finite alphabet $\mathcal{A}$. Subshifts of finite type (SFTs) are defined by a finite transition matrix and have the richest structure. Sofic shifts are factor maps of SFTs — exactly the hidden Markov sources of information theory.

The connection between symbolic dynamics and continuous dynamics runs through Markov partitions (Chapter 9): the coding map from a hyperbolic system to its symbolic model is a factor map from the system to a subshift. The theory of sliding block codes — maps between shifts defined by a local rule — is the symbolic analogue of morphisms, and the fundamental theorem of symbolic dynamics (every factor map between SFTs is the composition of an edge shift map and a sliding block code) is the starting point for the classification theory.

Topological entropy of a shift is computable from the word complexity function $p(n)$ (number of words of length $n$): $h_{\text{top}} = \lim (1/n) \log p(n) = \log \lambda_{\text{PF}}(A)$ for SFTs. The zeta function $\zeta(z) = \exp(\sum_{n \geq 1} |{\rm Fix}(f^n)| z^n / n)$ counts periodic orbits and is rational for SFTs — a dynamical analogue of the Weil conjectures.

### Chapter 13 — Complex Dynamics

The iteration of holomorphic maps $f: \hat{\mathbb{C}} \to \hat{\mathbb{C}}$ on the Riemann sphere is one of the most visually and mathematically rich areas of dynamics. The Julia set $J(f)$ — the closure of repelling periodic points, also the boundary between regions of regular and chaotic behavior — has a fractal structure that varies dramatically with the parameter. The Fatou set $F(f) = \hat{\mathbb{C}} \setminus J(f)$ is the open set where the dynamics is equicontinuous.

For quadratic maps $f_c(z) = z^2 + c$, the Mandelbrot set $\mathcal{M} = \{c \in \mathbb{C} : f_c^n(0) \not\to \infty\}$ is the parameter space of connected Julia sets — it classifies the dynamical behavior by the orbit of the critical point $0$. The MLC conjecture — that $\mathcal{M}$ is locally connected — is one of the most important open problems in dynamics; a positive answer would give a complete combinatorial description of the dynamics of all quadratic maps.

Sullivan's No Wandering Domains theorem (1985), proved using quasiconformal deformation theory, classifies the connected components of the Fatou set: each is periodic or preperiodic, and the periodic components are either attracting, parabolic, Siegel disks, or Herman rings. This theorem, together with the Fatou-Sullivan classification, gives a complete qualitative picture of the dynamics of rational maps.

### Chapter 14 — Hamiltonian Systems and KAM Theory

Hamiltonian mechanics studies systems that preserve a symplectic form $\omega$ on phase space. Hamilton's equations $\dot{q}_i = \partial H/\partial p_i$, $\dot{p}_i = -\partial H/\partial q_i$ generate a flow that preserves $\omega$ and hence phase space volume (Liouville's theorem). *Integrable* systems — those with $n$ commuting integrals of motion $F_1 = H, F_2, \ldots, F_n$ — have phase space foliated by invariant $n$-tori, as guaranteed by the Liouville-Arnold theorem. On each torus, the motion is a linear flow with constant frequencies.

The KAM theorem (Kolmogorov-Arnold-Moser, 1950s–60s) is one of the great theorems of 20th century mathematics: for an *integrable* Hamiltonian system with $n$ degrees of freedom, *most* invariant tori persist under small perturbations. The surviving tori are those with *Diophantine* frequencies — frequencies that are not well-approximated by rationals, in a quantitative sense. The tori with resonant (rational) frequencies are destroyed by perturbation, and the behavior near them is extremely complex (Arnold diffusion). The KAM theorem explains the long-term stability of the solar system and is a prototype for the mathematics of stability in the presence of resonances.

### Chapter 15 — Infinite-Dimensional and Random Dynamics

Chapter 15 extends the theory in two directions. The first is infinite-dimensional dynamics: PDEs — the Navier-Stokes equations, reaction-diffusion equations, the complex Ginzburg-Landau equation — can be viewed as dynamical systems on infinite-dimensional function spaces. The key results are: the global attractor (finite-dimensional compact invariant set that attracts all bounded sets), the inertial manifold (finite-dimensional positively invariant manifold that contains the global attractor and to which all trajectories converge exponentially), and the connection between the dimension of the global attractor and the large-scale structure of the solution.

The second direction is random (stochastic) dynamics: systems driven by noise, described by stochastic differential equations $dX = f(X) \, dt + \sigma \, dW$. The Oseledec multiplicative ergodic theorem extends to random systems, giving random Lyapunov exponents. Random attractors (pullback attractors) replace the deterministic attractor. Stochastic bifurcations describe qualitative changes in the stationary distribution of the system as parameters vary. The interplay between determinism and noise — when does noise "stabilize" a chaotic system, and when does it destabilize a stable one? — is a rich and active area.

---

## Key Mathematical Concepts

### Topological Conjugacy

Two dynamical systems $(X, f)$ and $(Y, g)$ are *topologically conjugate* if there exists a homeomorphism $h: X \to Y$ with $h \circ f = g \circ h$. Conjugacy is the correct notion of "the same dynamical system" for topological dynamics: conjugate systems have the same orbit structure, the same periodic points, the same topological entropy, and the same $\omega$-limit sets (up to the homeomorphism $h$). The classification problem — determining when two given systems are conjugate — is one of the central problems of the field.

### Lyapunov Exponents

For a differentiable map $f: M \to M$ with an ergodic invariant measure $\mu$, the *Lyapunov exponents* are the limits
$$\lambda_i(x) = \lim_{n \to \infty} \frac{1}{n} \log \|Df^n(x) e_i(x)\|$$
where $e_1(x), \ldots, e_n(x)$ is the Oseledec basis at $x$. By the Oseledec theorem, these limits exist $\mu$-a.e. and are $\mu$-a.e. constant for ergodic $\mu$. Positive Lyapunov exponents mean nearby orbits diverge exponentially — the signature of chaos. Pesin's formula connects Lyapunov exponents to entropy: $h_\mu(f) = \sum_{\lambda_i > 0} \lambda_i$ for SRB measures.

### Hyperbolic Sets and Structural Stability

A compact invariant set $\Lambda$ is *hyperbolic* if the tangent bundle over $\Lambda$ splits as $T_\Lambda M = E^s \oplus E^u$ with uniform contraction in $E^s$ and expansion in $E^u$. Hyperbolic sets are structurally stable: any $C^1$-small perturbation of the diffeomorphism has a hyperbolic set that is topologically conjugate to $\Lambda$. The theory of hyperbolic sets — Markov partitions, SRB measures, shadowing — is the "ideal" case of the general theory.

### Entropy

The *topological entropy* $h_{\text{top}}(f)$ measures the complexity of the orbit structure: $h_{\text{top}} = \lim_{\epsilon \to 0} \lim_{n \to \infty} (1/n) \log s(n, \epsilon)$ where $s(n, \epsilon)$ is the maximum number of orbit segments of length $n$ that are mutually $\epsilon$-separated. The *Kolmogorov-Sinai (metric) entropy* $h_\mu(f)$ is the measure-theoretic analogue, defined via partitions. The *variational principle* — $h_{\text{top}}(f) = \sup_\mu h_\mu(f)$ — connects the two.

---

## Key Theorems

1. **Birkhoff Ergodic Theorem.** For an ergodic measure-preserving transformation $(X, \mathcal{B}, \mu, T)$ and $f \in L^1(\mu)$: $\frac{1}{N}\sum_{n=0}^{N-1} f(T^n x) \to \int f \, d\mu$ for $\mu$-a.e. $x$.

2. **Oseledec Multiplicative Ergodic Theorem.** For a $C^1$ diffeomorphism $f: M \to M$ with ergodic invariant measure $\mu$ and $\log\|Df\| \in L^1(\mu)$: Lyapunov exponents $\lambda_1 \geq \cdots \geq \lambda_n$ exist $\mu$-a.e., and the tangent space has a $Df$-invariant splitting into Oseledec subspaces.

3. **Smale's Spectral Decomposition.** For an Axiom A diffeomorphism: the nonwandering set decomposes as $\Omega(f) = \Lambda_1 \cup \cdots \cup \Lambda_k$, each $\Lambda_i$ being a closed, $f$-invariant, topologically transitive basic set. There is no cycle in the partial order among basic sets.

4. **Shadowing Lemma.** For a hyperbolic set $\Lambda$: every $\delta$-pseudo-orbit in $\Lambda$ is $\varepsilon$-shadowed by a true orbit, for appropriate $\delta = \delta(\varepsilon)$. This is the rigorous foundation for the numerical simulation of hyperbolic systems.

5. **Hopf Bifurcation Theorem.** For a family $f_\mu$ of vector fields with purely imaginary eigenvalues $\pm i\omega$ at $\mu = \mu_0$, satisfying appropriate non-degeneracy conditions: a unique branch of periodic orbits bifurcates from the equilibrium at $\mu_0$. If the first Lyapunov coefficient $\ell_1 < 0$, the periodic orbits are stable (supercritical Hopf); if $\ell_1 > 0$, they are unstable (subcritical Hopf).

6. **Sullivan's No Wandering Domains.** For a rational map $f: \hat{\mathbb{C}} \to \hat{\mathbb{C}}$, every Fatou component is eventually periodic. (The proof uses the measurable Riemann mapping theorem and quasiconformal deformations.)

7. **KAM Theorem.** For a nearly-integrable Hamiltonian system $H = H_0(I) + \varepsilon H_1(I, \theta)$ with $H_0$ non-degenerate (the frequency map $I \mapsto \partial H_0/\partial I$ is a diffeomorphism), there exists $\varepsilon_0 > 0$ such that for $|\varepsilon| < \varepsilon_0$, a positive-measure set of invariant tori from the unperturbed system persists.

---

## Connections to Other Parts

Part II is both self-contained and a launching point for the entire book:

- **Part III (Information Theory)** finds its measure-theoretic entropy directly generalized from the Kolmogorov-Sinai entropy of Chapter 7. The AEP (Asymptotic Equipartition Property) is the ergodic theorem applied to log-probabilities. Every stationary stochastic process is a measure-preserving transformation, and its entropy rate is its KS entropy.

- **Part IV (Bridges)** makes the connections between dynamical entropy, symbolic dynamics, and information theory fully explicit. The Markov partitions of Chapter 9 are exactly what is needed to code a hyperbolic system as a hidden Markov source.

- **Part V (Foundations of CS and Mathematics)** uses symbolic dynamics (Chapter 12) as a bridge: the classification of subshifts of finite type by their dimension groups connects to $K$-theory; the undecidability of the emptiness problem for 2D SFTs is a fundamental computability-theory result.

- **Part VI (Frontiers)** builds directly on Chapters 7 and 9: orbit equivalence theory (Chapter 33) extends the classification of measure-preserving systems; sofic entropy (Chapter 34) generalizes the KS entropy to actions of non-amenable groups.
