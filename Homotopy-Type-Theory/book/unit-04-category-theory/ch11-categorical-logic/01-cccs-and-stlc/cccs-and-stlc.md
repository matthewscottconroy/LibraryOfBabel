# Cartesian Closed Categories and STLC

## The Correspondence

Simply typed lambda calculus (STLC) and cartesian closed categories (CCCs) are the same mathematical structure, expressed in different languages. This is not a vague analogy. It is a theorem: there is a bijection between STLC types and objects of any CCC, between STLC terms and morphisms in that CCC, and between STLC equations (beta/eta) and equalities of morphisms.

The precise statement: the *classifying CCC* of STLC is a CCC $\mathcal{C}_\mathsf{STLC}$ with the property that any model of STLC in a CCC $\mathcal{D}$ corresponds to a unique (up to natural isomorphism) CCC-functor $\mathcal{C}_\mathsf{STLC} \to \mathcal{D}$. And conversely, any CCC gives a model of STLC. The correspondence is fully bidirectional.

## Cartesian Closed Categories

**Definition.** A category $\mathcal{C}$ is *cartesian closed* (a CCC) if it has:

1. **A terminal object** $1$: for every object $A$, a unique morphism $! : A \to 1$.

2. **Binary products**: for every $A, B$, an object $A \times B$ with projections $\pi_1 : A \times B \to A$ and $\pi_2 : A \times B \to B$, and for every pair of morphisms $f : C \to A$ and $g : C \to B$, a unique morphism $\langle f, g \rangle : C \to A \times B$ with $\pi_1 \circ \langle f, g \rangle = f$ and $\pi_2 \circ \langle f, g \rangle = g$.

3. **Exponentials**: for every $A, B$, an object $[A, B]$ (the *exponential*) with an *evaluation morphism* $\mathsf{ev} : [A, B] \times A \to B$, and for every morphism $f : C \times A \to B$, a unique morphism $\Lambda(f) : C \to [A, B]$ with $\mathsf{ev} \circ (\Lambda(f) \times \mathsf{id}_A) = f$.

The last condition is the adjunction: $\mathsf{Hom}(C \times A, B) \cong \mathsf{Hom}(C, [A, B])$, via the bijection $f \leftrightarrow \Lambda(f)$.

## The Dictionary

The translation between STLC and CCC:

| STLC | CCC |
|---|---|
| Base type $o$ | Object $A$ |
| Type $A \to B$ | Exponential object $[A, B]$ |
| Type $A \times B$ | Product object $A \times B$ |
| Type $\mathbf{1}$ | Terminal object $1$ |
| Context $x_1:A_1, \ldots, x_n:A_n$ | Object $A_1 \times \cdots \times A_n$ |
| Term $\Gamma \vdash t : A$ | Morphism $\llbracket \Gamma \rrbracket \to \llbracket A \rrbracket$ |
| Variable $x_i : A_i$ in context $\Gamma$ | Projection morphism $\pi_i : \llbracket \Gamma \rrbracket \to \llbracket A_i \rrbracket$ |
| Application $t\, s$ | $\mathsf{ev} \circ (\llbracket t \rrbracket, \llbracket s \rrbracket)$ |
| Abstraction $\lambda x : A. t$ | $\Lambda(\llbracket t \rrbracket)$ |
| Pairing $(t, s)$ | $\langle \llbracket t \rrbracket, \llbracket s \rrbracket \rangle$ |
| Projection $\pi_1\, t$ | $\pi_1 \circ \llbracket t \rrbracket$ |
| Substitution $t[s/x]$ | Composition of morphisms |
| $\beta$-reduction $(\lambda x. t)\, s \to t[s/x]$ | Counit equation of the adjunction |
| $\eta$-expansion $f = \lambda x. (f\, x)$ | Uniqueness clause of the adjunction |

## Interpreting Terms

**Variables.** In context $\Gamma = x_1:A_1, \ldots, x_n:A_n$, the interpretation is $\llbracket \Gamma \rrbracket = A_1 \times \cdots \times A_n$. The variable $x_i : A_i$ is interpreted as the $i$-th projection $\pi_i : A_1 \times \cdots \times A_n \to A_i$.

**Abstraction.** The term $\Gamma \vdash \lambda x : A. t : A \to B$ is interpreted as $\llbracket \lambda x. t \rrbracket = \Lambda(\llbracket t \rrbracket)$ where $\llbracket t \rrbracket : \llbracket \Gamma \rrbracket \times A \to B$. The CCC adjunction gives $\Lambda(\llbracket t \rrbracket) : \llbracket \Gamma \rrbracket \to [A, B]$.

**Application.** The term $\Gamma \vdash t\, s : B$ (where $t : A \to B$ and $s : A$) is interpreted as:
$$\llbracket t\, s \rrbracket = \mathsf{ev} \circ \langle \llbracket t \rrbracket, \llbracket s \rrbracket \rangle : \llbracket \Gamma \rrbracket \to B$$

This is evaluation of the exponential at the argument.

**Substitution.** The substitution $t[s/x]$ corresponds to precomposition: $\llbracket t[s/x] \rrbracket = \llbracket t \rrbracket \circ (\mathsf{id}_\Gamma, \llbracket s \rrbracket)$ where $(\mathsf{id}_\Gamma, \llbracket s \rrbracket) : \llbracket \Gamma \rrbracket \to \llbracket \Gamma \rrbracket \times A$ is the pairing of the identity and $s$.

## The $\beta$ and $\eta$ Laws as Categorical Equations

**$\beta$-reduction.** The rule $(\lambda x. t)\, s \to_\beta t[s/x]$ corresponds to the counit equation of the product-exponential adjunction:

$$\mathsf{ev} \circ (\Lambda(f) \times \mathsf{id}_A) = f$$

This is the equation stating that $\Lambda(f)$ followed by evaluation gives back $f$. In type theory: $(\lambda x. t)$ followed by application to $s$ gives $t[s/x]$.

**$\eta$-expansion.** The rule $f =_\eta \lambda x. (f\, x)$ for $f : A \to B$ corresponds to the uniqueness clause of the adjunction: $\Lambda(\mathsf{ev} \circ (f \times \mathsf{id}_A)) = f$. Any morphism $f : C \to [A, B]$ is equal to $\Lambda(\mathsf{ev} \circ (f \times \mathsf{id}_A))$ — abstracting the evaluation of $f$ gives back $f$ itself.

## Soundness and Completeness

**Soundness.** If $\Gamma \vdash t = s : A$ (propositional equality in STLC via $\beta\eta$-conversion), then $\llbracket t \rrbracket = \llbracket s \rrbracket$ in any CCC. Every CCC is a model of STLC.

*Proof sketch.* The $\beta$ equation follows from the adjunction counit; the $\eta$ equation from uniqueness; all other equations from naturality and the axioms of a CCC. By induction on the proof of $t = s$, $\llbracket t \rrbracket = \llbracket s \rrbracket$.

**Completeness.** If $\llbracket t \rrbracket = \llbracket s \rrbracket$ in the *classifying CCC* of STLC, then $\Gamma \vdash t = s : A$. The classifying CCC is constructed directly from the syntax of STLC.

*Construction of the classifying CCC.* Objects: types of STLC. Morphisms $A \to B$: $\beta\eta$-equivalence classes of terms $x:A \vdash t : B$. Composition: substitution. Products and exponentials: given by the type constructors. This category is a CCC, and by definition the interpretation of a term in it recovers its $\beta\eta$-equivalence class.

## The Product Adjunction: Currying

The central adjunction of a CCC is currying:

$$\mathsf{Hom}(C \times A, B) \cong \mathsf{Hom}(C, [A, B])$$

In type theory: functions $C \times A \to B$ correspond to functions $C \to (A \to B)$.

**Unit:** $\eta_C : C \to [A, C \times A]$, $c \mapsto \lambda a. (c, a)$.
**Counit:** $\varepsilon_B : [A, B] \times A \to B$, $(f, a) \mapsto f(a)$ (function application).

**Triangular identities:**
- $\varepsilon_{[A,B]} \circ ([A, (\varepsilon_B)] \times \mathsf{id}_A) = \mathsf{id}_{[A,B]}$: evaluating the currying of evaluation gives the identity.
- $\Lambda(\varepsilon_B \circ (\Lambda(\mathsf{id}) \times \mathsf{id})) = \mathsf{id}_B$: the other direction.

## Examples of CCCs

**$\mathbf{Set}$:** The exponential is the set of functions $[A, B] = B^A = \{f : A \to B\}$. Currying is the set-theoretic bijection between $\{f : C \times A \to B\}$ and $\{f : C \to B^A\}$.

**Presheaves $[\mathcal{C}^{op}, \mathbf{Set}]$:** For any small category $\mathcal{C}$, the presheaf category is a CCC. The exponential of presheaves $F$ and $G$ is $[F, G](A) = \mathsf{Nat}(F \times \mathsf{y}(A), G)$ where $\mathsf{y}$ is the Yoneda embedding.

**Domains (Scott domains):** The category of Scott domains and continuous functions is a CCC. This is the semantics of the untyped lambda calculus: a domain $D$ satisfying $D \cong [D, D]$ gives a self-interpretation of the lambda calculus. Dana Scott's construction of this domain in 1969 was the first denotational semantics of the untyped lambda calculus.

**Types in MLTT:** The category of types in Martin-Löf Type Theory is a CCC (and more: it is an LCCC). Products are $\Sigma$ types with a fixed left component; exponentials are $\Pi$ types with a fixed domain.

## Beyond STLC: Product Types and Co-products

STLC with products is modeled by CCCs. Adding coproducts ($+$ types) requires *bicartesian closed categories* (BCCCs) — CCCs with coproducts (initial objects and binary coproducts). In a BCCC:

- Initial object $0$ ↔ empty type $\mathbf{0}$
- Coproduct $A + B$ ↔ sum type $A + B$
- Morphisms out of $A + B$ ↔ case analysis

The distributive law $A \times (B + C) \cong (A \times B) + (A \times C)$ holds in any BCCC. This is the type-theoretic distribution of products over sums — a theorem, not an axiom, in any BCCC.

Adding natural numbers as an initial algebra for the "successor" functor $X \mapsto 1 + X$ gives the category with a natural numbers object (NNO), modeling STLC + $\mathbb{N}$. The recursion principle for $\mathbb{N}$ is exactly the universal property of the initial algebra.
