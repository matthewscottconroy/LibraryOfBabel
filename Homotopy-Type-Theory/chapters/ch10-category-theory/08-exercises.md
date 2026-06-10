# Exercises

---

**Exercise 10.1 (Category Verification).** Verify that the following are categories by checking all axioms:

(a) **Matrices:** Objects are natural numbers; $\mathsf{Hom}(m, n)$ = set of $n \times m$ matrices over $\mathbb{R}$; composition is matrix multiplication; identity at $n$ is the $n \times n$ identity matrix.

(b) **Relations:** Objects are sets; $\mathsf{Hom}(A, B)$ = set of relations $R \subseteq A \times B$; composition: $(R; S)(a, c) = \exists b. R(a,b) \land S(b,c)$; identity is the equality relation.

(c) **Partial functions:** Objects are sets; $\mathsf{Hom}(A, B)$ = partial functions $A \rightharpoonup B$; composition is sequential composition of partial functions.

---

**Exercise 10.2 (Functor Axioms).** Verify that each of the following is a functor:

(a) $\mathsf{Hom}(A, -) : \mathcal{C} \to \mathbf{Set}$ for fixed $A \in \mathcal{C}$.

(b) The power set functor $\mathcal{P} : \mathbf{Set} \to \mathbf{Set}$, $A \mapsto \mathcal{P}(A)$, $f \mapsto f_*$ (direct image).

(c) The contravariant power set functor $\mathcal{P}^{op} : \mathbf{Set}^{op} \to \mathbf{Set}$, $A \mapsto \mathcal{P}(A)$, $f \mapsto f^{-1}$ (preimage). Why does this require $\mathbf{Set}^{op}$ rather than $\mathbf{Set}$?

---

**Exercise 10.3 (Yoneda in Detail).** Prove the Yoneda Lemma.

(a) Define the bijection $\Phi : [\mathcal{C}^{op}, \mathbf{Set}](\mathsf{Hom}(-, A), F) \to F(A)$.
(b) Define the inverse $\Psi : F(A) \to [\mathcal{C}^{op}, \mathbf{Set}](\mathsf{Hom}(-, A), F)$.
(c) Verify $\Phi \circ \Psi = \mathsf{id}$.
(d) Verify $\Psi \circ \Phi = \mathsf{id}$ using naturality of $\alpha$.

---

**Exercise 10.4 (Universal Properties).** For each construction, state the universal property and prove uniqueness up to isomorphism:

(a) Terminal object.

(b) Product $A \times B$.

(c) Equalizer of $f, g : A \to B$.

---

**Exercise 10.5 (Pullbacks).** 

(a) Show that in $\mathbf{Set}$, the pullback of $f : A \to C$ and $g : B \to C$ is $\{(a, b) \in A \times B \mid f(a) = g(b)\}$.

(b) In type theory, the pullback is $\sum_{a:A} \sum_{b:B} f(a) = g(b)$. Verify the universal property.

(c) Show that a pullback of a monomorphism along any morphism is a monomorphism.

(d) Prove: a morphism $f : A \to B$ is a monomorphism if and only if the square
$$\begin{array}{ccc} A & \xrightarrow{\mathsf{id}} & A \\ \mathsf{id}\downarrow & & \downarrow f \\ A & \xrightarrow{f} & B \end{array}$$
is a pullback.

---

**Exercise 10.6 (Adjunctions).** 

(a) Show that in $\mathbf{Set}$, $(-) \times A \dashv [A, -]$ (product is left adjoint to exponential).

(b) Verify the triangular identities for this adjunction.

(c) The *curry* and *uncurry* maps: identify the unit and counit.

---

**Exercise 10.7 (Left Adjoints Preserve Colimits).** 

(a) Prove that if $F \dashv G$, then $F$ preserves coproducts: $F(A + B) \cong F(A) + F(B)$.

(b) Use this to show $F(S \sqcup T) \cong F(S) * F(T)$ for the free group functor $F$ (free product of free groups is the free group on the disjoint union).

(c) Use this to show $(A + B) \times C \cong (A \times C) + (B \times C)$ in $\mathbf{Set}$ (and in type theory).

---

**Exercise 10.8 (Monads).** 

(a) Verify that the triple $(T, \eta, \mu)$ defined by the free group adjunction satisfies the monad axioms.

(b) Define the Kleisli category for the Maybe monad ($T(A) = A + \{*\}$). Describe what the morphisms are and how they compose.

(c) Show that a monoid $(M, \cdot, e)$ determines a monad on the one-object category $\mathbf{1}$ (the category with one object and one morphism). What are $T$, $\eta$, and $\mu$ in this case?

---

**Exercise 10.9 (CCC Internal Language).** For a CCC $\mathcal{C}$:

(a) Define what it means for $\mathcal{C}$ to be a CCC (give the precise definition with terminal object, products, and exponentials).

(b) Interpret STLC terms in $\mathcal{C}$: what is $\lambda x : A. t$ interpreted as? What is function application?

(c) Show that the $\beta$-rule ($(\lambda x. t)\, a = t[a/x]$) corresponds to the universal property of the exponential.

---

**Exercise 10.10 (Categorical Semantics in Lean 4).** 

Lean 4's `CategoryTheory` library formalizes category theory. Investigate:

(a) What is the type `Category C` in Lean 4? What are `Hom`, `id`, and `comp`?

(b) Find the definition of `Functor` in Lean 4. What data does it contain? What axioms?

(c) State the Yoneda Lemma (`yoneda_sections_small` or similar) as it appears in Mathlib. What are the types involved?

(d) Find the definition of `Adjunction F G` in Mathlib. How are the unit, counit, and triangular identities formalized?

---

**Exercise 10.11 (Research: $\infty$-Toposes and HoTT).** 

(a) Look up Lurie's definition of an $\infty$-topos. What are the key axioms?

(b) Explain in your own words why the $\infty$-topos semantics is the "right" semantics for HoTT, while an ordinary topos (1-categorical) is not.

(c) What additional structure does an $\infty$-topos have compared to a Quillen model category? Why does this additional structure model Univalence?

(d) The Homotopy Hypothesis (Grothendieck 1983, Lurie 2009): $\infty$-groupoids are the same as homotopy types. Explain how this hypothesis is relevant to the connection between HoTT and $\infty$-toposes.
