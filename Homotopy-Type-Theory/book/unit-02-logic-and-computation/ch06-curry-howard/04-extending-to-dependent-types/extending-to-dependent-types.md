# Extending to Dependent Types

## The Gap in the Correspondence

The Curry-Howard correspondence for STLC is clean and complete — for propositional logic. Every proposition in intuitionistic propositional logic corresponds to an STLC type, and every STLC type corresponds to a propositional formula.

But mathematics is not just propositional. Most interesting mathematical statements involve *quantification over values*: "for all natural numbers $n$," "there exists a prime $p$ with property $P(p)$." Propositional logic cannot express these statements. We need predicate logic — and predicate logic requires types that depend on values.

Consider the theorem: "for all natural numbers $n$, there exists a natural number $m$ greater than $n$." This is $\forall n : \mathbb{N}, \exists m : \mathbb{N}, m > n$. The proposition $\exists m : \mathbb{N}, m > n$ depends on $n$ — different values of $n$ give different propositions. In STLC, we cannot form a type that depends on a term.

To express predicate logic via Curry-Howard, we need *dependent types*: types that depend on values. This is the extension from STLC to Martin-Löf Type Theory.

## Why STLC Is Not Enough

Here are three things STLC cannot express:

**1. Vectors of length $n$.** A vector of natural numbers of length 3 and a vector of length 7 have fundamentally different types — you cannot append a length-3 vector to a length-3 vector to get a length-7 vector unless the type system tracks lengths. In STLC, all vectors have the same type `List Nat`, and the length is untracked. In dependent type theory, we can form the type `Vec Nat n` — the type of vectors of natural numbers of *length $n$* — and the append function has type `Vec A m → Vec A n → Vec A (m + n)`. The type tracks the length, and the type checker enforces that lengths add correctly.

**2. Sorted arrays.** An array of type `Array Nat` is just a sequence of numbers. But a *sorted* array — one where every element is $\leq$ the next — is an element of a more specific type. In STLC, we cannot express "a sorted array of length $n$" as a type. In dependent type theory, we form `SortedArray Nat n` as a dependent pair: an array together with a proof that it is sorted.

**3. The statement of the Pythagorean theorem.** "For all right triangles with legs $a$ and $b$ and hypotenuse $c$, $a^2 + b^2 = c^2$." This is $\forall a, b, c : \mathbb{R}, \forall h : \text{isRightTriangle}(a, b, c), a^2 + b^2 = c^2$. The hypothesis `h : isRightTriangle(a, b, c)` is a proof that $(a, b, c)$ forms a right triangle, and its type depends on $a$, $b$, $c$. STLC cannot express this.

## The $\Pi$-Type: Dependent Functions

The dependent product type $\Pi_{x:A} B(x)$ is the type of *dependent functions*: functions $f$ such that, given any $a : A$, $f(a) : B(a)$. The output type $B(a)$ depends on the input value $a$.

When $B$ does not depend on $x$ (is constant), $\Pi_{x:A} B = A \to B$ — the ordinary function type. Dependent functions generalize ordinary functions.

**Formation rule**:
$$\frac{A : \mathsf{Type} \quad x : A \vdash B(x) : \mathsf{Type}}{\Pi_{x:A} B(x) : \mathsf{Type}} \quad (\Pi\text{-form})$$

To form $\Pi_{x:A} B(x)$, $A$ must be a type, and $B(x)$ must be a type for each $x : A$.

**Introduction rule**:
$$\frac{x : A \vdash t : B(x)}{\lambda x.\, t : \Pi_{x:A} B(x)} \quad (\Pi\text{-intro})$$

**Elimination rule**:
$$\frac{f : \Pi_{x:A} B(x) \quad a : A}{f\, a : B(a)} \quad (\Pi\text{-elim})$$

**Computation rule**:
$$(\lambda x.\, t)\, a \to_\beta t[a/x] : B(a)$$

Under Curry-Howard, $\Pi_{x:A} B(x)$ corresponds to $\forall x : A, B(x)$. A proof of $\forall x : A, B(x)$ is a function that, given any $a : A$, produces a proof of $B(a)$.

## The $\Sigma$-Type: Dependent Pairs

The dependent sum type $\Sigma_{x:A} B(x)$ is the type of *dependent pairs*: pairs $(a, b)$ where $a : A$ and $b : B(a)$. The second component's type depends on the first component's value.

When $B$ is constant, $\Sigma_{x:A} B = A \times B$ — the ordinary product type. Dependent pairs generalize ordinary pairs.

**Formation rule**:
$$\frac{A : \mathsf{Type} \quad x : A \vdash B(x) : \mathsf{Type}}{\Sigma_{x:A} B(x) : \mathsf{Type}} \quad (\Sigma\text{-form})$$

**Introduction rule**:
$$\frac{a : A \quad b : B(a)}{(a, b) : \Sigma_{x:A} B(x)} \quad (\Sigma\text{-intro})$$

**Elimination rule** (dependent case analysis):
$$\frac{p : \Sigma_{x:A} B(x) \quad x : A, y : B(x) \vdash C(x, y) : \mathsf{Type} \quad x : A, y : B(x) \vdash f(x, y) : C(x, y)}{\text{rec}(p, f) : C(\pi_1\, p, \pi_2\, p)} \quad (\Sigma\text{-elim})$$

For the simple projections: $\pi_1 : \Sigma_{x:A} B(x) \to A$ and $\pi_2 : (p : \Sigma_{x:A} B(x)) \to B(\pi_1\, p)$.

Under Curry-Howard, $\Sigma_{x:A} B(x)$ corresponds to $\exists x : A, B(x)$. A proof of $\exists x : A, B(x)$ is a pair $(a, b)$ where $a : A$ is the witness and $b : B(a)$ is the proof that $a$ satisfies the predicate.

## The Identity Type: The Ultimate Extension

The most important dependent type — the one that distinguishes HoTT from all its predecessors — is the **identity type** $a =_A b$.

For any type $A$ and elements $a, b : A$, the identity type $a =_A b$ is the type of *proofs that $a$ equals $b$*. Under Curry-Howard, it corresponds to the proposition $a = b$.

**Formation**:
$$\frac{A : \mathsf{Type} \quad a : A \quad b : A}{a =_A b : \mathsf{Type}} \quad (\text{Id-form})$$

**Introduction** (reflexivity):
$$\frac{a : A}{\mathsf{refl}_a : a =_A a} \quad (\text{Id-intro})$$

The only canonical proof of equality is reflexivity: $a$ equals $a$ via $\mathsf{refl}_a$.

**Elimination** (path induction):
$$\frac{C : \Pi_{a:A} \Pi_{b:A} \Pi_{p : a=_A b} \mathsf{Type} \quad c : \Pi_{a:A} C(a, a, \mathsf{refl}_a) \quad p : a =_A b}{J(C, c, p) : C(a, b, p)} \quad (\text{Id-elim})$$

Path induction (the $J$ eliminator) says: to prove something about every proof $p$ of $a = b$ (for varying $a$, $b$), it suffices to prove it for the case $a = b$ and $p = \mathsf{refl}_a$.

**The key point**: unlike all previous type constructors, the identity type *is not a proposition in the classical sense*. In ordinary type theory (MLTT without HoTT), the uniqueness of identity proofs (UIP) axiom says any two proofs of $a = b$ are equal. HoTT *rejects* UIP: the type $a =_A b$ can have multiple distinct inhabitants, and these different proofs of equality are genuinely different mathematical objects — different paths between $a$ and $b$ in the type $A$, viewed as a topological space.

## The Lambda Cube and Beyond

The extension from STLC to dependent types is part of a systematic classification called the *lambda cube* (Barendregt 1991): eight type systems obtained by allowing or forbidding three kinds of dependency:

1. **Terms depending on terms** (functions): $\lambda\to$ (STLC).
2. **Types depending on types** (polymorphism): $\lambda 2$ (System F) or $\lambda\omega$ (System F$\omega$).
3. **Types depending on terms** (dependency): $\lambda P$ (LF, the logical framework).

The combination of all three is $\lambda C$ — the Calculus of Constructions (CoC), which is the type theory underlying Coq. It allows functions from types to types, functions from terms to types (dependent functions), functions from types to terms (type abstraction / polymorphism), and functions from terms to terms (ordinary computation).

Martin-Löf Type Theory extends CoC with:
- **Inductive types**: $\mathbb{N}$, lists, trees, etc., defined by constructors and recursion principles.
- **Universe types**: a hierarchy $\mathsf{Type}_0 : \mathsf{Type}_1 : \mathsf{Type}_2 : \ldots$ to avoid the paradox of a "type of all types."
- **The identity type**: with its path induction eliminator.

HoTT further adds:
- **Univalence**: an axiom asserting that equivalent types are equal ($A \simeq B \to A = B$).
- **Higher inductive types**: types defined by their points, paths, and higher paths.

The progression from STLC through the lambda cube to MLTT and HoTT is a progression in expressive power and mathematical richness. Each extension maintains the Curry-Howard correspondence — adding logical power on one side and computational power on the other. The endpoint — HoTT — is a type theory in which all of mathematics can be expressed and verified, and in which the structure of proofs (paths) is itself a subject of mathematical study.
