# 1.1 Naive Set Theory and Russell's Paradox

## The Naive Idea

Georg Cantor developed set theory in the 1870s–1890s with a liberating simplicity: a *set* is any collection of objects sharing a common property. You want the set of all even numbers? Fine: $\{n \in \mathbb{Z} \mid n \text{ is even}\}$. The set of all continuous functions? Sure. The set of all mathematical objects with a certain property? Go ahead.

This principle — that for any property $P$, there is a set $\{x \mid P(x)\}$ of all things satisfying $P$ — is called *unrestricted comprehension*. It is seductively simple and matches mathematical intuition.

Unfortunately, it is inconsistent.

## Russell's Paradox (1901)

**The Construction.** Let $R$ be the "set" of all sets that are not members of themselves:
$$R = \{x \mid x \notin x\}$$

This is a perfectly well-formed comprehension: the property $P(x) = (x \notin x)$ is definable in the language of sets.

**The Contradiction.** Is $R \in R$?

Suppose $R \in R$. Then $R$ satisfies the defining property of $R$, which is $R \notin R$. Contradiction.

Suppose $R \notin R$. Then $R$ does not satisfy the defining property, but the defining property says $R \notin R$... wait: $R \notin R$ means $R$ *does* satisfy the property "$x \notin x$", so $R$ should be in $R$. Contradiction.

Either possibility leads to a contradiction. Therefore, no consistent theory can contain a set $R$ defined by unrestricted comprehension from the property $x \notin x$. Unrestricted comprehension is *inconsistent*.

**Why this is serious.** In logic, a system that proves a contradiction proves *everything* (the principle of *ex contradictione quodlibet*: from a contradiction, any conclusion follows). A mathematical foundation that leads to contradiction is useless.

Russell's paradox showed that Cantor's intuitive set theory — and with it, Frege's formal system for logic (the *Grundgesetze der Arithmetik*) — was inconsistent. Frege received Russell's letter with the paradox just as the second volume of his Grundgesetze was going to press. He added a famous postscript: "Hardly anything more unwelcome can befall a scientific writer than to have one of the foundations of his edifice shaken after the work is finished. This is the position to which I have been placed by a letter from Mr Bertrand Russell..."

## Why Does $R$ Lead to Trouble?

The paradox exploits *self-reference*: $R$ is defined in terms of a property that $R$ itself might or might not satisfy. Self-reference is pervasive and often harmless (self-referential programs, self-describing sentences), but in the context of set comprehension, it produces a genuine contradiction.

The resolution must prevent this kind of self-reference. Two main approaches:

1. **Restriction:** Only allow comprehension within an *existing* set (restricted comprehension). You can form $\{x \in A \mid P(x)\}$ only when $A$ already exists. This prevents $R$ because forming $\{x \mid x \notin x\}$ would require a set of *all* sets, which is not given.

2. **Type stratification:** Assign *types* to objects and only allow $x \in y$ when $y$ has a higher type than $x$. Then $x \in x$ is a type error, and $R$'s defining property is ill-typed. This is Russell's own *type theory* (1903).

ZFC follows the first approach. Type theory (naturally) follows the second.

## Other Paradoxes

Russell's paradox was not alone. Several related paradoxes appeared around the same time, all exploiting the same unbounded self-reference.

**Burali-Forti paradox (1897).** An *ordinal number* is a canonical representative of a well-ordering. The class of all ordinals is itself well-ordered by the ordering it represents — so it would have an ordinal, call it $\Omega$. But $\Omega$ would be an ordinal larger than all ordinals — contradicting its own definition.

*Resolution:* There is no "set of all ordinals." The ordinals form a *proper class* in ZFC, not a set.

**Cantor's paradox.** The "set of all sets" $V$ would have to have a power set $\mathcal{P}(V)$ larger than $V$ (by Cantor's theorem). But $V$ contains all sets, including all elements of $\mathcal{P}(V)$, so $|\mathcal{P}(V)| \leq |V|$. Contradiction.

*Resolution:* There is no set of all sets. $V$ is again a proper class.

**Grelling-Nelson paradox (1908).** Call an adjective *autological* if it applies to itself ("short" is short, "English" is English) and *heterological* if it does not ("long" is not long, "French" is not French). Is "heterological" heterological?

This is a linguistic version of Russell's paradox, showing that the paradox is not specific to set theory but arises from any system with unrestricted self-reference.

## The Lesson

The paradoxes teach a simple but deep lesson: **you cannot have everything**. In particular:

- You cannot have a set of all sets.
- You cannot have a set defined by an arbitrary property of arbitrary objects.
- You must be careful about self-reference in definitions.

The ZFC axioms are a precise specification of *which* sets exist, designed to allow all the sets mathematicians actually need while blocking the problematic ones. The key technique is *restriction*: comprehension is only allowed within an existing set.

## A Note on Proper Classes

In ZFC, some mathematical "collections" are too big to be sets:
- The collection of all sets
- The collection of all ordinals
- The collection of all groups

These are called *proper classes*. They are not sets within ZFC (they don't "exist" as objects), but they can be discussed as syntactic abbreviations for formulas.

In *von Neumann-Bernays-Gödel* set theory (NBG), proper classes are first-class objects with their own rules. In *Morse-Kelley set theory* (MK), the rules are even more generous. For this curriculum, we stick with ZFC and treat proper classes informally.

In type theory, the analog of "too big to be a set" is *universe stratification*: the type of all types in $\mathsf{Type}_0$ is $\mathsf{Type}_1$, and there is no single type containing all types. This hierarchy prevents Russell-like paradoxes at the type-theoretic level (Girard's paradox shows that an impredicative universe $\mathsf{Type} : \mathsf{Type}$ is inconsistent).
