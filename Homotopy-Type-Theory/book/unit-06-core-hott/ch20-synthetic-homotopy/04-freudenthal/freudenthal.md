# The Freudenthal Suspension Theorem

## Stable Homotopy

One of the deepest structural phenomena in homotopy theory is *stability*: for a fixed k, the homotopy group πₙ₊ₖ(Sⁿ) is independent of n for sufficiently large n. For example:

- π₃(S²) = Z, π₄(S³) = Z/2Z, π₅(S⁴) = Z/2Z, π₆(S⁵) = Z/2Z, ...
- π₄(S²) = Z/2Z, π₅(S³) = Z/2Z, π₆(S⁴) = Z/2Z, ...

For k = 1, the groups πₙ₊₁(Sⁿ) stabilize to Z/2Z for all n ≥ 3. For k = 2, they stabilize for n ≥ 4. The stable range grows with k.

This stability is the content of the Freudenthal Suspension Theorem.

## The Statement

**Theorem (Freudenthal Suspension Theorem).** Let A be an (n-1)-connected type (meaning πₖ(A) = 0 for k < n, i.e., ‖A‖_{n-1} is contractible). Then the suspension map:
```
σ : A → ΩΣA
```

(the unit of the Σ ⊣ Ω adjunction) is (2n-1)-connected.

That is: σ induces isomorphisms on πₖ for k < 2n-1, and a surjection on π_{2n-1}.

**Corollary.** For A = Sⁿ (the n-sphere, which is (n-1)-connected):
```
πₖ(Sⁿ) ≅ πₖ₊₁(Sⁿ⁺¹) for k < 2n-1
```

The homotopy groups of spheres stabilize in a range that grows with n.

## The HoTT Proof Strategy

The HoTT proof of the Freudenthal theorem (Lumsdaine-Shulman) uses the *Blakers-Massey theorem* — a more general statement about connectivity of homotopy pushouts.

**Blakers-Massey.** If f : A → B and g : A → C are maps with f being m-connected and g being n-connected, then the canonical map `A → B ×_{B⊔_A C} C` (from A to the pullback over the pushout) is (m+n-1)-connected.

The Freudenthal theorem is the special case where B = C = 1 (the unit type) and A is (n-1)-connected:
- The suspension ΣA is the pushout 1 ← A → 1.
- Blakers-Massey gives connectivity of A → Ω(ΣA) = 1 ×_{ΣA} 1.

## The HoTT Proof of Blakers-Massey

The Blakers-Massey theorem in HoTT was proved by Anel, Biedermann, Finster, and Joyal (2017), building on earlier partial proofs. The proof uses:

1. **The connectivity of the fiber of the pushout map.** The map A → pushout has fiber equivalent to the join A * A (in the relevant sense). The connectivity of this fiber is estimated using the connectivity of A.

2. **Inductive argument.** By induction on the connectivity, using the fact that n-connected maps are characterized by their fibers being n-connected.

3. **The long exact sequence in connectivity.** From a fibration sequence F → E → B, connectivity of F and B implies connectivity of E.

The HoTT proof is cleaner than the classical proof (which uses spectral sequences and careful estimation of Serre's exact sequence). This is one of the cases where the synthetic approach gives a better proof.

## The Stable Range

**Definition.** The *stable homotopy group* πₖˢ is:
```
πₖˢ := πₖ₊ₙ(Sⁿ) for any n > k+2
```

By Freudenthal, this is well-defined (independent of n once n > k+2).

The stable homotopy groups form a graded ring (under composition of stable maps), called the *stable homotopy ring* π*ˢ. Computing π*ˢ is one of the central unsolved problems of algebraic topology.

Known values:
```
π₀ˢ = Z
π₁ˢ = Z/2Z
π₂ˢ = Z/2Z
π₃ˢ = Z/24Z
π₄ˢ = 0
π₅ˢ = 0
π₆ˢ = Z/2Z
π₇ˢ = Z/240Z
...
```

The pattern is irregular — no simple formula is known.

## Consequences for Spheres

The Freudenthal theorem gives the first non-trivial calculation of homotopy groups of spheres:

**Corollary.** π₂(S²) = Z.

*Proof.* S² is 1-connected. By Freudenthal, S² → ΩS³ is 3-connected. By the long exact sequence, π₂(S²) ≅ π₃(S³). And π₃(S³) = Z (the identity map S³ → S³ generates a copy of Z in the loop space). ∎

**Corollary.** π₃(S²) = Z.

This is harder — it uses the Hopf fibration (Section 5). The Freudenthal theorem gives a surjection π₃(S²) → π₃(S³) = Z... wait, that's not right (the direction is reversed). The long exact sequence of the Hopf fibration S¹ → S³ → S² gives the computation directly.

## The Blakers-Massey Theorem in HoTT

The Blakers-Massey theorem, proved in HoTT by Anel-Biedermann-Finster-Joyal, is actually more general than the classical statement. Their proof:

1. Works for any ∞-topos (not just the ∞-topos of spaces).
2. Is constructive (no use of choice or excluded middle).
3. Uses only basic type theory (no spectral sequences or other classical tools).
4. Has been formalized in Cubical Agda.

This is a case where the synthetic proof is strictly better than the classical proof — not just shorter, but more general and more constructive.

## Why These Theorems Matter

The Freudenthal theorem and Blakers-Massey are foundational for stable homotopy theory. They establish that:

1. **Stable homotopy groups exist.** Homotopy groups of spheres stabilize, giving well-defined stable groups.

2. **Stable homotopy is accessible.** The stable range is computable, and many techniques work in the stable range that don't work unstably.

3. **Suspension is well-behaved.** The suspension functor Σ and loop functor Ω are "approximately" inverse in a range, given by the connectivity bound.

In HoTT, these theorems are proved synthetically, using only the type theory. No point-set topology, no simplicial sets, no model categories — just types, paths, and the encode-decode method.

## Summary

| Theorem | Statement | HoTT proof method |
|---|---|---|
| Freudenthal | σ : A → ΩΣA is (2n-1)-connected for (n-1)-connected A | Blakers-Massey + connectivity |
| Blakers-Massey | Connectivity of map to fiber of pushout | Inductive connectivity argument |
| Stable groups | πₖˢ = πₖ₊ₙ(Sⁿ) for n > k+2 | Freudenthal stabilization |

The Freudenthal theorem is one of the central results of homotopy theory. Its synthetic proof in HoTT is one of the major achievements of the subject, demonstrating that HoTT is a genuine tool for cutting-edge mathematics, not just a foundational curiosity.
