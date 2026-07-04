# Ontology and Quantification

Ontology asks the shortest and hardest question in philosophy: *what is there?* The answer W.V.O. Quine gave in "On What There Is" (1948) is famously flippant — "everything" — and the real work begins with the disagreements about particular cases: numbers, properties, possible worlds, fictional characters. Quine's lasting contribution was not an answer but a **method**: fix what a theory says exists by inspecting the variables its quantifiers bind. Ontology becomes, on this approach, a branch of logic — the study of the existential commitments of our best regimented theories.

## Quine's Criterion of Ontological Commitment

> **To be is to be the value of a bound variable.**

A theory is **ontologically committed** to those entities that must lie in the range of its quantifiers for its sentences to be true. If a theory, regimented into first-order logic, asserts $\exists x\,\varphi(x)$, and there is no way to paraphrase this commitment away, then the theory says there is at least one $\varphi$-thing. The criterion is a test of *theories*, not of isolated sentences or of reality directly: it tells us what a theory *says* there is, leaving open whether the theory is true.

Two refinements matter. First, commitment attaches to **bound variables, not names**. A name like "Pegasus" carries no automatic commitment, because names can be recast as predicates (to *pegasize*) and eliminated by Russell's theory of descriptions (below). Second, the criterion applies only after **regimentation** into canonical (first-order) notation. "There are prime numbers between $10$ and $20$" regiments as
$$\exists x\,\bigl(\mathrm{Prime}(x) \land \mathrm{Number}(x) \land 10 < x < 20\bigr),$$
whose truth requires numbers among the values of $x$: the sentence is committed to numbers. To *avoid* the commitment, a nominalist must either reject the sentence or supply a paraphrase whose quantifiers range only over acceptable entities — the strategy of [Chapter 22 §3](../03_abstract/01_abstract_objects.md).

## Regimentation and Semantic Ascent

Because commitment is read off the *canonical form*, ontology depends on how we regiment ordinary language, and rival regimentations carry rival commitments. "The average family has $2.3$ children" does not commit us to an average family; its regimentation quantifies over families and a ratio, not over an odd fractional household. Disagreement about what exists is thus often disagreement about the correct paraphrase — which is why Quine insists ontology is done in the "canonical notation" of first-order logic, where the quantifiers are unambiguous. His related notion of **semantic ascent** — moving from talk of things to talk of sentences about things — lets disputants who share little agree on the logical form under debate.

## Meinong's Jungle and Russell's Razor

The criterion's chief historical rival is Alexius Meinong's theory of objects. Meinong held that "the golden mountain is golden" and "the round square is round" are true, and that their subjects must therefore have *being* of some attenuated kind — a realm of nonexistent-but-real objects Quine derided as "Meinong's jungle," "a slum of possibles," "a breeding ground for disorderly elements." The puzzle that drove it, **Plato's beard**, is the riddle of negative existentials: "Pegasus does not exist" seems to presuppose a Pegasus to predicate nonexistence of.

Bertrand Russell's theory of descriptions ("On Denoting," 1905) shaves the beard. "The $F$ is $G$" is not a subject-predicate claim about a mysterious object but the quantified
$$\exists x\,\bigl(F x \land \forall y\,(F y \to y = x) \land G x\bigr).$$
When nothing is $F$, this is simply **false** — no commitment to a nonexistent $F$ incurred. And "Pegasus does not exist" becomes $\neg\exists x\,\mathrm{Pegasizes}(x)$: true, meaningful, and committed to nothing. The apparent reference to a nonexistent object dissolves into ordinary quantification and negation. This is the paradigm of the whole method: a metaphysical embarrassment defused by attention to logical form.

## Existence Is Not a First-Order Predicate

A lesson runs through all of this, due to Kant and made precise by Frege: **existence is not a property of individuals but of concepts.** To say tigers exist is not to ascribe a feature, *existence*, to each tiger; it is to say the concept *tiger* is **instantiated** — $\exists x\,\mathrm{Tiger}(x)$. Existence is expressed by the quantifier, a second-order notion (a property of properties): $\exists x\,F x$ says that $F$ has at least one instance. Frege put it that existence is the denial of the number zero to a concept. This is why "$a$ exists" is logically awkward while "$Fs$ exist" is not, and why the ontological argument's treatment of existence as a perfection-predicate misfires: on the Fregean analysis there is no such predicate to be perfected.

## Free Logic and Non-Existent Objects

Classical first-order logic bakes in two existence assumptions that ordinary reasoning violates: every singular term denotes something, and the domain is non-empty. So classical logic proves $\exists x\,(x = t)$ for *every* term $t$ — including "Vulcan" and "the greatest prime" — and proves $\exists x\,(x = x)$, ruling out the empty domain. **Free logic** ("free" of existence assumptions for its terms) repairs this. It adds an existence predicate, definable as
$$E!(t) \;:=\; \exists x\,(x = t),$$
and *restricts universal instantiation*: from $\forall x\,\varphi(x)$ one may infer $\varphi(t)$ only given the extra premise $E!(t)$. Terms may now fail to denote, and the quantifiers range only over what exists.

Free logics differ on how to treat atomic sentences containing empty terms:

- **Positive** free logic lets some such sentences be true (e.g. "Pegasus is Pegasus," or truths within a fiction);
- **Negative** free logic counts every atomic sentence with an empty term as false;
- **Neutral** (supervaluational) free logic assigns them a truth-value **gap**, resolved only when the term denotes.

A Meinong-friendly variant uses an **outer domain** of nonexistent objects alongside an **inner domain** of existents: the quantifiers $\exists, \forall$ range over the inner domain, while terms may refer into the outer one — a disciplined, consistent reconstruction of Meinong's insight without Quine's jungle. Free logic is not merely a philosophers' toy: it is the natural setting for the logic of fiction, for definite-description theories, and — most practically — for reasoning with **partial functions** in formalized mathematics, where terms like $1/0$ or $\lim_n a_n$ may fail to denote. Proof assistants confront exactly this: Isabelle/HOL assigns undefined terms arbitrary "junk" values, while Lean and Coq use `Option` types or side conditions — engineering descendants of the free-logical treatment of non-denoting terms. The metaphysics of nonexistence, regimented, becomes a question about how a logic handles reference failure.

## Exercises
See [problems/ch22_metaphysics/](../../../problems/ch22_metaphysics/)
