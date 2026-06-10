# First-Order Structures

## What is a Structure?

A **first-order structure** (or **model** or **interpretation**) gives mathematical meaning to the symbols of a formal language.

A **signature** $\Sigma = (C, F, R, \text{ar})$ specifies:
- $C$: a set of constant symbols (e.g., $0$, $1$, $\pi$)
- $F$: a set of function symbols (e.g., $+$, $\times$, $\sin$)
- $R$: a set of relation symbols (e.g., $<$, $\in$, Prime)
- $\text{ar}$: an arity function assigning each symbol a number of arguments

A **$\Sigma$-structure** $\mathcal{M}$ consists of:
- A non-empty set $M$ (the **domain** or **universe**)
- For each constant symbol $c \in C$: an element $c^\mathcal{M} \in M$
- For each function symbol $f \in F$ of arity $n$: a function $f^\mathcal{M} : M^n \to M$
- For each relation symbol $R \in \text{Rel}$ of arity $n$: a relation $R^\mathcal{M} \subseteq M^n$

**Example**: The signature of ordered fields has constants $\{0, 1\}$, functions $\{+, \times, -, /\}$, and relations $\{<, =\}$. The real numbers $\mathbb{R}$ with their standard interpretations form a $\Sigma$-structure. So does $\mathbb{Q}$.

## Term Evaluation

Given a structure $\mathcal{M}$ and a **variable assignment** $s : \text{Var} \to M$, we evaluate terms recursively:
- $s(x)^\mathcal{M} = s(x)$ for variables
- $c^\mathcal{M}$ for constant symbols
- $f^\mathcal{M}(t_1^\mathcal{M}, \ldots, t_n^\mathcal{M})$ for function applications

**Example**: In the standard structure for arithmetic with $s(x) = 3$, $s(y) = 5$: $(x + y)^\mathcal{M} = 3 + 5 = 8$.

## Satisfaction

A formula $\varphi$ is **satisfied** in $\mathcal{M}$ by assignment $s$ (written $\mathcal{M}, s \models \varphi$):
- $\mathcal{M}, s \models R(t_1, \ldots, t_n)$ iff $(t_1^\mathcal{M}, \ldots, t_n^\mathcal{M}) \in R^\mathcal{M}$
- $\mathcal{M}, s \models t_1 = t_2$ iff $t_1^\mathcal{M} = t_2^\mathcal{M}$
- Propositional connectives: as before
- $\mathcal{M}, s \models \forall x\, \varphi$ iff for all $m \in M$: $\mathcal{M}, s[x \mapsto m] \models \varphi$
- $\mathcal{M}, s \models \exists x\, \varphi$ iff for some $m \in M$: $\mathcal{M}, s[x \mapsto m] \models \varphi$

A **sentence** (formula with no free variables) is true or false in a structure independently of any assignment.

## Examples of Structures

The same sentence can be true in one structure and false in another:

| Sentence | $(\mathbb{N}, +, 0)$ | $(\mathbb{Z}, +, 0)$ | $(\mathbb{Q}, +, 0)$ |
|----------|---------------------|---------------------|---------------------|
| $\forall x\, \exists y\, x + y = 0$ | False ($\mathbb{N}$ has no negatives) | True | True |
| $\forall x\, \forall y\, x + y = y + x$ | True | True | True |
| $\exists x\, (x \neq 0 \wedge x + x = 0)$ | False | False | False |

## In Lean 4

```lean
-- A structure in Lean corresponds to a typeclass or structure type
-- with an interpretation for each symbol

structure OrderedGroup where
  carrier : Type
  mul : carrier → carrier → carrier
  one : carrier
  inv : carrier → carrier
  le  : carrier → carrier → Prop
  -- ... axioms follow

-- The integers are a model:
instance : OrderedGroup where
  carrier := ℤ
  mul := (· + ·)
  one := 0
  inv := (- ·)
  le := (· ≤ ·)
```

## Exercises
See [problems/ch09_model_theory/](../../../problems/ch09_model_theory/)
