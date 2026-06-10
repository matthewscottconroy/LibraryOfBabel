# Homotopy Preview: Paths, Composition, and the Groupoid Seed

## The Key Observation

We have now defined all the pieces: paths are continuous functions from [0,1], composition is concatenation followed by reparametrization, and the fundamental group captures the homotopy classes of loops. But the fundamental group is only the beginning.

Here is the key observation that drives HoTT:

**Path composition is not strictly associative. It is associative up to homotopy. And the homotopy itself is a path of paths — a structure with its own algebra.**

This observation, which seems like a minor technical annoyance in classical topology, becomes the central organizing principle of Homotopy Type Theory. Let us trace it carefully.

## Paths Compose

Let γ: x → y and δ: y → z be paths (continuous functions [0,1] → X with appropriate endpoints). Their concatenation is:

(γ ∗ δ)(t) = γ(2t) for t ∈ [0, 1/2], δ(2t - 1) for t ∈ [1/2, 1].

The resulting function (γ ∗ δ): [0, 1] → X is a path from x to z.

**Associativity.** What is (γ ∗ δ) ∗ ε? It is the path that spends time [0, 1/4] on γ, [1/4, 1/2] on δ, [1/2, 1] on ε. What is γ ∗ (δ ∗ ε)? It spends [0, 1/2] on γ, [1/2, 3/4] on δ, [3/4, 1] on ε.

Both paths traverse γ, then δ, then ε. But at different speeds. They are *different functions* (given different values as elements of Map([0,1], X)), but they are *homotopic* — connected by the homotopy that linearly adjusts the speed.

Explicitly: define H: [0,1] × [0,1] → X by H(t, s) = ((γ ∗ δ) ∗ ε)((1 + s)t/(2)) for a suitable reparametrization. The details are computational, but the picture is clear: as s goes from 0 to 1, the path continuously deforms from (γ ∗ δ) ∗ ε to γ ∗ (δ ∗ ε).

So: **associativity holds up to a path of paths** — an *associativity homotopy* assoc(γ, δ, ε): (γ ∗ δ) ∗ ε ≃ γ ∗ (δ ∗ ε).

## The Identity Laws

Similarly, the identity law for path composition holds only up to homotopy.

The constant path c_x followed by γ: c_x ∗ γ. This path spends time [0, 1/2] sitting at x, then time [1/2, 1] traversing γ. It is different from γ (different speed). But it is homotopic to γ by the homotopy that "squishes" the initial pause.

So: c_x ∗ γ ≃ γ (left identity up to homotopy) and γ ∗ c_y ≃ γ (right identity up to homotopy).

## The Inverse Laws

The reversed path γ̄(t) = γ(1-t) is the "inverse" of γ. The concatenation γ ∗ γ̄ is the path that goes from x to y and then immediately returns from y to x. This path is homotopic to c_x (the constant path at x), but not equal to it: the concatenation is a path that goes out and comes back, while c_x stays put.

The homotopy: at time s, the path "turns around" at position γ(s/2) instead of γ(1). As s decreases from 1 to 0, the turnaround point moves from γ(1/2) back to x, and the path shrinks to c_x.

So: γ ∗ γ̄ ≃ c_x (left inverse up to homotopy) and γ̄ ∗ γ ≃ c_y (right inverse up to homotopy).

## The Groupoid Structure

The laws we have established:
- Composition: γ ∗ δ is defined when γ ends where δ starts.
- Associativity: (γ ∗ δ) ∗ ε ≃ γ ∗ (δ ∗ ε).
- Identity: c_x ∗ γ ≃ γ ≃ γ ∗ c_y.
- Inverses: γ ∗ γ̄ ≃ c_x.

This is the structure of a *groupoid* — a group where not every pair of elements can be composed (you can only compose γ and δ if γ ends where δ starts), but where composition, identity, and inverses all hold. A groupoid is a "many-object group," or a category where every morphism is invertible.

The *fundamental groupoid* Π₁(X) of a space X has:
- Objects: points of X.
- Morphisms from x to y: homotopy classes of paths from x to y.
- Composition: concatenation of homotopy classes.
- Identity at x: the class of c_x.
- Inverse: the class of the reversed path.

The fundamental group π₁(X, x₀) is the *vertex group* of Π₁(X) at the object x₀: the morphisms from x₀ to x₀.

## But the Homotopies Are Themselves Paths

Here is where the story gets richer. The homotopy assoc(γ, δ, ε): (γ ∗ δ) ∗ ε ≃ γ ∗ (δ ∗ ε) is itself a 2-dimensional path — a continuous map [0,1]² → X. We can ask: what is the relation between two such associativity homotopies? They might themselves be homotopic, via a 3-dimensional homotopy. And so on.

This is the *∞-groupoid* structure:
- 0-cells: points of X.
- 1-cells: paths between points.
- 2-cells: homotopies between paths (paths of paths).
- 3-cells: homotopies between homotopies.
- n-cells: n-dimensional homotopies.

Each level satisfies groupoid laws *up to the next level*. Composition at level n is strictly defined but associative only up to a path at level n+1. Identity laws hold at level n up to paths at level n+1. And so on.

An ∞-groupoid is a structure with cells at every level, with all cells invertible, and with the groupoid laws holding at each level up to cells of the next level.

**The Homotopy Hypothesis (Grothendieck).** ∞-Groupoids correspond, up to a suitable notion of equivalence, to *homotopy types* — topological spaces considered up to homotopy equivalence. This is not an accident: the ∞-groupoid structure of a space IS its homotopy type.

## From Analysis to HoTT

In HoTT, the Homotopy Hypothesis is not a theorem to be proved — it is an *axiom*, built into the interpretation of types:

**Types are homotopy types.** The cells of a type are:
- 0-cells (terms): the elements of the type.
- 1-cells (paths): elements of the identity type a =_A b.
- 2-cells (2-paths): elements of the identity type p =_{a=b} q.
- n-cells: elements of iterated identity types.

**The identity type is the path type.** A proof p: a = b is a path from a to b in the "space" A.

**Concatenation, reversal, and constant paths** correspond to the three term-forming operations on identity types:
- concat: (a = b) → (b = c) → (a = c)
- symm: (a = b) → (b = a)
- refl: (a = a)

**Associativity and identity laws** hold up to 2-paths (paths between paths), exactly as concatenation of topological paths is associative up to homotopy.

The analysis in this chapter — defining paths, concatenation, homotopy, the fundamental groupoid — is the geometric intuition that HoTT formalizes. When you write a proof of equality in Lean using the identity type, you are doing path arithmetic in a topological space. When you prove that two identity proofs are equal, you are constructing a homotopy between paths.

This is the culmination of Unit 1. We began with the grammar of proof (Chapter 0), moved to the classical foundation (Chapter 1), built the algebraic language of symmetry (Chapter 2), and now have the geometric intuition (Chapter 3) that makes HoTT interpretable. The remaining units develop the formal machinery. But the motivation — paths, homotopies, and the ∞-groupoid seed — is what we have now.
