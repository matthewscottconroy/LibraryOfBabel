# 2.1 Simply Typed Lambda Calculus: Rules and Reduction

## The Formal System

The **simply typed lambda calculus (STLC)** is the formal type-theoretic system corresponding to intuitionistic propositional logic. It's called "simply typed" because types don't depend on terms — a key distinction from dependent type theory.

STLC has three components: a grammar of types, a grammar of terms, and typing rules connecting them.

## Types

**Type grammar:**
$$A, B, C ::= o \mid A \to B \mid A \times B \mid A + B \mid \mathbf{1} \mid \mathbf{0}$$

Where:
- $o$ is a base type (think: booleans, natural numbers, or some atomic type)
- $A \to B$ is the function type (reads "A to B")
- $A \times B$ is the product type (pairs)
- $A + B$ is the sum type (disjoint union, tagged union)
- $\mathbf{1}$ is the unit type (one element)
- $\mathbf{0}$ is the empty type (no elements)

**Logical correspondence:**
$$\to \quad \leftrightarrow \quad \to$$
$$\times \quad \leftrightarrow \quad \wedge$$
$$+ \quad \leftrightarrow \quad \vee$$
$$\mathbf{1} \quad \leftrightarrow \quad \top$$
$$\mathbf{0} \quad \leftrightarrow \quad \bot$$

## Terms

**Term grammar:**
$$t, s, u ::= x \mid \lambda x : A.\, t \mid t\,s \mid (t, s) \mid \text{fst}(t) \mid \text{snd}(t)$$
$$\mid \text{inl}(t) \mid \text{inr}(s) \mid \text{case}(t, x.s, y.u)$$
$$\mid \star \mid \text{absurd}(t)$$

Where:
- $x$ is a variable
- $\lambda x : A.\, t$ is lambda abstraction (a function)
- $t\,s$ is application (apply function $t$ to argument $s$)
- $(t, s)$ is a pair
- $\text{fst}(t)$, $\text{snd}(t)$ are projections
- $\text{inl}(t)$, $\text{inr}(s)$ are injections into sum types
- $\text{case}(t, x.s, y.u)$ is case analysis on a sum
- $\star : \mathbf{1}$ is the unique element of the unit type
- $\text{absurd}(t) : A$ eliminates a term of the empty type

## Typing Rules

A **typing context** $\Gamma = x_1 : A_1, \ldots, x_n : A_n$ assigns types to variables.

A **typing judgment** $\Gamma \vdash t : A$ says "in context $\Gamma$, term $t$ has type $A$."

**Variable:**
$$\frac{}{\Gamma, x : A \vdash x : A}$$

**Lambda abstraction:**
$$\frac{\Gamma, x : A \vdash t : B}{\Gamma \vdash \lambda x : A.\, t : A \to B}$$

**Application:**
$$\frac{\Gamma \vdash f : A \to B \quad \Gamma \vdash a : A}{\Gamma \vdash f\, a : B}$$

**Pair:**
$$\frac{\Gamma \vdash a : A \quad \Gamma \vdash b : B}{\Gamma \vdash (a, b) : A \times B}$$

**First projection:**
$$\frac{\Gamma \vdash p : A \times B}{\Gamma \vdash \text{fst}(p) : A}$$

**Second projection:**
$$\frac{\Gamma \vdash p : A \times B}{\Gamma \vdash \text{snd}(p) : B}$$

**Left injection:**
$$\frac{\Gamma \vdash a : A}{\Gamma \vdash \text{inl}(a) : A + B}$$

**Right injection:**
$$\frac{\Gamma \vdash b : B}{\Gamma \vdash \text{inr}(b) : A + B}$$

**Case analysis:**
$$\frac{\Gamma \vdash e : A + B \quad \Gamma, x : A \vdash s : C \quad \Gamma, y : B \vdash u : C}{\Gamma \vdash \text{case}(e, x.s, y.u) : C}$$

**Unit:**
$$\frac{}{\Gamma \vdash \star : \mathbf{1}}$$

**Absurdity:**
$$\frac{\Gamma \vdash e : \mathbf{0}}{\Gamma \vdash \text{absurd}(e) : A}$$

## Properties of Typing

**Theorem (Uniqueness of Types).** If $\Gamma \vdash t : A$ and $\Gamma \vdash t : B$, then $A = B$.

This holds for STLC because type inference is decidable and deterministic.

**Theorem (Weakening).** If $\Gamma \vdash t : A$ then $\Gamma, x : B \vdash t : A$ (provided $x$ is fresh).

**Theorem (Substitution).** If $\Gamma, x : A \vdash t : B$ and $\Gamma \vdash s : A$, then $\Gamma \vdash t[s/x] : B$.

Substitution is the key lemma for the Curry-Howard correspondence: it corresponds to the cut rule in logic, and to function application.

## Beta Reduction Rules

**Beta reduction** is the basic computation step.

**For functions ($\lambda\beta$):**
$$(\lambda x : A.\, t)\, s \to_\beta t[s/x]$$

Applying a function to an argument is the same as substituting the argument for the variable.

**For products ($\text{fst}/\text{snd}\beta$):**
$$\text{fst}((a, b)) \to_\beta a$$
$$\text{snd}((a, b)) \to_\beta b$$

**For sums ($\text{case}\beta$):**
$$\text{case}(\text{inl}(a), x.s, y.u) \to_\beta s[a/x]$$
$$\text{case}(\text{inr}(b), x.s, y.u) \to_\beta u[b/y]$$

Each beta rule corresponds to the proof-normalization step that removes an "introduction followed by elimination" detour.

## Eta Reduction Rules

**Eta reduction** identifies "extensionally equal" terms.

**For functions ($\lambda\eta$):**
$$\lambda x.\, f\, x \to_\eta f \quad (\text{if } x \notin \text{FV}(f))$$

A function that just applies $f$ to its argument is the same as $f$ itself.

**For products ($\text{pair}\eta$):**
$$(\text{fst}(p), \text{snd}(p)) \to_\eta p$$

A pair formed by projecting from $p$ is the same as $p$.

**For sums ($\text{case}\eta$):**
$$\text{case}(e, x.\text{inl}(x), y.\text{inr}(y)) \to_\eta e$$

Eta rules express function extensionality for each type constructor.

## Type Checking is Decidable

**Theorem.** The problem "given $\Gamma$, $t$, $A$: does $\Gamma \vdash t : A$?" is decidable for STLC.

This is crucial: the type checker (proof checker) can be mechanically implemented. This is the foundation of proof assistants.

*Proof sketch.* By structural induction on $t$. At each constructor, there's a unique rule that could apply (given the shape of $t$), and the premises can be checked recursively. The base cases (variables) are decided by looking up in $\Gamma$. $\square$

**Note:** In the untyped λ-calculus, there's no decidable notion of "type." The power of STLC is precisely that types make proof checking decidable.

## Example Computations

**Identity:**
$$(\lambda x : A.\, x)\, a \to_\beta a$$

This terminates immediately: applying the identity function just returns the argument.

**Composition:**
$$(\lambda f.\, \lambda g.\, \lambda x.\, g\,(f\, x))\, s\, t\, a$$
$$\to_\beta (\lambda g.\, \lambda x.\, g\,(s\, x))\, t\, a$$
$$\to_\beta (\lambda x.\, t\,(s\, x))\, a$$
$$\to_\beta t\,(s\, a)$$

Three reduction steps, as expected for a three-argument function.

**Currying:**
$$\text{fst}((\lambda f.\, \lambda a.\, \lambda b.\, f\,(a, b))\, h\, x\, y)$$
$$\to_\beta \text{fst}((\lambda a.\, \lambda b.\, h\,(a, b))\, x\, y)$$
$$\to_\beta \text{fst}((\lambda b.\, h\,(x, b))\, y)$$
$$\to_\beta \text{fst}(h\,(x, y))$$

(If $h : A \times B \to C \times D$, this gives the first component of $h(x, y)$.)

## The Normal Forms

A term is in **beta-normal form** if no beta reduction can be applied.

Normal forms for STLC have a simple characterization:
- Variables $x$ are normal.
- Lambda abstractions $\lambda x.\, t$ where $t$ is normal are normal.
- Applications $f\, a$ where $f$ is a variable (not a lambda) and $a$ is normal are normal.
- Pairs $(a, b)$ where $a, b$ are normal are normal.
- Projections $\text{fst}(p)$, $\text{snd}(p)$ where $p$ is a normal variable are normal.
- Injections $\text{inl}(a)$, $\text{inr}(b)$ where $a, b$ are normal are normal.
- Case expressions where the scrutinee is a normal variable.
- $\star$ and $\text{absurd}(e)$ where $e$ is normal.

Crucially: a closed normal form of function type is a lambda abstraction; a closed normal form of product type is a pair; a closed normal form of sum type is an injection. This is the *canonicity* property.

## Canonicity

**Theorem (Canonicity).** Every closed term of STLC reduces to a canonical form:
- Type $A \to B$: reduces to $\lambda x.\, t$.
- Type $A \times B$: reduces to $(a, b)$.
- Type $A + B$: reduces to $\text{inl}(a)$ or $\text{inr}(b)$.
- Type $\mathbf{1}$: reduces to $\star$.
- Type $\mathbf{0}$: no closed term exists (consistent!).

This is the computational content of the disjunction and existence properties of IPC. A closed program of sum type computes to either "left" or "right."

Canonicity is one of the fundamental theorems of type theory — it shows the system is "well-behaved" and has genuine computational content.
