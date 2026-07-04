# Singular vs. Plural Reference

Natural language marks a grammatical distinction that first-order logic erases. "The book is on the shelf" refers to one object; "the books are on the shelf" refers to many. A **singular term** — "Socrates," "the least prime" — picks out a single object; a **plural term** — "Russell and Whitehead," "the natural numbers," "the critics" — refers to *several objects at once*, not to a single further object that collects them. Plural logic is the project of taking this distinction as logically fundamental rather than reducing it away.

## How First-Order Logic Copes, and What It Costs

First-order logic (Chapter 3) has only singular variables: each $x$ ranges over one object of the domain. It handles *distributive* plural predication smoothly, by universal quantification. "The books are heavy" — read distributively, each book is heavy — becomes
$$\forall x\,(\mathrm{Book}(x) \to \mathrm{Heavy}(x)).$$
No plural apparatus is needed, because the predicate distributes down to individuals.

The trouble is **collective** predication, where something is said of the objects *together* and of no one of them. "The rocks weigh a ton" does not mean each rock weighs a ton. "The students surrounded the building" is true though no student surrounded it. "These lines are parallel" is a relation among the lines collectively. Regimenting these in first-order logic forces a choice, and both options are unhappy:

- **Reify a set.** Introduce a set $S$ of the rocks and predicate the weight of $S$: $\mathrm{WeighsATon}(S)$. But now the sentence is committed to an abstract object — a set — that was nowhere in its original subject matter. "The rocks weigh a ton" says something about rocks, not about a set-theoretic entity; a nominalist who denies abstract objects should still be able to assert it. We have silently changed the ontology.
- **Reify a mereological sum.** Predicate the weight of the *fusion* of the rocks (Chapter 16). This avoids abstracta but distorts identity conditions: the fusion of the rocks is the same object as the fusion of their atoms or of two disjoint halves, so "the rocks" and "the two piles" would be forced to co-refer even when we want to count them differently.

Either way, a sentence whose surface subject is plurally *many* objects is reconstrued as being *singularly* about *one* object of a new kind. Plural logic's wager is that this detour is both unnecessary and distorting.

## The Geach–Kaplan Sentence

The sharpest case, due to Peter Geach and David Kaplan, resists even the set detour as a matter of expressive power, not just ontology:

> **"Some critics admire only one another."**

Read this as: there are some critics, each of whom admires, among people, only *others of them*, and there is no smaller such group hiding inside. The natural set-theoretic regimentation quantifies over a nonempty set $S$ of critics closed under the "admires" relation only internally:
$$\exists S\,\bigl[S \neq \varnothing \land \forall x(x \in S \to \mathrm{Critic}(x)) \land \forall x\forall y\,(x \in S \land \mathrm{Admires}(x,y) \to y \in S \land x \neq y)\bigr].$$
Kaplan proved (the argument is a Löwenheim–Skolem-style construction) that **no first-order sentence** in the vocabulary $\{\mathrm{Critic}, \mathrm{Admires}\}$ *without* the set variable is equivalent to this. The plural content is genuinely second-order: to say it, first-order logic must quantify over sets. Yet ordinary speakers assert "some critics admire only one another" with no thought of sets at all — the sentence is grasped by anyone who knows English. Boolos's diagnosis: the quantifier "some critics" is not a disguised set quantifier but an irreducibly **plural** quantifier, and the logic that formalizes it should say so directly.

## Plural Variables and Quantifiers

Plural logic accordingly extends first-order logic with new syntactic machinery, kept type-distinct from the singular vocabulary:

- **Plural variables** $xx, yy, zz$, ranging over *many* individuals of the domain plurally (by the standard convention, over *one or more* — pluralities are nonempty).
- **Plural quantifiers** $\exists xx$ ("there are some things $xx$ such that…") and $\forall xx$ ("for any things $xx$…").
- The **inclusion predicate** $x \prec xx$, read "$x$ is one of the $xx$."

The reading of $\exists xx\,\phi(xx)$ is "there are some things — the $xx$ — such that $\phi$ holds of them." Three points fix the intended meaning, and each will be made precise in later sections:

1. $xx$ **is not a set.** It is a plural term denoting individuals plurally, exactly as "Russell and Whitehead" denotes two people and not a third thing containing them.
2. $x \prec xx$ **is not membership.** Unlike $\in$, it is a primitive of the object language relating a *singular* term to a *plural* term; nothing in its logic mimics the iterative hierarchy of sets.
3. **No new object is posited.** The domain of quantification is unchanged; $\exists xx$ ranges over pluralities *drawn from the existing individuals*, adding nothing to the ontology. This is the ontological-innocence thesis, examined in the next two sections.

With this vocabulary the collective cases are stated without reifying anything. "The rocks weigh a ton" takes a collective predicate $W$ true of pluralities: $W(rr)$ where $rr$ are the rocks. "Some critics admire only one another" becomes a plural sentence with no set variable, written out in the [next section](02_boolos_plural.md). The claim to be defended is that this is not shorthand for set talk but a logic of its own — as basic as, and independent of, the singular quantifiers we started with.

## Exercises
See [problems/ch15_plural_logic/](../../../problems/ch15_plural_logic/)
