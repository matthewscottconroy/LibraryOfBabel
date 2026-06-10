# Exercises — Chapter 25: Modal HoTT and Cohesive Geometry

## Section 1: Modalities

**Exercise 1.** Define the notion of a modality in HoTT precisely, including:
- (a) The modal operator $\bigcirc : \mathsf{Type} \to \mathsf{Type}$ and unit $\eta_A : A \to \bigcirc A$
- (b) The notion of a $\bigcirc$-modal type
- (c) The universal property: precomposition with $\eta_A$ is an equivalence for maps to modal types
- (d) The idempotency: $\bigcirc \bigcirc A \simeq \bigcirc A$

**Exercise 2.** Show that the propositional truncation $\|-\|$ is a modality:
- (a) Identify the modal types (propositions)
- (b) Verify the universal property: maps $A \to P$ where $P$ is a proposition factor through $\|A\|$
- (c) Show that $\|-\|$ is NOT left exact: give a pullback that is not preserved
- (d) What property of propositions breaks lex-ness?

**Exercise 3.** Show that $n$-truncation $\|-\|_n$ is a modality for each $n \geq -2$:
- (a) Identify the modal types ($n$-truncated types)
- (b) State the universal property for maps to $n$-truncated types
- (c) Show that $n$-truncation for $n \geq 0$ IS left exact (preserves pullbacks)

**Exercise 4.** For a modality $\bigcirc$, the *$\bigcirc$-connected types* are types $A$ where $\bigcirc A \simeq \mathbf{1}$ (the modal image is contractible). Show:
- (a) Every map $f : A \to B$ factors as $A \to \bigcirc\text{-image}(f) \to B$ where the first map has $\bigcirc$-connected fibers and the second has $\bigcirc$-modal fibers
- (b) This factorization is unique up to equivalence

**Exercise 5.** Nullification: given a type $B$, the $B$-nullification $\mathsf{Null}_B(A)$ is the universal type where all maps from $B$ are null.
- (a) Characterize $B$-null types: what condition must $X$ satisfy?
- (b) Show that $S^0$-nullification gives a modality
- (c) What does $\mathbb{R}$-nullification do to a type?

**Exercise 6.** A modality is *left exact* (lex) if it preserves pullbacks. Show that a modality is lex iff it preserves identity types: $\bigcirc(a =_A b) \simeq (\bigcirc a =_{\bigcirc A} \bigcirc b)$.

**Exercise 7.** In Cubical Agda, import `Cubical.HITs.PropositionalTruncation` and study the type of `∥_∥`. Then:
- (a) Give the type signature of `rec₁` (the elimination principle)
- (b) Prove that `isProp (∥ A ∥₁)` using `rec₁`
- (c) Show that `∥ ∥ A ∥₁ ∥₁ ≃ ∥ A ∥₁` (idempotency of propositional truncation)

**Exercise 8.** The *orthogonal factorization system* (OFS) of a modality: define connected and modal maps, state the OFS axioms, and verify they hold for a modality of your choice.

## Section 2: Cohesive HoTT

**Exercise 9.** State the cohesion axioms precisely (using modalities and their adjunctions). Identify:
- (a) The three modalities $\int, \flat, \sharp$ and their units/counits
- (b) The adjunction data: which pairs are adjoint?
- (c) The two fully faithful conditions
- (d) The product-preservation condition for $\int$

**Exercise 10.** The flat modality $\flat$ satisfies $\flat(\flat A) \simeq \flat A$ (idempotency) and $\flat(\int A) \simeq \flat A$ (flat of shape is flat). Show:
- (a) Idempotency follows from the adjunction $\flat \dashv \sharp$ (or from the fully faithful condition)
- (b) $\flat(\int A) \simeq \flat A$ is one of the cohesion axioms — state it and explain its geometric meaning

**Exercise 11.** The real cohesion axiom: "$A$ is $\sharp$-modal iff every map $\mathbb{R} \to A$ is null-homotopic." Use this axiom to derive:
- (a) $\int \mathbb{R} \simeq \mathbf{1}$ (the real line is contractible as a cohesive space)
- (b) What would $\int \mathbb{R}^n \simeq ?$ be? Prove it.
- (c) Why does the real line have trivial shape but the circle $S^1 = \mathbb{R}/\mathbb{Z}$ does not?

**Exercise 12.** Crisp variables: in spatial type theory, a crisp variable `x :: A` is an element of `♭ A`. Show:
- (a) Why do maps `♭ A → B` for `B` a flat-modal type correspond to arbitrary functions `A → B`?
- (b) What would a "crisp function" `f : A → B` mean (all variables crisp)?
- (c) Give an example of a function that is definable with crisp inputs but not with cohesive inputs (or vice versa)

**Exercise 13.** The Brouwer fixed-point theorem proof in cohesive HoTT uses:
1. $\int D^2 \simeq \mathbf{1}$ (the disk is contractible)
2. $\int S^1 \simeq S^1$ (the circle has non-trivial shape)
3. $\pi_1(S^1) = \mathbb{Z}$
- (a) Prove (1) using the real cohesion axiom
- (b) State (2) and (3) precisely
- (c) Complete the proof sketch: why does a fixed-point-free map lead to a contradiction?

**Exercise 14.** Models of cohesive HoTT: the *smooth sets* model is the ∞-topos of sheaves on the site of Cartesian spaces $\{\mathbb{R}^n\}$ with smooth maps.
- (a) In this model, what is the shape $\int \mathbb{R}$? (It should be $\mathbf{1}$.)
- (b) What is $\flat \mathbb{R}$? (The underlying discrete set.)
- (c) What are the flat-modal types? (Types where only constant $\mathbb{R}$-paths exist.)

## Section 3: Differential Geometry Synthetically

**Exercise 15.** The locally constant functions on a cohesive space $M$ with values in a type $V$ factor through $\flat V$. Show:
- (a) A locally constant function $f : M \to V$ factors as $M \xrightarrow{g} \flat V \xrightarrow{\varepsilon^\flat} V$
- (b) The factorization is unique (since $\varepsilon^\flat$ is monomorphic)
- (c) Give an example of a function that is locally constant on the circle $S^1$ but not constant

**Exercise 16.** The de Rham theorem in cohesive HoTT states $H^n_{\mathsf{dR}}(M) \simeq H^n(\int M, \mathbb{R})$.
- (a) State the Poincaré lemma (as a consequence of the de Rham theorem and $\int \mathbb{R}^n \simeq \mathbf{1}$)
- (b) Compute $H^n_{\mathsf{dR}}(S^1)$ using the de Rham theorem and your knowledge of $\pi_1(S^1) = \mathbb{Z}$
- (c) What would $H^n_{\mathsf{dR}}(T^2)$ be for the 2-torus? (Use $\int T^2 \simeq T^2$ and your knowledge of $\pi_1(T^2) = \mathbb{Z}^2$)

**Exercise 17.** The Kock-Lawvere axiom in synthetic differential geometry: every map $D \to \mathbb{R}$ (where $D = \{x : \mathbb{R} \mid x^2 = 0\}$) is of the form $x \mapsto a + bx$.
- (a) Show that this axiom is inconsistent with classical logic (the axiom implies $D \neq \{0\}$, but classically $x^2 = 0 \Rightarrow x = 0$)
- (b) Show that the Kock-Lawvere axiom holds in the *smooth sets* model of cohesive HoTT
- (c) Using Kock-Lawvere, define the derivative of a function $f : \mathbb{R} \to \mathbb{R}$

**Exercise 18.** The de Rham differential $d : \Omega^0(M) \to \Omega^1(M)$:
- (a) Using the Kock-Lawvere axiom, define $df(v) :\equiv \lambda d. f(v(d))$ for $v : D \to M$ a tangent vector and $f : M \to \mathbb{R}$ a smooth function. Show this is well-defined.
- (b) Verify that $d(\text{const}_c) = 0$ (the differential of a constant is zero)
- (c) Verify the Leibniz rule: $d(fg) = f\, dg + g\, df$

**Exercise 19.** A *flat connection* on a trivial $G$-bundle over $M$ is a map $A : M \to \mathfrak{g}$ such that the holonomy around every contractible loop is trivial. In cohesive HoTT, show that flat connections correspond to maps $M \to \flat BG$.

**Exercise 20.** The *Maurer-Cartan form* on a Lie group $G$: using the flat modality, describe how the counit $\varepsilon^\flat : \flat G \to G$ gives rise to a canonical 1-form on $G$. (Hint: the difference between the identity map $G \to G$ and the composite $G \to \flat G \xrightarrow{\varepsilon^\flat} G$ measures the "deviation from flatness.")

## Section 4: Gauge Theory

**Exercise 21.** Principal $G$-bundles as maps $M \to BG$:
- (a) For $G = U(1)$ and $M = S^2$, describe the type $\mathsf{Map}(S^2, BU(1))$. What does the first Chern class measure?
- (b) The Hopf fibration is a specific element of $\pi_3(S^2) = \mathbb{Z}$. Describe it as a map $S^3 \to BU(1)$ (or $S^2 \to BU(1)$ after appropriate fibration).
- (c) What is the gauge group of the trivial $U(1)$-bundle over $S^2$?

**Exercise 22.** Gauge transformations as paths: a gauge transformation between connections $\nabla_1$ and $\nabla_2$ is a path $\gamma : \nabla_1 =_{\mathsf{Conn}(M,G)} \nabla_2$.
- (a) Describe what a gauge transformation looks like concretely for a $U(1)$-connection on $S^1$
- (b) What is the group structure on gauge transformations (composition of paths)?
- (c) Why does the type-theoretic account of gauge transformations automatically give them a group structure?

**Exercise 23.** The Chern-Weil homomorphism: for a $U(1)$-bundle with connection over a surface $\Sigma$:
- (a) Define the curvature 2-form $F_\nabla \in \Omega^2(\Sigma, i\mathbb{R})$
- (b) Show that $\int_\Sigma F_\nabla \in 2\pi i \mathbb{Z}$ (the curvature integral is quantized) using the fact that the first Chern class is integral
- (c) In cohesive HoTT, where does this quantization condition come from?

**Exercise 24.** The moduli stack of flat connections: a flat connection on $M$ is an element of $\mathsf{Map}(\int M, BG)$.
- (a) For $M = S^1$ and $G = U(1)$, compute $\mathsf{Map}(\int S^1, BU(1)) = \mathsf{Map}(S^1, BU(1))$. What does this classify?
- (b) What is the gauge group of a flat connection?
- (c) For $M = T^2 = S^1 \times S^1$ and $G = U(1)$, compute the moduli stack of flat connections.

**Exercise 25.** Chern-Simons theory: the Chern-Simons action for a $U(1)$-connection over a 3-manifold $M$ is $\mathsf{CS}(\nabla) = \int_M A \wedge dA$ where $A$ is the connection 1-form.
- (a) Show that $\mathsf{CS}$ is gauge-invariant up to an integer: $\mathsf{CS}(\nabla^g) - \mathsf{CS}(\nabla) \in \mathbb{Z}$ for a gauge transformation $g$
- (b) In cohesive HoTT, how does this gauge invariance follow from the type-theoretic structure?
- (c) The Chern-Simons partition function is $Z(M) = \int_{\mathsf{Conn}(M, U(1))} e^{2\pi i k \cdot \mathsf{CS}(\nabla)} \mathcal{D}\nabla$. What would "integrating over a type" mean in cohesive HoTT?

**Exercise 26.** Higher gauge theory: a *2-group* is a group in the category of groups — a group $G$ together with an action of $G$ on another group $A$, plus coherence data. In HoTT, a 2-group is an ∞-group $G$ with $\pi_0 G$ and $\pi_1 G$ non-trivial.
- (a) Define $B^2 U(1) = B(BU(1))$ (the double delooping of $U(1)$). What does a map $M \to B^2 U(1)$ classify?
- (b) The "string group" $\mathsf{String}(n)$ is a 3-connected cover of $\mathsf{Spin}(n)$ satisfying $\pi_3(\mathsf{String}(n)) = 0$. How would you define this in HoTT?
- (c) A string structure on a spin manifold $M$ is a lift of the structure group from $\mathsf{Spin}$ to $\mathsf{String}$. What is this in type-theoretic terms?

**Exercise 27.** (Advanced) The *Green-Schwarz mechanism* in string theory anomaly cancellation involves a specific relationship between the curvature of the gravitational and gauge fields:
$$dH = \mathrm{tr}(R \wedge R) - \mathrm{tr}(F \wedge F)$$
where $H$ is a 3-form (field strength of a 2-form $B$-field), $R$ is the Riemannian curvature, and $F$ is the gauge field strength.

In cohesive HoTT terms: the $B$-field is a connection on a circle 2-bundle $M \to B^2 U(1)_{\mathsf{conn}}$. State the Green-Schwarz equation as a condition on the curvature of this 2-bundle. What would it mean for this equation to be "automatically satisfied" in the type-theoretic setting?

**Exercise 28.** (Proof assistant project) In Cubical Agda with `--cohesion`:
- (a) Import the flat modality `♭` and the counit `♭-counit`
- (b) Show that `♭ (A × B) ≃ ♭ A × ♭ B` (flatness preserves products)
- (c) Define the notion of "crisp function" (a function definable with only crisp inputs) and give an example

**Exercise 29.** (Advanced) In cohesive HoTT, the *cohomology triangle* for differential cohomology is:
$$\hat{H}^n(M, \mathbb{Z}) \simeq H^n(\flat M, \mathbb{Z}) \times_{H^n(\int M, \mathbb{Z})} H^n_{\mathsf{dR}}(M)$$
Show that this is a pullback:
- (a) Identify the maps in the pullback square
- (b) Explain why the pullback captures "compatible integral and de Rham cohomology classes"
- (c) For $n = 2$ and $M = S^2$, compute $\hat{H}^2(S^2, \mathbb{Z})$

**Exercise 30.** (Research-level) Schreiber's program aims to formalize the Standard Model gauge theory in cohesive HoTT. The gauge group is $G = U(1) \times SU(2) \times SU(3)$. Outline what would be needed:
- (a) Define the classifying type $BG$ for this gauge group
- (b) Define the moduli stack $\mathsf{Conn}(M, G)$ for a 4-manifold $M$ (spacetime)
- (c) Write down the Yang-Mills action (schematically) as a function $\mathsf{YM} : \mathsf{Conn}(M, G) \to \mathbb{R}$
- (d) What additional structure (Higgs field, matter fields) would need to be added to complete the Standard Model?

**Exercise 31.** The *Atiyah-Singer index theorem* relates the analytical index and topological index of an elliptic operator. In cohesive HoTT:
- (a) What type would an elliptic differential operator on a smooth manifold $M$ be an element of?
- (b) What would the "analytical index" be in type-theoretic terms?
- (c) What would the "topological index" be (in terms of characteristic classes)?
- (d) State the Atiyah-Singer theorem as an equivalence of types. Is this currently provable in cohesive HoTT?
