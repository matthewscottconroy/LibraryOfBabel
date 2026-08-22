# Applied Exercises

The h-level hierarchy is not a classification for its own sake — it tells you what logical and computational principles are available in a given context. Working with propositions ($(-1)$-types) means proof-irrelevance: two proofs of the same proposition are equal, and you can freely discard proof terms. Working with sets ($0$-types) means classical set-theoretic reasoning applies, and the K axiom holds. Working above that requires tracking higher structure explicitly. The exercises below make these distinctions concrete, applying h-level theory to programming, logic, and mathematics.

---

## Exercise C.1: Classifying Types in a Program
*Domain: Type-Driven Development / Functional Programming*

**Setup:** In a dependently typed language, knowing the h-level of a type tells you what logical and computational operations are available on it. A proposition can be used as a boolean guard (it either holds or doesn't, and the proof term doesn't matter). A set can be stored in a data structure without worrying about which "copy" of an element you have. Understanding h-levels of the types in your program is like understanding the algebra of your data.

Consider the following types from a simple e-commerce domain:
- `OrderStatus : Type` — an enumeration: `Pending`, `Shipped`, `Delivered`, `Cancelled`
- `UniqueOrderId : Type` — a record wrapping a natural number
- `HasPendingOrder : Customer → Type` — the proposition "this customer has a pending order"
- `AllOrdersShipped : Customer → Type` — the proposition "all of this customer's orders are shipped"
- `OrderHistory : Customer → Type` — the list of all orders (with timestamps) for a customer

**Questions:**

1. For each of the five types above, determine its h-level. For `OrderStatus` and `UniqueOrderId`, prove they are h-sets by constructing a decidable equality. For `HasPendingOrder` and `AllOrdersShipped`, argue why they are propositions (h-level $-1$) — what would it mean if they were *not* propositions? For `OrderHistory`, explain why it is a set but not a proposition.

2. The principle of unique choice says: if $P : A \to \mathsf{Type}$ is a family of propositions and $\prod_{a:A} \|P(a)\|$, then $\prod_{a:A} P(a)$. In the e-commerce context: if `HasPendingOrder` is a proposition (which it is), and you know that every customer *merely* has a pending order (propositional existence), can you extract a concrete pending order for each customer? Explain why this works when `HasPendingOrder c` is a proposition but would require the axiom of choice if `OrderHistory c` were in its place.

3. Suppose you want to define a function `mostRecentOrder : Customer → Maybe Order` that returns the most recent order, or `Nothing` if there are no orders. The return type `Maybe Order` is an h-set. Show that `mostRecentOrder` can be defined without any choice axiom, even though `OrderHistory c` is a list (non-propositional existence). What is the key difference between this and the situation in Question 2? (Hint: the issue is whether we are choosing from a *propositionally* inhabited fiber or a *computationally* defined one.)

*Abstract concept illustrated: The distinction between $\|P\|_{-1}$ (mere proposition, propositional truncation of $P$) and $P$ itself; the principle of unique choice; the h-level classification as a practical guide to what logical operations are sound.*

---

## Exercise C.2: Propositional Truncation as a Quotient Type
*Domain: Type Theory Implementation / Programming Language Design*

**Setup:** In a language with quotient types (types defined by imposing an equivalence relation), propositional truncation $\|A\|$ can be constructed directly: define $\|A\|$ as $A$ quotiented by the total relation (all elements are equivalent). This gives a type with at most one element up to equality. Languages like Lean 4 (with its `Quotient` type), Agda with cubical mode, or Coq with the `Quotient` library support quotient types directly.

**Questions:**

1. Define propositional truncation using quotient types. In pseudocode or actual Lean 4/Agda:
   ```
   ||A|| := A / (λ a b → True)
   ```
   where we quotient by the trivially true relation (every pair $(a, b)$ is related). Verify the two properties: (a) `|a| : ||A||` for any `a : A` (the constructor), and (b) `||A||` is a proposition: for any `x y : ||A||`, `x = y`. Prove (b) using the quotient induction principle.

2. The universal property of propositional truncation says: for any proposition $P$ and function $f : A \to P$, there is a unique function $\bar{f} : \|A\| \to P$ with $\bar{f}(|a|) = f(a)$. Prove this using the quotient type construction: the uniqueness follows from $P$ being a proposition (any two functions $\|A\| \to P$ are equal, since $P$ is a proposition), and existence follows from the quotient elimination rule (you can define $\bar{f}$ because $f$ respects the quotient relation — trivially, since the relation is total and $P$ is a proposition, so $f(a) = f(b)$ for any $a, b : A$ when $P$ is a proposition).

3. Implement set truncation $\|A\|_0$ using quotient types: $\|A\|_0 := A / \sim$ where $a \sim b$ if and only if there merely exists a path $\|a = b\|_{-1}$ between them (i.e., $a$ and $b$ are in the same connected component of the identity type). Define the universal property: $\|A\|_0 \to B$ (for $B$ a set) corresponds to functions $A \to B$ that are "constant on connected components." As a concrete example, show that $\|\mathbb{S}^1\|_0 \cong \mathbf{1}$ (the set truncation of the circle is the unit type, since the circle is connected), while $\|\mathbb{S}^1\|_{-1} \cong \mathbf{1}$ also holds (the circle is merely inhabited).

*Abstract concept illustrated: Propositional truncation $\|A\|_{-1}$ and set truncation $\|A\|_0$ as quotient constructions; the universal property as the defining property; the relationship between truncation levels and quotient types.*

---

## Exercise C.3: Existential vs. Computational Witnesses
*Domain: Algorithm Design / Logic Programming*

**Setup:** One of the most practically important distinctions in HoTT is between $\exists x. P(x)$ (mere existence: $\|\Sigma_{x:A} P(x)\|_{-1}$) and $\Sigma_{x:A} P(x)$ (computational existence: a specific witness with a proof). In constructive logic, these are different: $\exists$ says "there is some $x$, but we don't need to know which one," while $\Sigma$ says "here is a specific $x$, together with its proof." The difference is subtle but has major consequences for algorithm design.

**Questions:**

1. Consider a sorting algorithm. Define `IsSorted : List ℕ → Type` and `IsPermutationOf : List ℕ → List ℕ → Type`. The specification of a sort is:
   - **Existential specification:** $\exists\ (\text{result} : \mathsf{List}\ \mathbb{N})\,.\, \mathsf{IsSorted}(\text{result}) \times \mathsf{IsPermutationOf}(\text{input}, \text{result})$
   - **Computational specification:** $\Sigma\ (\text{result} : \mathsf{List}\ \mathbb{N})\,.\, \mathsf{IsSorted}(\text{result}) \times \mathsf{IsPermutationOf}(\text{input}, \text{result})$
   
   Explain the difference. The existential specification says "a sorted permutation exists," while the computational specification says "here is a sorted permutation, explicitly." Why does the computational specification give you a *function* from input to output, while the existential one only gives you a proposition?

2. When `IsSorted` is a proposition (which it is, when defined as $\prod_{i < |xs| - 1} xs[i] \leq xs[i+1]$), and `IsPermutationOf` is also a proposition, the type $\Sigma\ (\text{result} : \mathsf{List}\ \mathbb{N})\,.\, \mathsf{IsSorted}(\text{result}) \times \mathsf{IsPermutationOf}(\text{input}, \text{result})$ is *not* itself a proposition — it is an h-set (a list of natural numbers). Explain why: the carrier type `List ℕ` is a set, and a $\Sigma$-type over a set with propositional fibers is a set. (Use the general lemma: if $A$ is an $n$-type and $B : A \to \mathsf{Type}$ is a family of $m$-types, then $\Sigma_{a:A} B(a)$ is a $\max(n, m)$-type.)

3. The principle of unique choice says: if the fibers $B(a)$ are propositions and $\prod_{a:A}\|B(a)\|$, then $\prod_{a:A} B(a)$. Apply this to the sorting example: if you know that every list *merely* has a sorted permutation (existential specification), and if `IsSorted` and `IsPermutationOf` are both propositions, can you extract a *function* `sort : List ℕ → List ℕ`? Discuss carefully: the fibers of your $\Sigma$-type are sets (not propositions), so the principle of unique choice does *not* directly apply. What additional assumption would you need? (Answer: you need the sorted permutation to be *unique*, i.e., the $\Sigma$-type to be a proposition — which it is only if the sorting result is unique, as in the case of sorting with a strict total order on $\mathbb{N}$.)

*Abstract concept illustrated: The distinction between $\exists$ (propositional truncation of $\Sigma$) and $\Sigma$ (computational); the principle of unique choice; h-level arithmetic for $\Sigma$-types; the role of uniqueness in extracting computational content from existential statements.*

---

## Exercise C.4: Constructing $\mathbb{Z}$ as a Set Truncation
*Domain: Mathematics / Type Theory*

**Setup:** The integers $\mathbb{Z}$ can be defined as the set of equivalence classes of pairs $(m, n) : \mathbb{N} \times \mathbb{N}$ under the relation $(m, n) \sim (m', n')$ iff $m + n' = m' + n$ (informal: $(m, n)$ represents $m - n$). In HoTT, this construction uses the set truncation: $\mathbb{Z} := \|\Sigma_{(m,n) : \mathbb{N} \times \mathbb{N}} \top\|_{\sim}$ where the quotient is by $\sim$.

More directly: $\mathbb{Z} = (\mathbb{N} \times \mathbb{N}) / \sim$ where the quotient is the set-quotient (the set truncation of the type of $\sim$-equivalence classes).

**Questions:**

1. Define the equivalence relation $\sim$ on $\mathbb{N} \times \mathbb{N}$ and verify it is a proposition-valued relation: for any $(m, n)$ and $(m', n')$, the type $(m, n) \sim (m', n')$ (which is $m + n' = m' + n$) is a proposition (it is a statement about natural numbers, which are an h-set). Why is it important that $\sim$ be proposition-valued for the quotient to be an h-set?

2. Define addition on $\mathbb{Z}$ using the quotient induction principle: $[(m, n)] + [(m', n')] := [(m + m', n + n')]$. To show this is well-defined, you need to prove that if $(m, n) \sim (p, q)$ and $(m', n') \sim (p', q')$, then $(m + m', n + n') \sim (p + p', q + q')$. Prove this using arithmetic on $\mathbb{N}$ and the commutativity and associativity of addition. Conclude that $+_{\mathbb{Z}}$ is well-defined on the quotient.

3. Prove that $\mathbb{Z}$ is an h-set (not just a set in the informal sense, but `isSet ℤ` in Agda). Use the following strategy: (a) $\mathbb{N} \times \mathbb{N}$ is an h-set (it is the product of h-sets); (b) a quotient of an h-set by a proposition-valued equivalence relation is an h-set (the set truncation of an h-set is an h-set). Identify which theorem from Section 3 of this chapter you are applying in step (b). Then state and prove: `isSet ℤ` follows from (b) applied to the relation $\sim$.

*Abstract concept illustrated: Quotient types as a special case of set truncation; the closure of h-sets under quotients by proposition-valued equivalence relations; the role of h-levels in ensuring that constructions of familiar mathematical objects (integers, rationals) yield types with the expected properties.*

---

## Exercise C.5: The Axiom of Choice in HoTT
*Domain: Logic / Foundations of Mathematics*

**Setup:** The axiom of choice (AC) in HoTT must be stated carefully with h-levels. The naive statement "if every fiber is nonempty, there is a choice function" is provable when "nonempty" means *computationally* nonempty (the $\Sigma$-type is inhabited) but is an independent axiom when "nonempty" means *merely* nonempty (the propositional truncation is inhabited). The key theorem: AC holds when the index type $A$ is a set and the fibers are merely nonempty (but may have interesting h-level themselves).

More precisely, the HoTT Book proves:

**Theorem (AC for Sets).** If $A$ is a set and $P : A \to \mathsf{Type}$ is a family with $\prod_{a:A} \|P(a)\|$, and if each $P(a)$ is a *set*, then $\|\prod_{a:A} P(a)\|$.

**Questions:**

1. State the "wrong" version of AC that fails in HoTT: $\big(\prod_{a:A} \|P(a)\|_{-1}\big) \to \prod_{a:A} P(a)$. Find a specific counterexample using the circle $\mathbb{S}^1$: let $A = \mathbf{1}$ (the unit type), $P(\star) = \Omega \mathbb{S}^1 = (\mathsf{base} = \mathsf{base})$ (the loop space of the circle). Show that $\|\Omega \mathbb{S}^1\|_{-1}$ is inhabited (since $\mathsf{refl}$ is a loop), but that providing a *specific* element of $\Omega \mathbb{S}^1$ requires making a choice (which loop?) that is not canonical.

2. Prove the "easy" version of AC: $\big(\prod_{a:A} P(a)\big) \to \prod_{a:A} P(a)$ (trivially). Now prove the "semi-interesting" version: if $P : A \to \mathsf{Type}$ is a family of *propositions*, then $\big(\prod_{a:A} \|P(a)\|_{-1}\big) \to \prod_{a:A} P(a)$. This is an instance of the principle of unique choice: since $P(a)$ is already a proposition, $\|P(a)\|_{-1} \simeq P(a)$, so the hypothesis directly gives what you want.

3. Explain why the axiom of choice fails for families of *sets* over a set base without the propositional truncation on the conclusion. As a specific failure: let $A = \mathbf{2}$ (a two-element type, which is a set), and let $P(\mathsf{false}) = \mathbf{1}$ and $P(\mathsf{true}) = \mathbf{2}$ (the two-element type). We have $\|P(\mathsf{false})\|_{-1} = \mathbf{1}$ and $\|P(\mathsf{true})\|_{-1} = \mathbf{1}$, so the hypothesis $\prod_{a:\mathbf{2}} \|P(a)\|_{-1}$ is satisfied. How many elements does $\prod_{a:\mathbf{2}} P(a)$ have? (Answer: two — $(\star, \mathsf{false})$ and $(\star, \mathsf{true})$.) Does providing a specific element of this dependent function type require a "choice"? Why is this type a set (h-level 0) rather than a proposition?

*Abstract concept illustrated: The axiom of choice in HoTT is h-level sensitive; it holds for families of sets over a set index (with merely inhabited fibers, and propositional truncation on the conclusion), but fails in general; the h-level of fibers determines when choice is valid.*

---

## Exercise C.6: Hedberg's Theorem in a Proof Assistant
*Domain: Formal Verification / Proof Engineering*

**Setup:** Hedberg's theorem (`DecEq A → isSet A`) is one of the workhorses of HoTT formalization: it gives h-set status to most "ordinary" types (natural numbers, booleans, lists, trees) without requiring a custom proof for each. The theorem's proof uses the "constant endofunction" technique: decidable equality gives a constant retraction on each identity type, and any type with a constant retraction is a proposition.

**Questions:**

1. In Agda, implement the key lemma: if `f : A → A` is a constant function (satisfying `const-f : ∀ x y → f x ≡ f y`) and `f` is also a *retraction* (there exists `g : A → A` with `g ∘ f ≡ id`), then `A` is a proposition. Formally: `const-retract : (f : A → A) → (∀ x y → f x ≡ f y) → (∀ x → f (f x) ≡ f x) → isProp A`.

   [Note: the "retraction" condition here is the idempotence `f ∘ f ≡ f`, which is weaker than having a separate retraction. The full Hedberg argument uses idempotence of the canonical retraction constructed from decidable equality.]

2. Using the key lemma, prove Hedberg's theorem: if `d : DecEq A` (decidable equality for $A$), construct a constant idempotent endofunction on each identity type `x ≡ y`, and apply the lemma to conclude `isSet A`. The construction: for each `x y : A`, define `f : x ≡ y → x ≡ y` by cases on `d x y`:
   - If `d x y = inl p`, then `f _ = p` (constant: always return the decided path `p`)
   - If `d x y = inr np`, then `f q = absurd (np q)` (there are no paths, so any `q : x ≡ y` leads to absurdity — the function cannot be called on a closed term)
   Verify that `f` is constant and idempotent, and conclude.

3. Apply Hedberg's theorem to conclude `isSet (List A)` when `A` is an h-set with decidable equality. You will need to prove `DecEq (List A)` first (by induction on list structure). Then use Hedberg's theorem. Discuss: does the conclusion `isSet (List A)` require `A` to have decidable equality, or just `isSet A`? (It turns out `isSet A` suffices, using a more general argument — the "K rule" for inductive types with injective constructors — but the Hedberg route via decidable equality is more elementary.)

*Abstract concept illustrated: Hedberg's theorem characterizes h-sets among types with decidable equality; the constant endofunction technique is the key lemma, and it generalizes to truncation questions throughout HoTT.*
