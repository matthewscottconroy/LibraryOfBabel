# Rings and Modules

## Two Operations

A group has one operation. Many algebraic structures we care about have two: an "additive" operation and a "multiplicative" operation. The interaction between these two operations — specifically, the distributive law — is what gives rings their power.

**Definition.** A *ring* is a set R equipped with two binary operations + and · satisfying:

**Ring axioms:**
1. (R, +, 0) is an abelian group. (We write 0 for the additive identity and -a for the additive inverse of a.)
2. Multiplication is associative: (a · b) · c = a · (b · c).
3. Distributivity: a · (b + c) = a · b + a · c and (a + b) · c = a · c + b · c.

If additionally R has a multiplicative identity 1 with 1 · a = a · 1 = a, we say R is a *ring with unity* (or *unital ring*). If multiplication is commutative, R is a *commutative ring*.

**Important convention.** We always assume our rings have unity (a multiplicative identity 1), unless explicitly stated otherwise.

## Examples

**The integers ℤ.** The archetypal ring: addition, multiplication, with usual distributivity. Commutative ring with unity.

**The polynomial ring R[x].** Given any commutative ring R, the ring R[x] consists of polynomials a₀ + a₁x + a₂x² + ... with coefficients in R. Addition is term-by-term. Multiplication is polynomial multiplication (using distributivity and xⁿ · xᵐ = xⁿ⁺ᵐ).

**Matrix rings M_n(R).** The n × n matrices over a ring R, with matrix addition and multiplication. Not commutative for n ≥ 2. This is the basic example of a non-commutative ring.

**ℤ/nℤ.** The integers mod n. Commutative ring. If n is prime, this is a field.

**Product rings.** Given rings R and S, R × S with componentwise operations is a ring.

**Endomorphism rings.** For an abelian group A, the set End(A) of group homomorphisms A → A forms a ring: addition is pointwise, multiplication is composition. The identity is the identity homomorphism.

## Zero Divisors and Domains

**Definition.** An element a ≠ 0 in a ring R is a *zero divisor* if there exists b ≠ 0 with a · b = 0 or b · a = 0.

**Example.** In ℤ/6ℤ: 2 · 3 = 6 = 0 (mod 6). So 2 and 3 are zero divisors.

**Definition.** A commutative ring with unity and no zero divisors is an *integral domain*.

Examples: ℤ, ℚ, ℝ, ℂ, k[x] (polynomials over a field k), ℤ[i] (Gaussian integers).

In an integral domain, we can cancel: ab = ac and a ≠ 0 implies b = c. Integral domains are the "nice" commutative rings where polynomial algebra works as expected.

## Ideals

**Definition.** An *ideal* I ⊆ R is a subring that is closed under multiplication by elements of R:
- I is a subgroup of (R, +): if a, b ∈ I then a - b ∈ I.
- For all r ∈ R and a ∈ I: r · a ∈ I and a · r ∈ I.

Ideals are to rings as normal subgroups are to groups. The key property is that you can form quotient rings R/I: the set of cosets {a + I | a ∈ R} with operations (a+I) + (b+I) = (a+b)+I and (a+I)(b+I) = (ab)+I.

**Why ideals close under multiplication by R.** For the quotient to be well-defined, we need (a + I)(b + I) = (ab + I) to be independent of the representatives. If a' = a + i₁ and b' = b + i₂, then a'b' = ab + ai₂ + i₁b + i₁i₂. The extra terms ai₂ + i₁b + i₁i₂ must lie in I, which requires the closure conditions.

**Examples:**
- nℤ = {nk | k ∈ ℤ} is an ideal in ℤ.
- ℤ/nℤ = ℤ/(nℤ) is the quotient ring.
- For a commutative ring R and element a ∈ R, the *principal ideal* ⟨a⟩ = aR = {ar | r ∈ R} is the smallest ideal containing a.
- The ideal ⟨x⟩ in k[x] consists of polynomials with no constant term. k[x]/⟨x⟩ ≅ k.
- The ideal ⟨x² + 1⟩ in ℝ[x]. The quotient ℝ[x]/⟨x² + 1⟩ ≅ ℂ (complex numbers! x plays the role of i).

**Maximal and prime ideals.** A *maximal ideal* M ⊆ R is a proper ideal not contained in any larger proper ideal. A *prime ideal* P ⊆ R is a proper ideal where ab ∈ P implies a ∈ P or b ∈ P.

In a commutative ring R:
- I is maximal ↔ R/I is a field.
- I is prime ↔ R/I is an integral domain.

Every maximal ideal is prime. The converse fails in general.

## Modules

**Definition.** Let R be a ring. A *left R-module* M is an abelian group (M, +, 0) with a scalar multiplication R × M → M, (r, m) ↦ r · m, satisfying:
- r(m + n) = rm + rn (distributivity over module addition)
- (r + s)m = rm + sm (distributivity over ring addition)
- (rs)m = r(sm) (associativity of scalar multiplication)
- 1 · m = m (if R has unity)

**Examples:**
- Every abelian group is a ℤ-module (with n · m = m + m + ... + m, n times).
- Every vector space over a field k is a k-module (and conversely).
- R itself is an R-module (by left multiplication).
- R[x] is an R-module.
- ℤ/nℤ is a ℤ-module.

Modules are the right level of generality for linear algebra over arbitrary rings. Vector spaces are modules over fields. When we compute homology groups in algebraic topology, we get abelian groups — which are ℤ-modules. When we tensor or dualize, we use module operations.

**Submodules and quotient modules.** A *submodule* N ⊆ M is a subgroup closed under scalar multiplication: if n ∈ N and r ∈ R, then rn ∈ N. The quotient M/N is again an R-module.

**Module homomorphisms.** A *module homomorphism* f: M → N is an abelian group homomorphism preserving scalar multiplication: f(rm) = rf(m). The kernel and image are submodules. The First Isomorphism Theorem holds for modules.

## Why Modules Matter for This Curriculum

**Homology groups are ℤ-modules.** The homology groups H_n(X; ℤ) of a topological space X are abelian groups — ℤ-modules. The operations (tensoring with other modules, taking Hom) are module operations. Without module theory, algebraic topology is opaque.

**Classification of finitely generated abelian groups.** Every finitely generated abelian group (ℤ-module) is isomorphic to:

ℤ^r ⊕ ℤ/n₁ℤ ⊕ ℤ/n₂ℤ ⊕ ... ⊕ ℤ/n_kℤ

where n₁ | n₂ | ... | n_k and r ≥ 0. The r is the *rank* (free part) and the ℤ/nᵢℤ are the *torsion* parts. This classification is a special case of the structure theorem for finitely generated modules over a principal ideal domain.

**Cohomology rings.** The cohomology groups H*(X; R) of a space X with coefficients in a ring R form a *graded ring* — a ring where elements have degrees (dimensions) and multiplication respects the grading. Understanding cohomology requires both ring theory (for the ring structure) and module theory (for the group structure).

**Representation theory.** A *representation* of a group G over a field k is a group homomorphism G → GL_n(k). Equivalently, it is a k[G]-module (where k[G] is the group ring). The classification of representations of finite groups is one of the central achievements of algebra and has applications in physics (symmetries of quantum systems) and combinatorics.

Rings and modules appear throughout the curriculum as the algebraic infrastructure for computing. Groups tell you *what kind* of algebraic structure exists. Modules tell you *how to compute with it*. The two together — rings acting on modules — are the language of homological algebra, which is the computational heart of algebraic topology.
