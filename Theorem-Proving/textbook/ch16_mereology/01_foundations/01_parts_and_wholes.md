# Parts and Wholes

Mereology (from the Greek *meros*, "part") is the formal theory of the part–whole relation. It was founded by Stanisław Leśniewski in 1916 as a nominalistically acceptable alternative to set theory — a response to Russell's paradox that avoids abstract sets altogether — and was independently redeveloped by Henry Leonard and Nelson Goodman as the *Calculus of Individuals* (1940). Where set theory builds a universe out of membership, mereology builds one out of parthood. The result is a first-order theory with a strikingly different logical profile — one in which, as we will see later in this chapter, the strongest classical system is actually *decidable*.

## The Primitive: Parthood

We work in a first-order language $\mathcal{L}_P$ with identity and a single binary predicate $P(x,y)$, read "$x$ is a part of $y$." Parthood is *primitive*: it is not defined in terms of anything else but implicitly characterized by axioms. Note that in this usage every object counts as an (improper) part of itself — "part" in the inclusive sense, like $\subseteq$ rather than $\subset$.

**Ground Mereology** $\mathbf{M}$ consists of three axioms:

$$
\begin{aligned}
&\textbf{(M1) Reflexivity:} && \forall x\, P(x,x)\\
&\textbf{(M2) Antisymmetry:} && \forall x \forall y\,\bigl(P(x,y) \land P(y,x) \to x = y\bigr)\\
&\textbf{(M3) Transitivity:} && \forall x \forall y \forall z\,\bigl(P(x,y) \land P(y,z) \to P(x,z)\bigr)
\end{aligned}
$$

M1–M3 say exactly that parthood is a **partial order** on the domain. Every partially ordered set is therefore a model of $\mathbf{M}$: the subsets of a set under $\subseteq$, the positive integers under divisibility, the subregions of a spatial region under inclusion. Ground mereology is thus extremely weak — it is, in effect, just the first-order theory of posets — and the systems of the next section will strengthen it considerably.

## Defined Notions

All further mereological vocabulary is introduced by explicit first-order definitions over $P$.

**Definition (Proper Part).** $PP(x,y) \equiv P(x,y) \land x \neq y$.

An equivalent definition given M2 is $P(x,y) \land \neg P(y,x)$; in non-antisymmetric settings the two come apart, but in $\mathbf{M}$ they coincide. $PP$ is irreflexive, asymmetric, and transitive — a strict partial order.

**Definition (Overlap).** $O(x,y) \equiv \exists z\,\bigl(P(z,x) \land P(z,y)\bigr)$.

Two things overlap when they share a common part. Overlap is reflexive (by M1, $x$ itself witnesses $O(x,x)$) and symmetric, but **not** transitive: the left half of a rod overlaps its middle third, the middle third overlaps the right half, yet the two halves are disjoint.

**Definition (Disjointness).** $D(x,y) \equiv \neg O(x,y)$.

**Definition (Underlap).** $U(x,y) \equiv \exists z\,\bigl(P(x,z) \land P(y,z)\bigr)$.

Underlap is the dual of overlap: $x$ and $y$ underlap when both are parts of some common whole. In systems with a universal object (Section 2), underlap holds universally and becomes trivial; in weaker systems it carries real information.

Also useful: $PP(x,y) \to O(x,y)$ and $P(x,y) \to \forall z(O(z,x) \to O(z,y))$ are theorems of $\mathbf{M}$ — the second by transitivity, since a common part of $z$ and $x$ is a part of $y$.

## Worked Examples

**Bodies.** Let $h$ = my left hand, $a$ = my left arm, $b$ = my body. Then $P(h,a)$ and $h \neq a$, so $PP(h,a)$; likewise $PP(a,b)$; by M3, $P(h,b)$, and since $h \neq b$, $PP(h,b)$. My left hand and my left arm overlap — indeed $h$ itself is the witness, since $P(h,h)$ and $P(h,a)$. My left hand and my right hand are disjoint (no common part), yet they underlap: $b$ contains both.

**Inscriptions.** Consider a particular written token of the word "cat." Its letter tokens are parts: the 'c' is a proper part of "cat." The initial segment "ca" and the final segment "at" overlap — the 'a' token is their common part — while 'c' and 't' are disjoint but underlap (in the whole word). One caution: this is a mereology of *tokens*, not *types*. The letter type 'a' occurs three times in a token of "banana"; it is the three distinct 'a'-*tokens* that are parts of the inscription, and a mereology of expression types requires more delicate treatment (an occurrence relation, not bare parthood).

## Is Parthood Really Transitive?

M3 has been challenged. The handle is part of the door; the door is part of the house; so, by transitivity, the handle is part of the house. Yet if asked to list the parts of a house we say: walls, roof, doors — not handles. "The handle is part of the house" can sound wrong, and similar cases abound (a soldier is part of a platoon, the platoon part of a battalion; is the soldier part of the battalion?).

The standard reply distinguishes the theory's relation from its natural-language counterparts. Ordinary "part of" frequently expresses a *restricted* relation — direct functional component of, organizational unit of — and restrictions of a transitive relation need not be transitive. Formally: define

$$P_\phi(x,y) \equiv P(x,y) \land \phi(x,y),$$

where $\phi$ encodes the restriction (say, "$x$ makes a direct functional contribution to $y$"). From $P_\phi(x,y)$ and $P_\phi(y,z)$ nothing follows about $\phi(x,z)$, so $P_\phi$ may fail to be transitive even though $P$ is. The objection targets the restricted relations; the unrestricted relation — bare mereological inclusion, on which the matter and region of the handle are included in those of the house — remains compellingly transitive. The lesson is methodological and recurs throughout applied logic: axioms govern the regimented relation, not every idiomatic use of the word.

## Atoms and Gunk

**Definition (Atom).** $\mathrm{Atom}(x) \equiv \neg\exists y\, PP(y,x)$.

An atom is an object with no proper parts — mereologically indivisible (which need not mean physically indivisible). Two mutually exclusive global hypotheses can be added to any mereological theory:

$$
\begin{aligned}
&\textbf{(Atomicity)} && \forall x\, \exists y\,\bigl(\mathrm{Atom}(y) \land P(y,x)\bigr)\\
&\textbf{(Atomlessness)} && \forall x\, \exists y\, PP(y,x)
\end{aligned}
$$

Atomicity says everything is composed of atoms; atomlessness says everything divides forever. An atomless object — every part of which has proper parts, all the way down — is called **gunk** (David Lewis's term).

Both hypotheses are consistent. Any finite poset with a bottom layer models atomicity. For gunk, take the *regular open* subsets of $\mathbb{R}$ (nonempty ones), ordered by inclusion: every nonempty regular open set properly includes another, so the model has no atoms — and it in fact satisfies all of classical mereology. Since both extensions are consistent with the strongest classical system (GEM, next section), that system neither proves nor refutes atomicity: the question is *independent*, and whether the physical world is atomic or gunky cannot be settled by mereological logic alone.

One last observation about how weak $\mathbf{M}$ is: it has models in which some $y$ has *exactly one* proper part $x$ — a "whole" exceeding its sole part with no remainder anywhere. Most mereologists find such models incoherent: if $x$ is all there is to $y$, what makes $y$ bigger? Ruling them out is the job of the *supplementation* principles, to which we now turn.

## Exercises
See [problems/ch16_mereology/](../../../problems/ch16_mereology/)
