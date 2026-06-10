# Kripke Semantics for Modal Logic

> "Possible worlds are not discovered; they are *stipulated*. To say that something might have been otherwise is to describe a counterfactual way things might have been."
> — Saul Kripke

## The Brilliant Move: Make Worlds Explicit

Before Kripke, modal logic had a perfectly good proof system (various axiom systems like S4 and S5) but no clear *semantics*. What does it mean for a modal formula to be "true"? Truth tables handle classical logic, but $\square\varphi$ is not a truth-functional operation: knowing the truth value of $\varphi$ in the actual world does not determine whether $\square\varphi$ is true.

Saul Kripke's insight (1963, published in a landmark series of papers starting when he was a teenager): give explicit mathematical content to "possible worlds." A modal formula's truth is not just about the actual world — it is relative to a *structure* consisting of multiple worlds and an *accessibility relation* between them.

This was not metaphysics; it was mathematics. "Possible worlds" in the Kripke sense are just points in a set — they need not be philosophically loaded. The semantics is clean, compositional, and immediately useful.

## Kripke Frames and Models

A **Kripke frame** is a pair $\mathcal{F} = (W, R)$ where:
- $W$ is a non-empty set of **possible worlds** (or **states**)
- $R \subseteq W \times W$ is the **accessibility relation**: $wRv$ means "world $v$ is accessible from world $w$"

A **Kripke model** adds a valuation:
$$\mathcal{M} = (W, R, V)$$

where $V : \text{Atoms} \to \mathcal{P}(W)$ assigns to each propositional atom $p$ the set of worlds at which $p$ is true.

## Satisfaction: Truth at a World

We define $\mathcal{M}, w \models \varphi$ (formula $\varphi$ is true at world $w$ in model $\mathcal{M}$) inductively:

- $\mathcal{M}, w \models p \iff w \in V(p)$ (atom $p$ is true at $w$ if $w$ is in $p$'s valuation)
- $\mathcal{M}, w \models \neg\varphi \iff \mathcal{M}, w \not\models \varphi$
- $\mathcal{M}, w \models \varphi \wedge \psi \iff \mathcal{M}, w \models \varphi$ and $\mathcal{M}, w \models \psi$
- $\mathcal{M}, w \models \varphi \to \psi \iff \mathcal{M}, w \not\models \varphi$ or $\mathcal{M}, w \models \psi$
- $\mathcal{M}, w \models \square\varphi \iff$ for all $v$ with $wRv$: $\mathcal{M}, v \models \varphi$
- $\mathcal{M}, w \models \Diamond\varphi \iff$ there exists $v$ with $wRv$ such that $\mathcal{M}, v \models \varphi$

The crucial clauses: **$\square\varphi$ is true at $w$ iff $\varphi$ is true at every world accessible from $w$**. **$\Diamond\varphi$ is true at $w$ iff $\varphi$ is true at some world accessible from $w$**.

This immediately makes sense of the operator duality: $\mathcal{M}, w \models \square\varphi \iff \mathcal{M}, w \not\models \Diamond\neg\varphi$.

## A Worked Example

Consider the model:

```
Worlds: W = {w₁, w₂, w₃}
Accessibility: w₁Rw₂, w₁Rw₃, w₂Rw₂
Valuation: V(p) = {w₂}, V(q) = {w₁, w₃}
```

Visualized:
```
     w₁ ──────► w₂ ──┐
      │           │◄──┘
      └──────► w₃
```

Truth values:
- $\mathcal{M}, w₂ \models p$ (since $w₂ \in V(p)$)
- $\mathcal{M}, w₁ \not\models p$ (since $w₁ \notin V(p)$)
- $\mathcal{M}, w₁ \models \Diamond p$: is there a world accessible from $w₁$ where $p$ holds? Yes — $w₂$. ✓
- $\mathcal{M}, w₁ \not\models \square p$: is $p$ true at all worlds accessible from $w₁$? No — $w₃ \notin V(p)$. ✗
- $\mathcal{M}, w₂ \models \square p$: the only world accessible from $w₂$ is $w₂$ itself, and $w₂ \models p$. ✓
- $\mathcal{M}, w₁ \models \Diamond q$: $w₃$ is accessible from $w₁$ and $w₃ \models q$. ✓

## Validity and Soundness

A formula $\varphi$ is:
- **Valid at $w$ in $\mathcal{M}$**: $\mathcal{M}, w \models \varphi$
- **Valid in $\mathcal{M}$**: $\mathcal{M} \models \varphi$ iff $\mathcal{M}, w \models \varphi$ for all $w \in W$
- **Valid on frame $\mathcal{F}$**: valid in every model based on $\mathcal{F}$
- **Valid in a class of frames**: valid on every frame in the class

A modal logic $\mathbf{L}$ is **complete** with respect to a class $\mathbf{C}$ of frames if: $\mathbf{L} \vdash \varphi$ iff $\varphi$ is valid on every frame in $\mathbf{C}$.

## The Frame Correspondence Results

The power of Kripke semantics is the **frame correspondence**: each modal axiom corresponds to a first-order property of the accessibility relation.

| Axiom | Schema | Frame property |
|-------|--------|----------------|
| T | $\square\varphi \to \varphi$ | Reflexivity: $\forall w,\; wRw$ |
| 4 | $\square\varphi \to \square\square\varphi$ | Transitivity: $wRv \wedge vRu \implies wRu$ |
| 5 | $\Diamond\varphi \to \square\Diamond\varphi$ | Euclidean: $wRv \wedge wRu \implies vRu$ |
| B | $\varphi \to \square\Diamond\varphi$ | Symmetry: $wRv \implies vRw$ |
| D | $\square\varphi \to \Diamond\varphi$ | Seriality: $\forall w\, \exists v,\; wRv$ |

**Proof that T corresponds to reflexivity**:

($\Rightarrow$) If $R$ is reflexive, then for any model $\mathcal{M}$ with $\mathcal{M}, w \models \square\varphi$, we have $\mathcal{M}, v \models \varphi$ for all $v$ with $wRv$. Since $wRw$ (reflexivity), $\mathcal{M}, w \models \varphi$. So $\square\varphi \to \varphi$ is valid on reflexive frames.

($\Leftarrow$) If $R$ is not reflexive, there is a $w$ with $\neg(wRw)$. Build a model where $p$ is false at $w$ but true at all other worlds accessible from $w$. Then $\square p$ is true at $w$ (since $p$ is true everywhere accessible) but $p$ is false at $w$. So T fails.

This connection between modal axioms and relational frame properties is elegant and deeply useful — it means the choice of modal logic (which axioms to use) corresponds to the choice of *what kind of accessibility structure* you are working with.

## S4 and S5 in Detail

**S4** = K + T + 4, valid on reflexive transitive frames.

In S4, the accessibility relation is a **preorder** (reflexive and transitive). S4 is used for:
- **Provability logic** (with modification): the Gödel translation embeds intuitionistic logic into S4 — $\varphi$ is intuitionistically provable iff $\square\varphi$ (its "necessitation") is S4-valid. This gives a precise connection between classical modal logic and constructive reasoning.
- **Topology**: Interior/closure operators in topology satisfy S4-like axioms. The Alexandroff correspondence makes this precise.

**S5** = K + T + 4 + 5 = K + T + B + 4, valid on equivalence relation frames.

In S5, all worlds in the same equivalence class are "co-possible." The modalities behave cleanly: $\square\varphi$ is either always true or always false relative to an equivalence class. S5 is the standard system for:
- **Metaphysical necessity** (Kripke, Lewis)
- **Logical validity** as a modal notion
- **Epistemic logic** when agents have perfect introspective access

## Python Model Checker for Propositional Modal Logic

```python
from typing import Set, Dict, FrozenSet

class KripkeModel:
    def __init__(self, worlds: Set[str],
                 access: Dict[str, Set[str]],
                 val: Dict[str, Set[str]]):
        self.W = worlds        # set of world names
        self.R = access        # world -> accessible worlds
        self.V = val           # atom -> set of worlds where true

    def satisfies(self, world: str, formula) -> bool:
        match formula:
            case ('atom', p):
                return world in self.V.get(p, set())
            case ('neg', phi):
                return not self.satisfies(world, phi)
            case ('and', phi, psi):
                return self.satisfies(world, phi) and self.satisfies(world, psi)
            case ('or', phi, psi):
                return self.satisfies(world, phi) or self.satisfies(world, psi)
            case ('impl', phi, psi):
                return not self.satisfies(world, phi) or self.satisfies(world, psi)
            case ('box', phi):  # □phi: true at all accessible worlds
                return all(self.satisfies(v, phi)
                          for v in self.R.get(world, set()))
            case ('dia', phi):  # ◇phi: true at some accessible world
                return any(self.satisfies(v, phi)
                          for v in self.R.get(world, set()))

    def valid(self, formula) -> bool:
        return all(self.satisfies(w, formula) for w in self.W)

# Build the example model from above
m = KripkeModel(
    worlds = {'w1', 'w2', 'w3'},
    access = {'w1': {'w2', 'w3'}, 'w2': {'w2'}, 'w3': set()},
    val    = {'p': {'w2'}, 'q': {'w1', 'w3'}}
)

p = ('atom', 'p')
dia_p = ('dia', p)
box_p = ('box', p)

print("w1 |= ◇p:", m.satisfies('w1', dia_p))  # True
print("w1 |= □p:", m.satisfies('w1', box_p))  # False
print("w2 |= □p:", m.satisfies('w2', box_p))  # True

# Check T axiom (□p → p) at each world:
t_axiom = ('impl', box_p, p)
for w in ['w1', 'w2', 'w3']:
    print(f"{w} |= □p→p:", m.satisfies(w, t_axiom))
# This model is NOT reflexive (w3 has no self-loop)
# so T axiom may fail at some world
```

## Exercises
See [problems/ch12_modal_logic/01_modal_logic_exercises.md](../../../problems/ch12_modal_logic/01_modal_logic_exercises.md)
