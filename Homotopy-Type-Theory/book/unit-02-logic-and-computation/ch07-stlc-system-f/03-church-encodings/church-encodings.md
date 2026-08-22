# Church Encodings

## The Art of Representing Data as Functions

The lambda calculus has no built-in data types. Everything must be encoded as a function. Church's encodings — named for Alonzo Church, who introduced the lambda calculus — show how to represent all standard data types as lambda terms.

The key principle: data is represented by its *eliminator*. A boolean is something that can select between two choices. A natural number is something that can iterate a function a specified number of times. A list is something that can fold over its elements. The *behavior* of the data type — how it is used — determines its representation.

This principle is not just a clever trick. It is the foundation of the Curry-Howard correspondence for inductive types: in type theory, an inductive type is characterized by its elimination rule (the recursion principle), and the Church encoding makes this explicit as a function.

## Booleans

$$\mathsf{Bool} = \mathsf{True} \to \mathsf{False} \to \mathsf{Choice}$$

More precisely, in STLC with a type variable $C$:
$$\mathsf{Bool}_C = C \to C \to C$$

$$\mathsf{true}_C = \lambda x : C.\, \lambda y : C.\, x : C \to C \to C$$
$$\mathsf{false}_C = \lambda x : C.\, \lambda y : C.\, y : C \to C \to C$$

But this requires fixing a result type $C$, which is inconvenient. In System F (polymorphic lambda calculus), we can quantify over $C$:

$$\mathsf{Bool} = \forall \alpha.\, \alpha \to \alpha \to \alpha$$
$$\mathsf{true} = \Lambda \alpha.\, \lambda x : \alpha.\, \lambda y : \alpha.\, x : \mathsf{Bool}$$
$$\mathsf{false} = \Lambda \alpha.\, \lambda x : \alpha.\, \lambda y : \alpha.\, y : \mathsf{Bool}$$

If-then-else:
$$\mathsf{if} : \mathsf{Bool} \to \forall \alpha.\, \alpha \to \alpha \to \alpha$$
$$\mathsf{if}\, b\, [C]\, t\, f = b\, [C]\, t\, f$$

**Verification**: $\mathsf{if}\, \mathsf{true}\, [C]\, t\, f = \mathsf{true}\, [C]\, t\, f = (\Lambda \alpha.\, \lambda x.\, \lambda y.\, x)\, [C]\, t\, f \to_\beta (\lambda x.\, \lambda y.\, x)\, t\, f \to_\beta t$. ✓

## Natural Numbers: Church Numerals

$$\mathsf{Nat} = \forall \alpha.\, (\alpha \to \alpha) \to \alpha \to \alpha$$

The Church numeral $\underline{n}$ represents "iterate $n$ times":
$$\underline{n} = \Lambda \alpha.\, \lambda f : \alpha \to \alpha.\, \lambda x : \alpha.\, f^n(x)$$

where $f^n(x)$ means $f$ applied $n$ times to $x$.

Key operations:

**Successor**:
$$\mathsf{succ} = \lambda n : \mathsf{Nat}.\, \Lambda \alpha.\, \lambda f : \alpha \to \alpha.\, \lambda x : \alpha.\, f\, (n\, [\alpha]\, f\, x)$$

**Addition**: $\mathsf{plus} = \lambda m : \mathsf{Nat}.\, \lambda n : \mathsf{Nat}.\, \Lambda \alpha.\, \lambda f.\, \lambda x.\, m\, [\alpha]\, f\, (n\, [\alpha]\, f\, x)$

**Multiplication**: $\mathsf{mult} = \lambda m.\, \lambda n.\, \Lambda \alpha.\, \lambda f.\, m\, [\alpha]\, (n\, [\alpha]\, f)$

**Exponentiation**: $\mathsf{exp} = \lambda m.\, \lambda n.\, n\, [\mathsf{Nat}]\, m$

**The Logical Reading**: $\underline{n}$ has type $\forall \alpha.\, (\alpha \to \alpha) \to \alpha \to \alpha$, which under Curry-Howard is $\forall P.\, (P \to P) \to P \to P$. This is exactly the induction principle: if a property $P$ is preserved by a step (the successor function corresponds to "and $P$ holds of the next"), and holds initially, then it holds after $n$ steps. Church numerals *are* proofs of induction.

## Predecessor: The Hard Problem

Implementing predecessor on Church numerals is non-trivial. The successor function is easy because it adds one step. But removing one step — decrementing by one — is harder because a Church numeral cannot "look at itself" to determine its predecesso.

The Kleene predecessor trick: use pairs to carry both the previous and current value through iteration.

$$\mathsf{pred} = \lambda n : \mathsf{Nat}.\, \pi_1\, (n\, [\mathsf{Nat} \times \mathsf{Nat}]\, (\lambda p.\, (\pi_2\, p, \mathsf{succ}\, (\pi_2\, p)))\, (\underline{0}, \underline{0}))$$

This iterates $n$ times the function "advance the pair $(a, b)$ to $(b, \mathsf{succ}(b))$", starting from $(0, 0)$. After $n$ steps, the pair is $(n-1, n)$. Project out the first component to get $n - 1$.

**Limitation**: $\mathsf{pred}\, \underline{0} = \underline{0}$ (monus: predecessor clips at zero). This is not a bug — natural number subtraction is partial in the usual sense, and the Church encoding clips at zero.

## Pairs and Sums

**Pairs**: a pair is something that, when given a function, applies the function to both components.

$$\mathsf{Pair}\, A\, B = \forall \alpha.\, (A \to B \to \alpha) \to \alpha$$
$$\mathsf{pair}\, a\, b = \Lambda \alpha.\, \lambda k : A \to B \to \alpha.\, k\, a\, b$$

$$\mathsf{fst} = \lambda p : \mathsf{Pair}\, A\, B.\, p\, [A]\, (\lambda x.\, \lambda y.\, x)$$
$$\mathsf{snd} = \lambda p : \mathsf{Pair}\, A\, B.\, p\, [B]\, (\lambda x.\, \lambda y.\, y)$$

**Sums** (disjoint unions): a sum is something that handles either case.

$$\mathsf{Sum}\, A\, B = \forall \alpha.\, (A \to \alpha) \to (B \to \alpha) \to \alpha$$
$$\mathsf{inl}\, a = \Lambda \alpha.\, \lambda f : A \to \alpha.\, \lambda g : B \to \alpha.\, f\, a$$
$$\mathsf{inr}\, b = \Lambda \alpha.\, \lambda f : A \to \alpha.\, \lambda g : B \to \alpha.\, g\, b$$

Case analysis: $\mathsf{case}\, e\, f\, g = e\, [\alpha]\, f\, g$.

## The Scott Encoding

The Church encoding represents numbers and data types by their iteration principles. There is another encoding — the *Scott encoding* — that represents data by their *pattern-matching* principles.

**Scott Booleans**:
$$\mathsf{true} = \lambda x.\, \lambda y.\, x \qquad \mathsf{false} = \lambda x.\, \lambda y.\, y$$

(Same as Church for booleans.)

**Scott Naturals** (using lazy representation):
$$\mathsf{zero} = \lambda z.\, \lambda s.\, z$$
$$\mathsf{succ}\, n = \lambda z.\, \lambda s.\, s\, n$$

A Scott numeral $\underline{n}$ is a term that, given a "zero case" $z$ and a "successor case" $s$, applies $s$ to the *predecessor* if $n > 0$.

Scott numerals support pattern matching:
$$\mathsf{pred}\, n = n\, \underline{0}\, (\lambda m.\, m)$$

This is simpler than the Church predecessor: $\mathsf{pred}\, (\mathsf{succ}\, m) = \lambda z.\, \lambda s.\, s\, m \mapsto \underline{0}\, (\lambda m.\, m)$... well, the Scott encoding makes predecessor obvious: $\mathsf{succ}\, m = \lambda z.\, \lambda s.\, s\, m$, so $\mathsf{succ}\, m\, \underline{0}\, (\lambda m.\, m) = (\lambda m.\, m)\, m = m$. ✓

But Scott numerals don't support iteration naturally: to compute $f^n(x)$ from a Scott numeral, you need recursion. This is where the Church encoding shines.

## Limitations of Church Encodings

Church encodings are elegant, but they have limitations that become apparent in type-theoretic settings:

**No dependent elimination**. In MLTT, the elimination rule for natural numbers (recursion) allows the *type of the result* to depend on the *natural number being eliminated*:
$$\mathsf{rec}_{\mathbb{N}} : \Pi_{P : \mathbb{N} \to \mathsf{Type}} P(0) \to (\Pi_{n:\mathbb{N}} P(n) \to P(\mathsf{succ}(n))) \to \Pi_{n:\mathbb{N}} P(n)$$

A Church-encoded natural number has type $\forall \alpha.\, (\alpha \to \alpha) \to \alpha \to \alpha$, which does not allow $\alpha$ to depend on the specific number being iterated. The dependent eliminator is strictly more powerful.

This limitation is why type theories use proper *inductive types* rather than Church encodings. The inductive natural numbers $\mathbb{N}$ have a dependent elimination principle (the recursion/induction principle) that Church numerals cannot provide.

**No intensional inspection**. Church-encoded data cannot be inspected for its constructor: there is no way to test whether a Church numeral is zero without applying it (which gives the result of case analysis, not the numeral itself). Proper inductive types support a distinction between "the numeral zero" and "the result of a computation on the numeral zero."

**Higher inductive types cannot be encoded**. HoTT's higher inductive types — types defined by paths and higher paths as well as points — have no analog in System F. They require a proper type theory with inductive types as primitives.

Despite these limitations, Church encodings are valuable: they demonstrate that the lambda calculus can express complex data, they provide a model for understanding inductive types (as the Curry-Howard interpretation makes clear), and they are used in practice in some encodings of data types for proof-theoretic purposes.
