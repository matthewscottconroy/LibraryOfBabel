# Normalization and Consistency

## The Stakes

The Curry-Howard correspondence, if taken seriously, has a striking implication: the consistency of intuitionistic propositional logic is equivalent to the strong normalization of the simply typed lambda calculus.

This needs unpacking. Consistency means: there is no proof of $\bot$. Under Curry-Howard, this means: there is no closed term of the empty type $\mathbf{0}$ (the type with no elements). And this is exactly strong normalization: if every well-typed closed term reduces to a value, and values of $\mathbf{0}$ cannot exist (since $\mathbf{0}$ has no values), then there is no term of type $\mathbf{0}$.

More precisely: if STLC is strongly normalizing, then every closed term of type $\mathbf{0}$ would reduce to a normal form, which would have to be a value of $\mathbf{0}$. But $\mathbf{0}$ has no values. So no closed term of type $\mathbf{0}$ exists. So the corresponding logic is consistent.

The normalization theorem for STLC is therefore not just a technical result about computation — it is a proof of the consistency of intuitionistic propositional logic.

## Strong Normalization: The Statement

**Theorem (Strong Normalization for STLC).** Every well-typed term of the simply typed lambda calculus is strongly normalizing: every reduction sequence from a well-typed term terminates in a normal form.

This says: programs terminate. Every well-typed STLC program, no matter how you choose to reduce it, eventually produces a value. There is no infinite computation. The computation tree is finite.

This is remarkable because untyped lambda calculus is not strongly normalizing (the $\Omega = (\lambda x.\, x\, x)(\lambda x.\, x\, x)$ term diverges). Types are what guarantee termination.

## The Proof: Logical Relations (Tait's Method)

The proof uses *logical relations* — a semantic method that defines a property of terms by induction on types, not on term structure.

**Definition of the reducibility predicate $\text{Red}(A)$:**

- $\text{Red}(\alpha)$ (for base types $\alpha$): the set of all strongly normalizing terms of type $\alpha$.
- $\text{Red}(A \to B)$: the set of terms $f$ of type $A \to B$ such that for every $a \in \text{Red}(A)$, the application $f\, a \in \text{Red}(B)$.
- $\text{Red}(A \times B)$: the set of terms $p$ of type $A \times B$ such that $\pi_1\, p \in \text{Red}(A)$ and $\pi_2\, p \in \text{Red}(B)$.

The definition is "logical" because it follows the structure of types. It says: a term of function type is reducible if it sends reducible arguments to reducible results.

**Key Properties (CR1–CR3) of reducibility:**

**CR1.** Every term in $\text{Red}(A)$ is strongly normalizing.
**CR2.** If $t \in \text{Red}(A)$ and $t \to t'$, then $t' \in \text{Red}(A)$ (reducibility is closed under reduction).
**CR3.** If $t$ is in normal form at type $A$, and for every one-step reduct $t'$ of $t$ we have $t' \in \text{Red}(A)$, then $t \in \text{Red}(A)$ (reducibility is closed under reverse expansion from normal forms).

**Lemma.** For every type $A$, the set $\text{Red}(A)$ satisfies CR1–CR3.

*Proof.* By induction on $A$.

For base types $\alpha$: CR1 holds by definition (we defined $\text{Red}(\alpha)$ as the strongly normalizing terms). CR2 holds because if $t$ is strongly normalizing and $t \to t'$, then $t'$ is also strongly normalizing (any reduction sequence from $t'$ can be prepended with the step $t \to t'$ to get a reduction sequence from $t$, which terminates, so the sequence from $t'$ terminates too). CR3 holds similarly.

For function types $A \to B$: CR1 holds by considering the reduction sequence of $t$ itself: any reduction of $t$ gives a term $t'$, and by CR2 for $A \to B$, $t' \in \text{Red}(A \to B)$. To show $t$ is strongly normalizing: pick a fixed neutral variable $x \in \text{Red}(A)$ (there always is one), and consider $t\, x \in \text{Red}(B)$ (by definition of $\text{Red}(A \to B)$). Any reduction sequence from $t$ induces a reduction sequence from $t\, x$ (apply $t$ to $x$ at each step), so if $t$ had an infinite reduction sequence, $t\, x$ would too. But $\text{Red}(B)$ satisfies CR1, so $t\, x$ is SN, so $t$ is SN. $\square$ (Details require more care.)

**Main Lemma.** If $\Gamma \vdash t : A$ and for each $x_i : A_i \in \Gamma$, there is $s_i \in \text{Red}(A_i)$, then $t[s_i/x_i] \in \text{Red}(A)$.

*Proof.* By induction on the typing derivation of $\Gamma \vdash t : A$.

- **Var**: $t = x_j$, so $t[s_i/x_i] = s_j \in \text{Red}(A_j) = \text{Red}(A)$. ✓
- **Abs**: $t = \lambda x : A_1.\, u$ and $A = A_1 \to A_2$. By induction, for any $a \in \text{Red}(A_1)$, we have $u[a/x] \in \text{Red}(A_2)$. Now $(\lambda x.\, u)\, a \to_\beta u[a/x] \in \text{Red}(A_2)$. Using CR3 for $A_2$, $(\lambda x.\, u)\, a \in \text{Red}(A_2)$. So $\lambda x.\, u \in \text{Red}(A_1 \to A_2) = \text{Red}(A)$. ✓
- **App**: $t = f\, s$ with $\Gamma \vdash f : A_1 \to A$ and $\Gamma \vdash s : A_1$. By induction, $f[s_i/x_i] \in \text{Red}(A_1 \to A)$ and $s[s_i/x_i] \in \text{Red}(A_1)$. So $f[s_i/x_i]\, s[s_i/x_i] = (f\, s)[s_i/x_i] \in \text{Red}(A)$ by definition of $\text{Red}(A_1 \to A)$. ✓

**Corollary (Strong Normalization).** Every well-typed term is strongly normalizing.

*Proof.* Variables are reducible (they are in normal form, hence SN, hence in $\text{Red}$ by CR3). Apply the Main Lemma to get the term in $\text{Red}$ at its type. By CR1, it is SN. $\square$

## Consistency as a Corollary

**Corollary.** Intuitionistic propositional logic is consistent: $\not\vdash_\text{IPC} \bot$.

*Proof.* Suppose $\vdash_\text{IPC} \bot$. By Curry-Howard, there is a closed term $t : \mathbf{0}$. By strong normalization, $t$ reduces to a normal form. A normal form of type $\mathbf{0}$ (the empty type) must be a canonical value of type $\mathbf{0}$. But $\mathbf{0}$ has no canonical values — that is the definition of the empty type. Contradiction. $\square$

This is a proof of logical consistency via a computational argument. It uses no appeal to models or semantics. It is a purely syntactic argument: well-typed terms compute, computation preserves types, and the empty type has no values.

## Church-Rosser Property

A related property: **Church-Rosser (Confluence)**. If $t \to_\beta^* s_1$ and $t \to_\beta^* s_2$, then there exists $u$ with $s_1 \to_\beta^* u$ and $s_2 \to_\beta^* u$.

Confluence says: any two reduction sequences from the same term eventually reach a common term. This implies uniqueness of normal forms: if $t$ reduces to two normal forms $s_1$ and $s_2$, they must be the same (since the common term $u$ must be the normal form itself, and normal forms are unique).

Church-Rosser follows from the *diamond property* for one-step reduction (which holds for STLC) and the general theorem that any relation with the diamond property is confluent.

## Normalization Proofs at Higher Levels

The logical relations method is the standard tool for normalization proofs in type theory, but it becomes more complex as the type theory grows:

**System F** (second-order polymorphism, Section 4 of Chapter 7): Girard's 1971 proof uses *candidates of reducibility* — sets of terms satisfying CR1–CR3 — rather than directly defining $\text{Red}(A)$. For $\forall \alpha. A$, reducibility is defined as the intersection over all candidates:
$$\text{Red}(\forall \alpha. A) = \{t \mid \forall \mathcal{C} \in \text{Cand}.\, \forall B: t\, [B] \in \text{Red}(A[\alpha := B]) \text{ where Red at $\alpha$ is $\mathcal{C}$}\}$$

**Martin-Löf Type Theory**: normalization for MLTT is harder still, requiring induction-recursion to define the reducibility predicate. Martin-Löf's 1975 proof introduced the notion of an "indexed" logical relation that handles the dependency of types on terms.

**HoTT**: normalization for HoTT is the subject of ongoing research. Cubical type theory provides a computational interpretation of the univalence axiom; whether the full HoTT (with all higher inductive types) is strongly normalizing remains an open question in some formulations.

## Normalization and Proof Irrelevance

Strong normalization has one more consequence that connects to HoTT.

In STLC, two terms of the same type that reduce to the same normal form are *definitionally equal*. The normal form is the "canonical representative" of the proof.

But in HoTT, definitional equality is just the first layer. There is also *propositional equality* — paths $p : a =_A b$. Two terms that are definitionally equal are propositionally equal (by reflexivity). But two terms that are propositionally equal need not be definitionally equal.

In particular: two proofs of the same proposition $A$ need not be definitionally equal (even if they both reduce to normal forms). They are propositionally equal if and only if there is a path between them in the type $A$. For h-propositions (mere propositions), all proofs are propositionally equal. For higher types, they may not be.

Strong normalization gives us one layer — definitional equality — via normalization. HoTT adds the full tower of propositional equalities, which is not determined by normalization alone. This is why HoTT requires additional axioms (like univalence) and additional structure (like the higher inductive types) beyond what STLC provides.

The normalization theorem of this section is the bedrock. Everything in HoTT is built on it, augmenting its clean termination guarantee with the richer structure of homotopy theory.
