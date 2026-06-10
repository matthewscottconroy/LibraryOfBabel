# Path Induction

## The Principle

Mathematical induction says: to prove P(n) for all n : ℕ, prove P(0) and prove that P(n) implies P(n+1). The key step is irreducible: you cannot prove P(n) for all n without some form of induction. The base case and the step cover all natural numbers because every natural number is either 0 or a successor.

Path induction says: to prove C(b, p) for all b : A and all p : a =_A b, prove C(a, refl_a). The key step is similarly irreducible: every path from a can, for the purpose of proving the property C, be contracted to the trivial path at a.

This is startling. In ordinary mathematics, a path from a to b (b ≠ a) cannot be deformed to the trivial path at a — that would require b = a. But in type theory, path induction is valid. Why?

The answer is that the based path space Σ(b:A).(a =_A b) is contractible. Every element (b, p) is connected to the canonical element (a, refl_a) by a path in this total space. When you induct over paths — when you say "it suffices to handle the refl case" — you are using this contractibility.

This section makes that precise.

## Based Path Induction (The J Rule)

The J rule, as stated in Section 3, is *based path induction*. It is "based" because the starting point a is fixed: we induct over all paths starting at a.

**Based J:**

Given:
- A basepoint a : A (fixed)
- A motive C : Π(b:A).(a =_A b) → Type
- A base case d : C(a, refl_a)

Conclude: for any b : A and p : a =_A b, there exists J(a, C, d, b, p) : C(b, p).

**Computation:** J(a, C, d, a, refl_a) ≡ d.

The motive C takes two arguments: the endpoint b and the path p. It is a type family over the based path space.

## Unbased Path Induction (J')

The *unbased* version quantifies over both endpoints simultaneously.

**Unbased J' (also called Chevalley, path induction, or the Martin-Löf rule in some texts):**

Given:
- A motive C : Π(a b : A).(a =_A b) → Type — a type family over all paths, not just paths from a fixed basepoint
- A base case d : Π(a:A).C(a, a, refl_a) — a proof for every reflexivity path

Conclude: for any a, b : A and p : a =_A b, there exists J'(C, d, a, b, p) : C(a, b, p).

**Computation:** J'(C, d, a, a, refl_a) ≡ d(a).

The key difference: in J', the starting point is not fixed. You handle *all* reflexivity paths simultaneously (d takes a : A as an argument).

## Deriving Each from the Other

The two versions are equivalent. Each can be derived from the other.

### Deriving J' from J

Suppose you have J. You want to derive J'.

Given C : Π(a b:A).(a=b) → Type and d : Π(a:A).C(a,a,refl_a), and given a, b : A, p : a = b, apply J with:
- Basepoint: a
- Motive: C_a(b, p) = C(a, b, p) — this is the same C, with the first argument fixed to a
- Base case: d(a) : C_a(a, refl_a) = C(a, a, refl_a)

The result: J(a, C_a, d(a), b, p) : C_a(b, p) = C(a, b, p). This is the desired output for J'.

### Deriving J from J'

Suppose you have J'. You want to derive J.

Given a : A (basepoint), C : Π(b:A).(a=b) → Type, d : C(a, refl_a), and given b : A, p : a = b, apply J' with:
- Motive: D(x, y, q) = (x = a) → C(y, transport^{b ↦ a=b}(q, ?)) — this requires some work
- Actually, the direct derivation uses the specific structure of the based path space

A cleaner derivation: Define D(x, y, q) = (x = a) → C(y, ... q ...) — the idea is to use the path from x to a to "move" the basepoint. The details require transport, which we develop in Section 5. We defer the complete derivation to that section.

The fact of equivalence is important even without the explicit derivation: any theorem proved from J can be proved from J', and vice versa. The choice between them is a matter of convenience.

## The Contractibility of the Based Path Space

The fundamental theorem that explains why path induction works:

**Theorem.** The based path space Σ(b:A).(a =_A b) is contractible: it has a center of contraction at (a, refl_a), and every other element is connected to this center.

More precisely: there is a function

$$\mathsf{contr} : \prod_{p : \sum_{b:A}(a =_A b)} (a, \mathsf{refl}_a) =_{\sum_{b:A}(a=b)} p$$

giving a path from (a, refl_a) to any element p of the total space.

**Proof sketch.** Let p = (b, q) where b : A and q : a =_A b. We need a path (a, refl_a) = (b, q) in Σ(b:A).(a=b). Paths in a Σ type are pairs of a path in the base and a path over it (this requires the characterization of paths in Σ types, a theorem of HoTT). The first component path is q : a = b. The second component path (over q) is a path from transport^{b ↦ (a=b)}(q, refl_a) to q — and it turns out that transport^{b ↦ (a=b)}(q, refl_a) = q, which gives us refl_q : transport^{b ↦ (a=b)}(q, refl_a) = q.

The full proof uses J to reduce to the case q = refl_a, where both paths are trivial.

**Why this explains path induction:** If the based path space is contractible, any property of its elements need only be checked at the center — the contractibility gives paths from every element to the center, and you can transport the property along those paths. J is exactly the formal version of this transport.

## Why "It Suffices to Handle Refl" is Non-Trivial

In propositional logic or set-theoretic mathematics, if you want to prove a universal statement about all equality proofs, you might reason: "there is only one proof of a = b (up to equality), namely refl, so we only need one case."

This reasoning is circular: it assumes UIP (Uniqueness of Identity Proofs) to conclude there is only one case. But we proved in Section 3 that UIP is not derivable.

So how can path induction (J) be valid, if there might be multiple distinct paths?

The answer is subtle. J does not say "there is only one path." It says "for the purpose of the specific property C, any path can be continuously deformed to refl without affecting whether C holds." This is the contractibility of the based path space — not the triviality of individual paths, but the contractibility of the whole family.

Concretely: let A be the circle S¹, a = base, b = base, and p = loop (a non-trivial self-loop). The based path space Σ(b:S¹).(base = b) is contractible — even though loop ≠ refl and there are infinitely many distinct paths from base to base. The contractibility means there is a path in the *total space* from (base, refl) to (base, loop) — a *path between elements of the path space*, i.e., a homotopy. J uses this homotopy to reduce any property to the refl case.

This is precisely the content of the homotopy interpretation: path induction is valid not because there is only one path, but because the space of all paths is contractible.

## Axiom K and Its Consequences

**Axiom K (Streicher):** K : Π(a:A).Π(p:a=_A a). p = refl_a.

Every loop (self-path) at any point is equal to the trivial loop. This is equivalent to UIP.

From J, you can derive the *weaker* fact: Π(a:A).Π(p:a=_A a). refl_a = refl_a (which is trivially true from refl). But you cannot derive p = refl_a for arbitrary p. Axiom K adds this.

**K is consistent with MLTT.** It holds in the "set-theoretic model" where types are sets and identity proofs are unique. So adding K gives a consistent extension of MLTT.

**K is inconsistent with HoTT.** If K held, then the circle S¹ (a type with a non-trivial loop constructor) would be forced to have loop = refl_base, collapsing S¹ to a contractible type. The fundamental group π₁(S¹) = ℤ theorem would be false. All of HoTT's geometric content would be destroyed.

**In proof assistants:**
- Agda uses `--without-K` by default (required for HoTT). The `--with-K` flag re-enables K for classical developments.
- Lean 4 has proof irrelevance for the `Prop` sort (which implies K for propositions) but not for `Type` sorts.
- Coq has `proof_irrel` and `propext` axioms that give K for propositions; for pure HoTT you avoid these.

## The Heterogeneous Identity Type

There is a variant identity type, the *heterogeneous identity type* or *John Major equality*:

$$\mathsf{HEq}(A, a, B, b) : \text{ a "path" from } a : A \text{ to } b : B$$

This type relates elements of possibly different types. It is useful when working with indexed inductive types where the type of the element changes with the index.

In Agda, this is available as `HEq`. It can be derived in HoTT from the ordinary identity type and transport: a : A and b : B are heterogeneously equal iff there exists a path p : A = B in the universe such that transport(p, a) = b.

Heterogeneous identity gives an alternative way to state the computation rules for J and for path induction — some presentations of HoTT use it systematically. The ordinary and heterogeneous versions are equivalent in strength.

## Path Induction as Universal Property

Path induction (J) is, in categorical language, the universal property of the *free path space fibration*. Given the map ev₀ : (A → A) → A that evaluates a path at its starting point, the identity type is the pullback of ev₀ along the diagonal Δ : A → A × A. The J rule is the universal property of this pullback: any map out of the diagonal extends uniquely to a map out of the path space.

This categorical formulation connects MLTT to homotopy theory more directly: the J rule is a statement about the *homotopy lifting property* of a specific fibration. The contractibility of the based path space is the statement that this fibration has a section over the diagonal.

These connections are not just metaphors. They are the mathematical content of the homotopy interpretation of type theory, which we will make fully precise in Unit 04.
