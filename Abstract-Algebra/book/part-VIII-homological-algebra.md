# Part VIII — Homological Algebra

**Chapters 38–41**

---

## What This Part Establishes

Part VIII develops homological algebra: the systematic study of chain complexes, exact sequences, derived functors, and the machinery for measuring the failure of exactness. Homological algebra grew out of algebraic topology (where chain complexes compute homology groups of spaces), and it has since become the universal language of algebra, geometry, and beyond.

Chapter 38 introduces chain complexes — sequences of modules connected by "boundary" maps with $d^2 = 0$ — and the homology groups that measure the gap between cycles and boundaries. Chapter 39 develops resolutions: the art of approximating an arbitrary module by a complex of "nice" modules (projective or injective), making derived functors possible. Chapter 40 constructs the main derived functors Ext and Tor — the canonical measurements of how far$\mathrm{Hom}$ and$\otimes$ are from being exact — and introduces group cohomology as a fundamental application. Chapter 41 develops spectral sequences, the iterated homological machine that allows computation of hard homology groups from easier local data.

By the end of Part VIII, the reader can compute derived functors, work with spectral sequences, and understand the algebraic backbone of algebraic topology, algebraic geometry, and representation theory.

---

## Internal Dependency Map

```
Ch 38 (Chain Complexes, Homology, Homotopy)
              |
              v
        Ch 39 (Resolutions)
              |
              v
        Ch 40 (Ext, Tor, Group Cohomology)
              |
              v
        Ch 41 (Spectral Sequences)
```

---

## Chapter 38 — Chain Complexes and Homology

**What it establishes:** The algebraic formalism of chain complexes as the setting for homological computations; the homology groups as the canonical measurement of "cycles modulo boundaries"; and chain homotopies as the correct notion of equivalence.

---

**38.1 Chain Complexes**

**38.1.1 Chain Complexes: Modules and Differentials with $d^2 = 0$**
A *chain complex* $(C_\bullet, d)$ of$R$-modules is a sequence
$$\cdots \to C_{n+1} \xrightarrow{d_{n+1}} C_n \xrightarrow{d_n} C_{n-1} \to \cdots$$
where each $d_n: C_n \to C_{n-1}$ is an$R$-linear map (the *differential* or *boundary map*) satisfying$d_n \circ d_{n+1} = 0$ (equivalently,$d^2 = 0$). The condition$d^2 = 0$ says "the boundary of a boundary is zero" — the fundamental relation underlying both algebraic topology (boundary maps on singular chains) and algebra (differentials in free resolutions). The index$n$ is the *degree*; the complex may extend infinitely in either or both directions.

**38.1.2 Cochain Complexes; Cohomological Conventions**
A *cochain complex* $(C^\bullet, d)$ uses superscript indexing and upward differentials:$d^n: C^n \to C^{n+1}$ with$d^{n+1} \circ d^n = 0$. Cochain complexes arise naturally in cohomology theories (de Rham cohomology, group cohomology, sheaf cohomology). The distinction between chain and cochain complexes is largely notational; a cochain complex is a chain complex with reversed indexing. We use "chain complex" to mean either, depending on context, and specify the convention when necessary.

**38.1.3 Morphisms of Chain Complexes**
A *chain map* $f: C_\bullet \to D_\bullet$ is a collection of$R$-linear maps$f_n: C_n \to D_n$ commuting with the differentials:$d_n^D \circ f_n = f_{n-1} \circ d_n^C$ for all$n$. Commutativity with differentials is the chain complex analogue of a group homomorphism commuting with the group structure: it says$f$ "respects the boundary relation." Chain maps induce maps on homology: if$f$ is a chain map, then$f_*: H_n(C) \to H_n(D)$ is a well-defined$R$-linear map.

**38.1.4 The Category of Chain Complexes $\mathrm{Ch}(R\text{-Mod})$**
Chain complexes of $R$-modules and chain maps form a category$\mathrm{Ch}(R\text{-Mod})$, which is itself an abelian category. The abelian structure on$\mathrm{Ch}(R\text{-Mod})$: kernels and cokernels of chain maps are computed degreewise; short exact sequences of chain complexes are sequences that are exact in every degree. The homology functor$H_n: \mathrm{Ch}(R\text{-Mod}) \to R\text{-Mod}$ is a functor from this abelian category to$R$-modules.

---

**38.2 Homology and Cohomology**

**38.2.1 Cycles, Boundaries, and Homology: $H_n = \ker d_n / \mathrm{im}\, d_{n+1}$**
The *cycles* in degree $n$ are$Z_n = \ker d_n \subseteq C_n$ (the elements "annihilated by the boundary map"). The *boundaries* are$B_n = \mathrm{im}\, d_{n+1} \subseteq C_n$ (elements "in the image of the previous boundary"). Since$d^2 = 0$, we have$B_n \subseteq Z_n$, so the *$n$th homology module*$H_n(C_\bullet) = Z_n/B_n = \ker d_n / \mathrm{im}\, d_{n+1}$ is well-defined. The homology measures the "gap" between cycles and boundaries: elements of$H_n$ are classes of cycles that are not themselves boundaries.

**38.2.2 Induced Maps on Homology**
A chain map $f: C_\bullet \to D_\bullet$ induces well-defined maps$H_n(f): H_n(C) \to H_n(D)$: if$z \in Z_n(C)$ is a cycle, then$f_n(z) \in Z_n(D)$ (since$d^D f_n = f_{n-1} d^C$ and$d^C z = 0$). The map on homology sends the class$[z]$ to$[f_n(z)]$. This is well-defined: if$z = d^C(c)$ is a boundary, then$f_n(z) = f_n d^C(c) = d^D f_{n+1}(c)$ is also a boundary. The functoriality of homology is the statement that$H_n$ is a functor$\mathrm{Ch}(R\text{-Mod}) \to R\text{-Mod}$.

**38.2.3 The Long Exact Sequence from a Short Exact Sequence of Complexes**
A short exact sequence of chain complexes $0 \to A_\bullet \to B_\bullet \to C_\bullet \to 0$ (exact in every degree) gives rise to a *long exact sequence in homology*:
$$\cdots \to H_{n+1}(C) \xrightarrow{\partial} H_n(A) \to H_n(B) \to H_n(C) \xrightarrow{\partial} H_{n-1}(A) \to \cdots$$
This is the fundamental theorem of homological algebra: a short exact sequence of chain complexes produces a long exact sequence in homology, connected by *connecting homomorphisms* $\partial$. Every long exact sequence in algebra arises from a short exact sequence of complexes by this construction.

**38.2.4 The Connecting Homomorphism $\partial_n$: Explicit Construction**
The connecting homomorphism $\partial_n: H_n(C) \to H_{n-1}(A)$ is constructed by "diagram chasing": given a cycle$[c] \in H_n(C)$, lift$c$ to an element$b \in B_n$, compute$d^B(b) \in B_{n-1}$, note that$d^B(b)$ maps to$0$ in$C_{n-1}$ (since$c$ was a cycle), so$d^B(b)$ comes from some$a \in A_{n-1}$, and$a$ is a cycle (since$d^A(a) = 0$ maps to$d^B(d^B(b)) = 0$). The class$[a] \in H_{n-1}(A)$ is$\partial_n([c])$. Every step of this construction is forced, verifying that$\partial$ is well-defined and that the long exact sequence is exact.

---

**38.3 Chain Homotopy**

**38.3.1 Chain Homotopies Between Maps of Complexes**
Two chain maps $f, g: C_\bullet \to D_\bullet$ are *chain homotopic* if there exist$R$-linear maps$h_n: C_n \to D_{n+1}$ (of degree$+1$) satisfying$f_n - g_n = d^D_{n+1} \circ h_n + h_{n-1} \circ d^C_n$ for all$n$. The maps$h_n$ are the *chain homotopy*. Chain homotopy is the algebraic analogue of topological homotopy between continuous maps: two homotopic chain maps are "continuously deformable into each other" in an algebraic sense.

**38.3.2 Chain Homotopic Maps Induce the Same Map on Homology**
If $f$ and$g$ are chain homotopic, then$H_n(f) = H_n(g)$ for all$n$. Proof: if$z$ is an$n$-cycle, then$f_n(z) - g_n(z) = d^D_{n+1}(h_n(z)) + h_{n-1}(d^C_n(z)) = d^D_{n+1}(h_n(z)) + 0$, which is a boundary. So$[f_n(z)] = [g_n(z)]$ in$H_n(D)$. Chain homotopy equivalence is thus the correct notion of "same up to homological information": it implies equality on all homology groups.

**38.3.3 Chain Homotopy Equivalences; Quasi-Isomorphisms**
A chain map $f: C_\bullet \to D_\bullet$ is a *quasi-isomorphism* if it induces isomorphisms$H_n(f): H_n(C) \xrightarrow{\sim} H_n(D)$ for all$n$ — it is an isomorphism "on homology" without being an isomorphism "on the nose." A *chain homotopy equivalence* is a chain map$f$ with a chain homotopy inverse$g: D_\bullet \to C_\bullet$ (maps with$g \circ f$ and$f \circ g$ chain homotopic to the respective identities). Every chain homotopy equivalence is a quasi-isomorphism; the converse fails. Quasi-isomorphisms are the "weak equivalences" of homological algebra: one "localizes at quasi-isomorphisms" to form the derived category.

**38.3.4 The Homotopy Category $K(R\text{-Mod})$**
The *homotopy category* $K(R\text{-Mod})$ has the same objects as$\mathrm{Ch}(R\text{-Mod})$ but morphisms are chain homotopy classes of chain maps. In$K(R\text{-Mod})$, chain homotopic maps become equal, so the induced maps on homology are well-defined on$K$. The *derived category*$D(R\text{-Mod})$ is obtained from$K(R\text{-Mod})$ by additionally inverting quasi-isomorphisms: two complexes with the same homology become isomorphic in$D(R\text{-Mod})$. The derived category is the natural home of derived functors and is one of the central objects of modern algebra and geometry.

---

## Chapter 39 — Resolutions

**What it establishes:** The technique of approximating an arbitrary module by a complex of "nice" modules (projective, injective, or free) — a fundamental tool that converts module theory into chain complex theory and enables the construction of derived functors.

---

**39.1 Projective Resolutions**

**39.1.1 Definition: Exact Complex of Projective Modules Mapping to $M$**
A *projective resolution* of an $R$-module$M$ is an exact complex
$$\cdots \to P_2 \xrightarrow{d_2} P_1 \xrightarrow{d_1} P_0 \xrightarrow{\varepsilon} M \to 0$$
where each $P_n$ is a projective$R$-module. The complex is exact everywhere including at$P_0$, meaning$\varepsilon$ is surjective and$\ker(d_n) = \mathrm{im}(d_{n+1})$ for all$n \geq 0$. A projective resolution "builds$M$ from projective pieces," replacing$M$ by a sequence of simpler (projective) modules connected by exact maps.

**39.1.2 Existence of Projective Resolutions: Every Module Has One**
Every $R$-module$M$ has a projective resolution. Construction: choose a surjection$\varepsilon: P_0 \to M$ with$P_0$ free (hence projective); let$M_1 = \ker \varepsilon$ and choose a surjection$P_1 \to M_1$ with$P_1$ free; let$M_2 = \ker(P_1 \to M_1)$; continue. This gives a free (hence projective) resolution of$M$. The resolution may be infinite (i.e.,$P_n \neq 0$ for infinitely many$n$) unless$M$ has finite projective dimension.

**39.1.3 The Comparison Theorem: Liftings Are Unique up to Chain Homotopy**
If $P_\bullet \to M$ and$Q_\bullet \to N$ are projective resolutions and$f: M \to N$ is an$R$-linear map, then there exists a chain map$\tilde f: P_\bullet \to Q_\bullet$ lifting$f$ (i.e., making the diagram commute). Moreover, any two such lifts are chain homotopic. This is the Comparison Theorem: the lift is unique up to chain homotopy, so the induced map on homology is canonical. The Comparison Theorem is why derived functors are well-defined: applying a functor to a projective resolution and taking homology gives a result independent (up to isomorphism) of the choice of resolution.

**39.1.4 Projective Dimension: The Length of the Shortest Projective Resolution**
The *projective dimension* $\mathrm{pd}(M)$ of$M$ is the smallest$n$ such that$M$ has a projective resolution of length$n$ (with$P_k = 0$ for$k > n$), or$\infty$ if no finite resolution exists. Equivalently,$\mathrm{pd}(M) \leq n$ iff$\mathrm{Ext}^{n+1}_R(M, N) = 0$ for all$N$. The *global dimension* of$R$ is$\sup_M \mathrm{pd}(M)$. Fields have global dimension 0 (all modules are projective = free);$\mathbb{Z}$ and$F[x]$ have global dimension 1 (all modules have projective dimension$\leq 1$, since every submodule of a free module is free).

---

**39.2 Injective Resolutions**

**39.2.1 Definition: Exact Complex of Injective Modules Out of $M$**
An *injective resolution* of $M$ is an exact complex
$$0 \to M \xrightarrow{\eta} I^0 \xrightarrow{d^0} I^1 \xrightarrow{d^1} I^2 \to \cdots$$
(using cohomological indexing) where each $I^n$ is an injective$R$-module. The map$\eta$ is the *augmentation*; exactness means$\eta$ is injective and$\ker(d^n) = \mathrm{im}(d^{n-1})$ for all$n \geq 1$. Injective resolutions are the "dual" to projective resolutions and are used to compute right derived functors (in particular,$\mathrm{Ext}$ from the other side).

**39.2.2 Existence via Injective Hulls**
Every $R$-module$M$ has an injective resolution. Construction: embed$M$ into its injective hull$E(M) = I^0$ (which exists by the Baer criterion and Zorn's Lemma); let$M_1 = I^0/M$ and embed it into$I^1 = E(M_1)$; continue. This builds an injective resolution by iterating the injective hull construction. Injective resolutions may also be infinite; the *injective dimension*$\mathrm{id}(M)$ is the length of the shortest injective resolution.

**39.2.3 Injective Dimension; Global Dimension of a Ring**
The *injective dimension* $\mathrm{id}(M)$ of$M$ is the smallest$n$ such that$M$ has an injective resolution of length$n$, or$\infty$. The global dimension of$R$ equals$\sup_M \mathrm{pd}(M) = \sup_M \mathrm{id}(M)$ — the two suprema agree. Over a Noetherian ring, the global dimension can often be computed from the Auslander–Buchsbaum theorem or from the formula for polynomial rings (Hilbert's syzygy theorem).

---

**39.3 Free Resolutions and Applications**

**39.3.1 Free Resolutions; Examples over $\mathbb{Z}$ and$F[x]$**
A *free resolution* is a projective resolution in which all modules are free (which is always achievable, since free modules are projective). Over $\mathbb{Z}$: the free resolution of$\mathbb{Z}/n\mathbb{Z}$ is$0 \to \mathbb{Z} \xrightarrow{\times n} \mathbb{Z} \to \mathbb{Z}/n\mathbb{Z} \to 0$ (length 1). Over$F[x]$: the free resolution of$F$ as an$F[x]$-module (where$x$ acts as 0) is$0 \to F[x] \xrightarrow{\times x} F[x] \to F \to 0$. These short free resolutions immediately give$\mathrm{Ext}$ and$\mathrm{Tor}$ computations.

**39.3.2 Computing Resolutions via Syzygies**
The *first syzygy* of a module $M$ (with respect to a choice of generators) is the kernel of the surjection$F_0 \to M$ from a free module. Hilbert called the generators of this kernel "syzygies" (Greek: yoke or conjunction). The second syzygy is the kernel of$F_1 \to \ker(F_0 \to M)$, and so on. Computing a free resolution amounts to computing successive syzygy modules. Modern computer algebra (using Gröbner bases) automates this process for polynomial rings.

**39.3.3 Hilbert's Syzygy Theorem: $F[x_1,\ldots,x_n]$ Has Global Dimension$n$**
*Hilbert's Syzygy Theorem:* The polynomial ring $F[x_1, \ldots, x_n]$ over a field$F$ has global dimension$n$: every finitely generated$F[x_1,\ldots,x_n]$-module has a free resolution of length at most$n$, and there exist modules requiring exactly length$n$. The proof uses induction on$n$: the$n$-th syzygy of any module over$F[x_1,\ldots,x_n]$ is free. Hilbert's Syzygy Theorem is a landmark result connecting the arithmetic of polynomial rings to the homological properties of their modules, and it initiated the study of projective dimension and free resolutions.

---

## Chapter 40 — Derived Functors: Ext and Tor

**What it establishes:** The canonical derived functors of Hom and Tensor — Ext and Tor — as the systematic measurements of the failure of left-exactness of Hom and right-exactness of Tensor; their computation from resolutions; and group cohomology as a fundamental example.

---

**40.1 Derived Functors: The Construction**

**40.1.1 Right Derived Functors via Injective Resolutions**
Let $F: \mathcal{A} \to \mathcal{B}$ be a left-exact functor between abelian categories. The *$n$th right derived functor*$R^n F$ is defined as follows: for any object$M$, choose an injective resolution$0 \to M \to I^0 \to I^1 \to \cdots$; apply$F$ to get a cochain complex$F(I^0) \to F(I^1) \to \cdots$ (no longer necessarily exact); take the$n$th cohomology:$R^n F(M) = H^n(F(I^\bullet))$. One has$R^0 F \cong F$ (since$F$ is left-exact). The Comparison Theorem ensures$R^n F(M)$ is independent (up to isomorphism) of the choice of injective resolution.

**40.1.2 Left Derived Functors via Projective Resolutions**
Let $F: \mathcal{A} \to \mathcal{B}$ be a right-exact functor. The *$n$th left derived functor*$L_n F$ is defined by: choose a projective resolution$\cdots \to P_1 \to P_0 \to M \to 0$; apply$F$ to get a chain complex$F(P_0) \leftarrow F(P_1) \leftarrow \cdots$; take the$n$th homology:$L_n F(M) = H_n(F(P_\bullet))$. One has$L_0 F \cong F$. The comparison theorem ensures independence of the projective resolution.

**40.1.3 Independence of Resolution; Well-Definedness**
The key to derived functors is that the groups $R^n F(M)$ and$L_n F(M)$ are independent of the chosen resolution. The proof: given two resolutions$I_\bullet$ and$J_\bullet$, the Comparison Theorem provides chain homotopy equivalences between them; applying$F$ gives chain homotopy equivalences of the resulting complexes; chain homotopy equivalences induce isomorphisms on homology. Thus the derived functors are canonical invariants of$M$, not artifacts of computational choices.

**40.1.4 Long Exact Sequences in Derived Functors**
A short exact sequence $0 \to A \to B \to C \to 0$ gives a long exact sequence:
$$0 \to F(A) \to F(B) \to F(C) \to R^1 F(A) \to R^1 F(B) \to R^1 F(C) \to R^2 F(A) \to \cdots$$
(for right derived functors of a left-exact functor $F$). This is the *long exact sequence of derived functors*, and it is the central computational tool: it allows one to compute derived functor groups by induction, using known computations at each stage. Every long exact sequence in algebra (in cohomology theories, in representation theory) is of this form.

---

**40.2 The Ext Groups**

**40.2.1 $\mathrm{Ext}^n_R(M,N)$: Derived Functor of$\mathrm{Hom}_R(M,-)$**
The *Ext groups* $\mathrm{Ext}^n_R(M, N) = R^n \mathrm{Hom}_R(M, -)(N)$ are the right derived functors of$\mathrm{Hom}_R(M, -)$. Equivalently, they can be computed as the left derived functors of$\mathrm{Hom}_R(-, N)$: both methods give isomorphic groups (a non-trivial theorem).$\mathrm{Ext}^0_R(M, N) = \mathrm{Hom}_R(M, N)$; higher Ext groups measure the "derived failure" of Hom to be exact.

**40.2.2 Computation via Projective Resolutions of $M$**
Given a projective resolution $\cdots \to P_1 \to P_0 \to M \to 0$, apply$\mathrm{Hom}_R(-, N)$ (contravariant) to get a cochain complex$\mathrm{Hom}(P_0, N) \to \mathrm{Hom}(P_1, N) \to \cdots$ and define$\mathrm{Ext}^n_R(M,N) = H^n$ of this complex. This is the standard computation: the complex$\mathrm{Hom}(P_\bullet, N)$ is assembled from the projective resolution of$M$, and its cohomology groups are the Ext groups.

**40.2.3 $\mathrm{Ext}^0 = \mathrm{Hom}$;$\mathrm{Ext}^1$ Classifies Short Exact Sequences**
$\mathrm{Ext}^0_R(M,N) = \ker(\mathrm{Hom}(P_0,N) \to \mathrm{Hom}(P_1,N)) = \mathrm{Hom}_R(M,N)$ (using the resolution). More profoundly,$\mathrm{Ext}^1_R(C, A)$ classifies isomorphism classes of short exact sequences$0 \to A \to B \to C \to 0$ (under the Baer sum operation). The zero element of$\mathrm{Ext}^1$ corresponds to the split exact sequence$0 \to A \to A \oplus C \to C \to 0$. This interpretation of$\mathrm{Ext}^1$ as extensions is a fundamental link between homological algebra and the structure theory of modules.

**40.2.4 The Baer Sum on Extensions**
If $\xi: 0 \to A \to B \to C \to 0$ and$\xi': 0 \to A \to B' \to C \to 0$ are two extensions, their *Baer sum* is a new extension$\xi + \xi': 0 \to A \to E \to C \to 0$ constructed by: taking the pullback of$B \oplus B' \to C \oplus C$ over the diagonal$C \to C \oplus C$; then pushing out along the addition map$A \oplus A \to A$. The Baer sum makes$\mathrm{Ext}^1_R(C, A)$ into an abelian group and is the concrete realization of the group structure on$\mathrm{Ext}^1$.

**40.2.5 $\mathrm{Ext}^n_{\mathbb{Z}}(\mathbb{Z}/m, \mathbb{Z}/n)$: Explicit Computation**
Using the free resolution $0 \to \mathbb{Z} \xrightarrow{\times m} \mathbb{Z} \to \mathbb{Z}/m \to 0$: apply$\mathrm{Hom}(-, \mathbb{Z}/n)$ to get$0 \to \mathrm{Hom}(\mathbb{Z}, \mathbb{Z}/n) \xrightarrow{\times m} \mathrm{Hom}(\mathbb{Z}, \mathbb{Z}/n) \to 0$, i.e.,$0 \to \mathbb{Z}/n \xrightarrow{\times m} \mathbb{Z}/n \to 0$. So$\mathrm{Ext}^0_{\mathbb{Z}}(\mathbb{Z}/m, \mathbb{Z}/n) = \ker(\times m) = \mathbb{Z}/\gcd(m,n)$ and$\mathrm{Ext}^1_{\mathbb{Z}}(\mathbb{Z}/m, \mathbb{Z}/n) = \mathrm{coker}(\times m) = \mathbb{Z}/\gcd(m,n)$, and all higher Ext vanish (since$\mathbb{Z}$ has global dimension 1).

**40.2.6 Vanishing of Ext and Projectivity/Injectivity**
$\mathrm{Ext}^n_R(M, N) = 0$ for all$N$ and$n \geq 1$ iff$M$ is projective.$\mathrm{Ext}^n_R(M, N) = 0$ for all$M$ and$n \geq 1$ iff$N$ is injective. These vanishing criteria connect the derived functor with the structure of the modules: "niceness" (projectivity/injectivity) is captured by the vanishing of derived functors.

---

**40.3 The Tor Groups**

**40.3.1 $\mathrm{Tor}_n^R(M,N)$: Left Derived Functor of$M \otimes_R -$**
The *Tor groups* $\mathrm{Tor}_n^R(M, N) = L_n(M \otimes_R -)(N)$ are the left derived functors of$M \otimes_R -$ (or equivalently of$- \otimes_R N$, since Tor is symmetric). Computation: choose a projective resolution$P_\bullet \to N$, apply$M \otimes_R -$ to get the chain complex$M \otimes P_\bullet$, and take homology:$\mathrm{Tor}_n^R(M,N) = H_n(M \otimes_R P_\bullet)$.$\mathrm{Tor}_0^R(M,N) = M \otimes_R N$.

**40.3.2 Symmetry: Tor Is Symmetric in Both Arguments**
$\mathrm{Tor}_n^R(M, N) \cong \mathrm{Tor}_n^R(N, M)$ for all$n$ (when$R$ is commutative, more precisely when both$M$ and$N$ are right and left$R$-modules respectively). The symmetry follows from a double complex argument: use a projective resolution of$M$ and one of$N$ to build a double complex, and compare the two spectral sequences associated to it. This symmetry justifies writing$\mathrm{Tor}_n^R(M,N)$ without specifying which argument is resolved.

**40.3.3 $\mathrm{Tor}_0 = \otimes$;$\mathrm{Tor}_1$ and Flatness**
$\mathrm{Tor}_0^R(M,N) = M \otimes_R N$. The module$N$ is flat iff$\mathrm{Tor}_1^R(M,N) = 0$ for all$M$, iff$\mathrm{Tor}_n^R(M,N) = 0$ for all$M$ and all$n \geq 1$. This characterizes flatness via Tor: the vanishing of Tor in degree 1 is the condition that$N$ does not "break" exactness when tensored with arbitrary modules.

**40.3.4 $\mathrm{Tor}_n^{\mathbb{Z}}(\mathbb{Z}/m, \mathbb{Z}/n)$: Explicit Computation**
Using the free resolution of $\mathbb{Z}/m$: apply$- \otimes \mathbb{Z}/n$ to$0 \to \mathbb{Z} \xrightarrow{\times m} \mathbb{Z} \to 0$ to get$0 \to \mathbb{Z}/n \xrightarrow{\times m} \mathbb{Z}/n \to 0$. So$\mathrm{Tor}_0^{\mathbb{Z}}(\mathbb{Z}/m, \mathbb{Z}/n) = \mathbb{Z}/n \otimes \mathbb{Z}/m = \mathbb{Z}/\gcd(m,n)$ (consistent with$\mathrm{Tor}_0 = \otimes$) and$\mathrm{Tor}_1^{\mathbb{Z}}(\mathbb{Z}/m, \mathbb{Z}/n) = \ker(\times m \text{ on } \mathbb{Z}/n) = \mathbb{Z}/\gcd(m,n)$.

**40.3.5 The Künneth Formula via Tor**
If $C_\bullet$ and$D_\bullet$ are chain complexes of flat (e.g., free) modules, the *Künneth formula* gives a (split) short exact sequence:
$$0 \to \bigoplus_{i+j=n} H_i(C) \otimes H_j(D) \to H_n(C \otimes D) \to \bigoplus_{i+j=n-1} \mathrm{Tor}_1(H_i(C), H_j(D)) \to 0$$
The Künneth formula computes the homology of a tensor product of complexes in terms of the homologies of the two factors and their Tor groups. In topology, this computes the homology of a product space $X \times Y$ from$H_*(X)$ and$H_*(Y)$.

---

**40.4 Group Cohomology**

**40.4.1 $G$-Modules and the Group Ring$\mathbb{Z}[G]$**
A *$G$-module* (for a group$G$) is an abelian group$M$ with a$G$-action by group automorphisms:$G \times M \to M$ with$(gh) \cdot m = g \cdot (h \cdot m)$ and$e \cdot m = m$. Equivalently, a$G$-module is a$\mathbb{Z}[G]$-module: the group ring$\mathbb{Z}[G]$ is the free abelian group on$G$ with multiplication extending the group law. Group cohomology is the right derived functor of the fixed-point functor$M \mapsto M^G = \{m \in M : g \cdot m = m \text{ for all } g\}$.

**40.4.2 $H^n(G,M) = \mathrm{Ext}^n_{\mathbb{Z}[G]}(\mathbb{Z}, M)$**
The *$n$th cohomology group* of$G$ with coefficients in$M$ is$H^n(G, M) = \mathrm{Ext}^n_{\mathbb{Z}[G]}(\mathbb{Z}, M)$, where$\mathbb{Z}$ is the trivial$G$-module ($g \cdot n = n$ for all$g, n$). Equivalently, choose a projective resolution$P_\bullet \to \mathbb{Z}$ of$\mathbb{Z}$ as a$\mathbb{Z}[G]$-module, apply$\mathrm{Hom}_{\mathbb{Z}[G]}(-, M)$, and take cohomology. The standard choice is the *bar resolution* (or normalized bar resolution), whose terms are free$\mathbb{Z}[G]$-modules on the Cartesian powers$G^n$.

**40.4.3 Low-Degree Interpretations: $H^0$ = Invariants,$H^1$ = Crossed Homomorphisms,$H^2$ = Extensions**
$H^0(G, M) = M^G$ (the$G$-invariants).$H^1(G, M)$ classifies crossed homomorphisms$f: G \to M$ (satisfying$f(gh) = f(g) + g \cdot f(h)$) modulo principal crossed homomorphisms (of the form$f(g) = g \cdot m - m$).$H^2(G, M)$ classifies extensions$0 \to M \to E \to G \to 1$ (short exact sequences of groups where$M$ is the abelian normal subgroup and$G$ acts on$M$ by conjugation). These interpretations motivate group cohomology from the classical perspective of automorphisms and extensions.

**40.4.4 The Bar Complex; Explicit Cocycles**
The *bar complex* provides an explicit free $\mathbb{Z}[G]$-resolution of$\mathbb{Z}$: in degree$n$, take$\mathbb{Z}[G] \otimes_{\mathbb{Z}} \mathbb{Z}[G^n]$ (free$\mathbb{Z}[G]$-module with basis the$n$-tuples$(g_1, \ldots, g_n) \in G^n$). The boundary map is the "face map" formula alternating over insertions and deletions of group elements (the standard simplicial differential). Applying$\mathrm{Hom}_{\mathbb{Z}[G]}(-, M)$ gives the cochain complex of functions$G^n \to M$ with the standard group cohomology differential, making cocycles (functions satisfying the cocycle condition) and coboundaries explicit.

---

## Chapter 41 — Spectral Sequences

**What it establishes:** The most powerful computational machine in homological algebra — a spectral sequence is a sequence of bigraded differential modules, each obtained from the last by taking homology, that converges to a target homology group. The machinery turns hard global computations into sequences of manageable local ones.

---

**41.1 The Idea and Setup**

**41.1.1 Why Spectral Sequences Arise: Iterated Exact Sequences**
Spectral sequences arise when a long exact sequence is not enough: when a computation involves multiple filtrations or multiple layers of structure that cannot be captured by a single exact sequence. The idea is to compute homology "iteratively": the $(r+1)$st page$E_{r+1}$ is the homology of the$r$th page$E_r$ with respect to a differential$d_r$. After finitely many steps (in good cases), the pages stabilize at$E_\infty$, which carries the "associated graded" of the target homology.

**41.1.2 Bigraded Pages $E_r^{p,q}$ and Differentials$d_r$**
A *spectral sequence* (in the first-quadrant case) consists of: bigraded abelian groups (or $R$-modules)$\{E_r^{p,q}\}$ for$r \geq 2$ and$p, q \geq 0$; differentials$d_r: E_r^{p,q} \to E_r^{p-r, q+r-1}$ (going$r$ to the left and$r-1$ upward in the$(p,q)$ grid) satisfying$d_r \circ d_r = 0$; and isomorphisms$E_{r+1}^{p,q} \cong H(E_r^{p,q})$ (the$(r+1)$st page is the homology of the$r$th with respect to$d_r$). The differential$d_r$ has bidegree$(-r, r-1)$, so each page involves "longer" differentials as$r$ increases.

**41.1.3 The $(r+1)$-st Page Is the Homology of the$r$-th**
The definition $E_{r+1}^{p,q} = \ker(d_r: E_r^{p,q} \to E_r^{p-r,q+r-1}) / \mathrm{im}(d_r: E_r^{p+r, q-r+1} \to E_r^{p,q})$ is the core recurrence of spectral sequence theory. Computing from$E_2$ to$E_3$ to$E_4$ requires knowing all the differentials$d_2, d_3, \ldots$ — determining these differentials is typically the hard part of a spectral sequence computation.

**41.1.4 Convergence: What $E_r \Rightarrow H^*$ Means**
A spectral sequence *converges* to $H^*$ (written$E_2^{p,q} \Rightarrow H^{p+q}$) if the pages eventually stabilize ($E_r^{p,q} = E_{r+1}^{p,q} = \cdots = E_\infty^{p,q}$ for$r \gg 0$) and the$E_\infty$ page provides the associated graded of a filtration on$H^*$:$H^n$ has a filtration$H^n = F^0 H^n \supseteq F^1 H^n \supseteq \cdots$ with$F^p H^n / F^{p+1} H^n \cong E_\infty^{p, n-p}$. Knowing$E_\infty$ does not fully determine$H^n$ (there is an "extension problem"), but it determines its associated graded.

---

**41.2 Double Complexes**

**41.2.1 Double Complexes and the Total Complex**
A *double complex* $C^{\bullet,\bullet}$ is a bigraded collection of modules$C^{p,q}$ with horizontal differentials$d^h: C^{p,q} \to C^{p+1,q}$ and vertical differentials$d^v: C^{p,q} \to C^{p,q+1}$, all squaring to zero and with$d^h d^v = d^v d^h$ (or$d^h d^v + d^v d^h = 0$ in the anti-commuting convention). The *total complex*$\mathrm{Tot}(C)^n = \bigoplus_{p+q=n} C^{p,q}$ with total differential$d = d^h + d^v$ (with appropriate signs) is a single chain complex. Computing$H_*(\mathrm{Tot}(C))$ is the goal; the two spectral sequences of the double complex provide two different approaches to this computation.

**41.2.2 The Two Spectral Sequences of a Double Complex**
For a double complex $C^{p,q}$, there are two spectral sequences converging to$H_*(\mathrm{Tot}(C))$: the *horizontal-first* sequence starts with${}^I E_2^{p,q} = H^p_h(H^q_v(C))$ (take vertical homology first, then horizontal); the *vertical-first* sequence starts with${}^{II} E_2^{p,q} = H^q_v(H^p_h(C))$. If one sequence degenerates quickly (e.g.,${}^I E_2 = E_\infty$), it computes the total homology directly. The comparison of the two spectral sequences gives powerful relations between homology groups.

**41.2.3 Degeneration: When $E_2 = E_\infty$**
A spectral sequence *degenerates at $E_2$* if all differentials$d_r = 0$ for$r \geq 2$, so$E_2 = E_\infty$. In this case,$H^n \cong \bigoplus_{p+q=n} E_2^{p,q}$ (with the extension problem trivially solved). Degeneration occurs in many important situations: over a field (by a dimension argument), in the Leray–Hirsch theorem, and in the Hochschild–Serre spectral sequence for group extensions with free abelian normal subgroup.

---

**41.3 The Lyndon–Hochschild–Serre Spectral Sequence**

**41.3.1 Setup: $1 \to N \to G \to Q \to 1$ and a$G$-Module$M$**
For a group extension $1 \to N \to G \to Q \to 1$ and a$G$-module$M$, the *Lyndon–Hochschild–Serre (LHS) spectral sequence* provides a systematic computation of$H^*(G, M)$ from$H^*(N, M)$ and$H^*(Q, H^*(N, M))$. The restriction and inflation maps in group cohomology are related by this spectral sequence, making it the fundamental tool for computing the cohomology of group extensions.

**41.3.2 The $E_2$ Page:$H^p(Q, H^q(N,M))$**
The second page of the LHS spectral sequence is $E_2^{p,q} = H^p(Q, H^q(N, M))$, where$H^q(N, M)$ is viewed as a$Q$-module via the$G$-module structure on$M$ and the conjugation action of$G$ on$N$. The differentials$d_r: E_r^{p,q} \to E_r^{p+r, q-r+1}$ are the "transgression" maps, which are generally hard to compute explicitly.

**41.3.3 Convergence to $H^{p+q}(G,M)$**
The LHS spectral sequence converges: $E_2^{p,q} = H^p(Q, H^q(N,M)) \Rightarrow H^{p+q}(G,M)$. This means the cohomology of$G$ is assembled from the cohomology of the quotient$Q$ with coefficients in the cohomology of the normal subgroup$N$. When the spectral sequence degenerates at$E_2$, the groups$H^n(G,M)$ split as direct sums$\bigoplus_{p+q=n} H^p(Q, H^q(N,M))$ — the cohomology of the extension is the "product" of the cohomologies of the factors.

**41.3.4 The 5-Term Exact Sequence from Low-Degree Terms**
The low-degree terms of the LHS spectral sequence give an exact sequence (the *inflation-restriction exact sequence*):
$$0 \to H^1(Q, M^N) \xrightarrow{\inf} H^1(G,M) \xrightarrow{\mathrm{res}} H^1(N,M)^Q \xrightarrow{d_2} H^2(Q, M^N) \xrightarrow{\inf} H^2(G,M)$$
This 5-term exact sequence relates the cohomology of $G$ to the cohomology of$N$ and$Q$ without needing to know the higher-page differentials. It is the standard tool for initial computations with group extensions.

**41.3.5 Inflation-Restriction; Applications to Galois Cohomology**
The *inflation* map $\inf: H^n(Q, M^N) \to H^n(G, M)$ and the *restriction* map$\mathrm{res}: H^n(G,M) \to H^n(N,M)^Q$ are the edge homomorphisms of the LHS spectral sequence. In Galois cohomology (where$G = \mathrm{Gal}(L/K)$ for a Galois extension$L/K$ and$M$ is a$G$-module), these maps are the fundamental tools for inductively computing Galois cohomology groups. The LHS spectral sequence is the algebraic engine behind many of the deepest results in number theory and Galois theory.

---

**41.4 Reading and Using Spectral Sequences**

**41.4.1 Edge Homomorphisms**
In a first-quadrant spectral sequence converging to $H^*$, the *edge homomorphisms* are the natural maps$H^n \to E_\infty^{n,0}$ and$E_\infty^{0,n} \to H^n$ arising from the filtration on$H^n$. These often have direct interpretations: in the LHS sequence, the edge maps are the inflation and restriction maps. Identifying and computing the edge homomorphisms is usually the first step in working with a new spectral sequence.

**41.4.2 The Extension Problem: From $E_\infty$ to$H^*$**
Knowing $E_\infty^{p,q}$ determines the associated graded of$H^n$, but not$H^n$ itself: there are group extensions$0 \to F^{p+1}H^n/F^{p+2}H^n \to F^p H^n/F^{p+2}H^n \to F^p H^n/F^{p+1}H^n \to 0$ whose splitting determines the structure of$H^n$. When working over a field, all extensions split and$H^n = \bigoplus_{p+q=n} E_\infty^{p,q}$. Over$\mathbb{Z}$, the extension problem is a serious obstruction and may require additional work.

**41.4.3 Collapse Arguments; Sparse Page Computations**
A spectral sequence *collapses* at $E_r$ if all differentials$d_s = 0$ for$s \geq r$. This occurs when the$E_r$ page is concentrated in positions where no differential can be nonzero (e.g.,$E_r^{p,q} = 0$ unless$q = 0$). Identifying collapse from sparsity conditions is the most common technique for reading spectral sequences in practice: one looks for reasons (degree, vanishing, rank) why differentials must be zero, then concludes$E_r = E_\infty$ and reads off the answer.

---

*Next: [Part IX — Representation Theory of Finite Groups](part-IX-representation-theory.md)*

*Prerequisites satisfied: Part I (logic, sets), Part II (linear algebra — exact sequences, eigentheory), Part III (group theory), Part IV (ring theory — group rings), Part V (module theory — the primary context for chain complexes), Part VII (category theory — abelian categories, exact functors, universal properties).*
