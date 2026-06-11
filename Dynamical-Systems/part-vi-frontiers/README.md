# Part VI — Research Frontiers and Open Problems

> *"These are the spires of the cathedral. You do not reach the spires by jumping — you climb the scaffolding built in Parts I–V. But if you have built that scaffolding, you can stand here."*

---

## Overview

Part VI is where the textbook ends and research begins. The eight chapters of this part correspond to eight active frontiers in the intersection of dynamical systems, ergodic theory, and information theory — areas where significant progress has been made in the last two decades, where major questions remain open, and where a well-prepared reader can begin to make original contributions.

The character of these chapters is different from Parts I–V. Rather than developing a theory from definitions through exercises, the frontier chapters survey the state of the art, identify the key open problems, sketch the major proof techniques, and point toward current research. The goal is not to teach you everything in these areas — that would require several additional books — but to situate you in the landscape: to help you understand what has been done, what remains to be done, and what tools are available.

The frontiers covered are:

- **Orbit Equivalence and Measured Group Theory** (Chapter 33): When are two group actions on measure spaces orbit equivalent? Dye's theorem (amenable case) vs. Popa's superrigidity (non-amenable case).

- **Sofic Entropy and Non-Amenable Group Actions** (Chapter 34): How do you define entropy for actions of groups that are not amenable? Bowen's sofic entropy and the open problem of soficity.

- **The Isomorphism Problem** (Chapter 35): The Foreman-Rudolph-Weiss theorem says the isomorphism of ergodic systems is not Borel — but what exactly is its complexity?

- **The Zimmer Program** (Chapter 36): What are the actions of higher-rank lattices (like $SL(n,\mathbb{Z})$ for $n \geq 3$) on compact manifolds? The Brown-Fisher-Hurtado theorem resolves Zimmer's conjecture for cocompact lattices.

- **Complex Dynamics Frontiers** (Chapter 37): The MLC conjecture (local connectivity of the Mandelbrot set) and its connections to combinatorics, renormalization, and the classification of quadratic polynomials.

- **Quantum Information Complexity** (Chapter 38): The quantum analogues of communication complexity and information complexity, and the open problem of quantum direct sum theorems.

- **One-Shot and Finite-Length Information Theory** (Chapter 39): The non-asymptotic theory: what is achievable at finite block length, and how fast do you approach the asymptotic limits?

- **Circuit Complexity and Information Theory** (Chapter 40): Information-theoretic lower bounds for circuit complexity, and the barriers (natural proofs) to resolving P vs. NP by these methods.

---

## Prerequisites

The reader should have completed Parts I–V. The specific prerequisites for each chapter are given at the start of that chapter. In general:

- Chapters 33–35 require mastery of ergodic theory (Chapter 7), orbit equivalence relations, and some familiarity with operator algebras (von Neumann algebras).
- Chapter 36 requires Lie groups, representation theory, and hyperbolic dynamics.
- Chapter 37 requires complex dynamics (Chapter 13) and quasiconformal maps.
- Chapters 38–40 require classical and quantum information theory (Chapters 16–21).

---

## Chapter Descriptions

### Chapter 33 — Orbit Equivalence Theory

Two free ergodic measure-preserving actions $\Gamma \curvearrowright (X, \mu)$ and $\Lambda \curvearrowright (Y, \nu)$ are *orbit equivalent (OE)* if there is a measure-space isomorphism $\phi: X \to Y$ sending $\Gamma$-orbits to $\Lambda$-orbits. Orbit equivalence is coarser than isomorphism (it forgets the group structure) but finer than measurable isomorphism (it remembers the orbits).

**The amenable case** (Dye-Ornstein-Weiss): All free ergodic actions of all countably infinite amenable groups are orbit equivalent to each other. The orbit equivalence class is the unique hyperfinite equivalence relation. This means that for amenable groups, the orbit structure carries no information about the group — the orbit structure is completely determined by the underlying probability space.

**The non-amenable case**: Non-amenable groups can have many non-orbit-equivalent actions. Furman (1999) showed that for $\Gamma = SL(n, \mathbb{Z})$, any group orbit equivalent to $\Gamma$ is virtually isomorphic to $\Gamma$. Gaboriau (2000) introduced $\ell^2$-Betti numbers as orbit-equivalence invariants: $\beta_n^{(2)}(\mathcal{R}_\Gamma) = \beta_n^{(2)}(\Gamma)$, which distinguishes the free groups $F_r$ and $F_s$ for $r \neq s$.

**Popa's deformation-rigidity theory**: Popa's cocycle superrigidity theorems (2005–2006) show that for groups with property (T) (or more generally, for "malleable" actions of "w-rigid" groups), OE implies isomorphism — a phenomenon called *OE-superrigidity*. This connects orbit equivalence theory to the classification of II$_1$ factors (von Neumann algebras).

### Chapter 34 — Sofic Entropy

Kolmogorov-Sinai entropy is defined for actions of $\mathbb{Z}$ (and amenable groups, by the Ornstein-Weiss theory). But how do you define entropy for actions of non-amenable groups — groups like free groups, $SL(n, \mathbb{Z})$, or Thompson's group $F$?

**Bowen's sofic entropy** (2010) answers this for *sofic* groups. A group $\Gamma$ is *sofic* if it can be approximated by finite symmetric groups: for every finite $F \subseteq \Gamma$ and $\varepsilon > 0$, there is a near-action of $\Gamma$ on a finite set that is $\varepsilon$-close to a genuine action on $F$. All amenable groups are sofic; free groups are sofic. The question "is every group sofic?" is arguably the most important open problem in geometric group theory.

Bowen defines the sofic entropy of a free pmp action $\Gamma \curvearrowright (X, \mu)$ with respect to a sofic approximation sequence $\sigma = (\sigma_i)$. When $\Gamma = \mathbb{Z}$ (which is amenable and sofic), sofic entropy recovers KS entropy. Bowen proved that Bernoulli shifts of sofic groups with different base measures of different entropy are non-isomorphic — extending Ornstein's theorem to the non-amenable setting.

**Open problems**: Is every group sofic? Does sofic entropy depend on the choice of sofic approximation sequence (the answer is no for Bernoulli shifts, but unknown in general)? Is there an analogue of the Ornstein theory for Bernoulli shifts over sofic groups?

### Chapter 35 — The Isomorphism Problem

The isomorphism of ergodic measure-preserving transformations is a natural equivalence relation on the Polish space of ergodic MPTs (with a suitable topology). The question "what is the complexity of this equivalence relation in the Borel reducibility hierarchy?" has a striking answer.

**Foreman-Rudolph-Weiss theorem** (2011): The isomorphism relation for ergodic MPTs is *not Borel*. More precisely, it is a $\Sigma_1^1$ (analytic) equivalence relation that is not Borel. This is in stark contrast to the situation for smooth conjugacy of smooth diffeomorphisms (which, for hyperbolic systems, is described by a finite-dimensional invariant) or for Bernoulli shifts (classified by entropy, a single real number).

The non-Borelaness of the isomorphism relation means: there is no countable sequence of Borel invariants that classifies all ergodic MPTs up to isomorphism. No finite or countable collection of invariants (topological entropy, spectrum, mixing properties, Bernoulli-ness, ...) can distinguish all non-isomorphic ergodic systems.

**The exact complexity**: The isomorphism relation is complete $\Sigma_1^1$ (analytic-complete) in the Borel reducibility hierarchy. This was proved by Foreman-Weiss (2019), completing the picture.

### Chapter 36 — The Zimmer Program

The Zimmer program (1980s) asks: what are the actions of higher-rank lattices $\Gamma \leq G$ (where $G$ is a simple Lie group of real rank $\geq 2$) on compact manifolds? Zimmer's conjecture, roughly, says that all such actions are essentially algebraic — coming from the Lie group structure.

**The conjecture**: For $\Gamma = SL(n, \mathbb{Z})$ with $n \geq 3$ and a smooth action of $\Gamma$ on a compact manifold $M$ with $\dim M < n - 1$: the action must factor through a finite quotient of $\Gamma$. (Smooth actions in dimensions below the rank are trivial, up to finite index.)

**Brown-Fisher-Hurtado theorem** (2020): Zimmer's conjecture holds for cocompact lattices in $SL(n, \mathbb{R})$ for $n \geq 3$. The proof combines hyperbolic dynamics (Lyapunov exponents, non-uniform hyperbolicity), harmonic analysis (superrigidity for cocycles), and topological methods.

**Open cases**: The conjecture for non-cocompact lattices (like $SL(n, \mathbb{Z})$) in dimensions above the lower bound remains open. The case of other Lie groups (e.g., $Sp(2n, \mathbb{R})$, $SO(p,q)$) is partially resolved.

### Chapter 37 — Complex Dynamics Frontiers

The parameter space of quadratic maps $f_c(z) = z^2 + c$ — the Mandelbrot set $\mathcal{M}$ — is the central object of complex dynamics. Its structure encodes the entire classification theory of quadratic polynomials, and the main open problem is whether it is locally connected (MLC conjecture).

**Local connectivity of the Mandelbrot set**: $\mathcal{M}$ is *locally connected* at $c$ if every neighborhood of $c$ in $\mathcal{M}$ contains a connected neighborhood. A positive answer to MLC would imply a complete *combinatorial* description of the Mandelbrot set: every parameter $c \in \mathcal{M}$ would be described by a combinatorial kneading sequence, and the topology of $\mathcal{M}$ would be determined by this combinatorics.

**What is known**: MLC holds for: all $c$ with $f_c$ having a hyperbolic periodic orbit (Douady-Hubbard); all finitely renormalizable $c$ (Yoccoz, for those with no parabolic cycle); Siegel and Cremer parameters under some conditions. The general case is open.

**Renormalization**: A quadratic polynomial $f_c$ is *renormalizable* if there is a period $n \geq 2$ such that $f_c^n$ restricted to a small neighborhood of 0 is polynomial-like (in the sense of Douady-Hubbard). Renormalization is the operation of "zooming in" on the small polynomial-like map. Infinitely renormalizable maps are the hardest to analyze; MLC for them is the key open case.

### Chapter 38 — Quantum Information Complexity

Classical communication complexity studies the minimum number of bits two parties must exchange to compute a joint function of their inputs. *Quantum communication complexity* allows the parties to exchange qubits (and possibly share entanglement). *Quantum information complexity* is the quantum analogue of classical information complexity.

**Quantum advantages in communication**: For some problems, quantum communication is exponentially more efficient than classical communication. The equality function (is $x = y$?) requires $\Omega(n)$ classical bits but $O(\log n)$ qubits using quantum fingerprinting (Buhrman-Cleve-Watrous-de Wolf). This separation uses quantum superposition and the quantum Schwartz-Zippel lemma.

**Quantum information complexity**: The *quantum information cost* of a protocol $\Pi$ computing $f$ is $\text{QIC}(\Pi) = I(XY; M)$ where $M$ is the quantum message register and $I$ is quantum mutual information. This is a lower bound on the quantum communication complexity. The direct sum theorem for quantum protocols is open: does $\text{QCC}(f^n) = n \cdot \text{QCC}(f)$ hold for quantum communication complexity?

**Open problems**: (1) Is there a quantum direct sum theorem? (2) What is the quantum information complexity of the equality function? (3) Can quantum communication complexity exponentially separate classical randomized communication complexity for all relation problems?

### Chapter 39 — One-Shot and Finite-Length Information Theory

Classical information theory gives asymptotic limits: as block length $n \to \infty$, rates approach capacity. But in practice, $n$ is finite. The one-shot and finite-length theory asks: what is achievable with $n$ channel uses, and how close to capacity can you get?

**Second-order coding rates**: The *second-order* analysis of the channel coding problem (Polyanskiy-Poor-Verdú 2010) gives the $\sqrt{n}$ correction to capacity: the maximum rate achievable at block length $n$ with error probability $\varepsilon$ is approximately
$$R_n(\varepsilon) \approx C - \sqrt{V/n} \cdot Q^{-1}(\varepsilon)$$
where $V = \text{Var}[\log(p(Y|X)/p(Y))]$ is the *channel dispersion* and $Q$ is the tail of the normal distribution. This gives a concrete, tight approximation for practical block lengths.

**One-shot information theory**: For a *single* use of a source or channel (no block coding), the relevant quantity is the *smooth min-entropy* $H_{\min}^\varepsilon$. The one-shot source coding rate is $H_{\min}^\varepsilon(X)$ bits; the one-shot channel capacity is the smooth min-entropy of the channel output. Smooth entropies recover Shannon entropy in the i.i.d. limit via the quantum information-spectrum method.

**Open problems**: Tight achievability and converse bounds for non-i.i.d. channels and sources at finite block length. The relationship between smooth entropies and communication complexity (one-shot capacity as communication complexity). One-shot quantum error correction: what is the exact minimum block length needed for reliable quantum computation?

### Chapter 40 — Circuit Complexity and Information Theory

The question "can information-theoretic lower bounds prove that $P \neq NP$?" motivates the study of circuit complexity using information theory. Several powerful techniques have been developed, but fundamental barriers prevent them from resolving the main conjecture.

**Natural proofs barrier** (Razborov-Rudich 1994): A lower bound proof is *natural* if it uses a *constructive* property (the distinguishing algorithm runs in polynomial time) and is *large* (the property holds for a constant fraction of functions). The natural proofs theorem says: if strong pseudorandom generators exist, then natural proofs cannot prove super-polynomial lower bounds against $P/\text{poly}$. Most known information-theoretic lower bounds (entropy arguments, counting arguments) are natural, so they cannot resolve P vs. NP.

**Communication complexity lower bounds**: Many circuit lower bounds can be proved via communication complexity, using information complexity as the key tool. The direct sum theorem for information complexity implies circuit lower bounds for composed functions. These techniques successfully prove exponential lower bounds for restricted circuit models (monotone circuits, $AC^0$, $TC^0$ with restricted resources) and for relations (multiparty communication), and they avoid the natural proofs barrier because they are not constructive in the required sense.

**Current state**: Shannon's counting argument shows that most $n$-bit Boolean functions require circuits of size $\Omega(2^n/n)$, but the best known lower bound for an explicit function in NP is $5n - o(n)$ gates (a result by Iwama and Morizumi). The gap between this lower bound and the circuit lower bounds needed for $P \neq NP$ ($n^{\omega(1)}$ vs. $n^k$ for all $k$) remains one of the great open problems in theoretical computer science.

---

## Key Open Problems

Here are the eight open problems at the frontier, with their current status:

1. **Orbit Equivalence Rigidity (Chapter 33)**: Is the cost of a free ergodic action of $\Gamma$ independent of the action? (Fixed price problem.) Open for $\Gamma = \mathbb{Z}^2$.

2. **Sofic Groups (Chapter 34)**: Is every group sofic? Unknown. No non-sofic group has been found; all classes of groups for which soficity can be verified (amenable, free, residually finite, ...) are sofic.

3. **Classification of Ergodic MPTs (Chapter 35)**: What is the exact complexity of the isomorphism relation in the Borel reducibility hierarchy? Foreman-Weiss proved it is $\Sigma_1^1$-complete; the position relative to other natural equivalence relations is still being mapped out.

4. **Zimmer's Conjecture (Chapter 36)**: Prove (or disprove) Zimmer's conjecture for non-cocompact lattices and for lattices in other Lie groups (e.g., $Sp(2n,\mathbb{R})$).

5. **MLC Conjecture (Chapter 37)**: Is the Mandelbrot set locally connected? This would imply a complete combinatorial model for the quadratic family.

6. **Quantum Direct Sum (Chapter 38)**: Is there a quantum direct sum theorem for quantum communication complexity?

7. **One-Shot Quantum Coding (Chapter 39)**: What are the tight one-shot achievability and converse bounds for quantum channel capacity, including the second-order term?

8. **Explicit Circuit Lower Bounds (Chapter 40)**: Find an explicit Boolean function in NP that requires circuits of size $n^{100}$ (or even $n^3$). This is the main open problem in circuit complexity.

---

## Connections to Other Parts

Part VI is the culmination of all prior parts. The connections flow upward:

- **From Part II:** Orbit equivalence (Chapter 33) is a direct extension of ergodic theory (Chapter 7) and Ornstein's isomorphism theory. The Zimmer program (Chapter 36) uses hyperbolic dynamics (Chapter 9) and Lyapunov exponents (Chapter 8). Complex dynamics frontiers (Chapter 37) build on Chapter 13.

- **From Part III:** Quantum information complexity (Chapter 38) extends the information theory of Chapters 16–21. One-shot information theory (Chapter 39) extends the entropy theory of Chapters 16–17. Circuit complexity (Chapter 40) builds on the information-theoretic methods of Chapter 26.

- **From Part IV:** The variational principle and sofic entropy (Chapter 34) build on the entropy bridges of Chapter 22.

- **From Part V:** The descriptive set theory of Chapter 32 is the framework for the isomorphism problem (Chapter 35). The computability theory of Chapter 27 provides the complexity-theoretic context for Chapter 40.

- **To Part VII:** The Zimmer program connects to the HoTT and category theory of Chapter 43 via cohomological methods. Quantum information (Chapter 38) connects to the quantum computing work of Chapter 42.
