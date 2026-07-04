# Well-Founded Induction

> "Termination is not obvious. Every loop you write is a claim about the future."
> — Anonymous

## Beyond the Natural Numbers

Ordinary mathematical induction works beautifully for properties of natural numbers, and structural induction extends it to trees, lists, and other recursively defined types. But what about proofs over **arbitrary partial orders** — over ordinals, over multisets, over terms in a term rewriting system?

The unifying concept is **well-foundedness**. A relation is well-founded if there are no infinite descending chains. In such a setting, you can always argue "consider a minimal counterexample" and derive a contradiction — or equivalently, prove that any property that propagates upward must hold everywhere.

## Well-Founded Relations

A binary relation $\prec$ on a set $A$ is **well-founded** if there is no infinite descending chain:
$$\ldots \prec a_2 \prec a_1 \prec a_0$$

Equivalently (classically): every non-empty subset of $A$ has a $\prec$-minimal element — an element $m$ such that no element of the subset is strictly $\prec$-below $m$.

**Examples of well-founded relations:**
- $<$ on $\mathbb{N}$: the basis for ordinary induction
- The proper subset relation $\subsetneq$ on finite sets: no set is a proper subset of itself, and chains terminate because size decreases
- The "structurally smaller" relation on terms: a subterm is smaller than the term containing it
- The **multiset ordering** on multisets: replace one element by finitely many smaller elements
- The **lexicographic ordering** on $\mathbb{N}^k$: pairs/tuples ordered dictionary-style
- The **ordinal order** $<$ on ordinals: this is the prototypical well-order

**Non-examples:**
- $<$ on $\mathbb{Z}$: the chain $\ldots < -3 < -2 < -1 < 0$ descends forever
- $<$ on $\mathbb{Q}$: between any two rationals is another; no minimum element in many subsets
- Divisibility on $\mathbb{Z}$: not well-founded (consider $\ldots | 8 | 4 | 2 | 1$? No, wait — 1 has no proper divisors, so this is actually well-founded on $\mathbb{N}$)

## The Principle of Well-Founded Induction

**Theorem**: Let $\prec$ be a well-founded relation on $A$, and let $P$ be a property of elements of $A$. If:
$$\forall x \in A,\, (\forall y \prec x,\, P(y)) \implies P(x)$$

Then $\forall x \in A,\, P(x)$.

The hypothesis says: to prove $P(x)$, you may assume $P(y)$ holds for all $y$ that are $\prec$-below $x$. This is the **induction hypothesis** in the well-founded setting.

This subsumes several familiar cases:
- **Ordinary induction**: $\prec$ is $<$ on $\mathbb{N}$, but restricted to the immediate predecessor (strong induction uses the full order)
- **Strong induction**: $\prec$ is $<$ on $\mathbb{N}$, IH assumes $P$ holds for all smaller naturals
- **Structural induction**: $\prec$ is "is a proper subterm of," IH assumes $P$ holds for all subterms

## Termination Proofs: The Killer Application

Well-founded induction is the mathematical foundation for **termination proofs** of recursive programs and rewriting systems. The idea:

**To prove a recursive function terminates:**
1. Find a well-founded relation $\prec$ on the inputs
2. Show that each recursive call receives an input that is $\prec$-smaller than the current input
3. By well-foundedness, the chain of calls must eventually stop

**Example: Euclidean Algorithm**

```python
def gcd(a, b):
    if b == 0:
        return a
    return gcd(b, a % b)
```

Why does this terminate? We need a well-founded measure. Consider the value of $b$ at each recursive call:
- Initial call: $(a, b)$ with $b > 0$
- Recursive call: $(b, a \bmod b)$

Key fact: $a \bmod b < b$ (by definition of modulo). So the second argument *strictly decreases* at each call. Since the second argument is a non-negative integer and $<$ is well-founded on $\mathbb{N}$, the recursion must terminate.

This is a **lexicographic** argument: we are measuring the pair $(a, b)$ and showing the second component decreases. If we needed to use a two-component measure, we would use the lexicographic order on pairs, which is well-founded when both components range over a well-founded set.

**Example: Ackermann Function**

```python
def ack(m, n):
    if m == 0:   return n + 1
    if n == 0:   return ack(m - 1, 1)
    return ack(m - 1, ack(m, n - 1))
```

This is famously not primitive recursive, but it *does* terminate. The well-founded measure is the **lexicographic order on pairs** $(m, n)$:
- When $m = 0$: no recursive call, terminates immediately
- When $n = 0$: $(m-1, 1) <_{\text{lex}} (m, 0)$ ✓
- Otherwise: we first need $\text{ack}(m, n-1)$ — here $(m, n-1) <_{\text{lex}} (m, n)$ ✓. Then we call $\text{ack}(m-1, \text{ack}(m, n-1))$ — and $(m-1, k) <_{\text{lex}} (m, n)$ for any $k$. ✓

By well-founded induction on $<_{\text{lex}}$, the Ackermann function terminates.

## Ordinal Assignments

For complex recursive functions, especially in mathematical logic and proof theory, termination arguments use **ordinal numbers** as the measure. Ordinals generalize $\mathbb{N}$ to transfinite well-orders:

$$0 < 1 < 2 < \ldots < \omega < \omega + 1 < \omega + 2 < \ldots < \omega \cdot 2 < \ldots < \omega^2 < \ldots < \omega^\omega < \ldots$$

Every ordinal has an immediate "next" element (its successor), and limit ordinals (like $\omega$) have no immediate predecessor but are limits of smaller ordinals. All of these are well-ordered.

**Proof-theoretic ordinals**: The *consistency strength* of a formal system can be measured by the smallest ordinal it cannot prove well-founded. For example:
- Peano Arithmetic has proof-theoretic ordinal $\varepsilon_0 = \omega^{\omega^{\omega^{\cdots}}}$
- ZF set theory has a much larger proof-theoretic ordinal
- Gödel showed that no sufficiently powerful system can prove its own consistency (ch10) — which is related to the fact that these ordinals cannot be proven well-founded within the system itself

## In Lean 4

Lean 4's termination checker is built on well-founded recursion. When you write a recursive function, Lean either finds a termination argument automatically or asks you to supply one:

```lean
-- Lean's termination checker handles this automatically
-- by finding that b decreases at each recursive call
def gcd : ℕ → ℕ → ℕ
  | a, 0 => a
  | a, b => gcd b (a % b)

-- For more complex termination, you supply a measure explicitly:
-- termination_by measure fun (a, b) => b

-- Structural recursion (automatically recognized):
def length : List α → ℕ
  | [] => 0
  | _ :: t => length t + 1
-- Termination follows from structural decrease on the list
```

When Lean cannot automatically verify termination, you use `termination_by` to provide a well-founded measure:

```lean
-- Ackermann with explicit measure
def ack : ℕ → ℕ → ℕ
  | 0,   n   => n + 1
  | m+1, 0   => ack m 1
  | m+1, n+1 => ack m (ack (m+1) n)
termination_by m n => (m, n)   -- lexicographic on ℕ × ℕ
```

Lean's `termination_by` clause specifies the well-founded measure. The lexicographic order on `ℕ × ℕ` is automatically inferred to be well-founded.

## The Philosophical Point

Well-foundedness is not just a technical convenience — it reflects a deep truth about mathematical existence. A well-founded relation ensures that any reasoning about "minimal" objects is meaningful: there always *is* a minimal element, so you can safely say "consider the smallest counterexample" without fear that no such thing exists.

In constructive mathematics and type theory, well-founded recursion is what gives recursive definitions their *meaning*. A function defined by recursion on a well-founded order is guaranteed to have a unique value at every input — otherwise it would be undefined at elements in a descending chain that never bottoms out.

## Exercises
See [problems/ch07_induction_and_recursion/03_well_founded_exercises.md](../../../problems/ch07_induction_and_recursion/03_well_founded_exercises.md)
