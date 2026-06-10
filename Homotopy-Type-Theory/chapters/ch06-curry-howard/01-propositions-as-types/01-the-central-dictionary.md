# 1.1 The Central Dictionary

## The Fundamental Identification

The Curry-Howard correspondence rests on a single, powerful identification:

> **A proposition $P$ is identified with the type of its proofs.**

The proposition $A \wedge B$ is identified with the type of pairs $(a, b)$ where $a$ proves $A$ and $b$ proves $B$. The proposition $A \to B$ is identified with the type of functions that map proofs of $A$ to proofs of $B$.

Under this identification, *having a proof of $P$* is the same as *having a term of type $P$*. A proof is a program; a proposition is a type.

Let's work through the dictionary systematically.

## The Logical Connectives

**Conjunction: $A \wedge B$ as the product type $A \times B$**

A proof of $A \wedge B$ is a pair: a proof of $A$ and a proof of $B$.

Type-theoretically: an element of $A \times B$ is a pair $(a, b)$ where $a : A$ and $b : B$.

The correspondence:
- $\wedge$-Introduction: $\frac{\vdash a : A \quad \vdash b : B}{\vdash (a, b) : A \times B}$
- $\wedge$-Elimination$_1$: $\frac{\vdash p : A \times B}{\vdash \text{fst}(p) : A}$
- $\wedge$-Elimination$_2$: $\frac{\vdash p : A \times B}{\vdash \text{snd}(p) : B}$

**Implication: $A \to B$ as the function type $A \to B$**

A proof of $A \to B$ is a function: given any proof of $A$, it produces a proof of $B$.

Type-theoretically: a function $f : A \to B$ takes any $a : A$ and returns $f(a) : B$.

The correspondence:
- $\to$-Introduction: $\frac{x : A \vdash t : B}{\vdash \lambda x : A.\, t : A \to B}$
- $\to$-Elimination (modus ponens): $\frac{\vdash f : A \to B \quad \vdash a : A}{\vdash f(a) : B}$

**Disjunction: $A \vee B$ as the coproduct type $A + B$**

A proof of $A \vee B$ is either a tagged proof of $A$ (left) or a tagged proof of $B$ (right).

Type-theoretically: an element of $A + B$ is either $\text{inl}(a)$ for $a : A$ or $\text{inr}(b)$ for $b : B$.

The correspondence:
- $\vee$-Introduction$_1$: $\frac{\vdash a : A}{\vdash \text{inl}(a) : A + B}$
- $\vee$-Introduction$_2$: $\frac{\vdash b : B}{\vdash \text{inr}(b) : A + B}$
- $\vee$-Elimination (case analysis): $\frac{\vdash e : A + B \quad x : A \vdash t : C \quad y : B \vdash s : C}{\vdash \text{case}(e, x.t, y.s) : C}$

**False: $\bot$ as the empty type $\mathbf{0}$**

$\bot$ has no proof. $\mathbf{0}$ has no elements.

The correspondence:
- $\bot$-Elimination (ex falso): $\frac{\vdash e : \mathbf{0}}{\vdash \text{absurd}(e) : A}$ for any type $A$.

A function $\mathbf{0} \to A$ can pattern-match on all zero cases — trivially.

**True: $\top$ as the unit type $\mathbf{1}$**

$\top$ has exactly one proof (the trivial proof). $\mathbf{1}$ has exactly one element: $\star$.

**Negation: $\neg A$ as $A \to \mathbf{0}$**

$\neg A = A \to \bot$ is the type of functions from $A$ to the empty type. If $A$ has no elements (is "false"), then there's a unique function $A \to \mathbf{0}$ (the empty function). If $A$ has an element $a$, then we could "apply" a function $f : A \to \mathbf{0}$ to $a$ to get $f(a) : \mathbf{0}$ — which is impossible. So $A \to \mathbf{0}$ is only inhabited when $A$ is empty.

## The Proof Rules as Typing Rules

The striking fact is that the natural deduction rules and the typing rules are the *same rules*, just with different notation.

**Natural deduction** writes: $\Gamma \vdash \varphi$ (context $\Gamma$ implies proposition $\varphi$)

**Type theory** writes: $\Gamma \vdash t : A$ (context $\Gamma$ implies term $t$ has type $A$)

The rules are:

| Natural Deduction | Type Theory |
|---|---|
| $\overline{\Gamma, \varphi \vdash \varphi}$ (Hyp) | $\overline{\Gamma, x:A \vdash x : A}$ (Var) |
| $\frac{\Gamma, \varphi \vdash \psi}{\Gamma \vdash \varphi \to \psi}$ ($\to$I) | $\frac{\Gamma, x:A \vdash t : B}{\Gamma \vdash \lambda x.\,t : A \to B}$ (Abs) |
| $\frac{\Gamma \vdash \varphi \to \psi \quad \Gamma \vdash \varphi}{\Gamma \vdash \psi}$ ($\to$E) | $\frac{\Gamma \vdash f : A \to B \quad \Gamma \vdash a : A}{\Gamma \vdash f\,a : B}$ (App) |
| $\frac{\Gamma \vdash \varphi \quad \Gamma \vdash \psi}{\Gamma \vdash \varphi \wedge \psi}$ ($\wedge$I) | $\frac{\Gamma \vdash a : A \quad \Gamma \vdash b : B}{\Gamma \vdash (a, b) : A \times B}$ (Pair) |
| $\frac{\Gamma \vdash \varphi \wedge \psi}{\Gamma \vdash \varphi}$ ($\wedge$E$_1$) | $\frac{\Gamma \vdash p : A \times B}{\Gamma \vdash \text{fst}(p) : A}$ (Proj$_1$) |

The columns are isomorphic. The proof rules and the typing rules are the same formal system, with the same inference rule shapes.

## Proof Normalization = Computation

Under the correspondence, the β-reductions of proof theory correspond to computation steps in the λ-calculus:

**Conjunction β-reduction:**
$$\text{fst}((a, b)) \to_\beta a$$
Corresponds to: the detour of proving $A \wedge B$ and then extracting $A$ simplifies to just having $A$.

**Implication β-reduction:**
$$(\lambda x : A.\, t)\, a \to_\beta t[a/x]$$
Corresponds to: the detour of proving $A \to B$ (by assuming $A$ and deriving $B$) and then applying it to a proof of $A$ simplifies to directly using $a$ in the derivation of $B$.

**Disjunction β-reduction (left):**
$$\text{case}(\text{inl}(a), x.t, y.s) \to_\beta t[a/x]$$
Corresponds to: if you know you're in the left case, take the left branch.

These are not metaphors — they are literally the same formal operation, described in two different notations.

## Examples of the Correspondence

**Example: $P \to P$ (Identity)**

Logic: The proof is the one-rule derivation: assume $P$, conclude $P$.

Type theory: The term is $\lambda x : P.\, x$ (the identity function). Type: $P \to P$.

Reduction: $(\lambda x.\, x)\, a \to_\beta a$. No interesting computation — the identity function is trivial.

**Example: Commutativity of conjunction: $A \wedge B \to B \wedge A$**

Logic: Assume $A \wedge B$. Extract $A$ and $B$. Pair in reverse order to get $B \wedge A$.

Type theory: $\lambda p : A \times B.\, (\text{snd}(p), \text{fst}(p))$. Type: $A \times B \to B \times A$.

Computation: Given any pair $(a, b)$, the function produces $(b, a)$.

**Example: Composition: $(A \to B) \to (B \to C) \to (A \to C)$**

Logic: Assume $A \to B$ and $B \to C$. Given any proof of $A$, apply the first function to get $B$, then the second to get $C$.

Type theory: $\lambda f : A \to B.\, \lambda g : B \to C.\, \lambda x : A.\, g\,(f\, x)$.

Computation: Function composition.

**Example: Curry: $(A \times B \to C) \to (A \to B \to C)$**

Logic: Given a function from pairs to $C$, produce a function that takes $A$ then $B$ then returns $C$.

Type theory: $\lambda f : A \times B \to C.\, \lambda a : A.\, \lambda b : B.\, f\,(a, b)$.

Computation: Currying — converting a function on pairs to a function taking arguments one at a time.

## Classical Logic and Computation

What about classical logic? LEM, DNE, Peirce's Law?

**Peirce's Law: $((A \to B) \to A) \to A$**

This would be a term of type $((A \to B) \to A) \to A$ in a type-theoretic system. No such closed term exists in the simply typed λ-calculus (STLC).

However, it *does* exist if we add *continuations* or *call-with-current-continuation (call/cc)* to the language. The term is:
$$\lambda k : (A \to B) \to A.\, k\,(\lambda x : A.\, \text{throw}(x))$$
where `throw` is the continuation — it "escapes" the current computation context with the value $x$.

This is the *computational content* of Peirce's Law: it's the control operator `call/cc` (Scheme) or similar constructs. Classical axioms correspond to *non-standard computational features* like continuations, which are valid programs but don't "return" in the usual sense.

This is why classical logic doesn't have clean computational content: its "extra" logical power corresponds to programs that can jump out of the current context (exceptions, continuations), rather than computing a direct value.

## The Propositions-as-Types Worldview

The propositions-as-types correspondence is more than a formal isomorphism. It suggests a deep reinterpretation of both logic and type theory:

- **Logic** is not just about truth and falsity — it's about *evidence and construction*.
- **Type theory** is not just about data and operations — it's about *propositions and proofs*.
- **Programming** is not just about computing values — it's about *constructing mathematical objects*.

This worldview underlies HoTT: in HoTT, mathematical structures are types, their properties are propositions (also types), proofs are terms, and equality proofs are paths. The entire edifice is a single unified system where logic, programming, and mathematics are aspects of the same thing.

The dictionary we've built in this section is the first step. In the remaining sections, we'll make it precise and extend it to the full type-theoretic setting.
