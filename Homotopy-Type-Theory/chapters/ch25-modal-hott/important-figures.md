# Important Figures

Modal HoTT and cohesive type theory sit at the intersection of synthetic geometry, categorical logic, and mathematical physics. The figures below span from the categorical origins of cohesion in the work of Lawvere and Kock through the development of cohesive ∞-toposes by Schreiber to the type-theoretic synthesis by Shulman and Wellen.

---

## F. William Lawvere (1937–2023)
*Founder of categorical logic and originator of the cohesion concept; category theorist at SUNY Buffalo*

William Lawvere received his PhD from Columbia University in 1963 under the supervision of Samuel Eilenberg. His thesis introduced the concept of a "theory" in categorical logic — what is now called a Lawvere theory — and launched the program of categorical semantics for logic and type theory. Over the following six decades, Lawvere developed categorical logic, topos theory, and synthetic geometry into major fields, often working against the mainstream and eventually being vindicated as the mainstream caught up.

Lawvere's contribution to the material of Chapter 25 is the concept of a cohesive topos, introduced in his 1991 lecture notes and developed in his 2007 paper "Axiomatic Cohesion." The central idea: a topos $\mathcal{E}$ is *cohesive* over Sets if there is an adjoint quadruple
$$\Pi_0 \dashv \Delta \dashv \Gamma \dashv \nabla$$
where $\Pi_0$ computes connected components, $\Delta$ produces discrete spaces, $\Gamma$ extracts underlying sets (global sections), and $\nabla$ produces codiscrete spaces. Lawvere's insight was that this quadruple captures the intuitive idea of "cohesion": points in a cohesive space can be connected by paths (hence $\Pi_0$ gives connected components) and can be discretized (hence $\Gamma$). The cohesion modalities of Chapter 25 — $\int$, $\flat$, $\sharp$ — are the type-theoretic internalization of $\Pi_0$, $\Delta \circ \Gamma$, and $\nabla \circ \Gamma$ respectively.

Lawvere's vision was sweeping: he believed that cohesion captures the conceptual distinction between "space" (continuous, cohesive) and "quantity" (discrete, set-theoretic). His work on "axiomatic cohesion" and on "categories of space and quantity" tried to give a topos-theoretic foundation for this distinction. Shulman's cohesive HoTT realized Lawvere's vision in a setting that is both formally precise (machine-checkable) and mathematically powerful (capable of proving the Brouwer fixed-point theorem). Lawvere did not live to see the full flowering of the program he initiated, but his foundational concepts permeate every section of Chapter 25.

---

## Anders Kock (1938–present)
*Founder of synthetic differential geometry; mathematician at Aarhus University*

Anders Kock received his PhD from Aarhus University and spent his career there, developing synthetic differential geometry (SDG) from its foundations in the 1970s through to its mature form in the 2006 second edition of "Synthetic Differential Geometry." SDG proposes to do differential calculus from axioms rather than from the classical epsilon-delta constructions, using a "nilpotent" infinitesimal interval $D = \{x : R \mid x^2 = 0\}$ in a topos with an appropriate line object $R$.

The Kock-Lawvere axiom — that any function $D \to R$ is of the form $x \mapsto a + bx$ for unique $a, b : R$ — is the key axiom of SDG. It says that $D$ is "the first-order part of $R$," and that functions on $D$ are exactly the first-order Taylor polynomials. From this axiom alone, Kock derives the chain rule, the product rule, the definitions of differential forms, the de Rham complex, and much of multivariate calculus, all without limits or epsilon-delta arguments.

For Chapter 25, Kock's contribution is foundational: the infinitesimal interval $D$ in cohesive HoTT is directly Kock's $D$, and the synthetic tangent bundle of a smooth type $M$ is defined as $TM = M^D$ — the type of maps $D \to M$ (infinitesimal paths in $M$). The de Rham theorem (Section 3 of Chapter 25) has its classical precursor in Kock's synthetic de Rham theorem, which holds in any well-adapted model of SDG. Kock also developed the synthetic theory of connections and covariant derivatives, which feeds directly into the gauge theory applications of cohesive HoTT. His 2006 textbook, freely available on his website, is the best introduction to SDG for someone with a background in abstract algebra and category theory.

---

## Mike Shulman (1980–present)
*Primary developer of cohesive HoTT and real-cohesive HoTT; logician at the University of San Diego*

Mike Shulman's contributions to Chapter 25 are discussed in detail in the Chapter 24 Important Figures entry (where his co-creation of simplicial type theory is described). Here we focus on his cohesive HoTT work specifically.

The central paper is "Brouwer's Fixed-Point Theorem in Real-Cohesive Homotopy Type Theory" (2018), which is at once a foundational development of cohesive HoTT (stating the cohesion axioms, deriving their consequences, and establishing the connection to classical topology) and a remarkable application (the Brouwer fixed-point theorem, a non-trivial result of classical topology, proved entirely from cohesion axioms without a single coordinate computation). The key cohesion axiom that drives the proof is *real cohesion*: $\int\mathbb{R} \simeq \mathbf{1}$ (the real line is contractible as a cohesive type, so its shape is the one-point type). From this, one derives that the shape of $\mathbb{R}^n$ is contractible, and hence the shape of $D^n$ (the closed unit disk) is contractible. The Brouwer theorem then follows from a standard topological argument about the non-contractibility of $S^{n-1}$.

Shulman is also responsible for the important computation $\int(\mathbb{R}/\mathbb{Z}) \simeq S^1$: the shape of the quotient $\mathbb{R}/\mathbb{Z}$ (which, as a smooth manifold, is the circle $S^1$) has the homotopy type of $S^1$. This is both a sanity check on the cohesion axioms and a non-trivial theorem. From it, one can compute the fundamental group of the circle synthetically within cohesive HoTT, giving a second (cohesive) proof of $\pi_1(S^1) = \mathbb{Z}$ that is independent of the HIT-based proof in Chapter 20.

His broader contributions to HoTT foundations — the semantics of modalities in ∞-toposes, the relationship between cohesive and non-cohesive HoTT, the connections to linear logic and quantum mechanics — have shaped the entire landscape of modal HoTT research.

---

## Urs Schreiber (1974–present)
*Developer of differential cohomology in cohesive ∞-toposes; mathematical physicist at NYUAD and MPI Bonn*

Urs Schreiber received his PhD in physics from the University of Essen in 2003 and has worked at the intersection of mathematical physics, higher category theory, and homotopy theory. His research program, developed over roughly two decades, is the construction of a cohesive ∞-topos that serves as the natural setting for modern differential geometry and its application to quantum field theory.

Schreiber's most important work for Chapter 25 is "Differential Cohomology in a Cohesive ∞-Topos" (2013, continuously updated on arXiv), a comprehensive development of gauge theory, higher gauge theory, topological field theory, and string theory within a cohesive ∞-topos. The key insight: a principal $G$-bundle over a space $X$ is a map $X \to BG$ (where $BG$ is the classifying space of $G$), and a connection on such a bundle is a lift of this map to $BG_\nabla$ (the "moduli stack of $G$-bundles with connection"). In cohesive HoTT, both $BG$ and $BG_\nabla$ are definable types, and the flat modality $\flat$ distinguishes flat connections (principal bundles over the discrete version of $X$) from general connections. This is a genuinely synthetic treatment: the gauge invariance is built into the type theory.

Schreiber's contribution extends to *higher gauge theory*: 2-bundles (where the structure group is a 2-group), the string theory B-field (a 2-connection on a 2-bundle), and the anomaly cancellation mechanism (a coherence condition on higher bundles). These structures, which require elaborate bookkeeping in classical differential geometry, have clean synthetic definitions in cohesive HoTT. Schreiber has also been an important popularizer of the program through the nLab, which he founded and continues to develop as a comprehensive reference for modern mathematical physics from the categorical perspective.

---

## Felix Wellen (present)
*Formalization of differential geometry in cohesive HoTT; researcher at Karlsruhe Institute of Technology and beyond*

Felix Wellen completed his PhD at the Karlsruhe Institute of Technology in 2017, advised by Ieke Moerdijk. His thesis, "Cartan Geometry in Modal Homotopy Type Theory," gave the first systematic formalization of differential geometry in cohesive HoTT, covering affine connections, Cartan geometries (a general framework that includes Riemannian geometry, conformal geometry, and projective geometry as special cases), and curvature.

Wellen's contribution to Chapter 25 is the demonstration that the abstract cohesion axioms support concrete differential geometry. His formalization in Agda (available at github.com/felixwellen/synthetic-geometry) provides machine-verified proofs of the basic constructions: the synthetic tangent bundle $TM = M^D$, the sheaf of differential forms $\Omega^n(M) = (M^{D^n})^{Sn}$ (alternating functions on $n$-fold infinitesimal thickenings), and the de Rham differential. The de Rham theorem — that the de Rham cohomology of $M$ agrees with the cohomology of $\int M$ (the shape of $M$) — is proved synthetically.

Beyond his thesis, Wellen has developed synthetic algebraic geometry in cohesive HoTT, defining affine schemes, projective spaces, and vector bundles synthetically. The surprising result is that classical algebraic geometry — typically developed using commutative algebra and abstract sheaf theory — has a clean synthetic formulation in cohesive HoTT, parallel to the smooth geometry formulation. Wellen's work shows that cohesive HoTT is not just a framework for one kind of geometry but a genuinely universal synthetic geometry.
