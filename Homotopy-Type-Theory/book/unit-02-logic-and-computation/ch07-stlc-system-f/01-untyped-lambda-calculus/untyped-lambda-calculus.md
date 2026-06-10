# The Untyped Lambda Calculus

## Everything Is a Function

Church's lambda calculus, introduced in the 1930s, is a radical simplification: everything is a function. There are no numbers, no booleans, no data structures — only functions and function application. And yet, this minimal language can represent all computable processes.

The syntax has three cases:
$$t ::= x \mid \lambda x.\, t \mid t\, s$$

- **Variable** $x$: a name.
- **Lambda abstraction** $\lambda x.\, t$: the function that takes input $x$ and returns $t$.
- **Application** $t\, s$: apply function $t$ to argument $s$.

That is all. No constants, no primitive operations, no built-in data types. Everything that can be computed in this language must be encoded as a function.

The lambda calculus is deceptively simple. Its apparent poverty is actually a richness: its minimalism makes it mathematically tractable while remaining computationally universal. Turing proved that the lambda calculus and Turing machines compute the same class of functions — the effectively computable functions.

## Alpha, Beta, Eta

Three notions of equivalence:

**Alpha-equivalence** ($\alpha$): renaming bound variables. $\lambda x.\, x$ and $\lambda y.\, y$ are the same function. The names of bound variables are irrelevant; only the structure of binding matters. We write $\alpha$-equivalent terms as equal.

**Beta-reduction** ($\beta$): applying a function to an argument. The fundamental computation step:
$$(\lambda x.\, t)\, s \to_\beta t[s/x]$$
Substitute $s$ for every free occurrence of $x$ in $t$.

Example: $(\lambda x.\, x + x)\, 3 \to_\beta 3 + 3 \to 6$ (if we have arithmetic). In the pure lambda calculus without arithmetic: $(\lambda x.\, x\, x)\, (\lambda y.\, y) \to_\beta (\lambda y.\, y)(\lambda y.\, y) \to_\beta \lambda y.\, y$.

**Eta-reduction** ($\eta$): $\lambda x.\, t\, x \to_\eta t$ when $x \notin \text{FV}(t)$. The function "take $x$ and apply $t$ to it" is extensionally equal to $t$ itself.

A term with no possible $\beta$-reductions is in *normal form*. Not all terms have normal forms.

## Church Encodings: Booleans

Without primitives, we must encode data types as functions. The most elegant encoding represents data by their *eliminators* — by what they do when used in case analysis.

**Booleans**: a boolean is something that, given two choices, selects one. So:
$$\mathsf{true} = \lambda x.\, \lambda y.\, x$$
$$\mathsf{false} = \lambda x.\, \lambda y.\, y$$

A boolean $b$ is a function that takes two arguments $x$ and $y$ and returns one. $\mathsf{true}$ returns the first; $\mathsf{false}$ returns the second.

**If-then-else**: $\mathsf{if}\, b\, \mathsf{then}\, t\, \mathsf{else}\, f = b\, t\, f$.

Check: $\mathsf{true}\, t\, f = (\lambda x.\, \lambda y.\, x)\, t\, f \to_\beta (\lambda y.\, t)\, f \to_\beta t$. Correct.

**Boolean operations**:
$$\mathsf{and} = \lambda p.\, \lambda q.\, p\, q\, \mathsf{false}$$
$$\mathsf{or} = \lambda p.\, \lambda q.\, p\, \mathsf{true}\, q$$
$$\mathsf{not} = \lambda p.\, p\, \mathsf{false}\, \mathsf{true}$$

Verify: $\mathsf{and}\, \mathsf{true}\, \mathsf{false} = \mathsf{true}\, \mathsf{false}\, \mathsf{false} \to_\beta \mathsf{false}$. Correct.

## Church Numerals

**Natural numbers**: a numeral $\underline{n}$ is something that applies a function $f$ exactly $n$ times to a starting value $x$.
$$\underline{0} = \lambda f.\, \lambda x.\, x$$
$$\underline{1} = \lambda f.\, \lambda x.\, f\, x$$
$$\underline{2} = \lambda f.\, \lambda x.\, f\, (f\, x)$$
$$\underline{n} = \lambda f.\, \lambda x.\, f^n(x)$$

where $f^n(x)$ means $f$ applied $n$ times to $x$.

**Successor**:
$$\mathsf{succ} = \lambda n.\, \lambda f.\, \lambda x.\, f\, (n\, f\, x)$$

Verify: $\mathsf{succ}\, \underline{2} = \lambda f.\, \lambda x.\, f\, (\underline{2}\, f\, x) \to_\beta \lambda f.\, \lambda x.\, f\, (f\, (f\, x)) = \underline{3}$. Correct.

**Addition**:
$$\mathsf{plus} = \lambda m.\, \lambda n.\, \lambda f.\, \lambda x.\, m\, f\, (n\, f\, x)$$

This says: apply $f$ $m$ times, starting from the result of applying $f$ $n$ times to $x$.

**Multiplication**:
$$\mathsf{mult} = \lambda m.\, \lambda n.\, \lambda f.\, m\, (n\, f)$$

This says: apply "$n$ applications of $f$" $m$ times. The composition of $m$ copies of "$n$ applications of $f$" gives $m \cdot n$ applications of $f$.

**Exponentiation**:
$$\mathsf{exp} = \lambda m.\, \lambda n.\, n\, m$$

This says: apply the function $m$ (which is "$\cdot$ applied $m$ times") $n$ times. It computes $m^n$.

## The Y Combinator and Recursion

Can we define recursion without a built-in "define by recursion" construct?

The factorial function $\mathsf{fact}$ satisfies $\mathsf{fact}(n) = \mathsf{if}\, n = 0\, \mathsf{then}\, 1\, \mathsf{else}\, n \cdot \mathsf{fact}(n-1)$. To define this in the lambda calculus, we need $\mathsf{fact}$ to refer to itself. But lambda abstraction only binds the immediately introduced variable.

The solution: the Y combinator.
$$Y = \lambda f.\, (\lambda x.\, f\, (x\, x))\, (\lambda x.\, f\, (x\, x))$$

The Y combinator is a *fixed-point combinator*: for any function $f$, $Y\, f$ is a fixed point of $f$, meaning $Y\, f = f\, (Y\, f)$.

Verify: $Y\, f = (\lambda x.\, f\, (x\, x))\, (\lambda x.\, f\, (x\, x)) \to_\beta f\, ((\lambda x.\, f\, (x\, x))\, (\lambda x.\, f\, (x\, x))) = f\, (Y\, f)$. ✓

To define factorial: let $F = \lambda \mathsf{rec}.\, \lambda n.\, \mathsf{if}\, (n = 0)\, \mathsf{then}\, 1\, \mathsf{else}\, n \cdot (\mathsf{rec}\, (n-1))$. Then $\mathsf{fact} = Y\, F$.

The Y combinator makes the lambda calculus Turing-complete: any computable function can be expressed using lambda abstraction and the Y combinator.

## Non-Termination: The Paradox

The price of the Y combinator is non-termination. Consider:
$$\Omega = (\lambda x.\, x\, x)\, (\lambda x.\, x\, x)$$

$\Omega$ reduces to itself: $(\lambda x.\, x\, x)\, (\lambda x.\, x\, x) \to_\beta (\lambda x.\, x\, x)\, (\lambda x.\, x\, x) = \Omega$.

$\Omega$ is the prototypical non-terminating computation. It represents an infinite loop that never produces a value.

More dramatically: the untyped lambda calculus is inconsistent as a logic. Under any Curry-Howard-style correspondence for the untyped lambda calculus, every type would be inhabited — because $\Omega$ has "any type" (it can be applied to any term and produces more computation). This is logical inconsistency: every proposition is provable.

The fix is types. The simply typed lambda calculus rules out self-application (since $x\, x$ requires $x$ to have type $A \to B$ and also type $A$, which forces $A = A \to B$, impossible in a simple type system). Without self-application, $\Omega$ cannot be formed. Without $\Omega$, non-termination is ruled out. Without non-termination, the system is consistent.

## Reduction Strategies

In the untyped lambda calculus, different reduction strategies can produce different results (or no result for diverging terms). The main strategies:

**Normal order** (leftmost-outermost): always reduce the leftmost, outermost redex. This is the strategy that finds the normal form if any exists — normal order is *normalizing*.

**Applicative order** (leftmost-innermost): always reduce the innermost, leftmost redex (i.e., reduce arguments before applying functions). This corresponds to *call by value* in programming languages. Applicative order may diverge even when a normal form exists: consider $(\lambda x.\, \lambda y.\, x)\, \Omega$ — under normal order, this reduces to $\lambda y.\, \Omega$, but under applicative order, we first try to reduce $\Omega$, which diverges.

**Lazy evaluation** (call by need): reduce the outermost redex, but share the argument (don't reduce it until needed, and share the result when reduced). This is the strategy used by Haskell: arguments are evaluated at most once, when first needed. Lazy evaluation is equivalent to normal order modulo sharing.

The choice of reduction strategy matters for practical languages:
- Call by value (applicative order) is used by most languages: ML, Java, Python, C.
- Call by need (lazy evaluation) is used by Haskell and some functional languages.
- Call by name (normal order without sharing) is used in Algol 60 and some logic programming systems.

In the typed lambda calculi that follow, the reduction strategy does not affect what value is produced (by the Church-Rosser property and strong normalization), only how quickly it is computed.

## The Untyped Lambda Calculus as a Foundation

Despite its inconsistency as a logic, the untyped lambda calculus has clean mathematical properties that make it useful as a foundation for denotational semantics.

*Domain theory* (Dana Scott, Christopher Strachey, 1960s–70s) provides models for the untyped lambda calculus by interpreting terms as elements of *domains* — partially ordered sets with certain completeness properties. In the *D$_\infty$* model (Scott's first solution), every element is a function from elements to elements, making self-application well-defined.

Domain theory is used to give mathematical meaning to programming language constructs: recursive functions are fixed points in the domain ordering (the least fixed point of a monotone function), and the denotational semantics of a program is an element of the domain associated with its type.

The interaction between domain-theoretic semantics and type theory is ongoing. In HoTT, types are interpreted as spaces (homotopy types), not domains. The homotopy interpretation and the domain-theoretic interpretation are different models of type theory, with different strengths: domain theory handles recursion and non-termination; homotopy theory handles higher-dimensional structure. Finding a unified model — a "homotopy domain theory" — is an open research problem.
