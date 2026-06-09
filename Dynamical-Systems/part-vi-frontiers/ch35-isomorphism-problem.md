# Chapter 35 — The Isomorphism Problem in Ergodic Theory

> *When are two measure-preserving systems the same? Ornstein's theorem answers for Bernoulli shifts: entropy is the complete invariant. For general systems, the Foreman-Rudolph-Weiss theorem shows no Borel complete invariant can exist. The isomorphism problem is inherently undecidable — not in the Gödelian sense, but in a descriptive set-theoretic sense.*

**Prerequisites:** Chapter 7 (KS entropy, Ornstein's theorem), Chapter 32 (descriptive set theory, Borel equivalence relations), Chapter 33 (orbit equivalence).

---

## 35.1 Isomorphism and Its Obstructions

**Definition 35.1.1.** Two MPTs $(X, \mu, T)$ and $(Y, \nu, S)$ are *isomorphic* (conjugate) if there is a measure-space isomorphism $\phi: X \to Y$ with $\phi \circ T = S \circ \phi$ a.e.

**Definition 35.1.2.** A *complete invariant* for a class $\mathcal{C}$ of MPTs is a Borel function $I: \mathcal{C} \to \mathcal{I}$ (to some standard Borel space $\mathcal{I}$) with $T \cong S \iff I(T) = I(S)$.

**Ornstein's Theorem (Complete Invariant for Bernoulli Shifts).** Entropy $h: \text{Bernoulli} \to [0, \infty]$ is a complete invariant for Bernoulli shifts (among all free ergodic MPTs). This is the central theorem of Chapter 7.

**The Problem:** Does a complete invariant exist for all ergodic MPTs?

---

## 35.2 The Foreman-Rudolph-Weiss Theorem

**Theorem 35.2.1 (Foreman-Rudolph-Weiss, 2011).** The isomorphism relation on ergodic MPTs of $[0,1]$ with Lebesgue measure is not classifiable by countable structures. More precisely, the isomorphism equivalence relation $\cong$ on $\text{Aut}(X, \mu)$ is not Borel reducible to the isomorphism relation of countable structures.

**Corollary 35.2.2.** There is no Borel complete invariant for ergodic MPTs. In particular:
- No sequence of reals, no countable group, no countable graph, no countable field can serve as a complete invariant.
- The isomorphism problem for ergodic MPTs is "more complex" than classifying any class of countable algebraic structures.

**Theorem 35.2.3 (Complexity of the Isomorphism Relation).** The isomorphism relation on ergodic MPTs is:
- $\Sigma^1_1$-complete (analytic, not Borel)
- Not Borel (the class of Bernoulli shifts is Borel, but isomorphism on Bernoulli shifts is "non-Borel equivalent")
- Strictly above all orbit equivalence relations of Polish group actions

---

## 35.3 Rank-One Systems and Anti-Classification

**Definition 35.3.1.** An MPT $T$ is *rank-one* if it has a sequence of Rohlin towers $(B_n, TB_n, \ldots, T^{h_n-1}B_n)$ with $\mu(B_n) \to 0$ and $\mu\left(\bigcup_{j=0}^{h_n-1}T^jB_n\right) \to 1$.

Rank-one systems are the "simplest" ergodic systems — they are approximated by rotations on cyclic groups.

**Examples 35.3.2.**
- The Chacón system: rank-one, weakly mixing, not mixing
- The von Neumann-Kakutani adding machine: rank-one, non-ergodic (!)
- The Staircase transformation: rank-one, mixing

**Theorem 35.3.3 (King, 1988).** Within the class of rank-one systems, the set of mixing systems is dense. In other words, mixing is "not generic" but is "dense" — rank-one systems can approximate mixing behavior.

**Theorem 35.3.4 (Foreman-Weiss, 2019).** The isomorphism relation restricted to rank-one systems is a complete $\Sigma^1_1$ equivalence relation. Thus even for the "simplest" class of ergodic systems, the isomorphism problem is maximally complex.

---

## 35.4 Classifiable Subclasses

Despite the general unclassifiability, several important subclasses have complete invariants:

**Theorem 35.4.1 (Halmos-von Neumann, 1942).** Ergodic MPTs with *discrete spectrum* (all eigenvalues are countable, generating the full algebra) are classified by their group of eigenvalues $\text{Eig}(T) \subseteq S^1$.

**Theorem 35.4.2 (Bernoulli Shifts — Ornstein, 1970).** Bernoulli shifts are classified by their KS entropy.

**Theorem 35.4.3 (Ornstein-Weiss, 1987).** Bernoulli shifts of amenable groups are classified by entropy.

**Theorem 35.4.4 (Bowen, 2012).** Bernoulli shifts of sofic groups are classified by sofic entropy.

**Theorem 35.4.5 (Giordano-Putnam-Skau, 1995).** Minimal ${\mathbb Z}$-actions on the Cantor set are classified (up to orbit equivalence) by their ordered $K_0$ group.

---

## 35.5 The Isomorphism Problem for Shifts

**Theorem 35.5.1 (Williams' Theorem Failure).** Williams conjectured (1973) that two SFTs are isomorphic iff their transition matrices are related by "elementary strong shifts equivalence." This was disproved by Kim-Roush (1992).

**Theorem 35.5.2 (Williams' Theorem for Flow Equivalence).** Two irreducible SFTs are *flow equivalent* iff their Bowen-Franks groups coincide: $\text{BF}(A) \cong \text{BF}(B)$ where $\text{BF}(A) = \text{coker}(I - A)$ (cokernel of $I - A: {\mathbb Z}^n \to {\mathbb Z}^n$).

**Theorem 35.5.3 (Sofic Shifts).** The isomorphism problem for sofic shifts (up to topological conjugacy) is undecidable: there is no algorithm that takes two sofic shift presentations and decides if they are conjugate.

**Remark 35.5.4.** The isomorphism problem for SFTs is one of the oldest open problems in symbolic dynamics. In dimension 1, it is open whether there exists a complete invariant. In dimension $\geq 2$, the problem is undecidable (Wang tiling connections from Chapter 25).

---

## Exercises

**Exercise 35.1.** Show that the eigenvalue group $\text{Eig}(T) = \{e^{2\pi i\alpha} : U_T e_\alpha = e^{2\pi i\alpha}e_\alpha\}$ is an isomorphism invariant. Verify that $\text{Eig}(R_\alpha) = \{e^{2\pi in\alpha} : n \in {\mathbb Z}\}$ for the rotation $R_\alpha$.

**Exercise 35.2.** Show that the Bernoulli shift $B(1/2, 1/2)$ and the Bernoulli shift $B(1/3, 1/3, 1/3)$ have the same entropy $\log 2$ (nats). By Ornstein's theorem, they are isomorphic. Can you construct an explicit isomorphism?

**Exercise 35.3.** Verify that the Chacón transformation (defined by the substitution $0 \mapsto 0010$, $1 \mapsto 1$) is rank-one. Compute its entropy.

**Exercise 35.4.** (Research) The classification problem for rank-one mixing systems is open. Propose a potential invariant (beyond entropy and spectrum) and explain why classifying these systems is hard.

---

## Chapter Notes

Ornstein's original papers: *Bernoulli shifts with the same entropy are isomorphic* (Advances in Math., 1970). His book *Ergodic Theory, Randomness, and Dynamical Systems* (Yale, 1974) is the standard reference.

Foreman-Rudolph-Weiss: *The conjugacy problem in ergodic theory* (Annals of Math., 2011). The anti-classification program is surveyed in Foreman's *Classifying and Unclassifying* (in *Descriptive Set Theory and Dynamical Systems*, Cambridge, 2000).

Kim-Roush's disproof of Williams' conjecture: *Decidability of shift equivalence* (Dynamical Systems, 1992). The state of the isomorphism problem for SFTs is surveyed in Boyle's *Open problems in symbolic dynamics* (2008).
