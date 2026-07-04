# Revenge: The Strengthened Liar

Every proposed solution to the Liar supplies new vocabulary for describing the Liar's defect — *gappy*, *ungrounded*, *unstable*, *indeterminate*. And every such vocabulary is raw material for a new Liar, phrased in the solution's own terms, against which the solution is helpless. This is the **revenge phenomenon**, and it is the central structural fact about the Liar literature.

## The Strengthened Liar

Distinguish two Liars:

- $L$: "This sentence is **false**."
- $\lambda$: "This sentence is **not true**."

Under bivalence they coincide. For a gap theory they come apart: "not true" covers both the false *and* the gappy. Kripke's theory handles $L$ by ruling it valueless — neither true nor false, so no contradiction. Now run the same play against $\lambda$, where $\lambda \leftrightarrow \neg T(\ulcorner\lambda\urcorner)$:

1. $\lambda$ is gappy. $\quad$ [the theory's own verdict: $\lambda$ is valueless in every fixed point]
2. Whatever is gappy is not true. $\quad$ [meaning of "gap"]
3. Therefore $\lambda$ is not true. $\quad$ [1, 2]
4. But "$\lambda$ is not true" is precisely what $\lambda$ says. $\quad$ [the fixed-point equation]
5. A sentence that says something, when what it says is the case, is true. $\quad$ [T-schema, ascending direction]
6. Therefore $\lambda$ is true. $\quad$ [3, 4, 5 — contradicting 3]

**Where is the illicit step?** Each step is individually compelling, and *within the object language* the argument cannot even be stated: if $\lambda$ is valueless, then $T(\ulcorner\lambda\urcorner)$ is valueless, so $\neg T(\ulcorner\lambda\urcorner)$ — the internal reading of step 3 — is valueless too, and Strong Kleene licenses asserting none of it. The negation in step 3, as we intend it, is **exclusion negation** ("lacks the value true"), which is not the object language's **choice negation** $\neg$; and step 5 applies the T-schema to a classification ("gappy") available only in the metalanguage. The reasoning is perfectly sound — *conducted one level up*. So the gap theory is not inconsistent; it is **expressively incomplete**: it cannot say, of its own problem sentence, the very thing its proponent says when explaining the theory. The Liar is not solved but relocated.

## The Revenge Recipe

The pattern generalizes. A theory $\mathcal{T}$ of truth for $L$ partitions $L$'s sentences into semantic categories — some "good" (true, false) and some pathological ($P$: gappy, ungrounded, paradoxical, unstable, indeterminate). Diagonalize on the theory's own classification:

$$\rho \;\leftrightarrow\; \bigl(F(\ulcorner\rho\urcorner) \vee P(\ulcorner\rho\urcorner)\bigr) \qquad \text{"this sentence is either false or pathological."}$$

If $\rho$ is true, it is false or pathological — either way not (cleanly) true. If false, then one disjunct holds, so it is true. If pathological, the second disjunct holds, so what it says is the case — true again. Each verdict the theory can issue refutes itself, *provided the predicate $P$ is expressible in $L$*.

- **Revenge on Kripke**: "this sentence is either false or ungrounded." Kripke's object language survives only because groundedness (a $\Pi^1_1$-complete notion, Section 3) is *not* expressible in it. Add a predicate $G$ for groundedness and the construction collapses.
- **Revenge on Tarski**: "this sentence is not true at any level $T_n$" — inexpressible, since no level quantifies over all levels. Again survival by enforced silence.

**Paracomplete determinacy hierarchies.** Field (*Saving Truth from Paradox*, 2008) builds a paracomplete theory that keeps the full intersubstitutivity of $T(\ulcorner\phi\urcorner)$ with $\phi$ by weakening classical logic (no excluded middle) and adding a new conditional $\to$, constructed by a transfinite revision sequence over Kripkean fixed points. The Liar's status is expressed *inside* the language via a **determinacy operator** $D\phi$ (roughly $\phi \wedge \neg(\phi \to \neg\phi)$): $\lambda$ is not determinately true and not determinately false. The strengthened Liar returns as "this sentence is not *determinately* true" — handled by $\neg DD$-classification; then "not determinately determinately true," and so on. Field iterates $D^\alpha$ through the transfinite; each new liar is classified one level further up, and no single operator "determinately at every level" is expressible. The Tarskian ladder, evicted from the truth predicate, re-erects itself inside the determinacy operators.

## Responses

**Contextualism** (Parsons 1974, Burge 1979, Glanzberg 2001). The extension of "true" shifts with context, like a quantifier domain. The Liar reasoning is sound but equivocates: the assessment "$\lambda$ is not true" is made in a *richer* context $c_1$ than the context $c_0$ of $\lambda$ itself, and asserts un-truth$_{c_0}$ truly$_{c_1}$. Revenge is absorbed as endless context-shift: every attempt to speak of "truth in all contexts at once" merely opens a new context. The view formalizes the strengthened-Liar reasoning above — steps 1–3 in $c_0$-vocabulary, steps 4–6 in $c_1$ — and declares the shift a feature of natural language, not a bug.

**Inexpressibility quietism.** Accept that any consistent theory has semantic notions it cannot express, and hold that this is a discovery about truth (a cousin of Tarski's and Gödel's theorems), not a defect of the theory. The cost is admitting that the theorist's own explanatory discourse outruns every theory she can state.

**Dialetheism.** Deny that revenge must be blocked: let $\rho$ be both true and false, accept the contradiction, and adopt a logic in which contradictions do not explode. Uniquely among the responses, this one claims *expressive completeness* — there is no classification the language must be protected from, hence no boundary for revenge to breach. Whether it delivers on that claim is the business of Section 5.

## A Formal Criterion

The moral can be stated as a theorem-shaped slogan:

**Observation (revenge template).** Let $\mathcal{T}$ be a theory of truth for $L$, stated in $L$, whose semantic classification includes a pathological category $P$, and suppose (i) $L$ has the diagonal lemma; (ii) $P(x)$ is expressible in $L$; (iii) $\mathcal{T}$ endorses "pathological sentences are not true," the T-schema ascent used in step 5 above, and reasoning by cases on its own classification. Then $\mathcal{T}$ is inconsistent — diagonalize on $F(x) \vee P(x)$ and run the case analysis. $\square$

Contrapositively: **a consistent theory of truth for $L$, stated in $L$, escapes revenge only where its own semantic classifications are inexpressible in $L$.** Every classical solution therefore purchases consistency with expressive incompleteness, hidden hierarchy, or context-relativity; the only announced alternative is to give up consistency itself.

## Exercises
See [problems/ch18_liars_paradox/](../../../problems/ch18_liars_paradox/)
