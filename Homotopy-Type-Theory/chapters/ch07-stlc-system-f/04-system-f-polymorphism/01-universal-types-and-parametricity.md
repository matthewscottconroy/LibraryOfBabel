# 4.1 System F: Universal Types and Parametricity

## The Limitation of STLC

STLC is clean and consistent, but it has a major weakness: no polymorphism.

In mathematics, the identity function is "one thing": for any set $A$, $\text{id}_A : A \to A$ is $x \mapsto x$. We don't write a separate identity function for each type.

In STLC, you'd need: $\text{id}_{\mathbb{N}} = \lambda x : \mathbb{N}. x$, $\text{id}_{\mathbb{B}} = \lambda x : \mathbb{B}. x$, $\text{id}_{A \to B} = \lambda x : A \to B. x$, one for each type. There's no single term of type "for all types $A$, $A \to A$."

System F solves this by adding quantification over types.

## System F: Types

**Type grammar for System F:**
$$A, B ::= \alpha \mid A \to B \mid \forall \alpha. A$$

Where $\alpha$ is a *type variable*. The quantifier $\forall \alpha. A$ binds $\alpha$ in $A$.

We can also add products and sums (they're definable, but adding them explicitly is convenient):
$$A, B ::= \alpha \mid A \to B \mid A \times B \mid A + B \mid \mathbf{1} \mid \mathbf{0} \mid \forall \alpha. A$$

**New type formation rule:**
$$\frac{\Gamma, \alpha : \mathsf{Type} \vdash A : \mathsf{Type}}{\Gamma \vdash \forall \alpha. A : \mathsf{Type}} \quad (\forall\text{-form})$$

## System F: Terms and Typing

**New term constructs:**
- **Type abstraction:** $\Lambda \alpha. t$ (abstract over a type variable)
- **Type application:** $t\, [A]$ (instantiate a polymorphic term at type $A$)

**New typing rules:**

**Universal introduction (type abstraction):**
$$\frac{\Gamma, \alpha : \mathsf{Type} \vdash t : A \quad \alpha \notin \text{FTV}(\Gamma)}{\Gamma \vdash \Lambda \alpha. t : \forall \alpha. A}$$

The side condition $\alpha \notin \text{FTV}(\Gamma)$ ensures $\alpha$ is truly a "fresh" type variable — we're abstracting over a type that doesn't appear in our assumptions.

**Universal elimination (type application):**
$$\frac{\Gamma \vdash t : \forall \alpha. A}{\Gamma \vdash t\, [B] : A[\alpha := B]}$$

Given a polymorphic term of type $\forall \alpha. A$, instantiate it at any specific type $B$ to get a term of type $A[\alpha := B]$ (substituting $B$ for $\alpha$).

**Type-level $\beta$-reduction:**
$$(\Lambda \alpha. t)\, [B] \to_\beta t[\alpha := B]$$

## The Polymorphic Identity

**Example:** The polymorphic identity function:
$$\mathsf{id} = \Lambda \alpha. \lambda x : \alpha. x : \forall \alpha. \alpha \to \alpha$$

To use it at type $\mathbb{N}$:
$$\mathsf{id}\, [\mathbb{N}]\, 5 = (\Lambda \alpha. \lambda x : \alpha. x)\, [\mathbb{N}]\, 5 \to_\beta (\lambda x : \mathbb{N}. x)\, 5 \to_\beta 5$$

The type abstraction $\Lambda \alpha$ takes a type, and the result is a function at that type.

## Church Booleans in System F

In System F, booleans can be fully polymorphic:

$$\mathsf{Bool} = \forall \alpha. \alpha \to \alpha \to \alpha$$
$$\mathsf{true} = \Lambda \alpha. \lambda x : \alpha. \lambda y : \alpha. x : \mathsf{Bool}$$
$$\mathsf{false} = \Lambda \alpha. \lambda x : \alpha. \lambda y : \alpha. y : \mathsf{Bool}$$

$$\mathsf{if} : \mathsf{Bool} \to \forall \alpha. \alpha \to \alpha \to \alpha$$
$$\mathsf{if}\, b\, [C]\, t\, f = b\, [C]\, t\, f$$

When $b = \mathsf{true}$: $\mathsf{true}\, [C]\, t\, f \to_\beta^* t$.
When $b = \mathsf{false}$: $\mathsf{false}\, [C]\, t\, f \to_\beta^* f$.

## Church Naturals in System F

$$\mathsf{Nat} = \forall \alpha. (\alpha \to \alpha) \to \alpha \to \alpha$$
$$\underline{n} = \Lambda \alpha. \lambda f : \alpha \to \alpha. \lambda x : \alpha. f^n(x) : \mathsf{Nat}$$

$$\mathsf{succ} = \lambda n : \mathsf{Nat}. \Lambda \alpha. \lambda f : \alpha \to \alpha. \lambda x : \alpha. f\, (n\, [\alpha]\, f\, x)$$
$$\mathsf{plus} = \lambda m : \mathsf{Nat}. \lambda n : \mathsf{Nat}. \Lambda \alpha. \lambda f : \alpha \to \alpha. \lambda x : \alpha. m\, [\alpha]\, f\, (n\, [\alpha]\, f\, x)$$
$$\mathsf{mult} = \lambda m : \mathsf{Nat}. \lambda n : \mathsf{Nat}. \Lambda \alpha. \lambda f : \alpha \to \alpha. m\, [\alpha]\, (n\, [\alpha]\, f)$$

Note how $\mathsf{mult}$ composes: $m$ times apply the operation "$n$ applications of $f$".

## The Logic: Second-Order Propositional Logic

Under the Curry-Howard correspondence, System F corresponds to *second-order intuitionistic propositional logic* (2IPC):

- $\forall \alpha. A$ corresponds to $\forall P. A$ (universal quantification over propositions)
- Type abstraction $\Lambda \alpha. t$ corresponds to the $\forall$-introduction rule
- Type application $t\, [B]$ corresponds to $\forall$-elimination (universal instantiation)

| **2IPC** | **System F** |
|---|---|
| $\forall P. A$ | $\forall \alpha. A$ |
| Proof of $\forall P. A$ | Term $t : \forall \alpha. A$ |
| Universal instantiation | $t\, [B]$ |
| Universal introduction | $\Lambda \alpha. t$ |

The Church numerals have type $\forall \alpha. (\alpha \to \alpha) \to \alpha \to \alpha$, which corresponds to the second-order formula:
$$\forall P. (P \to P) \to P \to P$$

This is exactly the Peano induction axiom for $P$! The Church numeral $\underline{n}$ is a proof that iteration is possible — for any property $P$, if $P$ holds of a base and is preserved by applying $f$, then $P$ holds after $n$ applications of $f$.

## Parametricity: Free Theorems

System F has a remarkable property called *parametricity* (Reynolds 1983).

**Informal statement:** A polymorphic term of type $\forall \alpha. A$ must treat elements of type $\alpha$ in a "uniform" way — it cannot inspect or distinguish elements of $\alpha$, since it works for all types.

**Free theorem for the identity type:** If $t : \forall \alpha. \alpha \to \alpha$, then $t$ is the identity function.

*Proof sketch.* Let $R$ be any relation between two types $B_1$ and $B_2$. Parametricity says: for any $x_1 : B_1$ and $x_2 : B_2$ with $x_1\, R\, x_2$, we have $(t\, [B_1]\, x_1)\, R\, (t\, [B_2]\, x_2)$.

Take $B_1 = B_2 = B$ and $R = \{(a, b) \mid a = b\}$ (equality). Then $x_1 = x_2$ implies $t\, x_1 = t\, x_2$, so $t$ preserves equality. But more strongly: take $R = \{(b, \text{some specific element})\}$. Then $t\, [B]\, b = t\, [B]\, b$, which forces $t\, [B]\, b = b$. So $t$ is the identity. $\square$

This "free theorem" is a theorem about System F — you don't have to prove it for each specific polymorphic function. The type signature alone forces the behavior.

**More free theorems:**
- A term of type $\forall \alpha. \alpha \to \alpha \to \alpha$ must be either $\lambda x. \lambda y. x$ (constant-true) or $\lambda x. \lambda y. y$ (constant-false). There are only two such terms.
- A term of type $\forall \alpha. \mathsf{List}\, \alpha \to \mathsf{List}\, \alpha$ must be a function that only rearranges or drops elements — it cannot inspect or create new elements.

Free theorems are an important tool in functional programming (Haskell): they let you prove properties of polymorphic functions from their types alone, without looking at their implementations.

## Strong Normalization for System F

**Theorem (Girard 1971).** System F is strongly normalizing.

The proof is significantly harder than for STLC. The difficulty: the reducibility predicate for $\forall \alpha. A$ must be defined uniformly for all type instantiations, but the type $A$ can contain $\alpha$, which creates a circularity.

Girard's solution introduces *candidates of reducibility* — a collection of sets of terms satisfying the CR1–CR3 properties. The reducibility predicate for $\forall \alpha. A$ is defined as an intersection over all candidates:
$$\text{Red}(\forall \alpha. A) = \{t \mid \forall \mathcal{C} \in \text{Cand}: t\, [B] \in \text{Red}(A[\alpha := B]) \text{ for all } B\}$$

This avoids the circularity by quantifying over candidates rather than specific types.

## System F is More Powerful Than STLC

System F defines functions that STLC cannot:

1. **Polymorphic identity, composition, etc.:** Single definitions that work for all types.
2. **Church numerals of type $\mathsf{Nat}$:** The full arithmetic including predecessor.
3. **Normalization of STLC terms:** System F can compute the normal forms of STLC terms (meta-theoretically).
4. **Strong normalization of STLC:** System F can prove that STLC terminates.

But System F is still not Turing-complete: it cannot define every total recursive function. The Halting Problem cannot be solved in System F. (System F is consistent as a logic.)

The frontier: every function definable in System F is *provably total* in second-order arithmetic. This is Girard's theorem, connecting System F to the proof-theoretic strength of second-order arithmetic.
