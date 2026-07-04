# Identity and Individuation

Identity looks like the simplest relation there is — everything is identical to itself and to nothing else — yet it anchors some of the sharpest arguments in metaphysics. The reason is that identity is governed by a single powerful principle, **Leibniz's Law**, which licenses substitution: if $a$ and $b$ are one thing, whatever holds of $a$ holds of $b$. Turn the law around, apply it in modal or temporal contexts, and it generates the necessity of identity, the puzzle of the statue and the clay, and the question whether identity is absolute at all.

## Identity in First- and Second-Order Logic

In first-order logic with identity, $=$ is a distinguished binary predicate fixed by two principles:

- **Reflexivity.** $\forall x\,(x = x)$.
- **Substitutivity (Indiscernibility of Identicals).** $x = y \to (\varphi(x) \to \varphi(y))$, for every formula $\varphi$.

Together these make $=$ an equivalence relation and a *congruence* for every predicate and function of the language — the logical minimum any identity relation must satisfy. First-order logic cannot pin identity down completely: a model may interpret $=$ as any congruence, and only the standard (normal) models read it as genuine identity.

**Second-order logic can define identity outright.** Quantifying over all properties $X$,
$$x = y \;:=\; \forall X\,\bigl(X x \leftrightarrow X y\bigr).$$
Two things are identical iff they share every property. This makes the second direction of Leibniz's Law — the **Identity of Indiscernibles** — true by definition, which is exactly why it is philosophically loaded: whether it is a substantive truth depends on which "properties" the quantifier $\forall X$ is allowed to range over.

## Leibniz's Law, Both Directions

**Indiscernibility of identicals**, $a = b \to (Fa \leftrightarrow Fb)$, is close to non-negotiable — it is constitutive of what identity is. Its apparent counterexamples all arise in **non-extensional contexts**. Lois Lane believes Superman can fly but not that Clark Kent can, though Superman $=$ Clark Kent; "necessarily $9 > 7$" is true but "necessarily the number of planets $> 7$" is false, though the number of planets $= 9$. The diagnosis (Frege, Quine) is that belief-, modal-, and quotation-contexts are **referentially opaque**: the substituted position is not purely referential, so Leibniz's Law does not apply there unrestricted. Restricted to extensional contexts — or with intensional operators explicitly regimented (Chapter 12) — the law stands.

**The Identity of Indiscernibles**, its converse $\forall X(Xa \leftrightarrow Xb) \to a = b$, is genuinely controversial. If "properties" include *impure* ones like *being identical to $a$*, the principle is trivial (only $a$ has that property). Over purely *qualitative* properties it is a substantive metaphysical thesis, and arguably false. Max Black's thought experiment (1952): a perfectly symmetric universe containing just **two exactly similar iron spheres**, two miles apart. They share every qualitative property, yet there are two of them — so distinctness need not rest on any qualitative difference. Identity, Black concludes, is not reducible to indiscernibility; the individuation of objects can be brute. This is not a question logic settles — the substitutivity axiom is silent on it — but one the metaphysics must.

## The Necessity of Identity

One of the most consequential short proofs in analytic metaphysics, due to Ruth Barcan Marcus (1947) and sharpened by Saul Kripke, shows that **all identities are necessary**. Working in quantified modal logic (Chapter 12) with Leibniz's Law:

1. $\forall x\,\Box(x = x)$.  *(Necessity of self-identity: $x=x$ is a theorem, so $\Box(x=x)$ by necessitation.)*
2. Instantiate Leibniz's Law with the property $\varphi(z) := \Box(x = z)$:
$$x = y \;\to\; \bigl(\Box(x = x) \to \Box(x = y)\bigr).$$
3. From (1) the antecedent $\Box(x = x)$ holds, so
$$x = y \;\to\; \Box(x = y). \qquad \square$$

If $a$ and $b$ are in fact identical, they are *necessarily* identical. How, then, can "Hesperus = Phosphorus" be an informative, empirical discovery? Kripke's answer (*Naming and Necessity*, 1980) distinguishes metaphysical from epistemic modality. Names are **rigid designators** — each picks out the same object in every possible world — so a true identity between names is necessary. But its truth may be knowable only *a posteriori*, since the descriptions by which we *fix* the referents ("the evening star," "the morning star") are contingent. "Hesperus = Phosphorus" is thus **necessary yet a posteriori**: a category Kripke's argument opened up, overturning the assumption that the necessary and the a priori coincide.

## Contingent Identity and Material Constitution

The necessity of identity collides head-on with a mereological puzzle (Chapter 16). Consider a clay statue: call the lump of clay **Lumpl** and the statue **Goliath** (Gibbard, 1975). They are made of exactly the same matter at exactly the same time — seemingly one object. Yet they differ in their *modal* properties: Goliath cannot survive being squashed into a ball, whereas Lumpl can. Let $F$ be the property *possibly survives flattening*. Then $F(\text{Lumpl})$ and $\neg F(\text{Goliath})$, so by the contrapositive of Leibniz's Law,
$$F(\text{Lumpl}) \land \neg F(\text{Goliath}) \;\Rightarrow\; \text{Lumpl} \neq \text{Goliath}.$$
Two distinct objects coincide in all their matter. This is **constitution without identity**: the clay *constitutes* the statue but is not identical to it (see [Chapter 16 §4](../../ch16_mereology/03_applications/01_physical_objects.md)). The alternatives are all costly: deny that modal predicates are genuine properties eligible for Leibniz's Law (Lewis's **counterpart theory**, on which modal predicates are inconstant, varying with how the object is described); adopt **four-dimensionalism**, on which statue and lump are different space-time worms sharing a temporal part; or bite the bullet and accept coincident objects (Wiggins). Each choice reverberates through the logic of identity, modality, and parthood.

## Persistence and Relative Identity

Identity over *time* raises a parallel question. **Endurantism** holds that an object is wholly present at each moment; **perdurantism** that it persists by having distinct temporal parts, a 4D worm; **stage theory** that ordinary objects are momentary stages linked by a genidentity relation. The choice is not settled by logic but selects the formal framework — tensed vs. tenseless quantification, temporal parts vs. bare persistence — in which change is described without contradicting Leibniz's Law (an object cannot be both bent and straight *simpliciter*, but may be bent-at-$t_1$ and straight-at-$t_2$).

Peter Geach pressed a more radical thesis: **relative identity**. There is, he argued, no absolute "$x = y$", only sortal-relative "$x$ is the same $F$ as $y$" — one might have the same clay but a different statue. This would abolish the classical identity predicate and with it the unrestricted Leibniz's Law. Most logicians resist, retaining absolute identity for its clean second-order definition and its indispensability to counting: to ask *how many* objects there are presupposes a criterion of *which count as the same one* (Frege). The number one assigns to a concept — the bridge to abstract objects and [Chapter 22 §3](../03_abstract/01_abstract_objects.md) — is well-defined only because the concept carries determinate identity conditions.

## Exercises
See [problems/ch22_metaphysics/](../../../problems/ch22_metaphysics/)
