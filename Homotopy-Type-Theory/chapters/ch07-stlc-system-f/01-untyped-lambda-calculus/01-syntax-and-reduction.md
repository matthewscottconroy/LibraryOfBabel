# 1.1 The Untyped Lambda Calculus: Syntax and Reduction

## Church's Minimal Language

Alonzo Church invented the lambda calculus in the 1930s as a foundation for mathematics, before the development of computer science. He wanted to formalize the notion of "function" as purely as possible, without relying on set theory.

The result is strikingly minimal: the lambda calculus has only three constructs.

## Syntax of Untyped Lambda Terms

**Definition.** The set $\Lambda$ of *lambda terms* is defined inductively:
- **Variable:** If $x$ is a variable, then $x \in \Lambda$.
- **Abstraction:** If $x$ is a variable and $t \in \Lambda$, then $\lambda x. t \in \Lambda$.
- **Application:** If $t, s \in \Lambda$, then $(t\, s) \in \Lambda$.

That's the entire syntax. No constants, no base types, no numbers or booleans (yet — they'll be encoded).

**Conventions:**
- Application is left-associative: $f\, a\, b\, c = ((f\, a)\, b)\, c$.
- The body of a lambda abstraction extends as far right as possible: $\lambda x. f\, x = \lambda x. (f\, x)$.
- Abbreviation: $\lambda xyz. t$ for $\lambda x. \lambda y. \lambda z. t$.

**Free and bound variables:**
- $x$ is *bound* in $\lambda x. t$ — it's a formal parameter.
- $x$ is *free* in $t$ if it appears outside any $\lambda x$ binding it.
- $\text{FV}(t)$ = the set of free variables of $t$.

**Alpha equivalence:** $\lambda x. x = \lambda y. y$ (renaming bound variables doesn't change a function). Terms are considered equal up to alpha renaming.

## Substitution

The critical operation is *substitution*: $t[s/x]$ replaces free occurrences of $x$ in $t$ with $s$.

$$x[s/x] = s$$
$$y[s/x] = y \quad (y \neq x)$$
$$(t_1\, t_2)[s/x] = (t_1[s/x])(t_2[s/x])$$
$$(\lambda x. t)[s/x] = \lambda x. t \quad (x \text{ is bound, so no free occurrences to replace})$$
$$(\lambda y. t)[s/x] = \lambda y'. t[y'/y][s/x] \quad (y' \text{ fresh, to avoid capture})$$

The last case (capture-avoiding substitution) is technically important. When substituting $s$ for $x$ into $\lambda y. t$, if $y$ appears free in $s$, we rename $y$ to a fresh variable $y'$ to avoid $y$ getting "captured" by the lambda.

## Beta Reduction

The computation rule of the lambda calculus:

**$\beta$-reduction:** $(\lambda x. t)\, s \to_\beta t[s/x]$

A *$\beta$-redex* (reducible expression) is any subterm of the form $(\lambda x. t)\, s$.

$\to_\beta$ denotes one-step reduction; $\to_\beta^*$ denotes the reflexive-transitive closure (zero or more steps).

**Example reductions:**
- $(\lambda x. x)\, a \to_\beta a$ (identity)
- $(\lambda x. \lambda y. x)\, a\, b \to_\beta (\lambda y. a)\, b \to_\beta a$ (constant function, K combinator)
- $(\lambda f. \lambda x. f\, (f\, x))\, g\, y \to_\beta (\lambda x. g\, (g\, x))\, y \to_\beta g\, (g\, y)$ (applying twice)

## The Non-Termination Problem

In the untyped lambda calculus, terms can fail to terminate.

**$\Omega$ (the diverging term):**
$$\Omega = (\lambda x. x\, x)(\lambda x. x\, x)$$
$$\to_\beta (\lambda x. x\, x)(\lambda x. x\, x) = \Omega$$

$\Omega$ reduces to itself in one step. Any reduction sequence from $\Omega$ is infinite. This is non-termination, or $\bot$ computationally.

**The Y combinator (fixed-point combinator):**
$$Y = \lambda f. (\lambda x. f\, (x\, x))(\lambda x. f\, (x\, x))$$

For any $f$: $Y\, f \to_\beta f\, (Y\, f)$. This means $Y\, f$ is a fixed point of $f$ — it satisfies $Y\, f = f\, (Y\, f)$. The Y combinator implements general recursion.

With the Y combinator, the untyped lambda calculus is Turing-complete: any recursive function can be encoded.

But for foundational purposes (logic, proof theory), non-termination is fatal. A non-terminating "proof" of $\bot$ would make the system inconsistent.

## Church Numerals (Untyped)

Even without base types or primitives, we can encode data structures. Church numerals encode natural numbers as functions:

$$\underline{0} = \lambda f. \lambda x. x \quad (\text{apply } f \text{ zero times})$$
$$\underline{1} = \lambda f. \lambda x. f\, x \quad (\text{apply } f \text{ once})$$
$$\underline{n} = \lambda f. \lambda x. f^n\, x \quad (\text{apply } f \text{ exactly } n \text{ times})$$

$$\mathsf{succ} = \lambda n. \lambda f. \lambda x. f\, (n\, f\, x) \quad (\text{apply } f \text{ one more time})$$
$$\mathsf{plus} = \lambda m. \lambda n. \lambda f. \lambda x. m\, f\, (n\, f\, x) \quad (m + n \text{ applications of } f)$$
$$\mathsf{mult} = \lambda m. \lambda n. \lambda f. m\, (n\, f) \quad (m \cdot n \text{ applications of } f)$$

These are *representations* of numbers, not numbers themselves — but they compute correctly. $\underline{2}\, \mathsf{succ}\, \underline{0} \to_\beta^* \underline{2}$.

Church booleans similarly encode $\mathsf{true}$ and $\mathsf{false}$ as "selectors":
$$\mathsf{true} = \lambda x. \lambda y. x \quad (\text{return first argument})$$
$$\mathsf{false} = \lambda x. \lambda y. y \quad (\text{return second argument})$$
$$\mathsf{if}\, b\, t\, f = b\, t\, f \quad (\text{apply } b \text{ to the two branches})$$

## The Church-Rosser Theorem

**Theorem (Church-Rosser / Confluence).** If $t \to_\beta^* s_1$ and $t \to_\beta^* s_2$, then there exists $u$ with $s_1 \to_\beta^* u$ and $s_2 \to_\beta^* u$.

This means: even though terms might have multiple ways to reduce (different redexes to choose), they always reach the same "normal form" (if they terminate). The computation is *confluent*.

**Corollary.** Every term has at most one normal form (though it might not have any, if it diverges).

The Church-Rosser theorem shows the lambda calculus is well-behaved in terms of the order of evaluation: different strategies may give different computational paths, but they converge on the same result.

## Strategies

Different *reduction strategies* choose which redex to reduce first:

- **Normal order:** Reduce the leftmost, outermost redex first.
- **Applicative order (call-by-value):** Reduce the rightmost, innermost redex first (arguments before function body).
- **Lazy (call-by-need):** Reduce leftmost outermost but don't duplicate work.

**Theorem.** If a term has a normal form, normal order reduction will find it. Applicative order may not (it can diverge when a normal form exists).

**Example:** $(\lambda x. a)\, \Omega$. 
- Normal order: $\to_\beta a$ (apply the function immediately, ignoring the argument).
- Applicative order: first tries to reduce $\Omega$, which diverges.

This is why lazy evaluation (Haskell) can compute more things than strict evaluation (OCaml, Scheme) — but at the cost of more complex implementation.

## Types as a Solution

The problems with the untyped lambda calculus:
1. Non-termination ($\Omega$, Y combinator)
2. No type errors: $\mathsf{true}\,\mathsf{true}$ is a well-formed term
3. Self-application: $\lambda x. x\, x$ requires $x : A$ where $A = A \to B$, an impossible equation
4. No logical interpretation (can't be a logic directly)

Types solve all four problems:
1. **Strong normalization:** Every well-typed term terminates.
2. **Type safety:** Terms like $\mathsf{true}\,\mathsf{true}$ are simply not typeable.
3. **No self-application:** Self-application $x\, x$ requires $x : A$ where $A = A \to B$, which has no finite solution in STLC.
4. **Curry-Howard:** Well-typed terms are proofs; the empty type is $\bot$.

The next section develops STLC formally and proves these properties.
