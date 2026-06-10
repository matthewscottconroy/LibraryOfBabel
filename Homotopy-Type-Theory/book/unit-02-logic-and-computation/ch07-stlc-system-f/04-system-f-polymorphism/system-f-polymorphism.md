# System F: Polymorphism and Parametricity

## The Limitation of STLC

In STLC, every function has a fixed type. The identity function at type $\mathbb{N}$ is $\lambda x : \mathbb{N}.\, x : \mathbb{N} \to \mathbb{N}$; the identity function at type $\mathbb{B}$ is $\lambda x : \mathbb{B}.\, x : \mathbb{B} \to \mathbb{B}$. These are two different terms with two different types. There is no single "identity function" that works for all types.

This is mathematically unsatisfying. The identity function is one concept, not infinitely many. Function composition is one concept. The swap function on pairs is one concept. But in STLC, each of these must be instantiated separately at each type.

System F (polymorphic lambda calculus), developed by Girard (1971) and independently by Reynolds (1974), solves this by adding *universal quantification over types*: a function can take a type as an argument and return a value whose type depends on the type argument.

## System F: Types and Terms

**Types** in System F:
$$A, B ::= \alpha \mid A \to B \mid \forall \alpha.\, A$$

Where $\alpha$ is a *type variable* and $\forall \alpha.\, A$ binds $\alpha$ in $A$.

**New terms**:
- **Type abstraction**: $\Lambda \alpha.\, t$ — a term parameterized by a type variable $\alpha$.
- **Type application**: $t\, [B]$ — instantiate a polymorphic term at type $B$.

**New typing rules**:

**Type abstraction** (Universal introduction):
$$\frac{\Gamma, \alpha : \mathsf{Type} \vdash t : A \quad \alpha \notin \text{FTV}(\Gamma)}{\Gamma \vdash \Lambda \alpha.\, t : \forall \alpha.\, A} \quad (\forall\text{-intro})$$

The side condition $\alpha \notin \text{FTV}(\Gamma)$ ensures $\alpha$ is a fresh type variable — the term works for all types, not just specific ones.

**Type application** (Universal elimination):
$$\frac{\Gamma \vdash t : \forall \alpha.\, A}{\Gamma \vdash t\, [B] : A[\alpha := B]} \quad (\forall\text{-elim})$$

Instantiate the polymorphic term at a specific type $B$.

**Type-level beta reduction**:
$$(\Lambda \alpha.\, t)\, [B] \to_\beta t[\alpha := B]$$

## The Polymorphic Identity

The definitive example:

$$\mathsf{id} = \Lambda \alpha.\, \lambda x : \alpha.\, x : \forall \alpha.\, \alpha \to \alpha$$

This is a single term, not many. Applied to a type:

$$\mathsf{id}\, [\mathbb{N}] = (\Lambda \alpha.\, \lambda x : \alpha.\, x)\, [\mathbb{N}] \to_\beta \lambda x : \mathbb{N}.\, x : \mathbb{N} \to \mathbb{N}$$

$$\mathsf{id}\, [\mathbb{N}]\, 5 \to_\beta 5$$

The polymorphic identity takes a type $\alpha$ and returns the identity function for that type. It is one function, not many.

**Logical reading**: $\forall \alpha.\, \alpha \to \alpha$ is the proposition "for all propositions $\alpha$, $\alpha$ implies $\alpha$." This is the second-order tautology $\forall P.\, P \to P$. The polymorphic identity is the proof — a function that takes any proposition $P$ and any proof of $P$, and returns the proof.

## Polymorphic Composition and Other Functions

**Composition**:
$$\mathsf{comp} = \Lambda \alpha.\, \Lambda \beta.\, \Lambda \gamma.\, \lambda f : \alpha \to \beta.\, \lambda g : \beta \to \gamma.\, \lambda x : \alpha.\, g\, (f\, x)$$
$$: \forall \alpha.\, \forall \beta.\, \forall \gamma.\, (\alpha \to \beta) \to (\beta \to \gamma) \to \alpha \to \gamma$$

**Swap**:
$$\mathsf{swap} = \Lambda \alpha.\, \Lambda \beta.\, \lambda p : \alpha \times \beta.\, (\pi_2\, p, \pi_1\, p)$$
$$: \forall \alpha.\, \forall \beta.\, \alpha \times \beta \to \beta \times \alpha$$

**Flip** (argument order reversal):
$$\mathsf{flip} = \Lambda \alpha.\, \Lambda \beta.\, \Lambda \gamma.\, \lambda f : \alpha \to \beta \to \gamma.\, \lambda y : \beta.\, \lambda x : \alpha.\, f\, x\, y$$
$$: \forall \alpha.\, \forall \beta.\, \forall \gamma.\, (\alpha \to \beta \to \gamma) \to \beta \to \alpha \to \gamma$$

## The Curry-Howard Correspondence for System F

System F corresponds to *second-order intuitionistic propositional logic* (IPC$_2$). The key correspondence:

| System F | IPC$_2$ |
|---|---|
| $\forall \alpha.\, A$ | $\forall P.\, A$ |
| Type abstraction $\Lambda \alpha.\, t$ | $\forall$-introduction |
| Type application $t\, [B]$ | $\forall$-elimination (instantiation) |

Second-order propositional logic allows quantification over propositions. The proposition $\forall P.\, P \to P$ is provable (by the polymorphic identity). The proposition $\forall P.\, \forall Q.\, (P \to Q) \to P \to Q$ is provable (by polymorphic modus ponens). Church encodings give proofs of induction-like principles.

**Impredicativity**: System F is *impredicative* — the $\forall \alpha$ ranges over all types, including $\forall \alpha.\, A$ itself. This means:

$$\mathsf{Bool} = \forall \alpha.\, \alpha \to \alpha \to \alpha$$

and $\mathsf{Bool}$ is itself one of the types that $\alpha$ ranges over when forming $\mathsf{Bool}$. This circularity is what gives System F (and the Calculus of Constructions) their expressive power — and it is also why the naïve interpretation of System F in naive set theory is paradoxical. Consistent models of System F (like Girard's candidates of reducibility, or the relational model) must handle this circularity carefully.

## Reynolds' Parametricity Theorem

The most profound result about System F is Reynolds' *parametricity theorem* (1983). It formalizes the intuition that polymorphic functions must be "uniform" across types.

**Informal statement**: a term $t : \forall \alpha.\, A(\alpha)$ must behave in a way that is "natural" with respect to any relation between types. If two types $B$ and $C$ are related by a relation $R : B \to C \to \mathsf{Prop}$, and the inputs to $t$ at type $B$ and type $C$ are $R$-related, then the outputs are also $R$-related.

**Formal statement (Parametricity)**: for every term $t : \forall \alpha.\, A(\alpha)$ and every types $B$, $C$ and relation $R \subseteq B \times C$, we have $(t\, [B], t\, [C]) \in \mathcal{R}(A, R)$, where $\mathcal{R}(A, R)$ is the *logical relation* induced by $R$ at type $A$.

The logical relation is defined inductively on types:
- $\mathcal{R}(\alpha, R) = R$ (the given relation at type variables)
- $\mathcal{R}(A \to B, R) = \{(f, g) \mid \forall (a, a') \in \mathcal{R}(A, R).\, (f\, a, g\, a') \in \mathcal{R}(B, R)\}$
- $\mathcal{R}(\forall \alpha.\, A, R) = \{(t, t') \mid \forall (B, C, S).\, (t\, [B], t'\, [C]) \in \mathcal{R}(A, S)\}$

## Free Theorems

Parametricity gives *free theorems* — theorems about polymorphic functions that follow from their types alone, without examining their implementations.

**Free theorem for $\forall \alpha.\, \alpha \to \alpha$**: If $t : \forall \alpha.\, \alpha \to \alpha$, then for all types $B$ and all $b : B$, $t\, [B]\, b = b$.

*Proof by parametricity.* Take the relation $R = \{(b, b)\}$ (equality on $B$). Then $(b, b) \in \mathcal{R}(\alpha, R)$ and $(t, t) \in \mathcal{R}(\forall \alpha.\, \alpha \to \alpha, R)$. By definition, $(t\, [B]\, b, t\, [B]\, b) \in \mathcal{R}(\alpha, R) = R = \{(b, b)\}$, so $t\, [B]\, b = b$. $\square$

So $t$ must be the identity function.

**Free theorem for $\forall \alpha.\, \alpha \to \alpha \to \alpha$**: any term of this type is either "always return the first argument" ($\lambda x.\, \lambda y.\, x$) or "always return the second argument" ($\lambda x.\, \lambda y.\, y$). There are exactly two terms (up to beta-eta equivalence).

**Free theorem for $\forall \alpha.\, \mathsf{List}\, \alpha \to \mathsf{List}\, \alpha$**: any term of this type must be a function that rearranges or drops elements. It cannot inspect element values (since it doesn't know what $\alpha$ is) or create new elements (it has no source of $\alpha$ values). So it must be a function on positions: a permutation or deletion pattern.

More precisely: if $f : \forall \alpha.\, \mathsf{List}\, \alpha \to \mathsf{List}\, \alpha$ and $g : A \to B$ is any function, then $\mathsf{map}\, g\, (f\, [A]\, \ell) = f\, [B]\, (\mathsf{map}\, g\, \ell)$. The action of $f$ commutes with any map, meaning $f$ only rearranges, not inspects.

## Strong Normalization for System F

**Theorem (Girard, 1971).** System F is strongly normalizing: every reduction sequence from a well-typed System F term terminates.

The proof is significantly harder than for STLC. The difficulty: defining the reducibility predicate for $\forall \alpha.\, A$ requires quantifying over all types $\alpha$ might stand for — but $A$ can contain $\alpha$, creating a potential circularity.

Girard's solution: define reducibility not by reference to specific types but by reference to *candidates of reducibility* — sets of terms satisfying CR1–CR3. The reducibility predicate for $\forall \alpha.\, A$ is:

$$\text{Red}(\forall \alpha.\, A) = \{t \mid \forall \mathcal{C} \in \text{Cand},\, \forall B,\, t\, [B] \in \text{Red}(A[\alpha \mapsto \mathcal{C}])\}$$

where $\text{Red}(A[\alpha \mapsto \mathcal{C}])$ is the reducibility predicate for $A$ with $\alpha$ interpreted as the candidate $\mathcal{C}$.

The candidates form a family of sets closed under the CR properties. The definition of $\text{Red}(\forall \alpha.\, A)$ requires all type instantiations to be reducible — including instantiation with other candidates, not just specific types. This handles the impredicativity.

**Corollary (Consistency of IPC$_2$).** Second-order intuitionistic propositional logic is consistent.

Strong normalization for System F implies there is no closed term of type $\mathbf{0}$ in System F, so $\bot$ is not provable in IPC$_2$.

## System F and the Strength of Mathematics

System F is more powerful than STLC, and this additional power has a proof-theoretic reading: System F corresponds to second-order arithmetic (the logic of natural numbers with quantification over sets of numbers).

Specifically: Girard's theorem states that a function $f : \mathbb{N} \to \mathbb{N}$ is definable in System F if and only if it is provably total in second-order arithmetic ($\Sigma_1^1$-AC, or equivalently, if and only if its totality can be proved using comprehension axioms for arithmetic predicates).

This places System F in a precise position in the hierarchy of mathematical theories: more powerful than first-order Peano Arithmetic (which corresponds to Gödel's System T), but not as powerful as full ZFC (which requires dependent types and higher-order logic).

The expressiveness of System F means it can define many functions that STLC cannot — including the Ackermann function, the normalization function for STLC terms, and the characteristic function of every primitive recursive set. But there are still functions that are computable but not System-F-definable: non-elementary functions whose totality requires higher-order comprehension.
