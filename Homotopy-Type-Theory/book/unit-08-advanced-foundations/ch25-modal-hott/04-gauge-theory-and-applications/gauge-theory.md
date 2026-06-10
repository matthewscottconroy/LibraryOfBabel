# 25.4 Gauge Theory and Applications to Physics

## Physics in Type Theory

The question sounds preposterous: can you do physics in type theory? Physics is about the world — particles, fields, spacetime. Type theory is about formal proofs. What do they have to do with each other?

The answer begins with the observation that modern theoretical physics is largely mathematics. Quantum field theory is not experimental data — it is a mathematical structure: functionals on configuration spaces, path integrals over spaces of fields, symmetry groups acting on bundles. General relativity is differential geometry applied to a 4-manifold. String theory is the theory of maps from 2-manifolds into target spaces satisfying certain geometric conditions.

If physics is mathematics, and mathematics can be done in type theory, then physics can be done in type theory. The question is whether the *right* mathematics is there — whether the type theory can express gauge fields, connections, curvature, and the rest without too much overhead.

Urs Schreiber's research program — *Differential Cohomology in a Cohesive Topos* and its sequel works — shows that the answer is yes, and that cohesive HoTT is the correct setting. Not merely an encoding of physics in type theory, but a setting in which the fundamental objects of physics *are* types, the symmetries are equivalences, and the physical quantities are type-theoretic invariants.

## Principal Bundles as Types

A *principal $G$-bundle* over a base space $M$ is a space $P$ together with a free, transitive action of the Lie group $G$ on $P$ and a projection $\pi : P \to M$ such that $M \simeq P/G$.

The classical description is in terms of transition functions: an open cover $\{U_i\}$ of $M$ and maps $g_{ij} : U_i \cap U_j \to G$ satisfying the cocycle condition $g_{ij} g_{jk} = g_{ik}$.

In cohesive HoTT, a principal $G$-bundle over $M$ is simply a map:
$$P : M \to BG$$

where $BG$ is the *classifying type* of $G$ — the type of $G$-torsors (free transitive $G$-sets). The classifying type exists as a HIT (for discrete groups) or as a cohesive type (for Lie groups).

**Why this works**: The classical classification theorem says that principal $G$-bundles over $M$ are classified by homotopy classes of maps $M \to BG$. In HoTT (with univalence), maps $M \to BG$ are literally the same thing as bundles — not just classified by them. The univalence axiom collapses "homotopy classes of maps" to "maps," and the HoTT circle (as a HIT) is literally $B\mathbb{Z}$.

In cohesive HoTT, the map $P : M \to BG$ captures both the topological classification (via $\int P : \int M \to B\int G$) and the smooth structure (via the cohesive type structure of $P$).

## Connections on Principal Bundles

A *connection* on a principal $G$-bundle $P : M \to BG$ is a "lifting" of paths in $M$ to paths in $P$ that is $G$-equivariant.

In cohesive HoTT, a connection is captured using the flat modality:

**Definition.** A *connection* on $P : M \to BG$ is a lift of $P$ to a map:
$$\nabla : M \to \flat BG$$

such that $\varepsilon^\flat_{BG} \circ \nabla = P$ (the flat lift projects down to $P$ via the counit).

Wait — this defines a *flat* connection. A general connection is a more subtle object.

The correct definition uses the *de Rham coefficient object*: for a Lie group $G$ with Lie algebra $\mathfrak{g}$, the *de Rham moduli stack of $G$-connections* is the type:
$$\mathbf{B}G_{\mathsf{conn}} :\equiv \{ \nabla : M \to BG \mid \nabla \text{ is a refinement of } P \}$$

In Schreiber's notation: $\mathbf{B}G_{\mathsf{conn}}$ is the homotopy pullback:
$$\mathbf{B}G_{\mathsf{conn}} :\equiv BG \times_{\flat BG} \flat BG$$

where the pullback is over the canonical map $\flat BG \to BG$ (the flat counit). An element of $\mathbf{B}G_{\mathsf{conn}}$ is a pair: a bundle $P : M \to BG$ and a connection $\nabla : M \to \flat BG$ that refines it.

**The moduli stack of connections**: For a fixed base manifold $M$ and group $G$, the type of connections is:
$$\mathsf{Conn}(M, G) :\equiv \mathsf{Map}(M, \mathbf{B}G_{\mathsf{conn}})$$

This is a genuine type in cohesive HoTT — not a formal construction, not a set of Christoffel symbols. The points of $\mathsf{Conn}(M, G)$ are connections; the paths are gauge transformations.

## Gauge Transformations as Equivalences

A *gauge transformation* between two connections $\nabla_1, \nabla_2 \in \mathsf{Conn}(M, G)$ is a path $\gamma : \nabla_1 =_{\mathsf{Conn}(M,G)} \nabla_2$.

This is literally the HoTT path type. A gauge transformation is a path in the type of connections. This is not a metaphor — in cohesive HoTT, this is the definition.

**The gauge group**: The *gauge group* $\mathcal{G}(P)$ of a bundle $P$ is the type of automorphisms of $P$:
$$\mathcal{G}(P) :\equiv \mathsf{Aut}_{M \to BG}(P) = (P =_{M \to BG} P)$$

The gauge group is the loop space of the moduli stack $\mathsf{Conn}(M, G)$ at the connection $\nabla$:
$$\mathcal{G}(P, \nabla) = \Omega_\nabla \mathsf{Conn}(M, G)$$

Gauge theory asks: what are the gauge-invariant quantities? In HoTT, these are exactly the *propositions* about connections that are invariant under the path symmetry — i.e., that are well-defined on the moduli stack.

**The correct moduli problem**: The *moduli space of flat connections* on $M$ is:
$$\mathcal{M}_{\mathsf{flat}}(M, G) :\equiv \mathsf{Map}(\int M, BG)$$

Maps from the shape of $M$ to $BG$ — not from $M$ itself, but from its homotopy type. This is the gauge-theoretic content of the shape modality: flat connections are connections for which the holonomy depends only on the homotopy class of the loop.

## The Chern-Weil Homomorphism Synthetically

The *Chern-Weil homomorphism* is the classical map from connections on a principal $G$-bundle to characteristic classes in de Rham cohomology. It sends an invariant polynomial $P$ on $\mathfrak{g}$ to the differential form $P(F_\nabla)$ where $F_\nabla$ is the curvature of the connection.

In cohesive HoTT, the Chern-Weil homomorphism arises from the natural maps between the different cohomology theories:

$$\hat{H}^n(M, \mathbb{Z}) \to H^n_{\mathsf{dR}}(M) \quad \text{(the curvature map)}$$

Given a connection $\nabla \in \mathsf{Conn}(M, G)$ and an invariant polynomial $P$ on $\mathfrak{g}$, the Chern-Weil invariant is an element of $\hat{H}^n(M, \mathbb{Z})$ (differential cohomology) that maps to the de Rham form $P(F_\nabla)$.

In Schreiber's formulation: the Chern-Weil map is a natural transformation:
$$\mathsf{CW}_P : \mathsf{Conn}(M, G) \to \hat{H}^n(M, \mathbb{Z})$$

This is a map of types in cohesive HoTT. Its naturality (invariance under gauge transformations) follows from the fact that it is a map — maps between types automatically preserve the path structure.

**Key properties** (all automatic from the type theory):
- $\mathsf{CW}_P$ is gauge-invariant: it sends gauge-equivalent connections to equal characteristic classes
- $\mathsf{CW}_P$ is natural in $M$: pullback of connections gives pullback of characteristic classes
- $\mathsf{CW}_P$ does not depend on the choice of connection for the topological (integral) part: two connections on the same bundle give cohomologous forms

All of these properties, which in the classical setting require lengthy calculations, are automatic in cohesive HoTT because they are properties of maps.

## The Maurer-Cartan Form

For a Lie group $G$ viewed as a smooth type, the *Maurer-Cartan form* $\theta_G \in \Omega^1(G, \mathfrak{g})$ is the canonical $\mathfrak{g}$-valued 1-form on $G$.

In cohesive HoTT, $\theta_G$ arises from the universal principal bundle $\mathsf{id} : BG \to BG$. The connection on this universal bundle is the Maurer-Cartan form.

More concretely: the flat counit $\varepsilon^\flat : \flat G \to G$ gives a map from the flat group to the smooth group. The difference between the identity map $G \to G$ and the composite $G \to \flat G \to G$ measures the failure of a function to be flat — this is the Maurer-Cartan form.

The Maurer-Cartan form satisfies the *Maurer-Cartan equation*:
$$d\theta_G + \frac{1}{2}[\theta_G, \theta_G] = 0$$

In cohesive HoTT, this equation is a theorem — derived from the cohesion axioms and the group structure of $G$ — not an additional axiom.

## Schreiber's Physics Formalization

Urs Schreiber's research program aims to formalize the foundations of quantum field theory in cohesive HoTT. The key objects:

**Prequantum field theory**: A prequantum field theory is a pair $(M, \mathcal{L})$ where $M$ is a smooth manifold (the "spacetime" or "worldvolume") and $\mathcal{L} : M \to B^n U(1)_{\mathsf{conn}}$ is a *prequantum $n$-bundle* — a higher principal bundle with connection.

In cohesive HoTT, this is just a map from a cohesive type $M$ to the classifying type $B^n U(1)_{\mathsf{conn}}$ for circle $n$-bundles with connection.

**The action functional**: The action functional of the prequantum field theory is the integral:
$$S[\phi] = \int_M \phi^* \mathcal{L}$$

In cohesive HoTT, integration is defined using the shape modality: the integral of a form over $M$ is a map $\int_M : \mathsf{Map}(M, \Omega^n) \to \mathbb{R}$ (or to $\mathbb{R}/\mathbb{Z}$ for differential cohomology reasons).

**Quantum field theory via factorization algebras**: The quantization of a prequantum field theory produces a *factorization algebra* — an algebraic structure that assigns observables to open subsets of $M$ and specifies how they compose.

In cohesive HoTT, a factorization algebra is a covariant fibration over the Segal type of open subsets of $M$ with values in $E_\infty$-algebras (ring spectra). The connection to STT (Chapter 24) is evident: the factorization algebra is a functor from a Segal type (open subsets under inclusion) to a Rezk type ($E_\infty$-algebras).

**The Chern-Simons theory**: One of the canonical examples of Schreiber's formalization is Chern-Simons theory. The Chern-Simons action is a map:
$$\mathsf{CS} : \mathsf{Conn}(M, G) \to U(1)$$

(for a 3-manifold $M$) defined by the integral of the Chern-Simons 3-form. In cohesive HoTT:
$$\mathsf{CS} = \int_M \mathsf{CW}_{P_2} \quad : \quad \mathsf{Map}(M, BG_{\mathsf{conn}}) \to U(1)$$

where $P_2$ is the second Casimir invariant of $G$. The gauge invariance of the Chern-Simons action — the fact that it changes only by an integer under large gauge transformations — becomes a statement about the type-theoretic structure of this map.

**Higher gauge theory**: Modern physics requires *higher gauge theory* — gauge theory with Lie 2-groups, Lie ∞-groups, and higher gauge fields. In cohesive HoTT, higher gauge theory is just gauge theory with a different classifying type $BG$ — where $G$ is now an ∞-group (a group in the ∞-categorical sense). The same definitions, the same theorems, the same language. The "higher" part is absorbed into the type theory's ∞-groupoid structure.

## What Remains to Be Done

The Schreiber program is not complete. Significant challenges remain:

**Quantization**: The step from classical field theory (connections on bundles) to quantum field theory (path integrals, operator algebras) requires additional machinery. In the cohesive setting, quantization should be a map from the classical moduli stack to a quantum observables algebra, compatible with the cohesive structure. This remains largely conjectural.

**Renormalization**: Quantum field theory requires renormalization — a procedure for removing divergences in infinite-dimensional integrals. In the cohesive setting, renormalization would be a specific type-theoretic operation on functional integrals. This is not yet formalized.

**The Standard Model**: The Standard Model of particle physics is a gauge theory with gauge group $U(1) \times SU(2) \times SU(3)$. In cohesive HoTT, the Standard Model Lagrangian should be a map $M \to \mathbf{B}G_{\mathsf{conn}}$ for $G = U(1) \times SU(2) \times SU(3)$. The formalization of this specific gauge theory, with its Higgs mechanism and Yukawa couplings, remains an open problem.

**String theory**: The most ambitious part of Schreiber's program concerns string theory. The string Chern-Simons theory, the Green-Schwarz anomaly cancellation, and the duality symmetries of M-theory all have formulations in cohesive HoTT using higher gauge fields. Parts of this have been formalized; much remains.

## The Significance of the Program

Why should type theorists care about physics? And why should physicists care about type theory?

For type theorists: physics provides a rich source of complex, formally demanding mathematical structures. Gauge theory, string theory, and quantum field theory are among the most sophisticated mathematical objects humans have constructed. A type theory that can handle these structures is demonstrably powerful.

For physicists: type theory provides the possibility of *formally verified* physics. A proof in cohesive HoTT that a physical theory is consistent, anomaly-free, and has the correct symmetries is a formal guarantee, checkable by a machine. The history of theoretical physics contains many claims (about anomaly cancellation, about consistency of string backgrounds, about quantum field theory constraints) that have been verified only by calculation. Formal verification would provide a different kind of certainty.

For mathematicians: the connection between physics and mathematics runs through cohesive HoTT in both directions. Physical intuition suggests mathematical constructions (Chern-Simons theory suggests knot invariants, string theory suggests mirror symmetry). Mathematical precision provides the framework for making physical intuitions rigorous. Cohesive HoTT is where these two influences meet at the level of foundations.

This is what it means to do physics in type theory: not to encode physics as a formal game, but to find the type-theoretic language in which the deep structure of physics is directly and naturally expressed.
