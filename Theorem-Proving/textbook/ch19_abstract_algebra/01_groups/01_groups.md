# Groups

A group is the simplest algebraic structure with nontrivial theory — and it appears everywhere: symmetries of geometric objects, permutations, number systems, cryptography, quantum mechanics.

## Definition

A *group* is a set G with a binary operation ∗ : G × G → G satisfying:

1. **Associativity**: (a ∗ b) ∗ c = a ∗ (b ∗ c) for all a, b, c ∈ G
2. **Identity**: there exists e ∈ G such that e ∗ a = a ∗ e = a for all a ∈ G
3. **Inverses**: for each a ∈ G, there exists a⁻¹ ∈ G with a ∗ a⁻¹ = a⁻¹ ∗ a = e

If additionally a ∗ b = b ∗ a for all a, b ∈ G, the group is *abelian* (commutative).

## Examples

| Group | Operation | Identity | Abelian? |
|-------|-----------|----------|----------|
| (ℤ, +) | Addition | 0 | Yes |
| (ℚ\{0}, ×) | Multiplication | 1 | Yes |
| (GL_n(ℝ), ×) | Matrix multiplication | I_n | No (n≥2) |
| S_n | Permutation composition | Identity permutation | No (n≥3) |
| ℤ/nℤ | Addition mod n | 0 | Yes |
| Rubik's Cube moves | Composition | Do nothing | No |

## Fundamental Theorems

**Lagrange's Theorem**: If H is a subgroup of finite group G, then |H| divides |G|.

*Proof sketch*: The cosets of H partition G into equally-sized sets, each of size |H|.

**Consequence**: The order of any element (smallest n with aⁿ = e) divides |G|.

**Cayley's Theorem**: Every group is isomorphic to a subgroup of a symmetric group. (Every group is a permutation group, in disguise.)

**Homomorphism theorems (isomorphism theorems)**:
- If φ: G → H is a group homomorphism, then G/ker(φ) ≅ im(φ).
- The structure of quotient groups is controlled by normal subgroups.

## Groups and Logic

Groups connect to logic in multiple ways:

**Decidability**: The word problem for a finitely presented group — given words in generators and relations, are two words equal? — is undecidable in general (Novikov-Adian, 1955). This is one of the earliest undecidability results outside arithmetic.

**Model theory**: The theory of groups is an equational theory — its axioms are universal sentences of the form ∀x∀y∀z. φ. Equational theories are complete in a sense (Birkhoff's theorem): a universal sentence holds in all groups iff it follows from the group axioms.

**Galois theory**: Every polynomial equation's symmetry group (Galois group) determines whether the equation is solvable by radicals. Galois' insight — connecting field extensions to group theory — was the first major application of groups and remains the prototype of *representation theory*.

**Cryptography**: The Diffie-Hellman key exchange and elliptic curve cryptography rely on the hardness of the discrete logarithm problem in certain groups. The security of modern cryptography is a group-theoretic statement.

## Simple Groups

A group with no normal subgroups (other than {e} and G itself) is *simple*. Simple groups are the "atoms" of group theory — all finite groups are built from them via extensions.

The *Classification of Finite Simple Groups* (CFSG, announced 1983, proof ~10,000 pages across hundreds of papers) enumerates all finite simple groups:
- Cyclic groups ℤ/pℤ (prime p)
- Alternating groups Aₙ (n ≥ 5)
- 16 families of groups of Lie type
- 26 *sporadic* groups (including the Monster, with ~8×10⁵³ elements)

The CFSG is one of the greatest mathematical achievements of the 20th century — and one of the primary motivations for formalizing mathematics in proof assistants (the proof is too long to verify by hand reliably).
