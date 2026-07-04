# Abstract Objects: Platonism and Nominalism

Numbers, sets, functions, propositions, properties, the letter-type *A* shared by its many tokens — these are the paradigm **abstract objects**, and whether any of them exist is the deepest question in the philosophy of mathematics. The dispute is not idle metaphysics: by Quine's criterion ([§1](../01_ontology/01_what_exists.md)), our best mathematical and scientific theories quantify over numbers and sets, so taking them at face value commits us to abstracta. The **platonist** accepts the commitment; the **nominalist** works to discharge it. This section develops the debate as a series of formal challenges and replies.

## The Abstract/Concrete Distinction

What makes an object abstract? The dominant answer, the **Way of Negation**, defines abstracta as objects that are neither spatiotemporally located nor causally efficacious: the number $7$ is nowhere, does nothing, and never changes. A rival **Way of Abstraction** (Frege) treats abstract objects as introduced by *abstraction principles* over an equivalence relation — directions abstracted from parallel lines, cardinal numbers from equinumerosity. The distinction is not perfectly sharp (are sets of concrete things abstract? is spacetime itself?), but the negative criterion suffices to frame the epistemological problem: if abstracta are causally inert, how could we ever come to know anything about them?

## Benacerraf's Two Problems

Paul Benacerraf posed the dilemma that organizes the field. In **"Mathematical Truth" (1973)** he set two desiderata against each other:

1. *Semantic uniformity.* "There are at least three perfect numbers greater than $17$" should receive the same Tarskian truth-conditions as "There are at least three large cities older than New York." Both have the form $\exists x_1 x_2 x_3(\dots)$, so the mathematical sentence is true only if numbers exist to witness the quantifier.
2. *Empiricist epistemology.* Knowledge requires some causal or reliable connection between the knower and what makes the belief true.

Platonism secures (1) but wrecks (2): if numbers are causally inert, no such connection can obtain, and mathematical knowledge becomes inexplicable. Anti-platonism secures (2) but strains (1), forcing a deviant semantics for mathematical language. Hartry Field sharpened the epistemic horn (1989): the challenge is to explain the **reliability** of mathematicians — the striking correlation between what they believe and what is true — which seems a brute miracle if the truths concern acausal objects.

Benacerraf's second problem, from **"What Numbers Could Not Be" (1965)**, is structural. The natural numbers can be modeled in set theory in incompatible ways — von Neumann's $2 = \{\varnothing,\{\varnothing\}\}$, Zermelo's $2 = \{\{\varnothing\}\}$ — with no fact of the matter selecting the "real" one. But an object has a determinate identity ([§2](../02_identity/01_identity.md)); if $2$ is no particular set, perhaps it is no *object* at all, only a **position in a structure**. This motivates *structuralism*, developed in [§4](../04_realism/01_realism.md).

## The Indispensability Argument

The strongest positive case for platonism is the **Quine–Putnam indispensability argument**, which even a naturalist must reckon with. Regimented:

- **(P1)** We ought to be ontologically committed to all and only the entities indispensable to our best scientific theories. *(Naturalism plus confirmational holism plus Quine's criterion.)*
- **(P2)** Mathematical objects are indispensable to our best scientific theories.
- **(C)** Therefore we ought to be ontologically committed to mathematical objects.

The argument grants mathematics no special a priori status; it earns its ontology the same way physics earns electrons — by being an ineliminable part of confirmed science. To resist platonism, the nominalist must break one of the premises: deny naturalism/holism (Maddy, Sober), or refute (P2) by showing science *can* be done without abstracta.

## Field's Nominalism: Science Without Numbers

Hartry Field's *Science Without Numbers* (1980) attacks (P2) head-on. His program has two parts. First, **nominalize** a physical theory — reformulate it so its quantifiers range only over concrete entities (spacetime points and regions, with primitive betweenness and congruence relations à la Hilbert's synthetic geometry, [Chapter 20 §3](../../ch20_geometry_and_logic/03_hilbert/01_hilbert_axioms.md)) and never over numbers. Field carries this out for Newtonian gravitation. Second, show that adding mathematics to the nominalistic theory is a **conservative extension** — it proves no new *nominalistic* consequences:

> **Conservativeness.** For any nominalistic theory $N$ and body of mathematics $M$, and any nominalistic statement $A$: if $N \cup M \vdash A$, then $N \vdash A$.

If mathematics is conservative, it is a mere *proof-shortening instrument*: useful, even indispensable in practice, but dispensable *in principle*, and therefore not something whose truth we must accept. Mathematics can then be read as **fiction** — its statements false (or truth-valueless) but "true in the story," on a par with claims about Sherlock Holmes. Field defends conservativeness by a model-theoretic argument leaning on compactness. The objections are serious: his spacetime substantivalism quantifies over *regions* (abstract enough to trouble a strict nominalist), his formulation uses **second-order** logic (which Shapiro argues smuggles in set-like resources), and no one has nominalized quantum mechanics, whose Hilbert-space apparatus resists a concrete reading.

## Neo-Fregean Abstraction and Frege's Theorem

A different platonism recovers abstract objects from something close to logic. Frege's idea was to introduce numbers by an **abstraction principle**, and the neo-Fregeans (Wright 1983, Hale) revive it with **Hume's Principle (HP)**:
$$\#F = \#G \;\leftrightarrow\; F \approx G,$$
"the number of $F$s equals the number of $G$s iff the $F$s and $G$s can be put in one-to-one correspondence" (where $F \approx G$ is the purely logical statement that a bijection exists). HP is offered as a near-analytic truth *implicitly defining* the number operator $\#$. Its power is remarkable:

> **Frege's Theorem** (Frege 1884, rediscovered by Wright and Boolos). Second-order logic plus Hume's Principle proves the **Dedekind–Peano axioms** of arithmetic.

One defines $0 := \#[x : x \neq x]$, defines successor via HP, and derives the infinity of the number series — arithmetic from a single abstraction principle plus logic. The neo-Fregean numbers are **thin objects**: their existence follows near-trivially from true equinumerosity facts, defusing Benacerraf's epistemology (we access numbers by grasping HP, not by causal contact).

The catch is the **Bad Company** problem. Frege's own *Basic Law V* — $\{x : Fx\} = \{x : Gx\} \leftrightarrow \forall x(Fx \leftrightarrow Gx)$ — has exactly HP's form yet is **inconsistent** in second-order logic (it yields Russell's paradox, [Chapter 6](../../ch06_set_theory/)). So some abstraction principles are good (HP) and some catastrophic (Law V), and the neo-Fregean owes a principled criterion — conservativeness, stability, or the like — separating the acceptable from the paradoxical (Linnebo, *Philosophy of Mathematics*, 2017; *Thin Objects*, 2018).

## Nominalist Strategies in Brief

The nominalist toolkit beyond Field's fictionalism includes: **modal if-then-ism** (Putnam), reading arithmetic as $\Box(\text{PA-axioms} \to \varphi)$, trading objects for necessity; **mereological nominalism** (Goodman and Quine, "Steps Toward a Constructive Nominalism," 1947), replacing classes by concrete fusions of inscriptions (Chapter 16) — which founders because there are too few concrete tokens to model transfinite mathematics, precisely the expressive limit noted in [Chapter 16's decidability section](../../ch16_mereology/02_comparison/01_mereology_vs_sets.md) (fusions cannot count); and **predicativism**, restricting the property-quantifiers of comprehension to tame the ontology. Each buys nominalist scruple at the price of expressive or explanatory power — the recurring exchange rate of this debate.

## Exercises
See [problems/ch22_metaphysics/](../../../problems/ch22_metaphysics/)
