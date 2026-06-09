# Chapter 3 — Exercises

## Important Figures

- **Georg Cantor (1845–1918)** — proved $|\mathbb{N}| < |\mathbb{R}|$ via diagonalization (1891); introduced $\aleph_0$ and the transfinite hierarchy; the Continuum Hypothesis
- **Ernst Zermelo (1871–1953)** — proved the Well-Ordering Theorem from the Axiom of Choice (1904); this equivalence was initially controversial
- **Kurt Gödel (1906–1978)** — proved AC and GCH are consistent with ZF (1938) via the constructible universe $L$
- **Paul Cohen (1934–2007)** — proved AC and CH are independent of ZF (1963) using the technique of forcing; Fields Medal 1966
- **Ernst Schröder (1841–1902) & Felix Bernstein (1878–1956)** — Schröder–Bernstein theorem: $|A| \leq |B|$ and $|B| \leq |A|$ implies $|A| = |B|$; no Choice needed

## References and Primary Sources

- **G. Cantor, "Über eine elementare Frage der Mannigfaltigkeitslehre" (1891)** — original diagonal argument
- **K. Gödel, *The Consistency of the Axiom of Choice and the Generalized Continuum Hypothesis* (1940)**
- **P. Cohen, *Set Theory and the Continuum Hypothesis* (1966)** — forcing explained by Cohen himself
- **T. Jech, *Set Theory* (3rd ed., Springer, 2003)** — comprehensive modern treatment; includes forcing
- **H. Herrlich, *Axiom of Choice* (Springer, 2006)** — survey of consequences and equivalents

## Examples, Applications, and Thought Experiments

- **Cantor's diagonal argument** — list all reals in $[0,1]$ as decimal expansions; construct a new real that differs from the $n$-th listed number in the $n$-th decimal place; this new real is not on the list; therefore $|\mathbb{N}| < |\mathbb{R}|$; the argument is self-undermining for any supposed enumeration
- **Hilbert's Hotel** — a hotel with $\aleph_0$ rooms, all occupied; a new guest arrives: shift guest $n$ to room $n+1$; room 1 is free; even $\aleph_0$ new guests can be accommodated by sending guest $n$ to room $2n$; builds intuition for infinite cardinality without paradox
- **The Banach–Tarski Paradox** — using AC, a solid ball in $\mathbb{R}^3$ can be decomposed into finitely many pieces and reassembled (by rotations and translations) into two balls of the same size; a dramatic consequence showing AC sanctions non-measurable sets
- **AC in algebra** — Zorn's Lemma (equivalent to AC) is used to prove every vector space has a basis, every ring has a maximal ideal, and every field has an algebraic closure; these are ubiquitous results whose proofs are non-constructive

## Exercises

1. Prove directly from the definition of equipotence (via bijection) that the following pairs of sets have the same cardinality. In each case, exhibit an explicit bijection and verify it is both injective and surjective.
   (a) $\mathbb{N}$ and the set of even natural numbers $E = \{0, 2, 4, 6, \ldots\}$
   (b) The open interval $(0, 1)$ and the open interval $(a, b)$ for any $a < b$ in $\mathbb{R}$
   (c) $(0, 1)$ and $\mathbb{R}$ (hint: consider $f(x) = \tan(\pi(x - 1/2))$ or an alternative of your choice)

2. Prove that $|\mathbb{Z}| = |\mathbb{N}|$ by constructing an explicit bijection. Then prove that $|\mathbb{Q}| = |\mathbb{N}|$ by describing a systematic enumeration of the rationals (you may use Cantor's "zigzag" argument, but you must explain precisely which rationals are listed at each step and why every rational appears exactly once).

3. Apply the Schröder–Bernstein theorem to prove that $|\mathbb{R}| = |(0,1)|$ without constructing an explicit bijection. That is, exhibit injections $f: (0,1) \to \mathbb{R}$ and $g: \mathbb{R} \to (0,1)$, state the Schröder–Bernstein theorem, and conclude. Then explain why the Schröder–Bernstein theorem itself does not require the Axiom of Choice and outline the key idea in its proof.

4. Prove Cantor's theorem: for any set $A$, there is no surjection from $A$ to $\mathcal{P}(A)$, and therefore $|A| < |\mathcal{P}(A)|$. Your proof should use the diagonal construction explicitly: given any function $f: A \to \mathcal{P}(A)$, define $D = \{a \in A : a \notin f(a)\}$ and show $D$ is not in the image of $f$. Apply this theorem to conclude that there is no largest cardinality — the cardinalities form a proper class, not a set.

5. Use Cantor's diagonal argument to prove that the set of all infinite binary sequences — functions $\mathbb{N} \to \{0, 1\}$ — is uncountable. Then deduce that $|\mathcal{P}(\mathbb{N})| = |\mathbb{R}|$ by establishing bijections between $\mathcal{P}(\mathbb{N})$, the set of infinite binary sequences, and an appropriate subset of $\mathbb{R}$.

6. A collection $\mathcal{C}$ of subsets of a set $X$ is said to have the *finite intersection property* if every finite subcollection has non-empty intersection. State precisely what Zorn's Lemma says (in terms of chains and upper bounds in partially ordered sets). Then use Zorn's Lemma to prove the following: if $\mathcal{C}$ is a collection of subsets of $X$ with the finite intersection property, then there exists a maximal such collection $\mathcal{C}' \supseteq \mathcal{C}$ (an *ultrafilter base*). Identify clearly where Zorn's Lemma is applied and what the partial order is.

7. Prove that the following three statements are equivalent (you need not prove all six implications; a cycle of three suffices):
   (A) The Axiom of Choice: for any family $\{A_i\}_{i \in I}$ of non-empty sets, $\prod_{i \in I} A_i \neq \emptyset$.
   (B) Zorn's Lemma: every non-empty partially ordered set in which every chain has an upper bound contains a maximal element.
   (C) The Well-Ordering Principle: every set can be well-ordered.
   You may assume (A) $\Rightarrow$ (B) and focus on proving (B) $\Rightarrow$ (C) and (C) $\Rightarrow$ (A), sketching the key ideas and identifying at which step each argument would break down without the hypothesis.

8. (Challenge) Let $\kappa$ and $\lambda$ be infinite cardinals with $\kappa \leq \lambda$. Assuming the Axiom of Choice, prove that $\kappa + \lambda = \lambda$ (where cardinal addition $\kappa + \lambda$ is defined as $|A \sqcup B|$ for disjoint sets $A$, $B$ with $|A| = \kappa$ and $|B| = \lambda$). Deduce that for any infinite set $A$, the set $A \times A$ satisfies $|A \times A| = |A|$. Explain why this result implies that a countable union of countable sets is countable, and identify precisely where the Axiom of Choice is used in the argument.
