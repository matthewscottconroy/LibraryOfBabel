# Kripke's Fixed-Point Construction

Saul Kripke's *Outline of a Theory of Truth* (1975) showed that a language **can** contain its own truth predicate — provided that predicate is allowed to be *partial*. Some sentences, the Liar among them, simply receive no truth value: they fall into a **truth-value gap**. What makes the proposal more than hand-waving is that Kripke gives an exact mathematical construction of the interpretation of $T$, as the least fixed point of a monotone operator.

## Strong Kleene Semantics

Work with three values $\{\mathbf{t}, \mathbf{u}, \mathbf{f}\}$, where $\mathbf{u}$ ("undefined") is not a third truth value but the *absence* of one. The **Strong Kleene scheme** $K_3$ evaluates compounds classically whenever enough components have values:

| $\phi$ | $\neg\phi$ |
|:---:|:---:|
| $\mathbf{t}$ | $\mathbf{f}$ |
| $\mathbf{u}$ | $\mathbf{u}$ |
| $\mathbf{f}$ | $\mathbf{t}$ |

| $\wedge$ | $\mathbf{t}$ | $\mathbf{u}$ | $\mathbf{f}$ |
|:---:|:---:|:---:|:---:|
| $\mathbf{t}$ | $\mathbf{t}$ | $\mathbf{u}$ | $\mathbf{f}$ |
| $\mathbf{u}$ | $\mathbf{u}$ | $\mathbf{u}$ | $\mathbf{f}$ |
| $\mathbf{f}$ | $\mathbf{f}$ | $\mathbf{f}$ | $\mathbf{f}$ |

| $\vee$ | $\mathbf{t}$ | $\mathbf{u}$ | $\mathbf{f}$ |
|:---:|:---:|:---:|:---:|
| $\mathbf{t}$ | $\mathbf{t}$ | $\mathbf{t}$ | $\mathbf{t}$ |
| $\mathbf{u}$ | $\mathbf{t}$ | $\mathbf{u}$ | $\mathbf{u}$ |
| $\mathbf{f}$ | $\mathbf{t}$ | $\mathbf{u}$ | $\mathbf{f}$ |

| $\to$ ($:= \neg\phi \vee \psi$) | $\mathbf{t}$ | $\mathbf{u}$ | $\mathbf{f}$ |
|:---:|:---:|:---:|:---:|
| $\mathbf{t}$ | $\mathbf{t}$ | $\mathbf{u}$ | $\mathbf{f}$ |
| $\mathbf{u}$ | $\mathbf{t}$ | $\mathbf{u}$ | $\mathbf{u}$ |
| $\mathbf{f}$ | $\mathbf{t}$ | $\mathbf{t}$ | $\mathbf{t}$ |

Quantifiers behave as infinitary conjunction/disjunction: $\forall x\,\phi$ is $\mathbf{t}$ if $\phi(a)$ is $\mathbf{t}$ for every $a$ in the domain, $\mathbf{f}$ if $\phi(a)$ is $\mathbf{f}$ for some $a$, and $\mathbf{u}$ otherwise; dually for $\exists$. Note the design principle: a compound gets a classical value as soon as the valued components *force* it ($\mathbf{f} \wedge \mathbf{u} = \mathbf{f}$), and no clause ever consults *undefinedness itself* as a condition. That principle is what makes the whole construction work.

## Partial Interpretations and the Jump

Let $L$ be the language of arithmetic plus a unary predicate $T$, interpreted over the standard model $\mathbb{N}$ with the arithmetic vocabulary fixed classically. A **partial interpretation** of $T$ is a pair $(E, A)$ of disjoint sets of numbers: the **extension** $E$ (codes of sentences counted true) and **anti-extension** $A$ (counted false; conventionally $A$ also absorbs all non-sentence codes). In the model $M(E,A)$, the atomic formula $T(\overline{n})$ is $\mathbf{t}$ if $n \in E$, $\mathbf{f}$ if $n \in A$, and $\mathbf{u}$ otherwise; all sentences then receive values in $\{\mathbf{t},\mathbf{u},\mathbf{f}\}$ by the $K_3$ clauses.

Define the **jump operator**
$$\kappa(E, A) = \bigl(\{\ulcorner\phi\urcorner : \phi \text{ is } \mathbf{t} \text{ in } M(E,A)\},\; \{\ulcorner\phi\urcorner : \phi \text{ is } \mathbf{f} \text{ in } M(E,A)\} \cup \overline{\mathrm{Sent}}\bigr).$$
The jump reads off which sentences are *made* true or false by the current hypothesis about $T$. An interpretation where hypothesis and outcome agree — $\kappa(E,A) = (E,A)$ — is a **fixed point**: there, $T(\ulcorner\phi\urcorner)$ is true iff $\phi$ is, false iff $\phi$ is. The T-schema holds in the strongest form available: $T(\ulcorner\phi\urcorner)$ and $\phi$ always have the *same* value. A fixed point is a language containing its own truth predicate.

Order partial interpretations by information: $(E,A) \le (E',A')$ iff $E \subseteq E'$ and $A \subseteq A'$.

**Lemma (Monotonicity).** If $(E,A) \le (E',A')$ then $\kappa(E,A) \le \kappa(E',A')$.

*Proof.* By induction on complexity, every sentence $\mathbf{t}$ (resp. $\mathbf{f}$) in $M(E,A)$ remains so in $M(E',A')$. Atomic arithmetic sentences: interpretation unchanged. $T(\overline{n})$: if $\mathbf{t}$ then $n \in E \subseteq E'$; similarly for $\mathbf{f}$. $\neg\phi$: immediate from the hypothesis with $\mathbf{t},\mathbf{f}$ swapped. $\phi \wedge \psi$ is $\mathbf{t}$ iff both conjuncts are $\mathbf{t}$ (preserved), $\mathbf{f}$ iff some conjunct is $\mathbf{f}$ (preserved); dually for $\vee$, and for $\forall,\exists$ with instances. Every $K_3$ clause states a *positive* condition on the values of subsentences — never the absence of a value — so gaining information never destroys a value. $\square$

## The Least Fixed Point Exists

**Theorem (Kripke 1975).** $\kappa$ has a least fixed point $(E_\infty, A_\infty)$.

*Proof sketch.* Iterate transfinitely from the empty ground: $(E_0, A_0) = (\varnothing, \varnothing)$; $(E_{\alpha+1}, A_{\alpha+1}) = \kappa(E_\alpha, A_\alpha)$; unions at limit stages. Since $(E_0,A_0) \le \kappa(E_0,A_0)$, monotonicity gives an increasing chain (each stage remains a *disjoint* pair: no sentence is both $\mathbf{t}$ and $\mathbf{f}$ in a partial model). A strictly increasing chain of subsets of a fixed countable set cannot run forever: by cardinality the chain stabilizes at some closure ordinal $\sigma$, and $(E_\sigma, A_\sigma)$ is a fixed point. For leastness, if $(E,A)$ is any fixed point, transfinite induction using monotonicity gives $(E_\alpha, A_\alpha) \le (E,A)$ for all $\alpha$. This is the Knaster–Tarski argument adapted to the chain-complete partial order of disjoint pairs. $\square$

For the arithmetic ground model the closure ordinal is exactly $\omega_1^{CK}$, the first non-recursive ordinal, and $E_\infty$ is a $\Pi^1_1$-complete set — vastly beyond arithmetic definability. Keep that fact in hand; it returns below.

## Grounded, Paradoxical, and In Between

Call $\phi$ **grounded** iff $\phi$ has a value in the least fixed point, i.e. $\ulcorner\phi\urcorner \in E_\infty \cup A_\infty$. Groundedness captures the intuition that truth-talk must ultimately be anchored in non-semantic fact.

**Worked example.** $2+2=4$ is $\mathbf{t}$ already in $M(\varnothing,\varnothing)$, so $\ulcorner 2+2=4 \urcorner \in E_1$. Then $T(\ulcorner 2+2=4\urcorner)$ is $\mathbf{t}$ in $M(E_1,A_1)$, so it enters $E_2$; the $n$-fold iteration $T(\ulcorner T(\ulcorner \cdots \urcorner)\urcorner)$ enters $E_{n+1}$. The sentence "all iterated truth-ascriptions of $2+2=4$ are true" acquires its value only at stage $\omega+1$. Semantic complexity is measured by ordinal birth-stage.

**The Liar is paradoxical.** Let $\lambda$ be $\neg T(\overline{n_\lambda})$ where $n_\lambda = \ulcorner\lambda\urcorner$ (diagonal lemma, Section 1). In the least fixed point $\lambda$ never receives a value: at stage $0$, $T(\overline{n_\lambda})$ is $\mathbf{u}$, so $\lambda$ is $\mathbf{u}$, so $n_\lambda$ enters neither set — and inductively never does. Stronger: $\lambda$ has no value in **any** fixed point. If $n_\lambda \in E$ for a fixed point $(E,A)$, then $T(\overline{n_\lambda})$ is $\mathbf{t}$ in $M(E,A)$, so $\lambda = \neg T(\overline{n_\lambda})$ is $\mathbf{f}$, so $n_\lambda \in A$ by fixedness — violating disjointness. Symmetrically if $n_\lambda \in A$. Sentences valueless in every fixed point are **paradoxical**. $\square$

**The Truthteller is ungrounded but not paradoxical.** Let $\tau$ be $T(\overline{n_\tau})$ with $n_\tau = \ulcorner\tau\urcorner$. From the empty ground $\tau$ never gets a value (it would need itself), so $\tau$ is ungrounded. But start instead from $(\{n_\tau\}, \varnothing)$: then $\tau$ evaluates $\mathbf{t}$, which re-certifies $n_\tau \in E$ — the hypothesis is self-sustaining, and iterating yields a fixed point with $\tau$ true. Starting from $(\varnothing, \{n_\tau\})$ yields one with $\tau$ false. The precise difference, then: the **Liar** can be valued in *no* fixed point (overdetermined — any value refutes itself); the **Truthteller** is valued in *some* fixed points but with different values in different ones (underdetermined — any value sustains itself, none is forced). Groundedness picks out the sentences whose value is forced by the non-semantic facts alone.

## Intrinsic Fixed Points

Among the many fixed points, which are reasonable? Call a fixed point **intrinsic** iff it conflicts with no fixed point whatsoever — it never assigns $\mathbf{t}$ where some other fixed point assigns $\mathbf{f}$, or vice versa. The least fixed point is intrinsic; the Truthteller-true fixed point is not (it conflicts with the Truthteller-false one). **Theorem (Kripke).** The union of all intrinsic fixed points is itself an intrinsic fixed point — the **largest intrinsic fixed point**. $\square$ It strictly extends the least: let $\sigma$ be (by diagonalization) the sentence $T(\ulcorner\sigma\urcorner) \wedge \neg T(\ulcorner\sigma\urcorner)$ — "this sentence is both true and not true." No fixed point can make $\sigma$ true (both conjuncts would need to hold, putting $n_\sigma$ in $E \cap A$), but hypothesizing $\sigma$ false is self-sustaining ($n_\sigma \in A$ makes the first conjunct $\mathbf{f}$, hence $\sigma$ $\mathbf{f}$). So $\sigma$ is ungrounded, yet *every* fixed point that values it at all values it $\mathbf{f}$ — assigning it $\mathbf{f}$ is safe, and $\sigma$ is false in the largest intrinsic fixed point. The largest intrinsic fixed point is the maximal *non-arbitrary* theory of truth the construction supports.

## The Ghost of the Tarski Hierarchy

In the least fixed point the Liar is undefined; in particular $\ulcorner\lambda\urcorner \notin E_\infty$, so $\lambda$ is *not true*. But "$\lambda$ is not true" is exactly what $\lambda$ says — and we just asserted it. Where? Not in the object language: there, "$\lambda$ is not true" ($\neg T(\ulcorner\lambda\urcorner)$) is itself valueless. We asserted it in the **metalanguage**, using classical negation and the classification "undefined" — notions the object language cannot express. Indeed it provably cannot: groundedness is $\Pi^1_1$-complete, far beyond what the object language defines. Kripke was candid about this: "the ghost of the Tarski hierarchy is still with us." The construction domesticates self-referential truth but exiles its own semantic vocabulary — *gap*, *grounded*, *paradoxical* — one level up. Section 4 shows this is no accident.

## Exercises
See [problems/ch18_liars_paradox/](../../../problems/ch18_liars_paradox/)
