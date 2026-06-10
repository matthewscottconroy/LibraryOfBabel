# Chapter 25 Exercises: Modal HoTT

---

## Section 1: Modalities

**Exercise 1.1.** Show that propositional truncation $\|-\|$ is a modality:

1. Define the unit map $\eta_A : A \to \|A\|$.
2. Identify the class of $\|-\|$-modal types.
3. State and prove the universal property: any map $f : A \to P$ where $P$ is a proposition extends uniquely through $\|A\|$.
4. Is $\|-\|$ left exact? (Does it preserve pullbacks?)

**Exercise 1.2.** For each of the following, determine whether it defines a modality and if so, identify the modal types and universal property:

1. $\bigcirc A = A \times A$ with $\eta_A(x) = (x, x)$
2. $\bigcirc A = \|A\|_1$ (the 1-truncation / set-truncation)
3. $\bigcirc A = \mathsf{Prop}$ (constant at the type of propositions)
4. $\bigcirc A = (A \to A)$ with $\eta_A(x) = \mathsf{id}_A$

**Exercise 1.3.** Show that every modality is idempotent: $\bigcirc(\bigcirc A) \simeq \bigcirc A$.

*Hint:* Use the universal property of $\bigcirc A$ to show that $\bigcirc A$ is itself $\bigcirc$-modal.

**Exercise 1.4.** The *left exact modality* condition requires that $\bigcirc$ preserves pullbacks. Show that the $n$-truncation $\|-\|_n$ is left exact for $n \geq 0$ but not for $n = -1$ (propositional truncation).

*Hint:* For $n = -1$, consider the pullback of $\{0, 1\} \to \{*\} \leftarrow \{0, 1\}$. What is the truncation of the pullback vs. the pullback of the truncations?

---

## Section 2: Cohesive HoTT

**Exercise 2.1.** In cohesive HoTT, state the cohesion axioms precisely:

1. What are the three modalities and how are they related by adjunctions?
2. What are the unit and counit maps?
3. State the conditions $\int(\flat A) \simeq \flat A$ and $\flat(\int A) \simeq \flat A$.

**Exercise 2.2.** Using the axioms of real-cohesive HoTT:

1. Show that $\int \mathbb{R} \simeq \mathbf{1}$ (the real line is contractible as a cohesive space).
2. Show that $\int S^1 \simeq S^1$ (the circle's shape is the HoTT circle).
3. Show that $\flat \mathbb{Z} \simeq \mathbb{Z}$ (the integers are already discrete).

**Exercise 2.3.** The adjunction $\int \dashv \flat$ gives:

$$\mathsf{hom}(\int A, B) \simeq \mathsf{hom}(A, \flat B)$$

for any $B$ that is $\flat$-modal (discrete). Use this to show:

1. Maps from $S^1 = \int(\mathbb{R}/\mathbb{Z})$ to a discrete group $G$ correspond to maps from $\mathbb{R}/\mathbb{Z}$ to $\flat G$.
2. This classifies the homotopy classes of maps $S^1 \to BG$ (the monodromy of a flat connection on $S^1$).

**Exercise 2.4.** Define the *fundamental groupoid* of a cohesive type $A$ as:

$$\Pi_1(A) :\equiv \mathsf{hom}(\int A, \mathsf{Grpd})$$

Show that this recovers the expected fundamental groupoid. What is $\Pi_1(\mathbb{R}/\mathbb{Z})$?

**Exercise 2.5.** The Brouwer fixed-point theorem in cohesive HoTT:

1. State the theorem.
2. Identify the key step: where does $\pi_1(S^1) \neq 0$ enter the proof?
3. What cohesion axiom is needed? (Which modality, applied to what type?)

---

## Section 3: Differential Geometry

**Exercise 3.1.** In synthetic differential geometry, the infinitesimal interval is $D = \{x : \mathbb{R} \mid x^2 = 0\}$. 

1. What is a tangent vector at $a : M$ (where $M$ is a smooth type)?
2. Define the tangent bundle $TM$ as a type.
3. Why does this definition match the classical tangent bundle?

**Exercise 3.2.** Using the de Rham theorem in cohesive HoTT:

1. Compute $H^n_{dR}(S^1)$ (where $S^1$ is the smooth circle).
2. Compute $H^n_{dR}(\mathbb{R})$.
3. Compute $H^n_{dR}(T^2 = S^1 \times S^1)$.

*Hint:* Use $H^n_{dR}(A) \simeq H^n(\int A, \mathbb{R})$ and the known cohomology of these spaces.

**Exercise 3.3.** Define a *flat connection* on a principal $G$-bundle $P \to M$ in cohesive HoTT:

1. When is a connection $\nabla : M \to \mathbf{B}G_\nabla$ flat?
2. How does flatness relate to the flat modality $\flat$?
3. What is the holonomy of a flat connection?

**Exercise 3.4.** The Chern-Simons form for a $G$-connection on a 3-manifold $M$:

1. State what type $\mathsf{CS}(\nabla)$ has.
2. Explain why the Chern-Simons action is valued in $U(1)$ (not $\mathbb{R}$) for compact $M$.
3. How does the level quantization follow from the cohesion axioms?

---

## Section 4: Physics Applications

**Exercise 4.1.** The moduli stack of $G$-connections:

1. Define $\mathsf{Conn}_G(M)$ as a type in cohesive HoTT.
2. Identify the objects, morphisms, and 2-morphisms of this type (as an ∞-groupoid).
3. What is the $\pi_0$ of $\mathsf{Conn}_G(M)$? (The set of gauge-equivalence classes of connections.)

**Exercise 4.2.** A *gauge-invariant observable* is a real-valued function on the moduli stack of connections that is invariant under gauge transformations. Formulate this in cohesive HoTT:

1. What type does a gauge-invariant observable have?
2. How does the flat modality $\flat$ appear in this definition?
3. Give an example: the Yang-Mills functional $\int_M |F_\nabla|^2$.

**Exercise 4.3.** Higher gauge theory: a *2-bundle* (B-field) is a map $M \to \mathbf{B}^2 U(1)_\nabla$.

1. What is the field strength of a 2-bundle with connection?
2. What type does the field strength have?
3. How is the Bianchi identity $dH = 0$ expressed in cohesive HoTT?

**Exercise 4.4 (Research).** Read Schreiber-Shulman, "Quantum Gauge Field Theory in Cohesive Homotopy Type Theory" (2012):

1. How is the moduli stack of connections defined in their paper?
2. What role does the shape modality play in defining the path integral?
3. What is the "quantomorphism group" and how is it related to the flat modality?

**Exercise 4.5 (Brouwer).** Using the cohesion axioms, prove the following version of the Brouwer fixed-point theorem:

> Every endomorphism of $D^2$ in the cohesive ∞-topos has a fixed point.

*Outline:*
1. Show that a fixed-point-free endomorphism would give a retraction $r : D^2 \to S^1$.
2. Apply the shape modality: $\int r : \int D^2 \to \int S^1 = S^1$.
3. But $\int D^2 \simeq \mathbf{1}$, so $\int r$ is constant.
4. A retraction $\int D^2 \to S^1$ is impossible because a constant map is not a retraction.
5. Derive a contradiction.
