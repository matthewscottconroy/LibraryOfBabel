# Part V — Foundations of CS and Mathematics

> *"The deepest problems in dynamics and information theory are not solved by clever techniques alone — they require conceptual frameworks, new languages in which to ask the right questions."*

---

## Overview

Part V occupies a distinctive position in the curriculum: it is where the dynamical and information-theoretic machinery developed in Parts II–IV encounters the deepest questions of foundations — computability, logic, category theory, and the mathematics of physical law. These are not merely "applications" of the earlier theory, nor are they pure abstraction for its own sake. They are the places where mathematics becomes reflexive: where it turns its tools on itself, asking what can be computed, what can be classified, what can be said at all.

The six chapters of Part V span a remarkable range: the limits of computation in dynamics (Chapter 27), the categorical structure of dynamical systems (Chapter 28), the thermodynamics of information processing (Chapter 29), the geometry of measure transport (Chapter 30), arithmetic progressions and dynamical methods in number theory (Chapter 31), and the logical complexity of classification problems in ergodic theory (Chapter 32). Each chapter deepens and enriches the earlier material while opening entirely new vistas.

A unifying theme is the *computability-complexity-expressibility* spectrum. Chapter 27 studies what dynamical properties are computable. Chapter 32 studies what classification problems are Borel (expressible by a countable sequence of open conditions). Chapter 28 asks what the *categorical* structure of dynamical systems is — what the right morphisms, functors, and universal constructions are. Chapter 29 asks what the *physical* meaning of information is — when information erasure has thermodynamic cost. These are all, at their root, questions about what can be said and at what cost.

The connections among the chapters are dense. Optimal transport (Chapter 30) provides the correct geometry for spaces of measures, enabling quantitative versions of ergodic convergence theorems. Ergodic number theory (Chapter 31) shows that dynamical methods — Furstenberg's correspondence principle — can prove combinatorial theorems (Szemerédi's theorem, Green-Tao theorem) that seemed far removed from dynamics. Descriptive set theory (Chapter 32) provides the logical framework for understanding why some classification problems are possible and others are provably impossible — a framework that connects directly to the research frontiers of Part VI.

---

## Prerequisites

The reader should have completed Parts I–IV. Chapter 28 (Category Theory) benefits from prior exposure to category theory (at the level of functors and natural transformations); the necessary definitions are provided within the chapter but the reader who has seen categories before will get more from it. Chapter 29 (Thermodynamics) benefits from physical intuition about entropy and energy. Chapter 31 (Ergodic Number Theory) uses ergodic theory (Chapter 7) heavily. Chapter 32 (Descriptive Set Theory) requires comfort with logic and set theory.

**What you gain from this part:**
- Understanding of which dynamical properties are computable, and which are undecidable.
- The categorical framework for dynamical systems: categories of flows, coalgebras, categorical entropy.
- The physical interpretation of information: Landauer's principle, Maxwell's demon, and the thermodynamics of computation.
- Optimal transport as a geometric framework for dynamics on measures.
- Furstenberg's correspondence principle and its applications: Szemerédi's theorem, equidistribution.
- Descriptive set theory: Polish spaces, Borel reducibility, turbulence, and the limits of classification.

---

## Chapter Descriptions

### Chapter 27 — Computability Theory and Dynamics

What can be decided about a dynamical system by an algorithm? This chapter studies the computational complexity of dynamical properties — transitivity, mixing, entropy, the existence of attractors — and shows that many natural questions are undecidable.

The emptiness problem for 2D subshifts of finite type (Wang tiles) is undecidable (Berger 1966): there is no algorithm that, given a finite set of local rules for a 2D tiling, decides whether any valid tiling of the plane exists. This implies the existence of *aperiodic* tile sets — sets of rules that can tile the plane but only non-periodically. The undecidability proof uses a construction that simulates an arbitrary Turing machine in the tile set: an aperiodic tile set is one that "correctly computes" a non-terminating computation.

The Turing degree structure provides a hierarchy of undecidable properties. For 1D subshifts: the emptiness problem is decidable (for SFTs), but the isomorphism problem is not. For sofic shifts: the entropy is computable (it is the logarithm of the Perron root of the transition matrix, computable by polynomial algorithms), but the zeta function is computable with multiplicity. For 2D subshifts: entropy can take any $\Pi_1^0$-computable real value, and the class of possible entropies is exactly the $\Pi_1^0$ reals above $\log 2$ (Simpson's theorem and extensions).

Computable analysis — Weihrauch's theory of represented spaces — provides the framework for asking computability questions about continuous mathematical objects. Julia sets of quadratic polynomials are computable (Braverman-Yampolsky 2006), but there exist rational maps with non-computable Julia sets (Rettinger-Weihrauch). The computability of the Mandelbrot set boundary is open — connected to MLC.

### Chapter 28 — Category Theory and Dynamics

Category theory provides a unifying language for dynamics: it allows precise statements about what is structure-preserving (morphisms), what structure a system has (functors), and when two descriptions are equivalent (natural isomorphisms). This chapter develops the categorical perspective on dynamical systems from first principles, with connections to topos theory and type theory.

A dynamical system is a pair $(X, f)$ where $f: X \to X$. The *category of dynamical systems* $\mathbf{DS}$ has these pairs as objects and morphisms $\phi: (X, f) \to (Y, g)$ as continuous maps $\phi: X \to Y$ with $\phi \circ f = g \circ \phi$ (equivariant maps, the dynamical analogue of group homomorphisms). The full categorical framework — products, coproducts, limits, colimits — gives a powerful language for constructing new dynamical systems from old ones.

*Coalgebras*: A dynamical system $(X, f)$ is a coalgebra for the identity endofunctor $F = \text{Id}: \mathbf{Top} \to \mathbf{Top}$. More generally, a coalgebra for an endofunctor $F: \mathcal{C} \to \mathcal{C}$ is a pair $(X, f: X \to F(X))$. *Coalgebraic bisimilarity* is the appropriate notion of equivalence for state-based systems, and it coincides with topological conjugacy in many settings.

*Categorical entropy*: Leinster (2012) derived Shannon entropy from first principles using category theory: given a certain functor from the category of finite probability spaces to the category of metric spaces, the only natural transformation satisfying a list of axioms is the Shannon entropy. This categorical derivation is more conceptually transparent than the Khinchin axioms and connects information theory to deep categorical structures.

*The topos of $G$-sets*: For a group $G$ (or monoid, semigroup), the *topos of $G$-sets* $\mathbf{Set}^G$ consists of sets equipped with a $G$-action. This is the natural categorical setting for studying symmetries of dynamical systems. The internal logic of this topos is the type theory of spaces with $G$-symmetry — connecting directly to HoTT (Chapter 43).

### Chapter 29 — Thermodynamics, Information, and Physics

Thermodynamics and information theory share a vocabulary — entropy, temperature, work, heat — and this shared vocabulary reflects a deep structural connection, not a superficial analogy. Chapter 29 develops this connection rigorously.

*Maxwell's demon* is a thought experiment in which an intelligent agent sorts gas molecules by observing their velocities, apparently reducing entropy without doing work. The resolution — that the demon's memory accumulates information, and erasing this information requires energy — is Landauer's principle (1961): erasing one bit of information dissipates at least $kT \ln 2$ of energy as heat. This connects the Shannon entropy of a program to the thermodynamic entropy of a physical system.

*Landauer's principle* is now understood as a consequence of the second law of thermodynamics applied to the joint system (computer + heat bath). When the computer erases a bit, the bit's information must go somewhere: it goes into the heat bath as thermal motion. The minimum heat dissipated is $kT \ln 2$ per bit erased, regardless of how efficiently the computation is implemented. Reversible computation — in which no information is erased — in principle requires no heat dissipation. Fredkin and Toffoli gates are the building blocks of reversible computation.

*Fluctuation theorems* (Jarzynski 1997, Crooks 1998) are exact nonequilibrium identities that go far beyond Landauer's principle. Jarzynski's equality $\langle e^{-W/kT} \rangle = e^{-\Delta F/kT}$ relates the free energy difference $\Delta F$ between initial and final equilibrium states to the work $W$ done in any (possibly irreversible) protocol. This identity — exact for all processes, fast or slow — is one of the deepest results in non-equilibrium statistical mechanics. Crooks' theorem refines it: the ratio of the probability of forward and reverse work fluctuations equals $e^{W - \Delta F)/kT}$.

*Ruelle's pressure and free energy*: In the thermodynamic formalism for dynamical systems, the *pressure* $P(\phi) = \sup_\mu [h_\mu(f) + \int \phi \, d\mu]$ is the Legendre transform of the entropy. For a potential $\phi = -\beta E$ (where $E$ is an "energy function" on phase space and $\beta = 1/kT$), the equilibrium measure maximizing $P(\phi)$ is the *Gibbs state* — the dynamical analogue of the canonical ensemble. This connects the ergodic theory of hyperbolic systems to the statistical mechanics of gases.

### Chapter 30 — Optimal Transport and Dynamical Systems

Optimal transport provides a geometry for probability distributions that is tailored to dynamics: the Wasserstein distance $W_p(\mu, \nu)$ measures how much "work" is needed to rearrange the mass of $\mu$ into the mass of $\nu$, where work is measured by a cost function. This is the natural distance for studying how probability distributions evolve under dynamical systems.

*Brenier's theorem*: The optimal transport map from $\mu$ to $\nu$ (the map $T: \mathbb{R}^n \to \mathbb{R}^n$ that minimizes $\int |x - T(x)|^2 \, d\mu(x)$ and satisfies $T_\# \mu = \nu$) is the gradient of a convex function. This is a remarkable structure theorem: the optimal rearrangement is always the gradient flow of a convex potential.

*Gradient flows in Wasserstein space*: The Fokker-Planck equation $\partial_t \rho = \nabla \cdot (\rho \nabla V) + \Delta \rho$ (which governs the evolution of the density of a diffusion process $dX = -\nabla V(X) \, dt + \sqrt{2} \, dW$) can be written as a gradient flow in the Wasserstein metric: $\partial_t \rho = -\text{grad}_{W_2} F(\rho)$ where $F(\rho) = \int [V(x) + \log \rho(x)] \rho(x) \, dx$ is the free energy. This perspective — due to Jordan-Kinderlehrer-Otto (JKO 1998) — gives the PDE a variational structure that enables new proof techniques (existence, stability, convergence to equilibrium).

*Optimal transport and entropy*: The relative entropy $H(\rho \| \gamma) = \int \rho \log(\rho/\gamma) \, dx$ is a Lyapunov function for the Fokker-Planck equation: it decreases monotonically to 0 (as the density approaches the equilibrium Gibbs density $\gamma \propto e^{-V}$). The *log-Sobolev inequality* gives a quantitative rate: $H(\rho_t \| \gamma) \leq e^{-2\lambda t} H(\rho_0 \| \gamma)$, where $\lambda$ is the spectral gap of the generator.

### Chapter 31 — Ergodic Theory and Number Theory

One of the most spectacular successes of ergodic theory is its application to number theory: dynamical methods have proved results in arithmetic that seemed inaccessible by purely combinatorial or number-theoretic means.

*Weyl's equidistribution theorem*: For any irrational $\alpha$, the sequence $n\alpha \pmod 1$ is equidistributed in $[0,1]$: for every continuous $f: [0,1] \to \mathbb{R}$,
$$\frac{1}{N} \sum_{n=0}^{N-1} f(n\alpha) \to \int_0^1 f(x) \, dx.$$
The dynamical proof uses the rotation $R_\alpha: x \mapsto x + \alpha \pmod 1$ on the circle, which is ergodic iff $\alpha$ is irrational (a consequence of the Birkhoff ergodic theorem).

*Furstenberg's correspondence principle* (1977): A set $A \subseteq \mathbb{Z}$ of positive upper density $d^+(A) = \limsup_{N \to \infty} |A \cap [0,N]|/N > 0$ corresponds to a measurable set in a measure-preserving system with positive measure. Szemerédi's theorem — that $A$ contains arithmetic progressions of arbitrary length — follows from a multiple recurrence theorem in ergodic theory: for any measure-preserving $T$ and set $A$ with $\mu(A) > 0$,
$$\liminf_{N \to \infty} \frac{1}{N} \sum_{n=0}^{N-1} \mu(A \cap T^{-n}A \cap T^{-2n}A \cap \cdots \cap T^{-kn}A) > 0.$$

*Green-Tao theorem* (2004): The primes contain arbitrarily long arithmetic progressions. The proof uses ergodic theory (Furstenberg's machinery), additive combinatorics (Gowers uniformity norms), and analytic number theory (the Hardy-Littlewood circle method). This is a triumph of the modern synthesis: a purely number-theoretic result proved by a combination of ergodic theory and combinatorics.

*Continued fractions as a dynamical system*: The Gauss map $G: (0,1] \to [0,1)$, $G(x) = \{1/x\}$ (fractional part of $1/x$), generates the continued fraction expansion of $x$. The Gauss measure $\mu_G = \frac{1}{\ln 2} \cdot \frac{1}{1+x}$ is the unique absolutely continuous ergodic invariant measure. By the Birkhoff ergodic theorem, for $\mu_G$-a.e. $x$: the partial quotients $a_n(x) = \lfloor 1/G^{n-1}(x) \rfloor$ have geometric mean $\prod a_n^{1/n} \to K = \prod_{k=1}^\infty (1 + 1/(k(k+2)))^{\log k/\log 2} \approx 2.685$ (Khintchine's constant).

### Chapter 32 — Descriptive Set Theory and Dynamics

Descriptive set theory studies the logical complexity of subsets of Polish spaces (separable completely metrizable spaces). The key objects are the Borel hierarchy (sets defined by countably many open conditions), analytic sets (projections of Borel sets), and the theory of Borel equivalence relations (which models classification problems in mathematics).

A *Borel equivalence relation* on a Polish space $X$ is a Borel subset $E \subseteq X \times X$ that is an equivalence relation. Two equivalence relations $E$ and $F$ satisfy $E \leq_B F$ (*Borel reducibility*) if there is a Borel map $\phi: X \to Y$ with $x \, E \, x' \Leftrightarrow \phi(x) \, F \, \phi(x')$. This is the right notion of "classification by $F$-invariants": if $E \leq_B F$, then the $F$-equivalence class of $\phi(x)$ is a complete invariant for the $E$-equivalence class of $x$.

*Foreman-Rudolph-Weiss theorem* (2011): The isomorphism relation for ergodic measure-preserving transformations is *not Borel* — it has complexity beyond the Borel hierarchy. This is one of the most striking results in the theory: it shows that there is no countable list of invariants that classifies all ergodic systems. The classification problem is provably harder than anything achievable by Borel means.

*Hjorth's turbulence theory* (2000): A *turbulent* action is one that cannot be Borel-reduced to the orbit equivalence relation of a countable group action. Turbulence is the correct notion of "unclassifiability by countable invariants." Hjorth showed that the action of the unitary group of a Hilbert space on itself (by conjugation) is turbulent, which implies that unitary equivalence of operators is not classifiable by countable invariants.

*Applications to ergodic theory*: Gaboriau's $\ell^2$-Betti numbers are complete orbit-equivalence invariants for a large class of groups (those for which the $\ell^2$-Betti numbers separate all free ergodic actions). The study of which invariants are sufficient to classify ergodic systems — and which classification problems are provably impossible — is one of the main themes of the research frontier (Part VI, Chapter 35).

---

## Key Mathematical Concepts

### Computability and the Arithmetical Hierarchy

The *arithmetical hierarchy* classifies definable sets by the number of alternating quantifiers needed to define them: $\Sigma_1^0$ (computably enumerable = r.e.) sets are of the form $\{x : \exists y. R(x,y)\}$ with $R$ computable; $\Pi_1^0$ sets are complements of $\Sigma_1^0$ sets (i.e., the halting problem's complement); $\Delta_1^0 = \Sigma_1^0 \cap \Pi_1^0$ are the computable sets. Higher levels $\Sigma_n^0$, $\Pi_n^0$ alternate existential and universal quantifiers.

For dynamical systems: the emptiness problem for 1D SFTs is $\Sigma_1^0$ (decidable). The emptiness problem for 2D SFTs is $\Pi_1^0$-complete (as hard as the halting problem). The set of possible entropies of 2D SFTs is exactly the set of $\Pi_1^0$-computable non-negative reals.

### Wasserstein Distance

The *$p$-Wasserstein distance* between probability measures $\mu, \nu$ on a metric space $(X, d)$ is:
$$W_p(\mu, \nu) = \left(\inf_{\pi \in \Pi(\mu,\nu)} \int_{X \times X} d(x,y)^p \, d\pi(x,y)\right)^{1/p}$$
where $\Pi(\mu,\nu)$ is the set of couplings (joint measures with marginals $\mu$ and $\nu$). For $p = 1$: $W_1(\mu,\nu) = \sup_{\|f\|_{\text{Lip}} \leq 1} \int f \, d(\mu-\nu)$ (Kantorovich-Rubinstein duality).

### Borel Reducibility

For equivalence relations $E$ on $X$ and $F$ on $Y$: $E \leq_B F$ iff there is a Borel $\phi: X \to Y$ with $x E x' \Leftrightarrow \phi(x) F \phi(x')$. The equivalence relation $=_{\mathbb{R}}$ (equality of reals) is "small" (classifiable by real numbers). The isomorphism of countably infinite graphs is "Borel complete" (the hardest of its complexity class). The isomorphism of ergodic MPTs (Foreman-Rudolph-Weiss) is beyond Borel — it is analytic but not Borel.

---

## Key Theorems

1. **Berger's Theorem.** The domino problem (emptiness of 2D SFTs) is undecidable. Equivalently, there exist aperiodic sets of Wang tiles.

2. **Furstenberg's Multiple Recurrence Theorem.** For an ergodic MPT $T$ on $(X, \mu)$, any set $A$ with $\mu(A) > 0$, and any $k \geq 1$: $\liminf_{N \to \infty} \frac{1}{N} \sum_{n=1}^N \mu(A \cap T^{-n}A \cap \cdots \cap T^{-kn}A) > 0$.

3. **Brenier's Theorem.** For absolutely continuous $\mu$ and any $\nu$ on $\mathbb{R}^n$: the unique optimal transport map (for quadratic cost $|x-y|^2$) from $\mu$ to $\nu$ is the gradient of a convex function $u: \mathbb{R}^n \to \mathbb{R}$.

4. **Leinster's Entropy Theorem.** Shannon entropy is the unique family of functionals $H_n: \Delta^n \to \mathbb{R}$ satisfying: symmetry, continuity, normalization, and the chain rule. This characterization follows from a universal property in the category of finite probability spaces.

5. **Foreman-Rudolph-Weiss Theorem.** The isomorphism relation for ergodic MPTs is not Borel. In particular, there is no countable set of real-valued invariants that classifies all ergodic systems.

6. **Landauer's Principle.** Logically irreversible computation — in particular, erasing one bit of information — necessarily dissipates at least $kT \ln 2$ of energy to the environment.

---

## Connections to Other Parts

Part V connects in both directions — to the established theory and to the research frontier:

- **From Parts II–IV:** All the main tools of the book are brought to bear here. Ergodic theory (Chapter 7) drives the number theory of Chapter 31. Symbolic dynamics (Chapter 12) provides the examples for the computability theory of Chapter 27. Optimal transport (Chapter 30) is the rigorous version of the informal notion of "distance between measures" that appears throughout ergodic theory.

- **To Part VI (Frontiers):** Chapter 32's descriptive set theory framework is the foundation for understanding the research frontiers. The Foreman-Rudolph-Weiss theorem (Chapter 32) directly motivates the study of which specific systems *can* be classified (Chapter 35 — the Zimmer program for group actions). Categorical entropy (Chapter 28) motivates the sofic entropy of Chapter 34.

- **To Part VII (Connections):** Category theory (Chapter 28) prepares for the HoTT connections (Chapter 43): toposes, internal logic, and synthetic dynamics. The thermodynamics of Chapter 29 connects to the physical intuitions behind the quantum computing connections of Chapter 42.
