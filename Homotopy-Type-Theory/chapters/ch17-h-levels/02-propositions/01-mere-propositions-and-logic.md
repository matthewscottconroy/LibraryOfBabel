# 2.1 Mere Propositions and Logic

## What Is a Proposition in HoTT?

Classical logic has a clean notion of proposition: something that is either true or false. If it's true, you have a proof; if it's false, you have a refutation. And crucially: all proofs of the same true proposition are interchangeable — there's only one notion of "being true."

HoTT starts from dependent type theory, where types carry content. A type can have many elements, and different elements of the same type can be genuinely different (carry different information). This is good for mathematics — it lets us talk about "which proof" or "which witness."

But for *propositions*, we want the classical behavior: proof shouldn't matter. The type represents a truth value, and all proofs are the same.

**Definition 2.1 (Mere proposition / h-Prop).** A type $A$ is a *mere proposition* (or *h-prop*) if any two elements are equal:
$$\mathsf{isProp}(A) :\equiv \prod_{x, y : A}\, (x = y)$$

The word "mere" is Voevodsky's choice — it emphasizes that a proposition is "merely" a truth value, stripped of any additional information or computational content.

**The type $\mathsf{hProp}$** is the type of all mere propositions:
$$\mathsf{hProp} :\equiv \sum_{A : \mathsf{Type}}\, \mathsf{isProp}(A)$$

## Examples of Mere Propositions

**The empty type $\mathbf{0}$.** $\mathsf{isProp}(\mathbf{0})$ says: for any two elements of $\mathbf{0}$, they're equal. There are *no* elements of $\mathbf{0}$, so the universal quantifier is vacuously satisfied. The empty type is a proposition representing "False."

**The unit type $\mathbf{1}$.** $\mathsf{isProp}(\mathbf{1})$ says: $* = *$, which holds by reflexivity. The unit type is a proposition representing "True" (with the one element being the unique proof).

**Logical connectives.** If $P$ and $Q$ are propositions, then:
- $P \times Q$ is a proposition (both $P$ and $Q$ are true)
- $P \to Q$ is a proposition (if $P$ is true then $Q$ is true)
- $\neg P = P \to \mathbf{0}$ is a proposition ($P$ is false)

**Identity types of sets.** If $A$ is a set (defined in the next section), then $a =_A b$ is a proposition for any $a, b : A$. This is exactly the definition of a set.

**The truncation $\|A\|$.** For any type $A$, the propositional truncation $\|A\|$ (Section 5) is a proposition representing "there merely exists an element of $A$."

**Non-example: $\mathbb{N}$.** The natural numbers are not a proposition — there are many distinct natural numbers, and $0 \neq 1$ (they're not equal). In fact, $\mathbb{N}$ is a set (h-level 0), which is one level above propositions.

**Non-example: $\mathsf{Bool}$.** Similarly, $\mathsf{Bool}$ has two elements $\mathsf{true}$ and $\mathsf{false}$ that are not equal, so it's not a proposition.

## Propositions as Truth Values

The fundamental insight: the type of mere propositions $\mathsf{hProp}$ plays the role of the two-element set $\{\mathsf{True}, \mathsf{False}\}$ in classical logic, but in the constructive and proof-relevant setting of HoTT.

**Theorem 2.2 (Propositions are truth values).** If we assume the law of excluded middle (LEM: every proposition is either true or false), then $\mathsf{hProp}$ has exactly two elements up to equivalence: $\mathbf{0}$ (False) and $\mathbf{1}$ (True).

Without LEM, there are propositions that are neither provably true nor provably false — the "truth value" can be indeterminate (like undecidable statements in arithmetic).

**Logical operations on propositions.** For $P, Q : \mathsf{hProp}$:

| Logic | HoTT |
|---|---|
| $P \wedge Q$ | $P \times Q$ |
| $P \vee Q$ | $\|P + Q\|$ (propositional truncation of the sum) |
| $P \Rightarrow Q$ | $P \to Q$ |
| $\neg P$ | $P \to \mathbf{0}$ |
| $\forall x : A,\, P(x)$ | $\prod_{x:A} P(x)$ |
| $\exists x : A,\, P(x)$ | $\|\sum_{x:A} P(x)\|$ |

Note the disjunction and existence: they require propositional truncation because the sum type $P + Q$ and the Σ-type $\sum_{x:A}P(x)$ are *not* propositions in general — they carry the witness. Propositional truncation discards the witness and gives just the truth value.

This distinction between $\sum_{x:A} P(x)$ (constructive existence, with witness) and $\|\sum_{x:A} P(x)\|$ (mere existence, without witness) is one of the key logical insights of HoTT.

## Propositions and Contractibility

**Theorem 2.3.** A type $A$ is a proposition if and only if: for any two $x, y : A$, the type $x = y$ is contractible (has exactly one element).

*Proof.*

($\Rightarrow$) Suppose $A$ is a proposition, so we have $h : \prod_{x,y:A}(x = y)$. For any fixed $x, y : A$, $h(x, y) : x = y$ is a specific path. We claim $x = y$ is contractible with center $h(x, y)$.

For any $p : x = y$, we need $h(x, y) = p$. This follows from the fact that $A$ is a proposition: $A$ has the structure of a groupoid where all morphisms are inverses of each other (since any two paths between the same points in $A$ are equal — by applying the proposition condition to the path type itself, which requires induction). The detailed proof uses J on $p$ and the structure of paths in $A$. $\square$

Wait — this requires knowing something about the path types of $A$, which are the path types of a proposition. Let's be more careful.

**Lemma 2.4.** If $A$ is a proposition, then for any $x, y : A$ and $p, q : x = y$, we have $p = q$.

*Proof.* By the characterization of paths in $\Sigma$-types (and the contractibility of the total path space), the path space $x = y$ is a retract of the total path space over $x$, which is contractible. Specifically, transport along $h(x, y) : x = y$ gives a function... 

The cleaner proof: Consider the function $c : A \to A \times A$ given by $c(a) = (a, a)$. Since $A$ is a proposition, the diagonal $c$ is surjective (any $(x, y)$ has $x = y$, so $(x, y) = c(x)$). The fiber of $c$ over $(x, y)$ is the type $x = y$, and... this is getting circular.

The standard proof: Let $f : \prod_{x:A}(x = x)$ be the reflexivity section. Since $A$ is a proposition, $h : \prod_{x,y:A}(x=y)$. For any $p : x = y$, the path $h(x, x) \cdot p : x = y$ is a path, and $h(x, y)$ is another. By the path space of a proposition (which we're trying to prove), they're equal. The circularity is resolved by a "Whitehead argument": use J to reduce to the case $p = \mathsf{refl}_x$, and then $h(x, x) = \mathsf{refl}_x$ follows from... 

Actually, the standard argument goes: since $A$ is a proposition with $h : \prod_{x,y:A}(x=y)$, define for any $x : A$:
$$K_x : \prod_{y:A}(x = y) :\equiv h(x)$$

For any $p : x = y$, we want $K_x(x) \cdot p = K_x(y)$ (naturality of $K_x$ as a function $A \to (x = -)$). By J on $p$, it suffices to check $K_x(x) \cdot \mathsf{refl}_x = K_x(x)$, which holds by the right unit law.

But $K_x(y) = K_x(x) \cdot p$ for any $p : x = y$. In particular, for any two paths $p, q : x = y$:
$$K_x(x) \cdot p = K_x(y) = K_x(x) \cdot q$$

Canceling $K_x(x)$ on the left (it's invertible), we get $p = q$. $\square$

**Corollary 2.5.** If $A$ is a proposition, then $A$ is a set (h-level 0): all identity types $x = y$ are propositions (they have at most one element, by Lemma 2.4). In fact, $A$ is an $n$-type for all $n \geq -1$.

This says: propositions are "below sets" in the h-level hierarchy, which makes sense. If a type has at most one element, then all path types (between the one element and itself) are also propositions, and so on up the tower.

## Propositions and Contractibility (Revisited)

**Theorem 2.6.** A type $A$ is contractible if and only if $A$ is a proposition and $A$ is inhabited.

*Proof.*

($\Rightarrow$) If $A$ is contractible with center $c$, then $A$ is inhabited (by $c$). $A$ is a proposition: for any $x, y : A$, $c = x$ (from the contracting homotopy) and $c = y$, so $x = c^{-1} \cdot c \cdot y = ...$ wait let me be careful: we have $h(x) : c = x$ and $h(y) : c = y$, so $h(x)^{-1} \cdot h(y) : x = y$.

($\Leftarrow$) If $A$ is a proposition and has an element $a : A$, then for any $x : A$, the proposition gives $a = x$. So $(a, \lambda x.\, \mathsf{prop}(a, x))$ witnesses contractibility. $\square$

**The takeaway:** Propositions are "contractible or empty." A proposition is either False ($\mathbf{0}$, empty, no inhabitants) or True (inhabited, and then it's contractible).

This exactly matches classical logic: a proposition is either false (no proof) or true (a proof, and all proofs are the same).

## Closure Properties of Propositions

**Theorem 2.7 (Products of propositions).** If $P$ and $Q$ are propositions, then $P \times Q$ is a proposition.

*Proof.* Let $(p_1, q_1), (p_2, q_2) : P \times Q$. By paths in products (Chapter 16), $(p_1, q_1) = (p_2, q_2)$ iff $p_1 = p_2$ and $q_1 = q_2$. Since $P$ and $Q$ are propositions, both hold. $\square$

**Theorem 2.8 (Function types to propositions).** If $Q$ is a proposition, then $A \to Q$ is a proposition for any $A$.

*Proof.* Let $f, g : A \to Q$. By funext, $f = g$ iff $f(a) = g(a)$ for all $a : A$. Since $Q$ is a proposition, $f(a) = g(a)$ for all $a$. So $f = g$. $\square$

**Corollary 2.9.** If $B : A \to \mathsf{hProp}$ is a predicate taking values in propositions, then $\prod_{a:A} B(a)$ is a proposition.

This follows from Theorem 2.8 applied to $Q = B(a)$ at each $a$.

**Theorem 2.10 (Propositions closed under truncation).** $\mathsf{isProp}(A)$ is itself a proposition for any $A$.

*Proof.* $\mathsf{isProp}(A) = \prod_{x,y:A}(x=y)$. This is a product (over $A \times A$) of types that are propositions when $A$ is a proposition (by Lemma 2.4). But wait — we're trying to prove $\mathsf{isProp}(A)$ is a proposition, not assuming $A$ is a proposition.

The full proof: Two elements $p, q : \mathsf{isProp}(A)$ are functions $p, q : \prod_{x,y:A}(x=y)$. By funext (applied twice), $p = q$ iff $p(x)(y) = q(x)(y)$ for all $x, y : A$. Each $p(x)(y)$ and $q(x)(y)$ are paths in $A$. Since $p$ itself says "$A$ is a proposition," $p(x)(y) = q(x)(y)$ follows. So $p = q$. $\square$

## The Singleton Contract

There's a beautiful way to characterize contractibility using propositions:

**Theorem 2.11 (Singleton = contractible).** For any $a : A$, the type $\sum_{x:A}(a = x)$ is contractible. And moreover, it is the *propositional version* of "there is exactly one element equal to $a$."

Generalizing: a proposition is a type with at most one element. A contractible type is a proposition with exactly one element. The two facts together characterize the bottom of the h-level hierarchy:
- h-level $-2$ (contractible): exactly one element
- h-level $-1$ (proposition): at most one element
- h-level $0$ (set): paths between elements are propositions (at most one path)

## Why "Mere"?

Let's reflect on the word "mere" in "mere proposition." 

In classical logic, propositions are just truth values — they have no internal structure, no "meaning" beyond true or false. In constructive type theory, types can carry substantial computational content. A "proposition" $A$ might have many different proofs, each computing something different.

HoTT makes a distinction:
- A type $A$ carries *data* — its elements are meaningful, different elements are different.
- A *mere proposition* $A$ carries only *truth* — its elements are all equal, proof doesn't matter.

The "mere" signals that we're stripping away the computational content and retaining only the truth value. $\|A\|$ (the propositional truncation) is the "mere content" of $A$ — it knows whether $A$ is inhabited, but not how.

This is a profound logical distinction with computational consequences. When writing a program, you often care about the witness: not just "there exists a path" but "what is the path." Propositional truncation lets you forget the witness when it genuinely doesn't matter, while preserving it when it does.

## Propositions in Proof Assistants

In Lean 4, propositions are handled by the `Prop` universe:
```lean
-- Prop is the universe of propositions
example : Prop := 1 = 1
example : Prop := ∀ n : Nat, n + 0 = n

-- propext: propositions are equal iff logically equivalent
axiom propext : ∀ {a b : Prop}, (a ↔ b) → a = b
```

In Agda with `--without-K` and `--safe`:
```agda
-- isProp is defined as:
isProp : ∀ {ℓ} → Set ℓ → Set ℓ
isProp A = ∀ (x y : A) → x ≡ y
```

The `--without-K` flag is crucial: it disables the K axiom (Uniqueness of Identity Proofs), which would make every type a set. Without K, not every type is a proposition, and the h-level hierarchy is non-trivial.

The distinction between propositions and data types is central to practical proof assistant use: propositions can be marked as "proof-irrelevant" (the runtime can erase them), while data types retain their computational content.
