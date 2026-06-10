# Exercises — Chapter 23: Cubical Type Theory

## Section 1: The Interval and Paths

**Exercise 1.** Verify the De Morgan laws for the CCHM interval. For dimension variables $i, j : \mathbb{I}$, show that the axioms $\sim(i \wedge j) = \sim i \vee \sim j$ and $\sim(i \vee j) = \sim i \wedge \sim j$ hold. Then verify that $i \wedge \sim i = 0$ and $i \vee \sim i = 1$. (These are axioms of the system, so "showing" them means stating them precisely and explaining why they are the right axioms for the intended interpretation.)

**Exercise 2.** Define the following path operations using only the interval algebra (complement, meet, join) — no `hcomp` or `transp` required:
- (a) Path reversal: `sym : (a =_A b) → (b =_A a)`
- (b) The action of a function on paths: `ap : (f : A → B) → (a =_A b) → (f a =_B f b)`
- (c) The dependent action: `apd : (f : (x : A) → B x) → (p : a =_A b) → PathP (λ i → B (p i)) (f a) (f b)`

**Exercise 3.** In Cubical Agda, open the `Cubical.Foundations.Prelude` module and locate the definitions of `sym`, `cong`, and `congP`. Identify which interval operations each definition uses. Write a brief explanation of why each uses the specific operations it does.

**Exercise 4.** Explain why the CCHM interval cannot be a *set* in the homotopy-theoretic sense (an h-set with decidable equality). In particular: if the interval had decidable equality, what would that imply about paths in any type? What goes wrong with univalence if the interval is an h-set?

**Exercise 5.** Define the 2-dimensional path concatenation (whiskering): given paths $p : a =_A b$, $q : a =_A b$, $r : b =_A c$, and a homotopy $H : p \sim q$, construct the homotopy $r \circ H : r \cdot p \sim r \cdot q$ (right whiskering) as an explicit element of $\mathbb{I}^2 \to A$.

**Exercise 6.** Show that function extensionality holds in cubical type theory by writing out the explicit proof:
$$\mathsf{funExt}(h) :\equiv \lambda i. \lambda x. h(x)(i) : f =_{\Pi_{x:A} B(x)} g$$
Verify both endpoints definitionally and check that the computation rule $\mathsf{funExt}(h)(i)(x) = h(x)(i)$ holds by $\beta$-reduction.

## Section 2: Composition and Transport

**Exercise 7.** Write the explicit `hcomp` expression for path concatenation $p \cdot q$ where $p : a =_A b$ and $q : b =_A c$. Show in detail how it gives the correct endpoints:
- At $i = 0$: the result is $b$ (following from the face condition)
- At $i = 1$: the result is $c$ (following from the base being $q(1)$ with empty tube)

**Exercise 8.** The *loop concatenation* on $\Omega(A, a) = (a =_A a)$ should form a group. In cubical type theory, verify the following:
- (a) $\text{refl} \cdot p \sim p$ (left unit): exhibit the explicit 2-cube $\lambda i\, j. \, p(i \wedge j)$ and check all four faces.
- (b) $p \cdot \text{refl} \sim p$ (right unit): exhibit the analogous 2-cube using join.
- (c) Are either of these definitional equalities or only propositional? Explain.

**Exercise 9.** Compute `transp` for the product type $A \times B$. Show that:
$$\mathsf{transp}^{\lambda i. A(i) \times B(i)}_\phi((a_0, b_0)) = (\mathsf{transp}^A_\phi(a_0), \mathsf{transp}^B_\phi(b_0))$$
Verify that this is consistent with the $\phi = 1$ triviality rule.

**Exercise 10.** In Cubical Agda, prove the "square lemma": if $p : a =_A b$ and $q : a =_A c$ and $r : b =_A d$ and $s : c =_A d$, and there exists a 2-cube $H$ with those paths as faces, then $r \cdot p = s \cdot q$ (as a path in $A$). Write the Agda proof.

**Exercise 11.** Show that in a type $A$, the *Eckmann-Hilton argument* holds: for loops $\alpha, \beta : \Omega^2(A, a)$, horizontal and vertical composition commute, $\alpha \star \beta = \beta \star \alpha$ (and both equal $\alpha \cdot \beta$). Use the explicit cubical witnesses.

**Exercise 12.** Prove associativity of path concatenation: $(p \cdot q) \cdot r \sim p \cdot (q \cdot r)$. Exhibit an explicit 3-cube (a term $\mathbb{I}^3 \to A$) that witnesses this homotopy.

**Exercise 13.** In Cubical Agda, import `Cubical.Foundations.GroupoidLaws` and study the proof of `assoc`. How does it use `hcomp`? Compare the cubical proof to the J-induction proof of associativity in Book HoTT.

## Section 3: The Glue Type and Univalence

**Exercise 14.** Write out the type of the Glue constructor `glue` in full detail, including all the face conditions and coherence requirements. Then write the type of `unglue`. Verify the three computation rules:
- $\mathsf{Glue}[1 \vdash (T, e)]\, B = T$
- $\mathsf{glue}[1 \vdash t]\, b = t$  
- $\mathsf{unglue}[1 \vdash e](t) = e(t)$

**Exercise 15.** Using the Glue type, define:
$$\mathsf{ua}(e : A \simeq B) :\equiv \lambda i.\; \mathsf{Glue}[(i=0) \mapsto (A,e),\; (i=1) \mapsto (B, \mathsf{id}_B)]\, B$$
Verify:
- (a) $\mathsf{ua}(e)(0) = A$ definitionally
- (b) $\mathsf{ua}(e)(1) = B$ definitionally
- (c) $\mathsf{ua}(\mathsf{id}_A) = \mathsf{refl}_A$ (requires reasoning about Glue with identity equivalence)

**Exercise 16.** The computation rule for `ua` states that `transport (ua e) a = e a`. Trace through the computation for a specific example: let $A = B = \mathbb{N}$ and $e = \mathsf{succEquiv}$ (the successor equivalence on $\mathbb{N}$). Show step-by-step how `transport (ua succEquiv) 3` reduces to `4`. At each step, identify which computation rule (Glue computation, transp rule for Glue, etc.) applies.

**Exercise 17.** Prove propositional extensionality (propext) from univalence: for propositions $P, Q : \mathsf{Prop}$, if $P \leftrightarrow Q$, then $P = Q$. Explicitly:
- (a) Show that a bi-implication $(h_1 : P \to Q, h_2 : Q \to P)$ gives an equivalence $P \simeq Q$.
- (b) Apply `ua` to this equivalence.
- (c) Why is the coherence condition for `ua` trivially satisfied for propositions?

**Exercise 18.** In Cubical Agda, import `Cubical.Foundations.Univalence` and locate the proof of `uaβ` (the β-rule for ua: `transport (ua e) a ≡ e .fst a`). Read the proof and explain each step.

**Exercise 19.** Prove that $\mathsf{ua}(\mathsf{idEquiv}_A) = \mathsf{refl}_A$ in cubical type theory. What property of the Glue type does this use? (This is one of the two conditions needed to show that `ua` and `idToEquiv` are mutual inverses.)

**Exercise 20.** State and prove the *embedding condition*: for any equivalence $e : A \simeq B$ and elements $a_1, a_2 : A$, the function $\mathsf{ap}_{\mathsf{ua}(e)} : (a_1 = a_2) \to (\mathsf{ua}(e) \cdot a_1 = \mathsf{ua}(e) \cdot a_2)$ corresponds, via the computation rule, to $\mathsf{ap}_e : (a_1 = a_2) \to (e(a_1) = e(a_2))$.

## Section 4: Variations and Metatheory

**Exercise 21.** In Cartesian cubical type theory (without complement), path reversal is not definitional. Write the `hcomp`-based construction of `sym p` for $p : a =_A b$. Show that it has the correct endpoints and compare the number of composition operations needed with the CCHM definition.

**Exercise 22.** Explain the boundary separation principle in XTT with a concrete example. Specifically:
- (a) Give two terms $t, s : \Pi_{i:\mathbb{I}} A$ that agree on all faces ($t(0) = s(0)$ and $t(1) = s(1)$) but are *not* definitionally equal in CCHM.
- (b) Explain why boundary separation in XTT would make them definitionally equal.
- (c) Give one advantage and one disadvantage of this.

**Exercise 23.** The *normalization by evaluation* algorithm for cubical type theory needs to handle neutral `hcomp` terms — compositions where the type is a free variable. Describe the shape of a neutral `hcomp` term and explain why it cannot be further reduced. What is its normal form?

**Exercise 24.** In 2LTT, there is a functor $\iota$ from fibrant types (inner level) to strict types (outer level). Show that:
- (a) If $A$ is fibrant and $B$ is strict, there is a "restriction" map from strict functions $\iota(A) \to B$ to fibrant functions $A \to B'$ for some fibrant $B'$.
- (b) Give an example of a strict type that is not the image of any fibrant type.

**Exercise 25.** Cubical Agda's `--safe` flag disallows `trustMe` and unsafe axioms. Determine whether the following can be proved in safe Cubical Agda (without postulates):
- (a) Univalence (for types in all universes)
- (b) Propositional truncation for arbitrary types
- (c) The Brunerie number is $-2$
- (d) The Seifert-van Kampen theorem
For each, identify the key cubical primitive that makes it provable.

**Exercise 26.** The *Brunerie number* is defined as a term $n : \mathbb{Z}$ such that $\pi_4(S^3) \cong \mathbb{Z}/n$. In Cubical Agda, the computation of this number requires evaluating `transp` through `ua` multiple times. Sketch (without full detail) how the evaluation of `transport (ua e) a` would reduce for a simple case, and explain what kind of normalization the Brunerie computation relies on.

**Exercise 27.** (Advanced) Prove that in CCHM cubical type theory, the *type of equivalences* $A \simeq B$ is equivalent to the *type of paths in the universe* $A =_{\mathsf{Type}} B$. Your proof should use `ua` and `idToEquiv` and show they are mutual quasi-inverses. Identify which steps use the computation rules for Glue and which use the $\eta$-rule for equivalences.

**Exercise 28.** (Advanced) In Cubical Agda, formalize the statement that the circle $S^1$ (defined as a HIT with one point `base` and one loop `loop : base = base`) has $\pi_1(S^1) \cong \mathbb{Z}$. Outline the proof strategy using the universal cover (the real line winding around the circle) and identify which cubical primitives are used at each step.

**Exercise 29.** (Research-level) The problem of combining cubical type theory (undirected paths) with simplicial type theory (directed morphisms) is open. Propose a design for a type theory with both intervals $\mathbb{I}$ (De Morgan) and $\mathbf{2}$ (directed, no complement). What would the type formers look like? What would composition in the universe need to provide?

**Exercise 30.** (Proof assistant project) In Cubical Agda, import `Cubical.HITs.Pushout` and study the pushout HIT. Then:
- (a) Define the suspension $\Sigma A$ as a pushout of $A \leftarrow A \rightarrow A$ (the two maps being the identity).
- (b) Show that $\Sigma \mathsf{Bool} \simeq S^1$.
- (c) Compute the fundamental group of $\Sigma \mathsf{Bool}$ using the `windingNumber` map.
