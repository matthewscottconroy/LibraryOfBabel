# Exercises: Category Theory

## Section 1: Categories and Functors

**Exercise 1.** Verify that the following are categories (check all axioms):

(a) $\mathbf{Set}$: sets and functions.
(b) A preorder $(P, \leq)$ viewed as a category.
(c) A monoid $(M, \cdot, e)$ viewed as a one-object category $\mathbf{B}M$.
(d) The category $\mathbf{Type}$ of types in MLTT with functions as morphisms.

**Exercise 2.** For each of the following, either construct the required category or show it doesn't exist:

(a) A category with exactly two objects and exactly three morphisms (including identities).
(b) A category with exactly two objects and exactly four morphisms.
(c) A category with exactly three objects and exactly three morphisms.

**Exercise 3.** Describe the opposite category $\mathcal{C}^{op}$ for each of the following:

(a) $\mathbf{Set}^{op}$: what are the morphisms?
(b) The opposite of a preorder $(P, \leq)$: what preorder is it?
(c) The opposite of a monoid $\mathbf{B}M$: what monoid is it?

**Exercise 4.** Prove that isomorphisms in a category are unique: if $g$ and $g'$ are both inverses of $f : A \to B$, then $g = g'$.

**Exercise 5.** Let $F : \mathcal{C} \to \mathcal{D}$ and $G : \mathcal{D} \to \mathcal{E}$ be functors. Show that the composite $G \circ F$ is a functor. What are its action on objects and morphisms?

**Exercise 6.** A functor $F : \mathbf{B}G \to \mathbf{Set}$ (where $G$ is a group viewed as a one-object category) corresponds to what classical mathematical structure? What is a natural transformation between two such functors?

**Exercise 7.** Show that the identity functor $\mathsf{Id}_\mathcal{C} : \mathcal{C} \to \mathcal{C}$ and the composite of functors give $\mathbf{Cat}$ (small categories and functors) the structure of a category.

## Section 2: Natural Transformations and Yoneda

**Exercise 8.** Let $F, G : \mathcal{C} \to \mathcal{D}$ be functors and $\alpha : F \Rightarrow G$ a natural transformation. Prove that the components $(\alpha_A)$ can be assembled into a functor from $\mathcal{C}$ to the "arrow category" of $\mathcal{D}$ (whose objects are morphisms of $\mathcal{D}$).

**Exercise 9.** Verify that vertical composition of natural transformations is associative and that the identity natural transformation is a unit.

**Exercise 10.** In the functor category $[\mathcal{C}, \mathcal{D}]$, what is the identity natural transformation? What is the composition of two natural transformations?

**Exercise 11.** The Yoneda Lemma: State and prove the bijection $\mathsf{Nat}(\mathsf{Hom}(A, -), F) \cong F(A)$ for a functor $F : \mathcal{C} \to \mathbf{Set}$ and object $A$. Verify the bijection is natural in both $A$ and $F$.

**Exercise 12.** Use the Yoneda Lemma to prove the Yoneda embedding $\mathsf{y} : \mathcal{C} \to [\mathcal{C}^{op}, \mathbf{Set}]$ is fully faithful.

**Exercise 13.** Let $\mathcal{C} = \mathbf{B}G$ for a group $G$. Describe the Yoneda embedding $\mathsf{y} : \mathbf{B}G \to [\mathbf{B}G^{op}, \mathbf{Set}]$ explicitly. What representation of $G$ does it produce? What is the classical name of this representation?

**Exercise 14.** A functor $F : \mathcal{C} \to \mathbf{Set}$ is *representable* if there exists $A \in \mathcal{C}$ with $F \cong \mathsf{Hom}(A, -)$. Show that if $F$ is representable, the representing object $A$ is unique up to unique isomorphism.

## Section 3: Limits and Colimits

**Exercise 15.** In a preorder category $(P, \leq)$:

(a) What is a product of objects $a$ and $b$?
(b) What is a coproduct of $a$ and $b$?
(c) What is a terminal object?
(d) What is an initial object?

**Exercise 16.** In $\mathbf{Set}$:

(a) Compute the pullback of $f : A \to C$ and $g : B \to C$.
(b) Compute the pushout of $f : C \to A$ and $g : C \to B$.
(c) Compute the equalizer of $f, g : A \rightrightarrows B$.
(d) Compute the coequalizer of $f, g : A \rightrightarrows B$.

**Exercise 17.** Prove that limits in $\mathcal{C}$ correspond to limits in $[\mathcal{J}, \mathcal{C}]$ (functor categories). More precisely: a cone over $D : \mathcal{J} \to \mathcal{C}$ with vertex $C$ is a natural transformation from $\Delta_C$ to $D$, and the limit is the representing object.

**Exercise 18.** Prove that in a category with all products and equalizers, all finite limits exist. (Hint: express a general limit as an equalizer of products.)

**Exercise 19.** In type theory, the pullback of $f : A \to C$ and $g : B \to C$ is the type $\sum_{a:A} \sum_{b:B} (f(a) = g(b))$. Write out the universal property of this type and verify it matches the categorical definition of a pullback.

**Exercise 20.** The pushout of $f : C \to A$ and $g : C \to B$ (as a HIT) has constructors $\mathsf{inl} : A \to P$, $\mathsf{inr} : B \to P$, and $\mathsf{glue} : \prod_{c:C} (\mathsf{inl}(f(c)) = \mathsf{inr}(g(c)))$. Write the eliminator for this HIT and verify it satisfies the universal property of the pushout.

## Section 4: Adjunctions

**Exercise 21.** Verify the triangular identities for the free-forgetful adjunction $F \dashv U : \mathbf{Grp} \to \mathbf{Set}$. Compute the unit $\eta_S : S \to U(F(S))$ and counit $\varepsilon_G : F(U(G)) \to G$ explicitly.

**Exercise 22.** Show that $f : A \to G(B)$ and its transpose $\hat{f} : F(A) \to B$ satisfy the adjunction equations. (Given $F \dashv G$ and $f : A \to G(B)$, define $\hat{f} = \varepsilon_B \circ F(f)$ and show this gives the bijection $\mathsf{Hom}(A, G(B)) \cong \mathsf{Hom}(F(A), B)$.)

**Exercise 23.** Let $F \dashv G$. Prove that $G$ preserves the terminal object (if it exists in $\mathcal{D}$). What does $G$ send the terminal object of $\mathcal{D}$ to?

**Exercise 24.** Prove that right adjoints preserve pullbacks. (Use the general theorem that right adjoints preserve limits.)

**Exercise 25.** In type theory: the currying equivalence $(A \times B \to C) \simeq (A \to (B \to C))$ is an adjunction. Identify the functors, the unit, and the counit. Verify the triangular identities.

**Exercise 26.** Show that propositional truncation $\|-\| : \mathcal{U} \to \mathsf{Prop}$ is left adjoint to the inclusion $\iota : \mathsf{Prop} \hookrightarrow \mathcal{U}$ by verifying the hom-set bijection: $\mathsf{Hom}_\mathsf{Prop}(\|A\|, P) \simeq \mathsf{Hom}_\mathcal{U}(A, P)$ for any proposition $P$.

## Section 5: Monads

**Exercise 27.** Let $(T, \eta, \mu)$ be a monad on $\mathcal{C}$. Verify the monad laws for the monad arising from the adjunction $F \dashv U : \mathbf{Grp} \to \mathbf{Set}$ (the "free group monad" on $\mathbf{Set}$).

**Exercise 28.** A $T$-algebra for the "maybe monad" $M(A) = A + \{*\}$ (on $\mathbf{Set}$) is a pointed set: a set $A$ with a distinguished element $a_0 : A$ (the image of $*$). Verify this directly from the definition of a $T$-algebra.

**Exercise 29.** Prove that any monad $(T, \eta, \mu)$ on $\mathcal{C}$ arises from the adjunction $F^T \dashv U^T$ between $\mathcal{C}$ and the Eilenberg-Moore category $\mathcal{C}^T$.

## Proof-Level Exercises

**Exercise 30.** Prove the Yoneda Lemma in full generality: the bijection $\mathsf{Nat}(\mathsf{Hom}(A, -), F) \cong F(A)$ is natural in both $A$ and $F$. (Naturality in $A$: for $h : A' \to A$, the bijection commutes with precomposition by $\mathsf{Hom}(h, -)$. Naturality in $F$: for a natural transformation $\phi : F \Rightarrow G$, the bijection commutes with post-composition.)

**Exercise 31.** Prove the General Adjoint Functor Theorem: a functor $G : \mathcal{D} \to \mathcal{C}$ has a left adjoint if and only if $G$ preserves all small limits and satisfies the solution set condition. (Hint for the forward direction: construct the left adjoint as the initial object of a certain comma category.)

**Exercise 32.** Prove that equivalences of categories satisfy the "two-out-of-three property": if $F : \mathcal{C} \to \mathcal{D}$ and $G : \mathcal{D} \to \mathcal{E}$ are functors and any two of $\{F, G, G \circ F\}$ are equivalences, then so is the third. Compare this to the analogous property for weak equivalences of topological spaces.
