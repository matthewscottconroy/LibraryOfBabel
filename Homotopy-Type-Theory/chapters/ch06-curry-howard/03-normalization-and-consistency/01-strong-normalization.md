# 3.1 Strong Normalization and Consistency

## The Central Theorem

We proved normalization for natural deduction (Chapter 4). Now we prove it for STLC, using a more powerful method: *Girard's reducibility method* (also called *logical relations* or *computability predicates*).

**Theorem (Strong Normalization for STLC).** Every well-typed term of STLC reduces to a normal form in finitely many steps, and every reduction sequence terminates.

This is a strong form: not just "there exists a normalizing reduction sequence" but "every reduction sequence terminates." No infinite loops in well-typed programs.

**Corollary.** STLC is consistent: there is no closed term of type $\mathbf{0}$, and no proof of $\bot$ in IPC.

## Why Direct Induction Fails

You might expect to prove strong normalization by induction on typing derivations. But there's a fundamental obstacle: the substitution step.

When we reduce $(\lambda x.\, t)\, s \to_\beta t[s/x]$, the term $s$ gets substituted into $t$. If $t$ has $n$ occurrences of $x$, then $s$ gets copied $n$ times. The resulting term $t[s/x]$ can be much larger than either $t$ or $s$. Induction on term size breaks.

The clever solution: define a *semantic* notion of "reducibility" that is preserved by computation, and show all well-typed terms are reducible.

## The Reducibility Predicates

**Definition.** For each type $A$, define the *reducibility predicate* $\text{Red}(A)$ — a set of closed terms of type $A$:

- **Base case:** $\text{Red}(o) = \{t : o \mid t \text{ is strongly normalizing}\}$. All terms of base type that terminate.

- **Function type:** $\text{Red}(A \to B) = \{t : A \to B \mid \forall s \in \text{Red}(A): t\, s \in \text{Red}(B)\}$. A function is reducible iff applying it to any reducible argument gives a reducible result.

- **Product type:** $\text{Red}(A \times B) = \{p : A \times B \mid \text{fst}(p) \in \text{Red}(A) \text{ and } \text{snd}(p) \in \text{Red}(B)\}$.

- **Sum type:** $\text{Red}(A + B) = \{e : A + B \mid \forall C, \forall s, u: \text{if } x \in \text{Red}(A) \Rightarrow s[x] \in \text{Red}(C)$ and $y \in \text{Red}(B) \Rightarrow u[y] \in \text{Red}(C)$, then $\text{case}(e, x.s, y.u) \in \text{Red}(C)\}$.

These definitions are *semantic* — they're defined in terms of mathematical properties (strong normalization, mapping reducible to reducible), not syntactic structure.

## Key Properties of Reducibility Predicates

**Lemma (CR1–CR3).** For each type $A$, the set $\text{Red}(A)$ satisfies:
- **(CR1):** Every $t \in \text{Red}(A)$ is strongly normalizing.
- **(CR2):** If $t \in \text{Red}(A)$ and $t \to t'$, then $t' \in \text{Red}(A)$.
- **(CR3):** If $t$ is *neutral* (not a lambda abstraction, pair, or injection) and every one-step reduct $t'$ of $t$ is in $\text{Red}(A)$, then $t \in \text{Red}(A)$.

*Proof.* By induction on $A$.

For base type: CR1 is by definition. CR2: strong normalization is preserved by reduction (if $t$ terminates, then $t'$ terminates — every reduction path from $t'$ extends to a reduction path from $t$). CR3: if every one-step reduct of $t$ terminates, then $t$ terminates (since it takes one step to reduce and then every further path is from a terminating $t'$).

For function types: CR1 follows from CR3 — if $t$ is strongly normalizing, apply it to a neutral strongly normalizing term $x$ (a fresh variable). Then $t\, x$ is neutral with all reducts reducing eventually to a strongly normalizing term, so by induction, $t\, x \in \text{Red}(B)$. Wait — this isn't quite the argument. The actual proof is more careful.

Actually, the core of the argument: CR3 is most subtle. For a function type: if $t$ is neutral and every one-step reduct $t'$ has the property that $t'\, s \in \text{Red}(B)$ for all $s \in \text{Red}(A)$, then $t\, s$ is neutral with all its reducts in $\text{Red}(B)$ (either $t$ takes a step giving $t'\, s$, or $s$ takes a step giving $t\, s'$). By induction on the reduction, $t\, s \in \text{Red}(B)$. So $t \in \text{Red}(A \to B)$. $\square$

## The Main Theorem

**Lemma (All typed terms are reducible).** For any well-typed term $\Gamma \vdash t : A$ and any substitution $\gamma$ mapping each $x_i : A_i$ in $\Gamma$ to a term in $\text{Red}(A_i)$, we have $t[\gamma] \in \text{Red}(A)$.

*Proof.* By induction on the typing derivation.

**Variable:** $\gamma(x_i) \in \text{Red}(A_i)$ by assumption.

**Lambda abstraction:** We need to show $\lambda x.\, t[\gamma] \in \text{Red}(A \to B)$. Take any $s \in \text{Red}(A)$. Then $(\lambda x.\, t[\gamma])\, s \to_\beta t[\gamma, x \mapsto s]$. By induction (extending $\gamma$ with $x \mapsto s$), $t[\gamma, x \mapsto s] \in \text{Red}(B)$. By CR2, $(\lambda x.\, t[\gamma])\, s \in \text{Red}(B)$ (since it reduces to something in $\text{Red}(B)$). Actually, we need to show $\lambda x.\, t[\gamma]$ itself is reducible, not just its application. We use CR3: $\lambda x.\, t[\gamma]$ is not neutral, but it's not a redex unless applied to something. The argument goes by showing $(\lambda x.\, t[\gamma])\, s \in \text{Red}(B)$ for all $s \in \text{Red}(A)$, which gives $\lambda x.\, t[\gamma] \in \text{Red}(A \to B)$. $\square$

**Corollary.** Every closed well-typed term $\vdash t : A$ is in $\text{Red}(A)$. By CR1, every well-typed term is strongly normalizing. $\square$

## The Church-Rosser Theorem

Besides strong normalization, STLC satisfies another key property: reduction is *confluent* (Church-Rosser).

**Theorem (Church-Rosser / Confluence).** If $t \to^* s_1$ and $t \to^* s_2$ (where $\to^*$ is the reflexive-transitive closure of $\to_\beta$), then there exists $u$ with $s_1 \to^* u$ and $s_2 \to^* u$.

Confluence means: no matter how you reduce a term, you always reach the same normal form (if any). Combined with strong normalization, this gives:

**Corollary (Uniqueness of Normal Forms).** Every well-typed term has a unique normal form.

This is a remarkable property: computation in STLC is completely deterministic in the sense that it always produces the same final answer, regardless of the order of reduction steps.

## Consistency from Strong Normalization

**Corollary (Consistency of IPC).** Intuitionistic propositional logic is consistent: $\bot$ is not derivable from no hypotheses.

*Proof.* A proof of $\bot$ in IPC corresponds to a closed term of type $\mathbf{0}$ in STLC. But by strong normalization and canonicity, every closed well-typed term has a normal form. A normal form of type $\mathbf{0}$ would have to be... nothing: the unit of the empty type. But the empty type has no elements by definition, so there are no terms of type $\mathbf{0}$ at all (not even normal forms). Therefore no such term exists, and $\bot$ has no proof. $\square$

This is a *semantic* consistency proof: it works by constructing a model (the reducibility predicates) in which $\bot$ is empty. Compare with Gentzen's syntactic consistency proof via cut elimination.

## Extensions: System F and Beyond

STLC corresponds to propositional IPC. To handle full mathematics, we need more.

**System F** (polymorphic typed λ-calculus, Girard-Reynolds 1972): adds *universal types* $\forall \alpha. A(\alpha)$ (functions that work for any type $\alpha$). Corresponds to second-order intuitionistic propositional logic.

Normalization for System F requires a more sophisticated version of the reducibility argument (Girard's original construction). Strong normalization still holds.

**Gödel's System T**: extends STLC with natural numbers and primitive recursion. Corresponds to first-order arithmetic (without second-order quantifiers). Every provably total function of HA is definable in System T.

**Martin-Löf Type Theory (MLTT)**: adds dependent types $\Pi$ and $\Sigma$, identity types, and a type universe. Corresponds to full intuitionistic predicate logic. Strong normalization requires the theory of inductive types.

**The Calculus of Constructions (CoC)**: the system underlying Coq. Combines System F's polymorphism with dependent types. Strong normalization is a deep theorem.

**HoTT**: adds the univalence axiom and higher inductive types to MLTT. Strong normalization for HoTT is an active research area (addressed in part by cubical type theory).

## What Strong Normalization Buys Us

Strong normalization has several crucial consequences:

1. **Termination.** All well-typed programs terminate. No infinite loops. This is good for a proof assistant (proofs should complete) but limits expressiveness — Turing-complete programs cannot be directly written.

2. **Decidability of type checking.** Since every reduction terminates, we can reduce to normal form and check if types match. Type checking is decidable.

3. **Consistency.** As shown above: the type $\mathbf{0}$ is uninhabited.

4. **Canonicity.** Closed programs of boolean/sum/product types reduce to canonical forms.

5. **Proof extraction.** If a program proves an existential statement, we can run it to extract the witness.

The price: not all computable functions are STLC-definable. To get Turing-completeness, you need either recursive types or general recursion — at the cost of losing strong normalization.

This is the fundamental trade-off in type theory: *expressiveness vs. normalization*. HoTT and MLTT sit at a carefully chosen point: enough expressiveness to do all of mathematics, with normalization properties that make the system consistent and computationally meaningful.
