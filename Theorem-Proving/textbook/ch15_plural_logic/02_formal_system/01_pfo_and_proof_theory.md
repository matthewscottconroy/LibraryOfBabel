# The Formal System and Its Proof Theory

We now set out plural first-order logic as a deductive system in its own right — syntax, axioms, and inference rules — and prove theorems in it. The design goal is a logic that stands to the plural quantifiers exactly as Chapter 3's natural deduction stands to the singular ones, plus one distinctive axiom schema, **plural comprehension**, that says which pluralities exist.

## Syntax, Recalled and Fixed

The language is that of [Section 2](../01_foundations/02_boolos_plural.md): singular variables $x, y, z$; plural variables $xx, yy, zz$; the inclusion predicate $t \prec xx$ (singular term left, plural variable right); singular and plural quantifiers; the usual connectives. A **plurality** is what a plural variable denotes: one or more individuals, taken together. Two typing conventions govern everything below.

1. **Sortal discipline.** $\prec$ takes a singular term on the left and a plural term on the right. Neither $xx \prec x$ nor $xx \prec yy$ is a formula. In particular $xx \prec xx$ is ungrammatical — there is no question whether a plurality is one of itself.
2. **Nonemptiness.** By convention a plurality has at least one member: $\forall xx\,\exists x\,(x \prec xx)$ is a logical truth. (Oliver and Smiley's system admits an empty plural term "zilch"; we follow the mainstream Boolosian convention that there is no empty plurality, and flag where it matters.)

## Axioms and Rules

**I. First-order base.** All axioms and rules of classical first-order logic with identity (Chapter 4) for the singular apparatus.

**II. Plural quantifier rules.** The plural quantifiers obey introduction and elimination rules formally identical to the singular ones, with plural variables and terms in place of singular. Writing $\Gamma \vdash \phi$ for derivability:

$$
\frac{\Gamma \vdash \forall xx\,\phi(xx)}{\Gamma \vdash \phi(yy)}\ (\forall\text{-E})
\qquad
\frac{\Gamma \vdash \phi(yy) \quad yy \text{ not free in } \Gamma}{\Gamma \vdash \forall xx\,\phi(xx)}\ (\forall\text{-I})
$$
$$
\frac{\Gamma \vdash \phi(yy)}{\Gamma \vdash \exists xx\,\phi(xx)}\ (\exists\text{-I})
\qquad
\frac{\Gamma \vdash \exists xx\,\phi(xx) \quad \Gamma,\phi(yy) \vdash \psi \quad yy \text{ not free in } \Gamma, \psi}{\Gamma \vdash \psi}\ (\exists\text{-E})
$$

**III. Indiscernibility of coextensive pluralities.** Pluralities with the same members are logically indistinguishable:
$$\forall x\,(x \prec xx \leftrightarrow x \prec yy) \;\to\; \bigl(\phi(xx) \leftrightarrow \phi(yy)\bigr) \qquad (\text{PL-Ext})$$
for every formula $\phi$. This is the plural analogue of the substitutivity of identity, not an axiom of extensionality *for objects*: it does not say a plurality *is* an object determined by its members, only that the logic cannot tell coextensive pluralities apart.

**IV. Plural comprehension.** For every formula $\phi(x)$ in which $xx$ is not free:
$$\exists x\,\phi(x) \;\to\; \exists xx\,\forall x\,\bigl(x \prec xx \leftrightarrow \phi(x)\bigr). \qquad (\text{P-Comp})$$
Whenever *some* object satisfies $\phi$, there are some things that are exactly the $\phi$-satisfiers. The antecedent $\exists x\,\phi(x)$ enforces nonemptiness; the proviso "$xx$ not free in $\phi$" is essential (it blocks the self-referential formula $\neg(x \prec xx)$ from being used to define the very $xx$ it constrains). P-Comp is the plural counterpart of Separation, but note what it does *not* do: it introduces no object, only asserts the existence of a plurality among the objects already present. This is precisely where the innocence thesis is cashed out — and precisely where critics locate a hidden existence assumption ([Section 4](02_expressive_power.md)).

## Worked Derivations

**Derivation 1 (Singular terms embed as pluralities).** *For any $a$ there are some things whose sole member is $a$*: $\vdash \exists xx\,\forall x\,(x \prec xx \leftrightarrow x = a)$.

*Proof.* Take $\phi(x) := (x = a)$. Then $a = a$ by reflexivity of identity, so $\exists x\,(x = a)$ by $\exists$-I. P-Comp for this $\phi$ gives $\exists x(x=a) \to \exists xx\,\forall x(x \prec xx \leftrightarrow x = a)$; modus ponens delivers the conclusion. $\square$

Write these things $[a]$. By PL-Ext, $[a]$ is unique up to indiscernibility, and $a \prec [a]$. So every object determines a one-membered plurality, and the singular world sits inside the plural world.

**Derivation 2 (Pluralities are closed under union).** *For any $xx$ and $yy$ there are some things that are exactly the things among the $xx$ or the $yy$*: $\vdash \exists zz\,\forall x\,\bigl(x \prec zz \leftrightarrow (x \prec xx \lor x \prec yy)\bigr)$.

*Proof.* Fix $xx, yy$ and put $\phi(x) := (x \prec xx \lor x \prec yy)$. By nonemptiness $\exists x\,(x \prec xx)$, and any such $x$ satisfies $\phi$, so $\exists x\,\phi(x)$. Because $\phi$ does not mention $zz$, P-Comp applies and yields the required $zz$ by modus ponens. $\square$

The analogous derivations give *intersection* (when the $xx$ and $yy$ share a member, so the antecedent holds) and *sub-plurality* comprehension. Notice we have reconstructed the Boolean operations on collections **without ever forming a set** — closure facts about pluralities, proved from P-Comp alone.

**Derivation 3 (The universal plurality).** *If anything exists, there are some things that include everything*: $\exists x\,(x = x) \vdash \exists xx\,\forall x\,(x \prec xx)$.

*Proof.* With $\phi(x) := (x = x)$, P-Comp gives $\exists xx\,\forall x(x \prec xx \leftrightarrow x = x)$; and $\forall x(x = x)$ is logically valid, so $\forall x(x \prec xx)$. $\square$

There is a universal plurality — *all* the objects, plurally — even though (Chapter 6) there is no universal *set*. The plural "collection of everything" is unproblematic precisely because it is no object and so cannot be asked to contain itself.

**Derivation 4 (Russell's paradox is ungrammatical, not merely false).** In set theory the Russell set is defined by $R = \{x : x \notin x\}$ and $R \in R \leftrightarrow R \notin R$ yields contradiction. The plural imitation would need a plurality of all non-self-including pluralities — a formula like $\neg(xx \prec xx)$. But $xx \prec xx$ **is not well-formed**: $\prec$ demands a singular left argument. There is no formula to run the diagonal argument on, so P-Comp, unlike naïve set comprehension, is consistent. The type discipline of the language does what type theory (Chapter 11) and the cumulative hierarchy do for sets, but earlier — at the level of syntax.

## Semantics

A **plural structure** for $\mathcal{L}$ is a first-order structure $\mathfrak{A}$ with domain $A$, together with the stipulation that plural variables are assigned **nonempty subsets** $S \subseteq A$ and that $\mathfrak{A} \models a \prec xx$ iff $a \in S$, where $S$ is the assignment to $xx$. Under this semantics the rules I–IV are **sound**, and P-Comp holds because every nonempty definable $S = \{a : \mathfrak{A} \models \phi(a)\}$ is available as a value.

Two clarifications keep the philosophy honest. First, using subsets of $A$ in the *metalanguage* to model plural variables does **not** refute innocence: the object language commits to no sets, and one can — as Boolos, McKay, and Rayo do — give the metatheory itself plurally, interpreting plural variables by plural quantification rather than by sets. The set-valued semantics is a convenient model, not the intended ontology. Second, when the intended domain is *absolutely everything* (including all sets), there is no set of all objects to draw subsets from, and a set-based semantics fails on its own terms; the plural metalanguage is then not a convenience but a necessity — a theme taken up in [Section 5](../03_developments/01_higher_order_and_foundations.md).

With soundness in hand and the deductive system fixed, we can measure what plural logic can *say*. The next section proves it matches monadic second-order logic and, with a pairing device, full second-order logic — putting categorical arithmetic within reach of a logic that claims to add no objects.

## Exercises
See [problems/ch15_plural_logic/](../../../problems/ch15_plural_logic/)
