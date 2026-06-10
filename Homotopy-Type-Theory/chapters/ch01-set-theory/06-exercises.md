# Exercises

---

**Exercise 1.1.** Using only the ZFC axioms (Extensionality, Empty Set, Pairing, Union, Power Set, Separation), prove:

(a) The empty set is unique.

(b) For any sets $a$ and $b$, the set $\{a, b\}$ is unique.

(c) $\{a\} \neq \{b\}$ if and only if $a \neq b$.

---

**Exercise 1.2.** Prove Cantor's theorem: for any set $A$, there is no surjection $f : A \to \mathcal{P}(A)$.

*Hint:* Define $D = \{x \in A \mid x \notin f(x)\}$. Show $D$ is not in the image of $f$.

---

**Exercise 1.3.** Prove the Schröder-Bernstein theorem: if there are injections $f : A \to B$ and $g : B \to A$, then there is a bijection $A \to B$.

*Hint:* Define $C_0 = A \setminus g(B)$ and $C_{n+1} = g(f(C_n))$. Consider the partition $A = (\bigcup_{n \geq 0} C_n) \cup (A \setminus \bigcup_{n \geq 0} C_n)$.

---

**Exercise 1.4.** The *Burali-Forti paradox:* Show that if $\Omega$ were a "set of all ordinals," then $\Omega$ would itself be an ordinal with $\Omega \in \Omega$, contradicting Foundation. Conclude that no such set exists.

---

**Exercise 1.5.** Verify the Kuratowski ordered pair encoding: $\langle a, b \rangle = \{\{a\}, \{a,b\}\}$ satisfies $\langle a, b \rangle = \langle c, d \rangle \iff a = c \wedge b = d$.

Also verify that the alternate encoding $\langle a, b \rangle' = \{a, \{a, b\}\}$ works too. Why does ZFC pick one encoding rather than declare pairs to be "primitive"?

---

**Exercise 1.6.** Show that the following are provably equivalent in ZF (you may use the other equivalences as stepping stones):

(a) AC: Every collection of non-empty sets has a choice function.

(b) Every surjection has a right inverse.

(c) Every set can be well-ordered.

*Hint for (a)→(b):* Given a surjection $f: A \to B$, apply AC to the collection $\{f^{-1}(b) \mid b \in B\}$ of non-empty fibers.

---

**Exercise 1.7.** Explain informally why the following argument is *not* a proof of AC:

"Given non-empty sets $A_i$ for $i \in I$, for each $i$ just pick any element $a_i \in A_i$. The function $i \mapsto a_i$ is the choice function."

What happens when $I$ is infinite? What is missing in this "proof"?

---

**Exercise 1.8.** In ZFC, the integer $-3$ can be constructed as an equivalence class $[(0, 3)]$ in the construction $\mathbb{Z} = (\mathbb{N} \times \mathbb{N})/{\sim}$ where $(m,n) \sim (m',n') \iff m+n' = m'+n$.

(a) Write out explicitly what set $[(0,3)]$ is, in terms of sets of ordered pairs.

(b) What is the set representing $-3$ in this encoding? (Give the first few elements of the equivalence class.)

(c) Explain why $(-3) + 3 = 0$ follows from the definition of addition on equivalence classes.

---

**Exercise 1.9.** The *Axiom of Foundation* implies there is no infinite descending $\in$-chain. Prove this directly: assume there is a sequence $a_0 \ni a_1 \ni a_2 \ni \ldots$ and derive a contradiction using Foundation.

---

**Exercise 1.10 (Conceptual).** Consider the statement: "There are exactly two groups of order 4, up to isomorphism: $\mathbb{Z}/4\mathbb{Z}$ and $\mathbb{Z}/2\mathbb{Z} \times \mathbb{Z}/2\mathbb{Z}$."

(a) In ZFC, what does "up to isomorphism" mean formally?

(b) How many actual *sets* (in the universe of ZFC) are groups of order 4? Are there finitely or infinitely many?

(c) Why do mathematicians say "two groups of order 4" when there are (in the set-theoretic sense) infinitely many?

(d) How does Univalence in HoTT make the mathematician's statement literally true?

---

**Exercise 1.11 (Research).** Cohen's method of *forcing* constructs models of set theory with specific properties. Look up a high-level description of forcing and answer:

(a) What is a "generic extension" of a model of ZFC?

(b) How does forcing prove the independence of CH (the Continuum Hypothesis)?

(c) What does it mean, philosophically, for a mathematical statement to be "undecidable in ZFC"?

---

**Exercise 1.12 (Challenge).** Gödel's constructible universe $L$ is defined by:
$$L_0 = \emptyset, \quad L_{\alpha+1} = \text{Def}(L_\alpha), \quad L_\lambda = \bigcup_{\alpha < \lambda} L_\alpha$$
where $\text{Def}(X)$ is the collection of all subsets of $X$ *definable* by a first-order formula with parameters from $X$.

(a) Explain why every element of $L$ is constructible (definable from simpler sets).

(b) Gödel showed $L \models \text{AC}$. Sketch why: every element of $L_\alpha$ has a canonical definition, so we can well-order $L_\alpha$ lexicographically. How does this give a choice function?

(c) Gödel also showed $L \models \text{GCH}$ (Generalized Continuum Hypothesis). Why does this imply AC and GCH are consistent with ZF?
