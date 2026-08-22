# Type Formers: The Full FIEC System

## The Pattern

Every type former in MLTT follows the same four-step structure:

1. **Formation:** When is this type well-formed? What are its prerequisites?
2. **Introduction:** How do you construct an element of this type? (The constructors.)
3. **Elimination:** How do you use an element of this type? (The recursor/inductor.)
4. **Computation:** What does the eliminator do when applied to the introduction forms?

This is the FIEC pattern (Formation-Introduction-Elimination-Computation), sometimes called the FIER pattern (with R for reduction). It is the universal template for type-theoretic definitions. If you understand FIEC for Π and Σ types, you understand the structure of every type former in the system.

Here we collect the rules for all the type formers of MLTT. We use Γ for contexts and suppress universe subscripts where they do not matter.

## Π Types (Dependent Function Types)

**Formation:**
$$\frac{\Gamma \vdash A\ \mathsf{type} \quad \Gamma, x : A \vdash B(x)\ \mathsf{type}}{\Gamma \vdash \prod_{x:A} B(x)\ \mathsf{type}}$$

**Introduction:**
$$\frac{\Gamma, x : A \vdash t : B(x)}{\Gamma \vdash \lambda x.\, t : \prod_{x:A} B(x)}$$

**Elimination:**
$$\frac{\Gamma \vdash f : \prod_{x:A} B(x) \quad \Gamma \vdash a : A}{\Gamma \vdash f\, a : B(a)}$$

**Computation (β):**
$$(\lambda x.\, t)\, a \equiv t[a/x] : B(a)$$

**Uniqueness (η):**
$$f \equiv \lambda x.\, f\, x : \prod_{x:A} B(x)$$

## Σ Types (Dependent Pair Types)

**Formation:**
$$\frac{\Gamma \vdash A\ \mathsf{type} \quad \Gamma, x : A \vdash B(x)\ \mathsf{type}}{\Gamma \vdash \sum_{x:A} B(x)\ \mathsf{type}}$$

**Introduction:**
$$\frac{\Gamma \vdash a : A \quad \Gamma \vdash b : B(a)}{\Gamma \vdash (a, b) : \sum_{x:A} B(x)}$$

**Elimination (projections):**
$$\frac{\Gamma \vdash p : \sum_{x:A} B(x)}{\Gamma \vdash \mathsf{fst}(p) : A} \qquad \frac{\Gamma \vdash p : \sum_{x:A} B(x)}{\Gamma \vdash \mathsf{snd}(p) : B(\mathsf{fst}(p))}$$

**Computation:**
$$\mathsf{fst}(a, b) \equiv a \qquad \mathsf{snd}(a, b) \equiv b$$

**General elimination (pattern matching):**
$$\frac{\Gamma \vdash C : \sum_{x:A} B(x) \to \mathsf{Type} \quad \Gamma \vdash f : \prod_{a:A}\prod_{b:B(a)} C((a,b)) \quad \Gamma \vdash p : \sum_{x:A} B(x)}{\Gamma \vdash \mathsf{ind}_\Sigma(C, f, p) : C(p)}$$
Computation: ind_Σ(C, f, (a, b)) ≡ f(a)(b).

## Coproduct Types (Sum Types, +)

**Formation:**
$$\frac{\Gamma \vdash A\ \mathsf{type} \quad \Gamma \vdash B\ \mathsf{type}}{\Gamma \vdash A + B\ \mathsf{type}}$$

**Introduction:**
$$\frac{\Gamma \vdash a : A}{\Gamma \vdash \mathsf{inl}(a) : A + B} \qquad \frac{\Gamma \vdash b : B}{\Gamma \vdash \mathsf{inr}(b) : A + B}$$

**Elimination:**
$$\frac{\Gamma \vdash C : A + B \to \mathsf{Type} \quad \Gamma \vdash f : \prod_{a:A} C(\mathsf{inl}(a)) \quad \Gamma \vdash g : \prod_{b:B} C(\mathsf{inr}(b)) \quad \Gamma \vdash s : A + B}{\Gamma \vdash \mathsf{ind}_+(C, f, g, s) : C(s)}$$

**Computation:**
$$\mathsf{ind}_+(C, f, g, \mathsf{inl}(a)) \equiv f(a) \qquad \mathsf{ind}_+(C, f, g, \mathsf{inr}(b)) \equiv g(b)$$

## Unit Type (𝟙)

**Formation:** Γ ⊢ 𝟙 type (no premises; 𝟙 is always a type)

**Introduction:** Γ ⊢ ⋆ : 𝟙 (the unique element)

**Elimination:**
$$\frac{\Gamma \vdash C : \mathbf{1} \to \mathsf{Type} \quad \Gamma \vdash c : C(\star) \quad \Gamma \vdash u : \mathbf{1}}{\Gamma \vdash \mathsf{ind}_\mathbf{1}(C, c, u) : C(u)}$$

**Computation:** ind_𝟙(C, c, ⋆) ≡ c. (And u ≡ ⋆ for any u : 𝟙 — this is the uniqueness of 𝟙.)

## Empty Type (𝟘)

**Formation:** Γ ⊢ 𝟘 type

**Introduction:** (none — there are no constructors)

**Elimination (ex falso):**
$$\frac{\Gamma \vdash C : \mathbf{0} \to \mathsf{Type} \quad \Gamma \vdash n : \mathbf{0}}{\Gamma \vdash \mathsf{ind}_\mathbf{0}(C, n) : C(n)}$$

**Computation:** (none — there are no constructors to compute on)

The elimination rule says: from an element of 𝟘 (the empty type, an element that does not exist), you can prove anything. This is the principle of explosion (ex falso quodlibet): ¬P is defined as P → 𝟘, so from a proof of ¬P and a proof of P, apply the proof of ¬P to the proof of P to get an element of 𝟘, then apply ind_𝟘 to get whatever you want.

## Natural Numbers (ℕ)

**Formation:** Γ ⊢ ℕ type

**Introduction:**
$$\Gamma \vdash \mathsf{zero} : \mathbb{N} \qquad \frac{\Gamma \vdash n : \mathbb{N}}{\Gamma \vdash \mathsf{succ}(n) : \mathbb{N}}$$

**Elimination:**
$$\frac{\Gamma \vdash C : \mathbb{N} \to \mathsf{Type} \quad \Gamma \vdash c_z : C(\mathsf{zero}) \quad \Gamma \vdash c_s : \prod_{n:\mathbb{N}} C(n) \to C(\mathsf{succ}(n)) \quad \Gamma \vdash n : \mathbb{N}}{\Gamma \vdash \mathsf{ind}_{\mathbb{N}}(C, c_z, c_s, n) : C(n)}$$

**Computation:**
$$\mathsf{ind}_{\mathbb{N}}(C, c_z, c_s, \mathsf{zero}) \equiv c_z$$
$$\mathsf{ind}_{\mathbb{N}}(C, c_z, c_s, \mathsf{succ}(n)) \equiv c_s(n,\, \mathsf{ind}_{\mathbb{N}}(C, c_z, c_s, n))$$

## W-Types (Well-Founded Trees)

**Formation:**
$$\frac{\Gamma \vdash A\ \mathsf{type} \quad \Gamma, x : A \vdash B(x)\ \mathsf{type}}{\Gamma \vdash W_{x:A} B(x)\ \mathsf{type}}$$

**Introduction:**
$$\frac{\Gamma \vdash a : A \quad \Gamma \vdash f : B(a) \to W_{x:A} B(x)}{\Gamma \vdash \mathsf{sup}(a, f) : W_{x:A} B(x)}$$

**Elimination:**
$$\frac{\Gamma \vdash C : W_{x:A} B(x) \to \mathsf{Type} \quad \Gamma \vdash g : \prod_{a:A}\prod_{f:B(a)\to W} \left(\prod_{b:B(a)} C(f(b))\right) \to C(\mathsf{sup}(a,f)) \quad \Gamma \vdash w : W}{\Gamma \vdash \mathsf{ind}_W(C, g, w) : C(w)}$$

where W abbreviates W_{x:A}B(x) for readability.

**Computation:** ind_W(C, g, sup(a, f)) ≡ g(a, f, λb. ind_W(C, g, f(b))).

## The Universe

**Formation:** For each universe level i, the universe Type_i is a type:
$$\Gamma \vdash \mathsf{Type}_i : \mathsf{Type}_{i+1}$$

**Introduction:** The elements of Type_i are the "small types" — types definable using formation rules for types in Type_i. Each type former yields a code in the appropriate universe:
- If A : Type_i and B : A → Type_i, then Π(x:A).B(x) : Type_i
- Similarly for Σ, +, 𝟙, 𝟘, ℕ, and W (all at level i, staying in Type_i)

**Cumulativity:** If A : Type_i, then A : Type_{i+1}.

**Elimination:** Since Type_i is itself a type, it has elements (namely, types at level i), and those elements can be used in type families, Π types, Σ types, etc. The universe is what allows us to quantify over all small types.

## The Big Picture

The rules above constitute the *inference system* of MLTT. A proof (or program) in MLTT is a sequence of applications of these rules, starting from structural rules (variable, weakening, substitution) and building up type judgments and term judgments step by step.

In practice, nobody writes out formal derivation trees. Proof assistants like Agda and Lean infer most of the structure automatically — you write the high-level proof term, and the system verifies that the full formal derivation exists. But the formal derivation is what gives the system its rigor.

The claim of MLTT as a foundation for mathematics is precisely that every mathematical argument can be rendered as a term in this system, and the verification of that argument is the type-checking of the term.

## What Is Not in This List

The rules above cover "basic" MLTT. Several important additions are not here:

- **Identity types** (Section 3): the J rule, which requires its own FIEC treatment
- **Higher Inductive Types** (Unit 05): inductive types with path constructors
- **Univalence** (Unit 04): an axiom about the identity type of the universe
- **Propositional truncation** (Unit 05): a higher inductive type collapsing all proofs to one

These additions are what distinguish HoTT from plain MLTT. Plain MLTT, as described here, is the ground. Everything else is built on it.
