# Applied Exercises

Higher inductive types give type theory the ability to define spaces directly by specifying their points and paths, rather than by imposing structure on sets. This is not merely a theoretical convenience: the same idea appears in many guises in computer science and mathematics — wherever one wants to "identify" or "glue" things together. Coequalizers in database theory, pushouts in API composition, quotient types in abstract algebra, and the propositional truncation used to hide computational witnesses in proof assistants all have a clean HIT-based treatment. The exercises below make these connections concrete, using the formal machinery of this chapter — HIT eliminators, transport, universal properties — to solve problems drawn from software engineering, algebra, topology, and verified proof.

---

## Exercise A.1: The Clock as S¹ — Periodic Data and the Circle HIT
*Domain: Systems Programming / Real-Time Systems / Circular Data Structures*

**Setup:** Many real-world data structures are inherently *periodic* or *circular*. A clock face is the canonical example: 12:00 and 0:00 are the same time, and time "wraps around." Formally, a clock can be modeled as the circle $S^1$: there is a base point `12` (or `0`), and there is a loop `tick : 12 = 12` that represents one full rotation.

A "time value" is an element of $S^1$. An "angle" (in radians) is modeled as an integer number of half-steps around the loop: the angle $\frac{2\pi k}{n}$ corresponds to the path `loop^k` (the loop traversed $k$ times) in a discrete approximation with $n$ steps. Functions from $S^1$ to another type $B$ must respect the circular structure: by the circle's non-dependent eliminator, a function $f : S^1 \to B$ is exactly a pair of a point $f(\mathsf{base}) : B$ and a loop $\mathsf{ap}_f(\mathsf{loop}) : f(\mathsf{base}) = f(\mathsf{base})$.

**Questions:**
1. Using the circle's non-dependent eliminator, define the function `hour-hand : S¹ → S¹` that maps a position on the circle to the position of the hour hand (which traverses the circle once for every 12 traversals of the minute hand). What are `hour-hand(base)` and `ap_{hour-hand}(loop)`? Write the definition formally.
2. A "periodic function" with period $n$ (over the discrete circle with $n$ points) is a function $f : \mathbb{Z}/n\mathbb{Z} \to B$. Formalize $\mathbb{Z}/n\mathbb{Z}$ as a HIT: give its point and path constructors, its non-dependent elimination principle, and its computation rules. (Hint: $\mathbb{Z}/n\mathbb{Z}$ has $n$ points and one path constructor identifying `succ^n(0)` with `0`.)
3. (Extension) A "schedule" on the circle is a function $\sigma : S^1 \to \mathsf{Prop}$ assigning to each time a proposition (the schedule is "active" at that time). Define the type of valid schedules as `Schedule ≡ Σ(σ : S¹ → Prop), σ(base) = σ(transport^σ(loop, base))`. What does the path constructor condition say about valid schedules? Can you give an example of a function `S¹ → Prop` that is *not* a valid schedule by this definition?

*Abstract concept illustrated: the circle HIT's non-dependent eliminator, functions out of S¹, transport in propositional families.*

---

## Exercise A.2: Pushouts as API Composition
*Domain: Software Engineering / API Design / Module Systems*

**Setup:** When two software systems share a common interface, their composition can be modeled as a pushout. Let `API_A` be the type of values produced by system A, `API_B` be the type of values produced by system B, and `Common` be the type of values in their shared interface. The gluing maps `f : Common → API_A` and `g : Common → API_B` embed the common interface into each system.

The pushout `API_A ⊔_{Common} API_B` is the type of values in the composed system: it has all values from `API_A` and `API_B`, plus a path `glue(c) : inl(f(c)) = inr(g(c))` for each common value `c`, identifying the two representations of common values.

Concretely: system A is a relational database returning `Row_A` (a record with fields `id : Nat` and `data : String`), system B is a key-value store returning `Row_B` (a pair `Nat × String`), and the common interface is `Common = Nat × String` with `f(n, s) = {id = n, data = s}` and `g = id`.

**Questions:**
1. Write out the pushout `Row_A ⊔_{Common} Row_B` explicitly as a HIT: give all point and path constructors, and state the non-dependent eliminator. What does the eliminator say about defining a function out of the composed system?
2. Prove that `Row_A ⊔_{Common} Row_B ≃ Row_A` (the pushout collapses to `Row_A` because `f : Common → Row_A` is already a surjection whose image can be retracted onto). What property of `f` makes this work? State the general lemma: if `f : C → A` is an equivalence, then `A ⊔_C B ≃ B`.
3. (Extension) The pushout universal property says: to define a function out of `A ⊔_C B` into a type `X`, it suffices to give functions `h_A : A → X` and `h_B : B → X` and a homotopy `H : ∀ c : C, h_A(f(c)) = h_B(g(c))`. Formalize this universal property as a type. In the API composition example, what does the universal property say about defining a "query" function that works on both APIs?

*Abstract concept illustrated: the pushout HIT, its universal property, colimits as HITs.*

---

## Exercise A.3: The Integers as a Pushout — Quotient Types via HITs
*Domain: Abstract Algebra / Number Theory / Type-Safe Arithmetic*

**Setup:** The integers $\mathbb{Z}$ can be constructed as a HIT in multiple ways. One clean approach is as a pushout: $\mathbb{Z} \cong \mathbb{N} \sqcup_{\mathbb{N}} \mathbb{N}$, where the two copies of $\mathbb{N}$ represent the non-negative and non-positive integers, and the pushout identifies $0$ in both copies. Another approach: the integers are the free group on one generator, definable as a HIT with one point and one loop (i.e., $\mathbb{Z} = \pi_1(S^1)$ as a set, or as the type of integers defined by the circle's universal cover).

A third approach (most relevant to quotients): $\mathbb{Z} = (\mathbb{N} \times \mathbb{N}) / \sim$ where $(a, b) \sim (c, d)$ iff $a + d = b + c$ (the pair $(a, b)$ represents $a - b$). This is a set quotient, and set quotients are HITs.

**Questions:**
1. Define the HIT for the quotient $(\mathbb{N} \times \mathbb{N}) / \sim$: give the point constructors (`class(a, b) : ℤ_HIT` for each `(a, b) : ℕ × ℕ`), the path constructor (`eq : ∀ a b c d, a + d = b + c → class(a, b) = class(c, d)`), and the set truncation constructor (making the result a set). Write the non-dependent eliminator for this HIT.
2. Define addition on `ℤ_HIT` using the eliminator. You must show that your definition respects the path constructor: if `class(a, b) = class(a', b')`, then `class(a + c, b + d) = class(a' + c, b' + d)` for any `c, d`. Prove this.
3. (Extension) The rational numbers $\mathbb{Q}$ can be defined similarly as a HIT: $\mathbb{Q} = (\mathbb{Z} \times \mathbb{Z}_{>0}) / \sim$ where $(p, q) \sim (p', q')$ iff $p \cdot q' = p' \cdot q$. Write the HIT definition, the non-dependent eliminator, and define addition. What is the key lemma needed to show that addition is well-defined? How does this compare to the `ℤ_HIT` case?

*Abstract concept illustrated: set quotients as HITs, the set-truncation constructor, defining operations on quotient types via the eliminator.*

---

## Exercise A.4: Propositional Truncation in a Proof Assistant
*Domain: Proof Assistants / Program Verification / Logic*

**Setup:** The propositional truncation $\|A\|$ of a type $A$ is the HIT with one point constructor `|_| : A → ‖A‖` and one path constructor `squash : ∀ x y : ‖A‖, x = y` (plus a set-truncation condition if desired). It represents the *mere existence* of an element of $A$: a proof of $\|A\|$ says "there exists something of type $A$," but does not tell you which one.

This is practically important in proof assistants. When using a computational witness (e.g., finding a sorting network by an exhaustive search) to prove a theorem, the proof of the theorem should not depend on *which* witness was found — only on the *existence* of a witness. The propositional truncation enforces this separation.

**Questions:**
1. State the non-dependent elimination principle for $\|A\|$: to define a function $f : \|A\| \to B$, what data must you provide? What condition on $B$ is required? Give the formal type signature. Explain informally why this condition on $B$ is necessary.
2. Consider the following situation: `A = Σ(n : ℕ), IsPrime(n) × (n > 100)` is the type of primes greater than 100. You have a proof `p : ‖A‖` (there exists a prime greater than 100, but you won't say which). You want to prove `B = ‖Σ(n : ℕ), n > 50‖` (there exists a natural number greater than 50). Show how to use the elimination principle for `‖A‖` to produce a proof of `B` from `p`. Why is the "squash" condition not needed here?
3. (Extension) Consider the distinction between `Σ(f : ℕ → ℕ), Injective(f)` (a specific injective function, with its proof of injectivity) and `‖Σ(f : ℕ → ℕ), Injective(f)‖` (mere existence of an injective function). In what situations does it matter which you use? Give an example of a proof that can be written using `Σ` but not using `‖Σ‖`, and an example of a mathematical statement that is properly formalized using `‖Σ‖`.

*Abstract concept illustrated: propositional truncation as a HIT, the elimination principle for truncations, the difference between existence and mere existence.*

---

## Exercise A.5: The Free Monoid as a HIT
*Domain: Algebraic Structures / Verified Functional Programming / Category Theory*

**Setup:** The free monoid on a type $A$ is the type $\mathsf{List}(A)$ of lists of elements of $A$, with concatenation as the monoid operation and the empty list as the unit. Alternatively, it can be characterized by a universal property: it is the initial monoid equipped with a function $\eta : A \to M$ (the inclusion of generators). This universal property is often the most useful characterization when working with monoids in a type-theoretic setting.

As a HIT, the free monoid on $A$ can be presented as: the type `FM(A)` generated by
- Point constructor: `nil : FM(A)` (the empty list)
- Point constructor: `cons : A → FM(A) → FM(A)` (prepend an element)
- Path constructor: `assoc : ∀ x y z : FM(A), (x ++ y) ++ z = x ++ (y ++ z)` (associativity)
- Path constructor: `left-unit : ∀ x : FM(A), nil ++ x = x`
- Path constructor: `right-unit : ∀ x : FM(A), x ++ nil = x`
- Set-truncation constructor

**Questions:**
1. Write the non-dependent eliminator for `FM(A)` (the "fold" or "monoid homomorphism" principle). Given a monoid `(M, *, e)` and a function `h : A → M`, the eliminator produces a monoid homomorphism `FM(A) → M`. State the types of all the data required and all the equations that must be satisfied.
2. Use the eliminator to define a "length" function `len : FM(A) → ℕ`, showing that it is a monoid homomorphism (`len(x ++ y) = len(x) + len(y)` and `len(nil) = 0`). Where does each path constructor of `FM(A)` contribute to the definition?
3. (Extension) The free *group* on $A$ is the free monoid on $A \sqcup A^{-1}$ (the disjoint union of $A$ and a copy $A^{-1}$ representing formal inverses), with two additional path constructors: `inv-left : ∀ a : A, a⁻¹ ++ a = nil` and `inv-right : ∀ a : A, a ++ a⁻¹ = nil`. Write the HIT definition of the free group on $A$. Compute: what is the free group on the one-element type `1`? What is the free group on `Bool`?

*Abstract concept illustrated: HITs as presentations of algebraic structures, the universal property as an elimination principle, free objects as initial algebras.*

---

## Exercise A.6: The Torus as a HIT and Surface Classification
*Domain: Topology / Computer Graphics / Topological Data Analysis*

**Setup:** The torus $T^2 = S^1 \times S^1$ can be defined as a HIT:
- Point constructor: `base : T²`
- Path constructor: `p : base = base` (one direction of the torus)
- Path constructor: `q : base = base` (the other direction)
- 2-path constructor: `f : p · q = q · p` (the fundamental relation: the torus is "abelian")

This gives a completely combinatorial, HIT-based definition of the torus, with the correct homotopy type: $\pi_1(T^2) = \mathbb{Z} \times \mathbb{Z}$ (the integers in each direction) and $\pi_2(T^2) = 0$.

The "non-abelian" analog — replacing `p · q = q · p` with no relation — gives the HIT for the wedge $S^1 \vee S^1$, whose fundamental group is the free group on two generators.

**Questions:**
1. Write the full HIT specification for the torus $T^2$, including all point constructors, path constructors, and the 2-path constructor. State the non-dependent eliminator: to define a function `f : T² → B`, what data must you provide?
2. Using the torus HIT, define the map `π : T² → S¹` that "projects onto the first circle" (collapses the second `S¹` direction). What are `π(base)`, `ap_π(p)`, and `ap_π(q)`? What is `ap₂_π(f)` (the image of the 2-path `f`)?
3. (Extension) The Klein bottle $K$ is the HIT with one point, two path constructors `p, q : base = base`, and the 2-path `f : p · q = q · p⁻¹` (instead of `p · q = q · p`). Show that the map `p ↦ (1, 0), q ↦ (0, 1)` defines a homomorphism from `π₁(K)` (the fundamental group of the Klein bottle) to `ℤ² / ⟨(0,2)⟩`. This is the abelianization of the Klein bottle group. How does the 2-path constructor `f` determine this abelianization?

*Abstract concept illustrated: 2-path constructors in HITs, higher-dimensional path constructors, HITs for surfaces.*
