# Transport and ap

## Two Operations, One Origin

Everything useful about the identity type is derived from the J rule. The J rule is the eliminator for paths; everything else is a consequence. Among all the derived operations, two stand out as foundational for all subsequent work in HoTT: **transport** and **ap**.

Transport says: a path in the base of a fibration induces a function between fibers. If a = b in A and B : A → Type is a type family over A, then an element of B(a) can be moved to B(b) along the path.

ap says: a function f : A → B sends a path in A to a path in B. If a = b in A, then f(a) = f(b) in B.

These are not just convenient facts. They are the two most-used tools in all of HoTT. Every argument about paths eventually comes down to transport and ap.

## Transport

**Definition.** Given B : A → Type, a path p : a =_A b, and an element u : B(a), define:

$$\mathsf{transport}^B(p, u) : B(b)$$

by J with:
- Motive: C(b, p) = B(a) → B(b)
- Base case: d = id_{B(a)} : B(a) → B(a) (the identity function)

$$\mathsf{transport}^B(p) = J(a,\, \lambda b\, p.\, B(a) \to B(b),\, \mathsf{id}_{B(a)},\, b,\, p)$$

**Computation:** transport^B(refl_a, u) ≡ u.

Moving along the trivial path does nothing. This makes sense geometrically: parallel transport along a constant path should be the identity.

**Notation:** We write p_*(u) for transport^B(p, u) when B is clear from context.

### Transport is Functorial

Transport respects composition:

$$\mathsf{transport}^B(q,\, \mathsf{transport}^B(p, u)) = \mathsf{transport}^B(p \cdot q,\, u)$$

for p : a = b and q : b = c. Moving first along p and then along q is the same as moving along p · q. This is proved by J on q.

Transport also respects inversion:

$$\mathsf{transport}^B(p^{-1},\, \mathsf{transport}^B(p, u)) = u$$

Moving along p and then back along p⁻¹ returns to the original element.

These functoriality laws make transport a functor from the fundamental groupoid of A to the category of types and functions.

### Transport as Substitution of Equals

The slogan for transport: if a = b, then B(a) and B(b) contain the same information (they are equivalent as types). Any element of B(a) can be moved to B(b) and back.

This is the type-theoretic version of *Leibniz's law*: if a = b, then any property P true of a is also true of b. In MLTT, transport makes this explicit: it provides the function B(a) → B(b) that moves witnesses.

### Examples of Transport

**Example 1: Transporting along a path of natural numbers.**

Let B : ℕ → Type be B(n) = Vec A n. If p : 3 + 0 =_ℕ 3, then transport^B(p) : Vec A (3+0) → Vec A 3. Since 3+0 ≡ 3 definitionally, this transport is (definitionally) the identity function.

**Example 2: Transporting a proof of a predicate.**

Let B : ℕ → Type be IsEven. If p : n = m and h : IsEven n, then transport^{IsEven}(p, h) : IsEven m. Moving the proof that n is even along the path p : n = m gives a proof that m is even. This is "substitution of equals" for the predicate IsEven.

**Example 3: The action of paths on fibers.**

For the identity type itself: fix a : A and let B(b) = (a =_A b). If q : b = c, then transport^B(q) : (a = b) → (a = c). This is the function that concatenates q on the right: transport^B(q, p) = p · q.

## ap (Application of Functions to Paths)

**Definition.** Given f : A → B and p : a =_A b, define:

$$\mathsf{ap}_f(p) : f(a) =_B f(b)$$

by J with:
- Motive: C(b, p) = f(a) =_B f(b)
- Base case: d = refl_{f(a)} : f(a) =_B f(a)

$$\mathsf{ap}_f(p) = J(a,\, \lambda b\, p.\, f(a) = f(b),\, \mathsf{refl}_{f(a)},\, b,\, p)$$

**Computation:** ap_f(refl_a) ≡ refl_{f(a)}.

Every function sends the trivial path to the trivial path.

**Intuition:** ap_f says that f is continuous. If a can be continuously deformed to b (via the path p), then f(a) can be continuously deformed to f(b) (via the path ap_f(p)). This is the type-theoretic statement that every function between types is automatically continuous — there are no discontinuous functions in MLTT.

### ap is Functorial

ap respects path operations:

$$\mathsf{ap}_f(\mathsf{refl}_a) = \mathsf{refl}_{f(a)}$$
$$\mathsf{ap}_f(p^{-1}) = (\mathsf{ap}_f(p))^{-1}$$
$$\mathsf{ap}_f(p \cdot q) = \mathsf{ap}_f(p) \cdot \mathsf{ap}_f(q)$$

The last equation says: ap_f is a *homomorphism* for path concatenation. And ap is functorial for function composition:

$$\mathsf{ap}_{g \circ f}(p) = \mathsf{ap}_g(\mathsf{ap}_f(p))$$
$$\mathsf{ap}_{\mathsf{id}_A}(p) = p$$

So ap defines a functor from A to B (viewed as groupoids) for each function f : A → B. Every function in MLTT gives rise to a functor between the fundamental groupoids of its domain and codomain.

### ap for Dependent Functions

For a dependent function f : Π(x:A).B(x), we cannot say f(a) =_B f(b) directly — f(a) : B(a) and f(b) : B(b) are elements of different types (different fibers). But we can define a dependent version:

$$\mathsf{apd}_f : \prod_{p : a = b} \mathsf{transport}^B(p, f(a)) = f(b)$$

This says: after transporting f(a) along p to the fiber B(b), it equals f(b). The proof:

Apply J with:
- Motive: C(b, p) = transport^B(p, f(a)) =_{B(b)} f(b)
- Base case: d = refl_{f(a)} — since transport^B(refl, f(a)) ≡ f(a)

**Computation:** apd_f(refl_a) ≡ refl_{f(a)}.

apd is the key lemma connecting paths in the base space to the sections of fibrations. It says: any section of a fibration sends paths to dependent paths (paths that live over the base path).

## The Path Space Fibration

Transport and ap together define the *path space fibration*. For any type A, the path space fibration is:

$$\mathsf{Paths}(A) = \sum_{a\, b : A} (a =_A b)$$

This is the type of all paths in A — pairs (a, b) together with a path between them. There is a projection π : Paths(A) → A × A sending (a, b, p) to (a, b).

The fiber over (a, b) is the type a =_A b itself — the space of all paths from a to b.

Transport in this fibration: a path from (a, b) to (a', b') in A × A — i.e., a pair (p, q) where p : a = a' and q : b = b' — acts on paths r : a = b by:

$$\mathsf{transport}^{x \mapsto \pi_1(x) = \pi_2(x)}((p, q), r) = p^{-1} \cdot r \cdot q$$

Conjugation: pre-compose with p⁻¹ and post-compose with q.

This is the action of the "conjugation" in the fundamental groupoid, and it will reappear in HoTT when we study higher paths.

## Homotopies as Sections

A *homotopy* between two functions f, g : Π(x:A).B(x) is a term:

$$H : \prod_{x:A} f(x) =_{B(x)} g(x)$$

This says: for every x, f(x) and g(x) are equal in B(x). H is a section of the fibration over A whose fiber over x is f(x) =_{B(x)} g(x).

Homotopies are the type-theoretic analogue of homotopies in topology: a continuous deformation from f to g. The type of homotopies from f to g is exactly the based path space in the function type Π(x:A).B(x), based at f, evaluated at g.

**Function extensionality (not provable in MLTT, but a consequence of univalence):** If H : f ~ g (a homotopy from f to g), then f = g (the functions are propositionally equal). This principle is the type-theoretic version of "two functions that agree on all inputs are equal."

Without univalence, function extensionality is independent of MLTT. With univalence, it follows as a theorem. This is one of the key consequences of the univalence axiom and connects the material in this section directly to HoTT's central axiom.

## Why Transport and ap Dominate HoTT Proofs

In practice, every HoTT proof about paths eventually reduces to:
1. Apply J to reduce to the refl case
2. Use transport to move elements along paths
3. Use ap to apply functions to paths
4. Use the functoriality laws to simplify

The combinatorics can be intricate — chains of transports and aps, coherence conditions between them — but the basic operations are always these three. They are the tools that convert the abstract J rule into concrete path-manipulating mathematics.

The next unit (Unit 04, The Univalence Axiom) is largely a study in transport and ap at the universe level: if f : A → B is an equivalence, then by univalence there is a path p : A =_{Type} B in the universe, and transport^{id}(p) : A → B is the function f (up to homotopy). This connection between equivalences and paths in the universe is the heart of HoTT, and it is built from the operations in this section.
