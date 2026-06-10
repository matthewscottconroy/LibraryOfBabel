# 2.2 The Axiom of Choice

## The Most Controversial Axiom

Of all the ZFC axioms, the Axiom of Choice (AC) is the most philosophically contentious, the most mathematically powerful, and the most important for understanding the relationship between classical and constructive mathematics.

**Axiom of Choice:**
$$\forall \mathcal{F}\, [\emptyset \notin \mathcal{F} \to \exists f\, (f : \mathcal{F} \to \bigcup\mathcal{F}\ \wedge\ \forall A \in \mathcal{F},\, f(A) \in A)]$$

Given a collection $\mathcal{F}$ of non-empty sets, there exists a *choice function* $f$ that selects one element from each set in $\mathcal{F}$.

## Why It Seems Obvious

If you have finitely many non-empty sets $A_1, A_2, \ldots, A_n$, you can certainly choose one element from each: pick $a_1 \in A_1$, pick $a_2 \in A_2$, ..., pick $a_n \in A_n$. This is just $n$ individual choices.

For infinite collections, the intuition extends easily: "just choose one from each." The choice function clearly *should* exist — for any non-empty set, there is at least one element to choose.

The problem: "just choose" is not a mathematical construction. For an infinite collection, you need a *rule* for which element to pick. If the sets have no additional structure (no ordering, no canonical element), there may be no *describable* rule. The Axiom of Choice asserts that such functions exist even when we can't describe them.

## Equivalent Forms

Several statements are provably equivalent to AC (in ZF):

**Zorn's Lemma:** Every non-empty partially ordered set in which every chain (totally ordered subset) has an upper bound contains a maximal element.

*Used for:* proving every vector space has a basis, every ring has a maximal ideal, every filter extends to an ultrafilter.

**Well-Ordering Theorem:** Every set can be well-ordered.

*Consequence:* $\mathbb{R}$ can be well-ordered — there is a first real number, a second, etc. under some ordering. But no such ordering can be described explicitly (without AC), and the well-ordering of $\mathbb{R}$ under any such ordering is not measurable.

**Tychonoff's Theorem:** An arbitrary product of compact topological spaces is compact.

*This is a theorem of topology*, but it requires AC in its full generality.

**König's Theorem:** If $|A_i| < |B_i|$ for all $i \in I$, then $|\sum_i A_i| < |\prod_i B_i|$ (where $\sum$ is disjoint union and $\prod$ is Cartesian product). Requires AC.

**Theorem:** The following are equivalent over ZF:
- AC
- Every surjection has a right inverse
- Every set can be well-ordered  
- Zorn's Lemma
- Tychonoff's Theorem (for $T_1$ spaces)
- Every vector space has a basis
- Every connected graph has a spanning tree

## Independence: AC from ZF

Gödel (1938) showed that AC is *consistent* with ZF: if ZF is consistent, then so is ZF + AC. Gödel did this by constructing the *constructible universe* $L$, a model where every set has an explicit definition, and showing AC holds in $L$.

Cohen (1963) showed AC is *independent* of ZF: if ZF is consistent, then ZF + ¬AC is also consistent. Cohen developed the technique of *forcing* to construct a model of ZF where AC fails.

Together, these results show:
- AC cannot be proved from ZF.
- AC cannot be disproved from ZF.
- ZFC (= ZF + AC) and ZF + ¬AC are both consistent (assuming ZF is).

So adding or not adding AC is a genuine choice (no pun intended) of foundation. Most mathematicians add it because it's extremely convenient and produces cleaner theorems.

## What Fails Without AC

In ZF without AC:
- A countable union of countable sets might not be countable.
- $\mathbb{R}$ cannot be well-ordered.
- Not every vector space has a basis.
- Not every ring has a maximal ideal.
- The product of infinitely many non-empty spaces might be empty.
- Cardinals might not be totally ordered ($|A| \leq |B|$ or $|B| \leq |A|$ might fail).

## What AC Constructs That Can't Be Made Explicit

With AC, one can prove:
- A non-measurable subset of $[0,1]$ (Vitali set): a subset of $\mathbb{R}$ that cannot be assigned a Lebesgue measure consistently with translation invariance.
- The Banach-Tarski paradox: a ball in $\mathbb{R}^3$ can be decomposed into finitely many pieces and reassembled into two balls of the same size. (This doesn't violate physics because the pieces are not measurable.)
- A well-ordering of $\mathbb{R}$.

None of these can be *described* without AC — they are proved to exist but cannot be constructed.

## The Constructive Objection

Constructive mathematicians reject AC because it asserts existence without construction. From a constructive standpoint (the BHK interpretation — see Chapter 5), a proof of $\exists f, P(f)$ must *exhibit* an $f$ satisfying $P$. The Axiom of Choice asserts existence without any such exhibition.

Brouwer, the founder of intuitionism, explicitly rejected AC. Bishop's constructive analysis (1967) does without it.

Interestingly, in Martin-Löf Type Theory (MLTT), there is a version of AC that *is* provable:

**Constructive AC (MLTT):** For any family of types $B(a)$ indexed by $a : A$, if for every $a$ there is an element of $B(a)$, then there is a function assigning to each $a$ an element of $B(a)$:
$$\left(\prod_{a:A} B(a)\right) \to \left(\prod_{a:A} B(a)\right)$$

This is trivially true — the identity function! The point: in type theory, "for all $a$, there exists $b$ such that..." is literally the type of a function $a \mapsto b$. The function *is* the choice function; it's not separate. There's no non-constructive existence assertion.

However, the classical Axiom of Choice, when stated using the truncated (propositional) existential, is *not* automatically available and is independent of MLTT without additional axioms.

## AC and HoTT

In HoTT, the axiom of choice is subtle because of the distinction between truncated and untruncated types. The "propositional AC" (where existence is truncated — meaning "there exists but we don't have explicit access to the witness") does not automatically hold and has interesting connections to the principle of excluded middle.

The *univalent foundations* perspective provides a nuanced picture: some forms of choice hold, others don't, and the structure of the hierarchy of propositions and sets determines which.

This is one reason the axiom of choice is not simply assumed globally in HoTT: its status depends on what kind of mathematical objects you're working with.
