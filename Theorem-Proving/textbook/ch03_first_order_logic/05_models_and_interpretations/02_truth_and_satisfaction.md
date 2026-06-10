# Truth and Satisfaction in FOL

## The Semantics of Quantifiers

The central semantic question in FOL: when is a formula $\forall x\, \varphi(x)$ true?

In the structure $\mathcal{M}$ with domain $M$: $\mathcal{M} \models \forall x\, \varphi(x)$ iff for *every* element $m \in M$, the formula $\varphi(m)$ holds in $\mathcal{M}$. The universal quantifier ranges over the entire domain.

Similarly: $\mathcal{M} \models \exists x\, \varphi(x)$ iff *some* $m \in M$ satisfies $\varphi(m)$.

This is **Tarski's truth definition** (1936), which gave the first rigorous semantics for formal languages and resolved the Liar paradox by distinguishing object language from metalanguage.

## The Tarski Hierarchy

Tarski noticed that truth predicates create hierarchical difficulties. The sentence "This sentence is false" (the Liar) cannot be consistently assigned a truth value. His solution: truth is always defined *from outside* the system.

The **T-schema**: For each sentence $\varphi$ of the object language:
$$T\ulcorner\varphi\urcorner \iff \varphi$$

"'Snow is white' is true iff snow is white."

But this schema cannot be stated within the same language as $\varphi$ — truth for language $\mathcal{L}$ must be defined in a richer metalanguage $\mathcal{L}'$. This creates the **Tarski hierarchy**: $\mathcal{L}_0 \subset \mathcal{L}_1 \subset \mathcal{L}_2 \subset \ldots$ where $\mathcal{L}_{n+1}$ can define truth for $\mathcal{L}_n$.

## Logical Consequence and Validity

$\Gamma \models \varphi$ (logical consequence): every structure satisfying all sentences in $\Gamma$ also satisfies $\varphi$.

$\models \varphi$ (validity/tautology): $\varphi$ is true in every structure.

**Important FOL validities**:
- $\forall x\, \varphi(x) \to \varphi(t)$ (universal instantiation)
- $\varphi(t) \to \exists x\, \varphi(x)$ (existential generalization)
- $\neg\forall x\, \varphi(x) \leftrightarrow \exists x\, \neg\varphi(x)$ (quantifier duality)

## Exercises
See [problems/ch03_first_order_logic/](../../../problems/ch03_first_order_logic/)
