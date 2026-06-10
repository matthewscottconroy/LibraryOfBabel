# 1.1 The Four Judgments of MLTT

## What a Judgment Is

A *judgment* is a basic form of assertion in a formal system — the primitive things you can say. In propositional logic, judgments are of the form "$P$ is provable" or "$P$ is true." In MLTT, the judgments are more fine-grained, distinguishing between types and terms, and between the two kinds of equality.

MLTT has exactly four judgment forms (always made in a context $\Gamma$):

1. $\Gamma \vdash A\ \mathsf{type}$ — "$A$ is a (well-formed) type in context $\Gamma$."
2. $\Gamma \vdash A = B\ \mathsf{type}$ — "$A$ and $B$ are definitionally equal types in context $\Gamma$."
3. $\Gamma \vdash a : A$ — "$a$ is a term of type $A$ in context $\Gamma$."
4. $\Gamma \vdash a = b : A$ — "$a$ and $b$ are definitionally equal terms of type $A$ in context $\Gamma$."

These four judgments, and the rules for deriving them, constitute MLTT.

## Contexts: What $\Gamma$ Is

A *context* $\Gamma$ is a list of variable declarations:

$$\Gamma = x_1 : A_1,\, x_2 : A_2(x_1),\, \ldots,\, x_n : A_n(x_1, \ldots, x_{n-1})$$

where each type $A_i$ may depend on the variables declared earlier. This is what "dependent" means: the type of a later variable can mention the values of earlier variables.

**Examples of contexts:**
- $()$ — the empty context, no assumptions
- $n : \mathbb{N}$ — one variable, a natural number
- $n : \mathbb{N},\, v : \mathsf{Vec}(A, n)$ — a number and a vector of that length
- $A : \mathsf{Type},\, x : A,\, y : A,\, p : x =_A y$ — a type, two elements, and a proof they're equal

The type of $v$ in the second example ($\mathsf{Vec}(A, n)$) depends on the value of $n$. The type of $p$ in the fourth example ($x =_A y$) depends on $x$ and $y$.

**Context extension.** If $\Gamma$ is a valid context and $\Gamma \vdash A\ \mathsf{type}$, then $\Gamma, x : A$ is a valid context for a fresh variable $x$ (one that doesn't already appear in $\Gamma$).

**The empty context.** In the empty context $()$, judgments hold "for free" — without any assumptions. A judgment $\vdash t : A$ (without $\Gamma$) means $t$ is a closed term of closed type $A$.

## Why Four Judgments?

Why four and not just two (terms have types, that's it)?

**Reason 1: Types need to be checked too.** In dependent type theory, forming a type can fail. $\mathsf{Vec}(A, x)$ requires $x : \mathbb{N}$, not $x : \mathbb{B}$. Before you can use a type, you need to verify it's well-formed. The judgment $A\ \mathsf{type}$ ensures this.

**Reason 2: Definitional equality matters.** The type checker needs to identify terms up to computational equivalence. $(\lambda x. x + 1)\, 4$ and $5$ are definitionally equal — the type checker treats them the same. Making this a first-class judgment allows the conversion rule: if $a : A$ and $A = B\ \mathsf{type}$, then $a : B$.

**Reason 3: Propositional vs. definitional equality are genuinely different.** The type $a =_A b$ (propositional equality, an element of the identity type) is something you prove. The judgment $a = b : A$ (definitional equality) is something the type checker verifies by reduction. Keeping them separate is crucial for the theory to be consistent and decidable.

## The Structural Rules

Several rules govern all judgment forms regardless of the specific type being dealt with.

**Variable rule:**
$$\frac{x : A \in \Gamma}{\Gamma \vdash x : A}$$
Every variable in the context can be used as a term of its declared type.

**Weakening:** If $\Gamma \vdash J$ (some judgment) and $\Gamma' \supseteq \Gamma$ is an extension, then $\Gamma' \vdash J$. Adding more variables to the context doesn't invalidate what we already know.

**Substitution:** If $\Gamma, x : A, \Gamma' \vdash J$ and $\Gamma \vdash a : A$, then $\Gamma, \Gamma'[a/x] \vdash J[a/x]$. Substituting a specific term for a variable is valid.

**Conversion (type conversion / *transport in contexts*):**
$$\frac{\Gamma \vdash a : A \quad \Gamma \vdash A = B\ \mathsf{type}}{\Gamma \vdash a : B}$$
If two types are definitionally equal, a term of one is also a term of the other. This is the rule that makes definitional equality useful: the type checker silently applies it.

These structural rules ensure that well-typed terms remain well-typed as we move through derivations.

## Definitional Equality: Properties

The judgment $\Gamma \vdash a = b : A$ (definitional equality of terms) is an equivalence relation:

**Reflexivity:** $\Gamma \vdash a = a : A$

**Symmetry:** If $\Gamma \vdash a = b : A$ then $\Gamma \vdash b = a : A$

**Transitivity:** If $\Gamma \vdash a = b : A$ and $\Gamma \vdash b = c : A$ then $\Gamma \vdash a = c : A$

**Congruence:** Definitional equality is preserved by all type formers. If $\Gamma, x : A \vdash t : B$ and $\Gamma \vdash a = a' : A$, then $\Gamma \vdash t[a/x] = t[a'/x] : B[a/x]$.

**Includes $\beta$-reduction:** $(\lambda x. t)\, a \equiv t[a/x]$ is definitional equality.

**Includes $\delta$-reduction:** The computation rules for recursors are definitional equalities. E.g., $\mathsf{ind}_\mathbb{N}(P, p_0, p_s, \mathsf{zero}) = p_0$ is a definitional equality.

**Includes $\eta$-expansion:** (In some formulations) $f = \lambda x. f\, x$ and $p = (\pi_1 p, \pi_2 p)$ are definitional.

The type checker for MLTT works by normalizing terms and checking if their normal forms are identical (modulo $\alpha$-equivalence of bound variables). Because every well-typed term normalizes (strong normalization), this is decidable.

## The Four Judgment Forms in Lean 4

Lean 4 makes these judgments visible (mostly implicitly):

```lean
-- Judgment 1: A type
-- In Lean, "A type" is written as "A : Type" or "A : Prop"
example : Type := Nat  -- Nat is a type

-- Judgment 3: a : A
example : Nat := 42  -- 42 is a natural number

-- Judgment 4: a = b : A (definitional equality)
-- These hold by rfl because they're definitionally equal
example : (fun x => x + 1) 4 = 5 := rfl
-- Lean normalizes both sides and checks they match

-- Judgment 2: A = B type (definitional equality of types)
-- Less commonly explicit, but the conversion rule uses it silently
-- e.g., if A =def B, you can use an A-term where B is expected
```

Definitional equality in Lean 4 is checked automatically by the kernel. Propositional equality (what you prove in theorems) is a separate matter.

## Telescopes: Dependent Contexts

A *telescope* is a context viewed as a type. If $\Gamma = x_1 : A_1, \ldots, x_n : A_n(x_1, \ldots, x_{n-1})$, the corresponding type is the iterated Σ type:

$$\llbracket \Gamma \rrbracket = \sum_{x_1:A_1} \sum_{x_2:A_2(x_1)} \cdots A_n(x_1, \ldots, x_{n-1})$$

This *telescoping* correspondence between contexts and Σ types is important for:
- Defining substitution into contexts
- The semantics of MLTT in categories (contextual categories)
- The formalization of MLTT inside another type theory

The judgment $\Gamma \vdash a : A$ becomes, in the telescoped view, "given an element of $\llbracket \Gamma \rrbracket$ (a choice of all the context variables), $a$ has type $A$."

## Well-Founded Derivations

A *derivation* (or *proof tree*) for a judgment $\Gamma \vdash J$ is a tree whose root is labeled $\Gamma \vdash J$ and whose leaves are axioms, with each internal node justified by one of the inference rules of MLTT.

The rules are designed so that derivations are *well-founded*: there are no circular derivations. Every derivation terminates, and the type checker can verify any judgment by a finite computation.

This is guaranteed by the structure of MLTT:
1. Types are formed from "simpler" types (Π requires A and B; B can only depend on A)
2. Contexts grow by adding variables, never shrinking in a circular way
3. The universe hierarchy prevents $\mathsf{Type}_i : \mathsf{Type}_i$

The decidability of type checking follows: given $\Gamma$, $a$, and $A$, you can determine if $\Gamma \vdash a : A$ by a terminating algorithm (normalize everything, check the normal forms).

## Summary

The four judgments of MLTT:
1. $\Gamma \vdash A\ \mathsf{type}$
2. $\Gamma \vdash A = B\ \mathsf{type}$
3. $\Gamma \vdash a : A$
4. $\Gamma \vdash a = b : A$

...are the primitive assertions of the system. They're closed under the structural rules (weakening, substitution, conversion) and the rules for each specific type former.

The two equality judgments distinguish MLTT from simpler type theories: definitional equality (judgment 2 and 4) is mechanical and decidable; propositional equality ($a =_A b$ as a type) is the identity type, which we study in Section 3. The relationship between the two is one of the deepest aspects of MLTT and the foundation of HoTT.
