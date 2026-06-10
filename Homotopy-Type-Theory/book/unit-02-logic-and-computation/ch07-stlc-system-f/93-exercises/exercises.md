# Exercises: STLC and System F

## Section 1: Untyped Lambda Calculus

**Exercise 1.** Reduce each of the following terms to normal form (or show they have no normal form):

(a) $(\lambda x.\, \lambda y.\, x)\, a\, b$
(b) $(\lambda x.\, x\, x\, x)\, (\lambda y.\, y)$
(c) $(\lambda x.\, \lambda y.\, y)\, \Omega$ where $\Omega = (\lambda z.\, z\, z)(\lambda z.\, z\, z)$
(d) $(\lambda f.\, f\, (f\, a))\, (\lambda x.\, x)$
(e) $Y\, (\lambda f.\, \lambda n.\, n)$ (what happens when we apply the Y combinator to the constant function?)

**Exercise 2.** Verify that the Church numerals behave correctly:

(a) Show $\mathsf{succ}\, \underline{2} \to_\beta^* \underline{3}$.
(b) Show $\mathsf{plus}\, \underline{2}\, \underline{3} \to_\beta^* \underline{5}$.
(c) Show $\mathsf{mult}\, \underline{2}\, \underline{3} \to_\beta^* \underline{6}$.
(d) Show $\mathsf{exp}\, \underline{2}\, \underline{3} \to_\beta^* \underline{8}$.

**Exercise 3.** Verify that the Y combinator gives the correct behavior:

(a) Show $Y\, f \to_\beta^* f\, (Y\, f)$.
(b) Define the factorial function $\mathsf{fact}$ using Y: write the function $F$ such that $\mathsf{fact} = Y\, F$, and verify $\mathsf{fact}\, \underline{3} \to_\beta^* \underline{6}$.

**Exercise 4.** Explain why each of the following reduction strategies might diverge while normal-order reduction terminates:

(a) $(\lambda x.\, \lambda y.\, y)\, \Omega$ under applicative order
(b) $Y\, (\lambda f.\, \lambda x.\, \text{if}\, (x = 0)\, \mathsf{then}\, 1\, \mathsf{else}\, f\, (x - 1))$ applied to $\underline{0}$ under applicative order

**Exercise 5.** Implement, as untyped lambda terms, each of the following:

(a) $\mathsf{and} = \lambda p.\, \lambda q.\, p\, q\, \mathsf{false}$: verify $\mathsf{and}\, \mathsf{true}\, \mathsf{false} \to_\beta^* \mathsf{false}$
(b) $\mathsf{not} = \lambda p.\, p\, \mathsf{false}\, \mathsf{true}$: verify $\mathsf{not}\, \mathsf{true} \to_\beta^* \mathsf{false}$
(c) A "zero test" $\mathsf{iszero} : \mathsf{Nat} \to \mathsf{Bool}$: verify $\mathsf{iszero}\, \underline{0} \to_\beta^* \mathsf{true}$ and $\mathsf{iszero}\, \underline{2} \to_\beta^* \mathsf{false}$

## Section 2: STLC Type Safety

**Exercise 6.** Give typing derivations for each of the following:

(a) $f : A \to B, g : B \to C, x : A \vdash g\, (f\, x) : C$
(b) $\vdash \lambda x : A.\, \lambda y : B.\, (y, x) : A \to B \to B \times A$
(c) $p : A \times B \vdash (\pi_2\, p, \pi_1\, p) : B \times A$

**Exercise 7.** Which of the following are well-typed in STLC (with base types $\mathsf{Nat}$ and $\mathsf{Bool}$)? For those that are, give the type; for those that are not, explain why.

(a) $\lambda x : \mathsf{Nat}.\, x\, x$
(b) $\lambda f : \mathsf{Nat} \to \mathsf{Bool}.\, \lambda x : \mathsf{Nat}.\, f\, x$
(c) $\lambda f : \mathsf{Nat} \to \mathsf{Bool}.\, f\, \mathsf{true}$
(d) $(\lambda x : \mathsf{Nat}.\, x)\, (\lambda y : \mathsf{Nat}.\, y)$

**Exercise 8.** State and prove the Canonical Forms Lemma for STLC: for each type $A$, what form must a closed value of type $A$ have?

**Exercise 9.** Prove the Substitution Lemma: if $\Gamma, x : A \vdash t : B$ and $\Gamma \vdash s : A$, then $\Gamma \vdash t[s/x] : B$. Give the proof for:

(a) $t = x$ (the substituted variable)
(b) $t = \lambda y : C.\, u$ (lambda abstraction, where $y \neq x$)
(c) $t = (t_1, t_2)$ (pairing)

**Exercise 10.** Prove the Progress Theorem for STLC: a closed, well-typed term is either a value or reduces in one step. Give the proof for:

(a) $t = \lambda x : A.\, u$ (values — already done)
(b) $t = f\, s$ (application — the interesting case)
(c) $t = \pi_1\, p$ (first projection)

## Section 3: Church Encodings

**Exercise 11.** Write the Church encoding (as System F terms) of:

(a) Booleans: $\mathsf{Bool}$, $\mathsf{true}$, $\mathsf{false}$, and $\mathsf{if}$.
(b) Natural numbers: $\mathsf{Nat}$, $\underline{0}$, $\mathsf{succ}$, $\mathsf{iszero}$.
(c) Products: $\mathsf{Pair}\, A\, B$, $\mathsf{pair}$, $\mathsf{fst}$, $\mathsf{snd}$.

**Exercise 12.** Derive the types of the Church-encoded terms:

(a) $\mathsf{succ} : \mathsf{Nat} \to \mathsf{Nat}$ where $\mathsf{Nat} = \forall \alpha.\, (\alpha \to \alpha) \to \alpha \to \alpha$.
(b) $\mathsf{plus} : \mathsf{Nat} \to \mathsf{Nat} \to \mathsf{Nat}$.
(c) $\mathsf{pred} : \mathsf{Nat} \to \mathsf{Nat}$ (using the Kleene trick with pairs).

**Exercise 13.** Verify that the Kleene predecessor gives the correct result: $\mathsf{pred}\, \underline{3} \to_\beta^* \underline{2}$. Trace through the reduction using the pair-based implementation.

**Exercise 14.** Explain why Church encodings cannot support *dependent elimination*. Give a specific example of a recursion principle for natural numbers that Church-encoded naturals cannot satisfy but proper inductive types can.

**Exercise 15.** Write the Scott encoding of natural numbers and compare predecessor with the Church encoding: why is Scott predecessor $O(1)$ while Church predecessor is $O(n)$?

## Section 4: System F

**Exercise 16.** Give System F typings for each of the following polymorphic functions:

(a) $\mathsf{const} = \Lambda \alpha.\, \Lambda \beta.\, \lambda x : \alpha.\, \lambda y : \beta.\, x$
(b) $\mathsf{apply} = \Lambda \alpha.\, \Lambda \beta.\, \lambda f : \alpha \to \beta.\, \lambda x : \alpha.\, f\, x$
(c) $\mathsf{pair} = \Lambda \alpha.\, \Lambda \beta.\, \lambda x : \alpha.\, \lambda y : \beta.\, \Lambda \gamma.\, \lambda k : \alpha \to \beta \to \gamma.\, k\, x\, y$

**Exercise 17.** State Reynolds' Parametricity Theorem. Use it to prove the following free theorems:

(a) Any $t : \forall \alpha.\, \alpha \to \alpha$ must be the identity.
(b) Any $t : \forall \alpha.\, \alpha \to \alpha \to \alpha$ must be one of exactly two functions.
(c) Any $t : \forall \alpha.\, \mathsf{List}\, \alpha \to \alpha$ must select an element from the list (if any exist) or be the "unsafe" function that coerces from empty.

**Exercise 18.** Under the Curry-Howard correspondence, what second-order logical proposition corresponds to each of the following System F types?

(a) $\forall \alpha.\, \alpha \to \alpha$
(b) $\forall \alpha.\, \forall \beta.\, \alpha \to \beta \to \alpha$
(c) $\mathsf{Nat} = \forall \alpha.\, (\alpha \to \alpha) \to \alpha \to \alpha$

Interpret (c) as a second-order proposition and explain its logical meaning.

**Exercise 19.** Sketch Girard's proof of strong normalization for System F. Why is the definition of $\text{Red}(\forall \alpha.\, A)$ more complex than in STLC? What role do "candidates of reducibility" play?

**Exercise 20.** Show that System F can express the following STLC-undefinable function:

The doubling function $D : \mathsf{Nat} \to \mathsf{Nat}$ defined by $D(n) = 2 \cdot n$ is easily STLC-definable. Show a function that System F can define but STLC cannot: a function that takes a function $f : \mathsf{Nat} \to \mathsf{Nat}$ and returns $f \circ f$ (i.e., $f$ applied twice). Why can this be polymorphically defined in System F but not in STLC?

## Section 5: System F$\omega$ and the Lambda Cube

**Exercise 21.** State the grammar of kinds in System F$\omega$. Give the kind of each of the following type operators:

(a) $\mathsf{Maybe} = \lambda \alpha : \star.\, \alpha + \mathbf{1} : ?$
(b) $\mathsf{List} : ?$ (a type operator taking a type to the type of lists over it)
(c) $\mathsf{Functor} : ?$ (a type-level predicate on type operators `* -> *`)

**Exercise 22.** Draw the lambda cube and label each vertex with its type system name. For each axis, state what kind of dependency it adds.

**Exercise 23.** State the typing rule for $\Pi_{x:A} B$ in the Calculus of Constructions (CoC). How does it unify the function type $A \to B$, the polymorphic type $\forall \alpha.\, A$, the dependent type $\Pi_{x:A} B(x)$, and the type operator $\lambda \alpha.\, A$ into a single rule?

**Exercise 24.** Explain the difference between *predicative* and *impredicative* polymorphism. Which systems in the lambda cube are impredicative? Why does impredicativity require more care in proving consistency?

## Proof-Level Exercises

**Exercise 25.** Prove that STLC is Church-Rosser (confluent): if $t \to^* s_1$ and $t \to^* s_2$, then there is $u$ with $s_1 \to^* u$ and $s_2 \to^* u$. Outline the proof via the diamond property for parallel beta reduction.

**Exercise 26.** Prove that the Church numeral $\underline{n}$ has type $\mathsf{Nat} = \forall \alpha.\, (\alpha \to \alpha) \to \alpha \to \alpha$ in System F. Give the full typing derivation for $\underline{2}$.

**Exercise 27.** Reynolds' parametricity theorem for the type $\forall \alpha.\, \mathsf{List}\, \alpha \to \mathsf{List}\, \alpha$ implies that any such function commutes with any function `map`. Prove this formally: if $f : \forall \alpha.\, \mathsf{List}\, \alpha \to \mathsf{List}\, \alpha$ and $g : A \to B$, then $\mathsf{map}\, g\, (f\, [A]\, \ell) = f\, [B]\, (\mathsf{map}\, g\, \ell)$.

**Exercise 28.** Prove that the Church-encoded $\mathsf{Bool}$ type in System F satisfies the free theorem: any term of type $\mathsf{Bool} = \forall \alpha.\, \alpha \to \alpha \to \alpha$ is either $\mathsf{true}$ or $\mathsf{false}$.

**Exercise 29.** Show that the following types are isomorphic in System F (by exhibiting inverse functions and verifying the isomorphism equations):

(a) $\forall \alpha.\, (\alpha \to A) \to (B \to \alpha) \to \alpha \simeq A \times B$ (the "lazy pair" encoding)
(b) $\mathsf{Bool} = \forall \alpha.\, \alpha \to \alpha \to \alpha \simeq \mathbf{1} + \mathbf{1}$ (booleans as a coproduct)

**Exercise 30.** (Hard) Prove that the type $\mathsf{Nat} = \forall \alpha.\, (\alpha \to \alpha) \to \alpha \to \alpha$ in System F satisfies the following: for any type $C$ and term $f : \mathsf{Nat} \to \mathsf{Nat} \to C$ satisfying $f\, m\, n = f\, n\, m$ (symmetry), and any term $n : \mathsf{Nat}$, the term $f\, n\, n$ depends only on $n$ and $f$. This is a consequence of parametricity applied to the Church numeral type.
