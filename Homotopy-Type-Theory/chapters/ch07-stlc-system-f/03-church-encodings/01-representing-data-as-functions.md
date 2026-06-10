# 3.1 Church Encodings: Representing Data as Functions

## The Representation Problem

In STLC (and in the untyped lambda calculus), we start with functions as primitive. But mathematics requires other structures: natural numbers, booleans, lists, trees.

Are these primitives, or can they be *defined* from functions alone?

Church's insight: every data structure can be represented as a function that captures its *pattern-matching interface*. The Church encoding identifies a data value with the function that describes how to *use* that value.

This is profound: data is nothing but its behavior under case analysis.

## Church Booleans

In pure STLC with base types, we can represent booleans as functions that choose between two alternatives:

$$\mathsf{true} : A \to A \to A = \lambda x : A. \lambda y : A. x$$
$$\mathsf{false} : A \to A \to A = \lambda x : A. \lambda y : A. y$$

$\mathsf{true}$ is the function that, given two alternatives, returns the first (the "then" branch).
$\mathsf{false}$ returns the second (the "else" branch).

$$\mathsf{if}\, b\, t\, f = b\, t\, f$$

This is *if-then-else*: $b$ chooses between $t$ and $f$. Since $\mathsf{true}$ returns the first argument and $\mathsf{false}$ returns the second:
$$\mathsf{if}\, \mathsf{true}\, t\, f = \mathsf{true}\, t\, f = t$$
$$\mathsf{if}\, \mathsf{false}\, t\, f = \mathsf{false}\, t\, f = f$$

Boolean operations:
$$\mathsf{and} = \lambda b. \lambda c. b\, c\, \mathsf{false} \quad (\text{if } b \text{, then } c, \text{ else false})$$
$$\mathsf{or} = \lambda b. \lambda c. b\, \mathsf{true}\, c \quad (\text{if } b \text{, then true, else } c)$$
$$\mathsf{not} = \lambda b. b\, \mathsf{false}\, \mathsf{true} \quad (\text{swap the branches})$$

In pure STLC, $\mathsf{true}$ and $\mathsf{false}$ both have type $A \to A \to A$ for a fixed $A$ — not yet polymorphic. In System F (below), they'll have the truly polymorphic type $\forall \alpha. \alpha \to \alpha \to \alpha$.

## Church Numerals (in System F / Polymorphic STLC)

The Church numeral $\underline{n}$ represents the natural number $n$ as "apply $f$ to $x$ exactly $n$ times":

$$\underline{0} = \lambda f. \lambda x. x$$
$$\underline{1} = \lambda f. \lambda x. f\, x$$
$$\underline{2} = \lambda f. \lambda x. f\, (f\, x)$$
$$\underline{n} = \lambda f. \lambda x. f^n\, x \quad \text{(} n \text{ applications of } f\text{)}$$

In System F, the type is: $\mathsf{Nat} = \forall \alpha. (\alpha \to \alpha) \to \alpha \to \alpha$.

The number $n$ is the function that applies its first argument $n$ times to its second argument. It captures the *interface* of a natural number: the only thing you can do with $n$ is iterate some operation $n$ times.

**Arithmetic:**

$$\mathsf{succ} = \lambda n. \lambda f. \lambda x. f\, (n\, f\, x) \quad (\text{apply } f \text{ one extra time})$$

$$\mathsf{plus} = \lambda m. \lambda n. \lambda f. \lambda x. m\, f\, (n\, f\, x) \quad (m + n \text{ applications})$$

$$\mathsf{mult} = \lambda m. \lambda n. \lambda f. m\, (n\, f) \quad (m \text{ times: apply } n \text{ applications of } f)$$

$$\mathsf{exp} = \lambda m. \lambda n. n\, m \quad (n \text{-fold composition of } m\text{-applications})$$

These all reduce correctly:
$$\mathsf{plus}\, \underline{2}\, \underline{3} \to_\beta^* \underline{5}$$
$$\mathsf{mult}\, \underline{2}\, \underline{3} \to_\beta^* \underline{6}$$
$$\mathsf{exp}\, \underline{2}\, \underline{3} \to_\beta^* \underline{8}$$

**The predecessor:** Defining $\mathsf{pred}$ (predecessor: $\mathsf{pred}\, \underline{n} = \underline{n-1}$ for $n > 0$, $\mathsf{pred}\, \underline{0} = \underline{0}$) in lambda calculus is surprisingly tricky. One construction uses pairs:

$$\mathsf{pred} = \lambda n. \mathsf{fst}\, (n\, (\lambda p. (\mathsf{snd}\, p, \mathsf{succ}\, (\mathsf{snd}\, p)))\, (0, 0))$$

This "counts up" with a pair $(i-1, i)$ and extracts the first component.

## Church Pairs (Dependent-Pair Style)

Church pairs represent pairs as functions that apply a "continuation" to both components:

$$\mathsf{pair} = \lambda a. \lambda b. \lambda f. f\, a\, b$$
$$\mathsf{fst} = \lambda p. p\, (\lambda a. \lambda b. a)$$
$$\mathsf{snd} = \lambda p. p\, (\lambda a. \lambda b. b)$$

Verification:
$$\mathsf{fst}\, (\mathsf{pair}\, x\, y) = (\lambda p. p\, (\lambda a. \lambda b. a))\, ((\lambda a. \lambda b. \lambda f. f\, a\, b)\, x\, y)$$
$$\to_\beta^* (\lambda f. f\, x\, y)\, (\lambda a. \lambda b. a)$$
$$\to_\beta (\lambda a. \lambda b. a)\, x\, y \to_\beta x$$

## Church Lists

Lists are encoded similarly:

$$\mathsf{nil} = \lambda f. \lambda e. e \quad (\text{base case: no elements})$$
$$\mathsf{cons} = \lambda h. \lambda t. \lambda f. \lambda e. f\, h\, (t\, f\, e)$$

Folding a list $[a_1, a_2, \ldots, a_n]$ with function $f$ and base $e$ gives $f\, a_1\, (f\, a_2\, \cdots (f\, a_n\, e))$.

$$\mathsf{fold} = \lambda l. \lambda f. \lambda e. l\, f\, e \quad (\text{already: } l \text{ is a fold!})$$

List operations:
$$\mathsf{map} = \lambda g. \lambda l. \lambda f. \lambda e. l\, (\lambda x. f\, (g\, x))\, e$$
$$\mathsf{length} = \lambda l. l\, (\lambda x. \lambda n. \mathsf{succ}\, n)\, \underline{0}$$

## The CPS Interpretation

The Church encoding is closely related to *continuation-passing style (CPS)*. Each Church-encoded value is its own "fold function" — you give it the "continuation" (what to do with each constructor), and it applies it.

For natural numbers: the continuation for $n$ says "what to do with $n$ applications of $f$." The Church numeral just applies $f$ $n$ times to $x$.

For lists: the continuation for a list says "what to do with the head and the fold of the tail." The Church-encoded list just applies this recursively.

This CPS perspective is the origin of *initial algebra semantics* in category theory: an inductive type is characterized by its fold (the catamorphism), and the Church encoding makes the fold function into the value itself.

## The Expressive Power of Church Encodings

Church encodings in System F can represent:
- All finite inductive data types (booleans, naturals, lists, trees, finite sets)
- All primitive recursive functions
- The parametric polymorphism of Haskell ADTs

What they *cannot* directly encode in a well-typed way:
- Coinductive types (potentially infinite streams)
- Dependent types (types indexed by values)
- Higher inductive types (HoTT)

Church encodings are therefore a "sweet spot": they show the lambda calculus is very expressive, but they don't give everything. Dependent types (Chapter 8) provide the richer encoding that HoTT requires.

## The Scott Encoding: An Alternative

The *Scott encoding* represents data by pattern matching differently:

$$\underline{0} = \lambda z. \lambda s. z \quad (\text{zero: take z-branch})$$
$$\underline{n+1} = \lambda z. \lambda s. s\, \underline{n} \quad (\text{succ: take s-branch with predecessor})$$

Unlike Church numerals (where applying $\underline{n}$ iterates), Scott numerals encode the *constructor* directly. Scott encoding is better for lazy recursion and for defining the predecessor cleanly.

The Church and Scott encodings represent different "styles" of defining inductive types: Church as fold (consuming the value), Scott as case-analysis (inspecting the constructor).

Both appear in dependently typed systems: the Church-style corresponds to the *eliminator-based* definition (the recursor), while the Scott-style corresponds to the *pattern-matching-based* definition (which is more natural but requires more care about termination).
