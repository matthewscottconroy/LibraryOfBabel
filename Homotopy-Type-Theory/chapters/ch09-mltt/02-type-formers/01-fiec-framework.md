# 2.1 The FIEC Framework for Type Formers

## The Pattern of Every Type

One of the most elegant aspects of MLTT is that every type former follows the same four-part pattern:

- **F**ormation: How to write the type expression (what syntactic conditions make it well-formed)
- **I**ntroduction: How to build elements (the *constructors*)
- **E**limination: How to use elements (the *recursor* or *eliminator*)
- **C**omputation: How the eliminator behaves on constructors ($\beta$-rules)

Plus an optional:
- **Uniqueness**: How elements are characterized by their eliminator behavior ($\eta$-rules)

This FIEC (or FIECU) pattern is Martin-Löf's systematic way of presenting type formers. It ensures that every type is fully defined: you know how to form it, construct its elements, use those elements, and compute with them.

The pattern is not arbitrary. It corresponds, under the Curry-Howard correspondence, to:
- Formation ↔ statement of a logical proposition
- Introduction ↔ proof rules (how to prove the proposition)
- Elimination ↔ use rules (what the proposition lets you derive)
- Computation ↔ the normalization of cut-elimination (reducing a proof to its introduction form)

Let's go through each type former systematically.

## Dependent Function Types (Π)

**Formation:**
$$\frac{\Gamma \vdash A\ \mathsf{type} \quad \Gamma, x : A \vdash B(x)\ \mathsf{type}}{\Gamma \vdash \prod_{x:A} B(x)\ \mathsf{type}}$$

**Introduction:**
$$\frac{\Gamma, x : A \vdash t(x) : B(x)}{\Gamma \vdash \lambda x.\, t(x) : \prod_{x:A} B(x)}$$

**Elimination:**
$$\frac{\Gamma \vdash f : \prod_{x:A} B(x) \quad \Gamma \vdash a : A}{\Gamma \vdash f\, a : B(a)}$$

**Computation ($\beta$):**
$$\Gamma \vdash (\lambda x.\, t)\, a = t[a/x] : B(a)$$

**Uniqueness ($\eta$):**
$$\Gamma \vdash f = \lambda x.\, f\, x : \prod_{x:A} B(x)$$

*Reading:* Π types are function spaces. To have a function in $\prod_{x:A} B(x)$, you must be able to produce, for any input $x : A$, an output of type $B(x)$. Function application is elimination. The $\beta$-rule is computation (applying a lambda reduces by substitution). The $\eta$-rule says every function *is* a lambda (uniqueness of functions up to behavior).

## Dependent Pair Types (Σ)

**Formation:**
$$\frac{\Gamma \vdash A\ \mathsf{type} \quad \Gamma, x : A \vdash B(x)\ \mathsf{type}}{\Gamma \vdash \sum_{x:A} B(x)\ \mathsf{type}}$$

**Introduction:**
$$\frac{\Gamma \vdash a : A \quad \Gamma \vdash b : B(a)}{\Gamma \vdash (a, b) : \sum_{x:A} B(x)}$$

**Elimination (projections):**
$$\frac{\Gamma \vdash p : \sum_{x:A} B(x)}{\Gamma \vdash \pi_1\, p : A} \qquad \frac{\Gamma \vdash p : \sum_{x:A} B(x)}{\Gamma \vdash \pi_2\, p : B(\pi_1\, p)}$$

**General Eliminator (for dependent functions out of Σ):**
$$\frac{\Gamma \vdash C : \sum_{x:A} B(x) \to \mathsf{Type} \quad \Gamma \vdash g : \prod_{x:A} \prod_{y:B(x)} C(x, y)}{\Gamma \vdash \mathsf{ind}_\Sigma(g) : \prod_{p:\sum_{x:A} B(x)} C(p)}$$

**Computation:**
$$\pi_1(a, b) = a \quad \pi_2(a, b) = b \quad \mathsf{ind}_\Sigma(g)(a, b) = g\, a\, b$$

**Uniqueness ($\eta$):**
$$p = (\pi_1\, p, \pi_2\, p) : \sum_{x:A} B(x)$$

*Reading:* Σ types are dependent pairs. To have a pair, give the first component ($a : A$) and the second ($b : B(a)$, whose type depends on $a$). Elimination is projection — you can extract either component. The general eliminator handles dependent types out of Σ (dependent pattern matching). The $\eta$-rule says every element of a Σ type is a pair.

## Unit Type (𝟏)

**Formation:** $\mathbf{1}\ \mathsf{type}$

**Introduction:** $\mathsf{tt} : \mathbf{1}$

**Eliminator:**
$$\frac{\Gamma \vdash C : \mathbf{1} \to \mathsf{Type} \quad \Gamma \vdash c : C(\mathsf{tt}) \quad \Gamma \vdash u : \mathbf{1}}{\Gamma \vdash \mathsf{ind}_\mathbf{1}(C, c, u) : C(u)}$$

**Computation:** $\mathsf{ind}_\mathbf{1}(C, c, \mathsf{tt}) = c$

**Uniqueness ($\eta$):** $u = \mathsf{tt} : \mathbf{1}$ (every element of $\mathbf{1}$ is $\mathsf{tt}$)

*Reading:* The unit type has one element ($\mathsf{tt}$). To define a function out of $\mathbf{1}$, just give the value at $\mathsf{tt}$. The $\eta$-rule says $\mathbf{1}$ is *contractible* — all its elements are equal.

## Empty Type (𝟎)

**Formation:** $\mathbf{0}\ \mathsf{type}$

**Introduction:** (none — $\mathbf{0}$ has no constructors)

**Eliminator (ex falso quodlibet):**
$$\frac{\Gamma \vdash C : \mathbf{0} \to \mathsf{Type} \quad \Gamma \vdash x : \mathbf{0}}{\Gamma \vdash \mathsf{ind}_\mathbf{0}(C, x) : C(x)}$$

**Computation:** (none — there are no constructors to compute on)

*Reading:* The empty type has no elements. From a proof of $\mathbf{0}$ (a contradiction), you can prove anything. The eliminator requires no cases because there are no constructors to handle. This is the *principle of explosion*.

## Natural Numbers (ℕ)

**Formation:** $\mathbb{N}\ \mathsf{type}$

**Introduction:**
$$\mathsf{zero} : \mathbb{N} \qquad \mathsf{succ} : \mathbb{N} \to \mathbb{N}$$

**Eliminator (dependent recursor / induction):**
$$\mathsf{ind}_{\mathbb{N}} : \prod_{P : \mathbb{N} \to \mathsf{Type}} P(\mathsf{zero}) \to \left(\prod_{n:\mathbb{N}} P(n) \to P(\mathsf{succ}(n))\right) \to \prod_{n:\mathbb{N}} P(n)$$

**Computation:**
$$\mathsf{ind}_{\mathbb{N}}(P, p_0, p_s, \mathsf{zero}) = p_0$$
$$\mathsf{ind}_{\mathbb{N}}(P, p_0, p_s, \mathsf{succ}(n)) = p_s\, n\, (\mathsf{ind}_{\mathbb{N}}(P, p_0, p_s, n))$$

*Reading:* Natural numbers have two constructors. Induction handles both: what happens at zero, and what happens at successors given the inductive hypothesis. The computation rules define what happens when the eliminator meets each constructor.

## Universe (Type)

**Formation:** $\mathsf{Type}_i\ \mathsf{type}$ (for each universe level $i$)

More specifically: elements of $\mathsf{Type}_i$ are type expressions at level $i$. The universe is "closed under" all the type formers:

$$\frac{\Gamma \vdash A : \mathsf{Type}_i \quad \Gamma, x : A \vdash B : \mathsf{Type}_i}{\Gamma \vdash \prod_{x:A} B : \mathsf{Type}_i}$$

(Similarly for Σ, $\mathbf{1}$, $\mathbf{0}$, $\mathbb{N}$, and the identity type.)

**Introduction (elements of the universe):** $\mathbb{N} : \mathsf{Type}_0$, $\mathbf{1} : \mathsf{Type}_0$, etc.

**Elimination (decoding):** An element of $\mathsf{Type}_i$ is a type — it can be used as a type in judgments. (In Tarski style, there's an explicit $\mathsf{El}$ decoding function; in Russell style, this is implicit.)

*Reading:* The universe is the type of small types. It allows quantifying over all types in a specific universe level, enabling polymorphism without a separate type-abstraction construct.

## The Identity Type

The identity type follows the FIEC pattern, but its elimination rule (J) is subtle enough to deserve its own section (Section 3). Here we give the overview:

**Formation:**
$$\frac{\Gamma \vdash A\ \mathsf{type} \quad \Gamma \vdash a : A \quad \Gamma \vdash b : A}{\Gamma \vdash a =_A b\ \mathsf{type}}$$

**Introduction (reflexivity):**
$$\frac{\Gamma \vdash a : A}{\Gamma \vdash \mathsf{refl}_a : a =_A a}$$

**Elimination (J-rule):** *See Section 3.*

**Computation (J-rule on refl):** *See Section 3.*

*Reading:* The identity type $a =_A b$ is the type of proofs that $a$ equals $b$. The only direct constructor is $\mathsf{refl}_a$ (reflexivity: $a$ equals itself). But through J, we can derive symmetry, transitivity, and more.

## The Conversion Rule and Definitional Equality

One rule that cuts across all type formers:

**Conversion rule:**
$$\frac{\Gamma \vdash a : A \quad \Gamma \vdash A = B\ \mathsf{type}}{\Gamma \vdash a : B}$$

If $A$ and $B$ are definitionally equal, an element of $A$ is also an element of $B$. The type checker applies this silently: whenever you apply a function to an argument, it checks that the argument type matches, up to definitional equality (i.e., after $\beta$/$\delta$ reduction).

This is what makes $\mathsf{ind}_{\mathbb{N}}(P, p_0, p_s, \mathsf{succ}(n))$ well-typed: the computation rule gives definitional equality with $p_s\, n\, (\ldots)$, and the conversion rule lets us use this in context.

## Why This Framework Is The Right One

The FIEC framework reflects the *logical structure* of type formers:

| FIEC Step | Logic Analog | Category Theory Analog |
|---|---|---|
| Formation | Statement of proposition | Object in category |
| Introduction | Proof (how to establish it) | Morphism into the type |
| Elimination | Use (what it lets you do) | Morphism out of the type |
| Computation | Cut elimination (normalization) | Universal property |
| Uniqueness | Proof irrelevance or η | Uniqueness in universal property |

This connection is deep: the universal property of a type (in the categorical sense) is exactly the elimination rule, with the introduction rules giving the canonical maps, and the computation rules ensuring they satisfy the universal property. This is the connection between MLTT and categorical semantics, developed in detail in Chapters 10-12.

The FIEC framework also explains why adding a type former to MLTT is a principled process. When HoTT adds the Univalence Axiom (Chapter 11) or Higher Inductive Types (Chapter 14), it does so by specifying new FIEC rules that fit the existing pattern.

## Consistency from the Framework

One consequence of the FIEC framework: the system is consistent. Why? Because:

1. The empty type $\mathbf{0}$ has no introduction rules — it has no elements.
2. Strong normalization means every proof term normalizes.
3. If you had a closed proof term $t : \mathbf{0}$, it would normalize to a value, but the only values of $\mathbf{0}$ are... there are none (no introduction rules).
4. Contradiction: there are no closed proofs of $\mathbf{0}$.

So MLTT is consistent — you cannot prove false. This relies critically on the FIEC structure being balanced: every elimination rule corresponds to something that can only eliminate what the introduction rules can produce. No introduction rule for $\mathbf{0}$ means no way to get stuck at the elimination rules.
