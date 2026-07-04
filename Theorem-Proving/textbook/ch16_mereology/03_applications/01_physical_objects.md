# Physical Objects: Constitution, Composition, Persistence

Mereology earns its keep in the metaphysics of material objects. Three classical puzzles — the statue and the clay, the Special Composition Question, and persistence through time — are all, at bottom, questions about which mereological axioms to accept. Each can be stated as a formal condition in $\mathcal{L}_P$, and the competing metaphysical positions differ precisely in which sentences they affirm.

## Material Constitution: The Statue and the Clay

A sculptor buys a lump of clay, *Lumpl*, and shapes it into a statue, *Goliath*; later both are squashed and destroyed together (Gibbard's version, in which the two exist at exactly the same times). At every moment the statue and the lump occupy the same place and are made of the same matter. Are they one object or two?

**Definition (Coincidence).** $x \approx y \equiv \forall z\,\bigl(P(z,x) \leftrightarrow P(z,y)\bigr)$.

In ground mereology $\mathbf{M}$, full coincidence already collapses into identity: instantiating $z := x$ gives $P(x,y)$ (since $P(x,x)$), and $z := y$ gives $P(y,x)$; antisymmetry yields $x = y$. So the substantive notion is *proper-part* coincidence, $\forall z\,(PP(z,x) \leftrightarrow PP(z,y))$ — and here the extensionality theorem of EM (Section 2) takes over: **composite objects with the same proper parts are identical**. Extensional mereology thus forces:

$$\text{Goliath and Lumpl share all proper parts} \;\Rightarrow\; \text{Goliath} = \text{Lumpl}.$$

The puzzle is that Leibniz's law seems to refute the identity. The lump *could survive* squashing; the statue could not:
$$\Diamond S(\text{Lumpl}), \quad \neg \Diamond S(\text{Goliath}) \quad \Rightarrow \quad \text{Lumpl} \neq \text{Goliath}$$
by the contrapositive of indiscernibility of identicals. Four premises — shared parts, extensionality, the modal difference, Leibniz's law for modal predicates — are jointly inconsistent, and each rejection defines a research program:

1. **Reject the modal argument** (Gibbard, Lewis): keep the identity and explain the appearance of difference by *counterpart theory* — "could survive squashing" evaluates one object under the lump-counterpart relation and under the statue-counterpart relation, so the modal "predicates" are inconstant and Leibniz's law does not apply to them.
2. **Reject shared parts**: with time-indexed parthood (below), if Lumpl outlives Goliath they differ in temporal parts; but in Gibbard's case, where lifespans coincide exactly, this exit is blocked.
3. **Reject extensionality**: non-extensional mereologies allow two objects with the same proper parts, distinguished by form or arrangement. The cost is giving up SSP or reinterpreting $PP$, and with it the clean Boolean model theory of Section 2.
4. **Reject the framework of enduring things**: four-dimensionalism, below.

The formal lesson: the constitution debate is not about murky intuition but about which of four precise sentences to deny.

## The Special Composition Question

Van Inwagen (*Material Beings*, 1990) asked: under what conditions do some things compose something? In fusion notation: **for which conditions $\phi$ does $\exists z\, \mathrm{Fu}(z,\phi)$ hold?** The main answers are formal restrictions on the fusion schema:

- **Nihilism**: fusions exist only trivially — $\exists z\,\mathrm{Fu}(z,\phi)$ only when $\phi$ is satisfied by exactly one thing. Equivalently, $\forall x\, \mathrm{Atom}(x)$: nothing is ever composed. "There are tables" gets paraphrased as "there are simples arranged tablewise."
- **Universalism**: the unrestricted fusion schema of GEM — every instantiated $\phi$ has a fusion, however scattered.
- **Restricted composition**: $\exists z\,\mathrm{Fu}(z,\phi) \leftrightarrow C[\phi]$ for some substantive condition $C$: the $\phi$-ers are in contact; are fastened together; or — van Inwagen's own answer — *their activity constitutes a life*, so that the only composites are organisms (plus mereological simples).

## The Vagueness Argument for Universalism

Why did Lewis and Sider find restricted composition untenable? The argument, in outline:

1. If composition is restricted by a condition like contact or life, there is a **sorites series** for composition: a sequence of cases (assembling a hammer molecule by molecule) sliding continuously from clear non-composition to clear composition.
2. No sharp cut-off in such a series is remotely plausible; so a restriction must be **vague** at some point.
3. But whether composition occurs cannot be vague. $\exists z\,\mathrm{Fu}(z,\phi)$ is built from quantifiers, connectives, identity, and $P$ alone. Vagueness is semantic indecision among candidate meanings — and the logical vocabulary has no candidate meanings to waver among. In a world with finitely many objects, "there are exactly $n$ things" is a sentence of pure logic with identity; if composition were vague, some such numerical sentence would be vague — which, Lewis and Sider urge, is impossible.
4. Hence composition is unrestricted or never occurs; and since some things clearly compose (there are molecules, say), it is unrestricted. $\square$ (as an argument sketch)

Resisters typically deny premise 3, accepting *ontic* vagueness or a vague existential quantifier; the cost is revising classical semantics for the quantifiers themselves. Sider (2001) sharpened the argument into a defense of temporal parts as well.

## Temporal Parts: Endurantism vs. Perdurantism

Persistence puzzles force a choice about the *logical form* of parthood. The two camps differ in signature:

- **Endurantism**: parthood is time-indexed — a three-place primitive $P(x,y,t)$, "$x$ is a part of $y$ *at* $t$." Objects are "wholly present" whenever they exist; nothing has temporal parts. Extensionality must be restated per time, and coincidence-at-every-$t$ no longer entails identity — which is exactly how endurantists block the statue-clay collapse.
- **Perdurantism** (four-dimensionalism): parthood is atemporal two-place $P(x,y)$, as in GEM, and objects persist by having **temporal parts**.

**Definition (Temporal part, after Sider).** $z$ is the temporal part of $x$ at interval $t$ iff (i) $P(z,x)$, (ii) $z$ exists at $t$ and only at $t$, and (iii) $z$ overlaps every part of $x$ that exists at $t$.

On this picture a persisting thing is a spacetime "worm," and *my-body-today* is a proper temporal part of my body. The statue-clay puzzle dissolves when lifespans differ — Lumpl and Goliath are distinct worms sharing a temporal segment, no threat to extensionality since their proper parts differ — while the perfect-coincidence case is handled by counterpart theory, as in response 1 above.

## Arbitrary Fusions: The Trout-Turkey

Universalism plus four-dimensionalism yields spectacularly unnatural objects. Lewis's example: the **trout-turkey**, the fusion of the front half of a trout and the back half of a turkey — spatially scattered, cross-classified, useless. Why tolerate it? Because for the universalist, composition is **ontologically innocent**: the fusion is nothing over and above its parts, so admitting it costs nothing — the trout-turkey's existence is no more mysterious than the trout's and the turkey's. What distinguishes cats from trout-turkeys is not *existence* but *naturalness*: some fusions carve reality at its joints and earn names, most do not. When common sense says "there is no such thing," it speaks with a restricted quantifier — ranging over salient, well-demarcated objects — and restricted quantification is a semantic phenomenon, not an ontological one. The alternative, restricting composition itself, runs headlong into the vagueness argument above.

## Exercises
See [problems/ch16_mereology/](../../../problems/ch16_mereology/)
