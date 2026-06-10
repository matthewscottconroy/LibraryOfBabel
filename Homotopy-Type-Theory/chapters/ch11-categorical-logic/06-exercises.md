# Exercises

---

**Exercise 11.1 (CCC Verification).** Show that $\mathbf{Set}$ is a cartesian closed category:

(a) Verify $\mathbf{Set}$ has all finite products.

(b) Construct the exponential $B^A$ (function set) and show it satisfies the universal property: there is a natural bijection $\mathsf{Hom}(C \times A, B) \cong \mathsf{Hom}(C, B^A)$.

(c) Identify the evaluation map $\mathsf{ev} : B^A \times A \to B$ and the currying map $\lambda : (C \times A \to B) \to (C \to B^A)$.

---

**Exercise 11.2 (Posets as CCCs).** The category $\mathbf{Pos}$ of posets and order-preserving maps:

(a) Is $\mathbf{Pos}$ a CCC? If so, describe the exponential $[A, B]$ (it should be the set of order-preserving maps with a natural ordering).

(b) In the poset of truth values $\{0, 1\}$ with $0 \leq 1$, what is $[A, B]$ for $A = B = \{0, 1\}$?

(c) How does the CCC structure of $\{0, 1\}$ (as a poset) correspond to classical propositional logic?

---

**Exercise 11.3 (Slice Categories).** Let $\mathcal{C} = \mathbf{Set}$ and $I = \{a, b\}$ (a two-element set).

(a) Describe the objects of $\mathbf{Set}/I$ explicitly (what are "type families over $I$"?).

(b) What is the terminal object of $\mathbf{Set}/I$?

(c) Compute the product $(A, f) \times_I (B, g)$ in $\mathbf{Set}/I$ (it should be the pullback over $I$).

(d) How does $\prod_{i \in I} A(i) \times B(i)$ relate to the Σ type $\sum_{i:I} A(i) \times B(i)$?

---

**Exercise 11.4 (Substitution as Pullback).** 

(a) In $\mathbf{Set}/I$, show that for a morphism $f : J \to I$ and an object $(A, p : A \to I)$, the pullback $f^*A = A \times_I J$ is the set $\{(a, j) \in A \times J \mid p(a) = f(j)\}$.

(b) Verify that this equals $A[f] = \{A(f(j)) \mid j \in J\}$ (substituting $f$ into the family $A$).

(c) Show that the functor $f^* : \mathbf{Set}/I \to \mathbf{Set}/J$ is left exact (preserves finite limits).

---

**Exercise 11.5 (Adjoint Triple Σ ⊣ Δ ⊣ Π).** For a function $f : I \to J$ in $\mathbf{Set}$:

(a) Define the left adjoint $\Sigma_f : \mathbf{Set}/I \to \mathbf{Set}/J$: it sends $(A, p)$ to $(\sum_{i:I} A(i), \text{projection to } J \text{ via } f)$.

(b) Define the right adjoint $\Pi_f : \mathbf{Set}/I \to \mathbf{Set}/J$: it sends $(A, p)$ to $(\prod_{i \in f^{-1}(j)} A(i), \ldots)$.

(c) Verify $\Sigma_f \dashv f^*$: give the natural bijection $\mathsf{Hom}(\Sigma_f X, Y) \cong \mathsf{Hom}(X, f^* Y)$.

(d) Verify $f^* \dashv \Pi_f$: give the natural bijection $\mathsf{Hom}(f^* Y, X) \cong \mathsf{Hom}(Y, \Pi_f X)$.

---

**Exercise 11.6 (Subobject Classifier).** 

(a) In $\mathbf{Set}$, verify that $\Omega = \{0, 1\}$ is a subobject classifier: show every subset $S \subseteq A$ has a unique characteristic morphism $\chi_S : A \to \Omega$ with $S = \chi_S^{-1}(1)$.

(b) In the presheaf category $[\mathcal{C}^{op}, \mathbf{Set}]$, the subobject classifier is $\Omega(c) = \{\text{sieves on } c\}$. For $\mathcal{C} = \mathbf{2} = \{0 \to 1\}$ (one morphism), describe $\Omega$ explicitly.

(c) In $\mathbf{Sh}(X)$ (sheaves on a topological space $X$), the subobject classifier is $\Omega(U) = \{V \text{ open} \mid V \subseteq U\}$. Describe the "propositions" in the internal logic of $\mathbf{Sh}(X)$.

---

**Exercise 11.7 (Groupoid Model).** The groupoid model of MLTT:

(a) Describe what "type $A$ in the groupoid model" means: $A$ is a groupoid, elements are objects, identity proofs are morphisms.

(b) Verify the J rule holds: given a property $P$ of groupoid morphisms that holds for identity morphisms $\mathsf{id}_a$, show it holds for all morphisms (this is a general property of groupoids).

(c) Compute $\mathsf{base} =_{\mathbf{B}\mathbb{Z}} \mathsf{base}$ (identity type in the groupoid $\mathbf{B}\mathbb{Z}$ with one object and automorphisms $\mathbb{Z}$).

(d) Explain why UIP fails: what is $p = q$ (identity type of identity types) for two different integers $p, q \in \mathbb{Z} = \mathsf{base} = \mathsf{base}$?

---

**Exercise 11.8 (Path Objects).** 

(a) In $\mathbf{Top}$, describe the path object $\mathsf{Path}(X) = X^{[0,1]}$ for a topological space $X$. What are its elements? What are the maps $s, t : \mathsf{Path}(X) \to X$?

(b) Verify the factorization $X \xrightarrow{r} \mathsf{Path}(X) \xrightarrow{(s,t)} X \times X$ (where $r$ sends $x$ to the constant path at $x$).

(c) Show that the J rule for identity types corresponds to the homotopy lifting property: any homotopy (path of maps) that starts at the identity can be extended.

---

**Exercise 11.9 (Univalence Preview).** The Univalence Axiom in the simplicial model:

(a) Informally: what does it mean for a path in the universe (a 1-simplex in the universe Kan complex) to be the same as an equivalence of types?

(b) If $A$ and $B$ are Kan complexes related by a homotopy equivalence $e : A \simeq B$, explain why $e$ determines a path $\tilde{e} : A = B$ in the universe.

(c) Conversely, if $p : A = B$ is a path in the universe (a homotopy equivalence between the classifying spaces), explain why $p$ gives an equivalence $A \simeq B$.

---

**Exercise 11.10 (Research: The HoTT Book's Models).** The HoTT Book (Chapter 2) introduces several models:

(a) The *set model*: types are sets, equalities are equality in sets. What axioms hold? What fails?

(b) The *groupoid model*: types are groupoids. What axioms hold? Does Univalence hold? (Hint: Univalence would say isomorphisms of groupoids = equalities in the "universe of groupoids" — what is that universe?)

(c) The *simplicial set model*: types are Kan complexes. What is the status of Univalence, LEM, AC, and UIP in this model?

(d) For each model, identify which HoTT theorems it validates and which it fails to validate. What does this tell you about the independence of various axioms?
