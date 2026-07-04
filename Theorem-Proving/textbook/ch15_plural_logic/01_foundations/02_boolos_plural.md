# Boolos's Plural Quantification

George Boolos launched plural logic as a research programme in two papers: *To Be Is to Be a Value of a Variable (or to Be Some Values of Some Variables)* (1984) and *Nominalist Platonism* (1985). His target was a philosophical puzzle about **second-order logic** and its apparent commitment to Platonic abstract objects. Second-order logic quantifies over the *subsets* (and relations) of the domain; on the standard reading those subsets are set-like entities, so second-order quantification looks like disguised set theory — Quine's verdict that it is "set theory in sheep's clothing." Boolos's response was that a large and important part of second-order quantification is really *plural* quantification over ordinary individuals, which commits us to no sets at all.

## Formalizing the Geach–Kaplan Sentence

The test case is "some critics admire only one another," which the previous section showed to be first-order-inexpressible without a set variable. In the plural language it is written with no set variable and no abstract object:
$$\exists xx\,\Bigl[\underbrace{\exists x\,(x \prec xx)}_{\text{nonempty}} \;\land\; \underbrace{\forall x\,(x \prec xx \to \mathrm{Critic}(x))}_{\text{all critics}} \;\land\; \underbrace{\forall x\,\forall y\,\bigl(x \prec xx \land \mathrm{Admires}(x,y) \to y \prec xx \land x \neq y\bigr)}_{\text{admire only others of them}}\Bigr].$$
In words: there are some things $xx$ such that at least one of them exists, each of them is a critic, and whenever one of them admires someone, that someone is again one of them and distinct from the admirer. Every quantifier ranges over the original domain of people; the plural quantifier $\exists xx$ ranges over *pluralities of people*. No set, class, or property appears. This is Boolos's paradigm: what looked like ineliminable second-order (set) quantification is captured by plural quantification over the very individuals the sentence was always about.

## The Language PFO$^+$

**Plural first-order logic** extends the first-order language $\mathcal{L}$ of Chapter 3. To the singular variables, function and predicate symbols, connectives, and singular quantifiers we add:

- **Plural variables** $xx, yy, zz, \dots$
- The **logical predicate** $x \prec xx$ (a singular term on the left, a plural variable on the right), read "$x$ is one of the $xx$";
- **Plural quantifiers** $\exists xx$ and $\forall xx$.

**Formation rules.** If $t$ is a singular term and $xx$ a plural variable, $t \prec xx$ is an atomic formula. If $\phi$ is a formula, so are $\exists xx\,\phi$ and $\forall xx\,\phi$ (binding $xx$). The system with plural *predicates* taking plural terms as arguments — needed for collective predication like $W(rr)$ — is called PFO$^+$; the fragment with $\prec$ as the only device relating the two sorts is **PFO**.

A grammatical fact does real logical work here. The predicate $\prec$ is **singular on the left, plural on the right**. Consequently "$xx \prec xx$" is *not well-formed*: one cannot ask whether a plurality is one of itself. As we will see in the [proof-theory section](../02_formal_system/01_pfo_and_proof_theory.md), this single type restriction is what blocks the plural analogue of Russell's paradox at the level of syntax, before any axiom is even stated.

## The Ontological-Innocence Thesis

Boolos's central philosophical claim:

> **Ontological Innocence (Boolos 1984).** Plural quantification incurs no ontological commitment beyond the individuals in the range of the singular quantifiers. To say "there are some sets that are all and only the non-self-membered sets" is to commit oneself to those sets, not to any further object — no *set of* them, no class, no plurality-as-entity.

The argument is a contrast in what the two quantifiers posit. Singular $\exists x\,\phi(x)$ says an *object* satisfies $\phi$; its witness is a member of the domain. Plural $\exists xx\,\phi(xx)$ says there are *some things* that jointly satisfy $\phi$; its "witness" is not one more object but several of the objects already there, referred to together. On Quine's own criterion — to be is to be the value of a (bound) variable — the values of a plural variable are just individuals, taken several at a time, so being *some values of* a plural variable is no additional way of being. Hence the paper's title. The intended contrast is with the set-theoretic reading, on which $\exists X$ ranges over set-objects and so does inflate the ontology.

Innocence is what makes plurals philosophically powerful, and it is also the thesis most contested by later writers (Resnik, Parsons, Linnebo); the [expressive-power section](../02_formal_system/02_expressive_power.md) weighs the objections. But its intended payoff is already visible: *if* plural quantification is innocent, then the expressive strength it shares with second-order logic is bought for free.

## Innocence Meets Power: The Link to Neo-Logicism

Boolos coupled innocence with a striking expressive claim, proved in Section 4: **monadic second-order logic is interpretable in plural logic**, by translating each monadic second-order variable $X$ to a plural variable $xx$ and each atom $X(t)$ to $t \prec xx$. Second-order quantification over subsets becomes plural quantification over individuals. If that translation preserves meaning *and* plurals are innocent, then a debate at the foundations of logic is reframed: second-order logic may be **genuine logic** after all, not concealed set theory, because it can be read as committing us only to the individuals we already accept.

This matters for **neo-logicism** — the program of Frege, revived by Crispin Wright and Bob Hale, of deriving arithmetic from logic plus definitions. Frege's own system collapsed into Russell's paradox through its commitment to extensions (courses-of-values, essentially sets). If the second-order logic that neo-logicism needs can be given an ontologically innocent plural reading, then arithmetic might reduce to *logic proper*, vindicating Frege's ambition without the paradox-breeding abstract objects. Whether plural comprehension is itself "pure logic" or a substantive existence assumption in disguise is the crux of that debate — and the question we take up once the formal system is on the table.

## Exercises
See [problems/ch15_plural_logic/](../../../problems/ch15_plural_logic/)
