# 43.1 HoTT Basics: Types as Spaces

## 43.1.1 The Correspondence

Homotopy Type Theory is built on a fundamental insight: types in type theory correspond to topological spaces in homotopy theory, and equality in type theory corresponds to paths. This is not just an analogy — it's the foundation of HoTT.

**The Homotopy Type Theory (HoTT) Correspondence:**

| HoTT | Homotopy Theory | Classical Mathematics |
|---|---|---|
| Type $A$ | Topological space | Set |
| Term $a: A$ | Point $a \in A$ | Element |
| Identity type $a =_A b$ | Path from $a$ to $b$ | Equality |
| Path $p: a =_A b$ | Continuous path | Proof of equality |
| Path composition $p \cdot q$ | Path concatenation | Transitivity of equality |
| Higher identity $p =_{a=b} q$ | Homotopy between paths | Equality of proofs |
| $n$-truncated type | $n$-type (homotopy $n$-type) | $n$-groupoid |
| Contractible type ($-2$-type) | Contractible space | Singleton |
| Proposition ($-1$-type) | Prop: at most one element | Classical proposition |
| Set ($0$-type) | Discrete space | Set |

**Key Principle:** Equality is not atomic — it has structure. Two proofs of equality $p, q: a =_A b$ may or may not be equal, leading to higher-dimensional structure.

In classical mathematics, equality is a relation: either $a = b$ or $a \neq b$, and there's nothing more to say. In HoTT, equality is a type: the type $a =_A b$ has elements (proofs of equality), and those elements can themselves be equal or unequal. A proof of equality is a path. Two paths between the same points can be homotopic or not. This higher-dimensional structure captures the topological information.

## 43.1.2 The Univalence Axiom

**Definition 43.1.1.** For types $A, B$, an *equivalence* is a function $f: A \to B$ with a quasi-inverse. The type of equivalences is $A \simeq B$.

**Axiom 43.1.2 (Univalence — Voevodsky).** For types $A, B$ in the same universe $\mathcal{U}$:
$$(A = B) \simeq (A \simeq B).$$

Equivalences of types are (equivalent to) equalities of types. This axiom is consistent with Martin-Löf type theory and is the foundation of HoTT.

**Dynamical Analogy:** Univalence says "equivalent structures are equal." In ergodic theory: Ornstein's theorem says isomorphic Bernoulli shifts are equal (as abstract dynamical systems). Univalence is the type-theoretic version of the principle that isomorphic objects should be identified.

Univalence has a remarkable consequence: any property of types that's provable in HoTT must be preserved by equivalences. You can never "tell" two equivalent types apart — any theorem that holds for $A$ holds for every type equivalent to $A$. This is the type-theoretic analogue of the principle that mathematical properties should be isomorphism-invariant.
