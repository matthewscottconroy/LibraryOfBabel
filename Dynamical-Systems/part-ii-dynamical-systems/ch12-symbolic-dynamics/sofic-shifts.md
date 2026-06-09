# 12.5 Sofic Shifts

Subshifts of finite type are defined by forbidden words. But there is a larger and natural class of subshifts: those whose language is a regular language — recognized by a finite-state automaton. These are the *sofic shifts*, and they are the symbolic dynamics version of hidden Markov models.

The name "sofic" comes from the Hebrew word for finite, introduced by Weiss in 1973. The concept is: a sofic shift is what you see when you observe an SFT through a finite-memory "lens" that can collapse different states.

**Definition 12.5.1.** A *sofic shift* is the image of an SFT under a *sliding block code*: a continuous, shift-commuting map $\pi: X_A \to \mathcal{B}^{\mathbb Z}$ that reads a window of $N+M+1$ consecutive symbols and outputs a single symbol in a new alphabet $\mathcal{B}$. Formally, $\pi(x)_n = \Phi(x_{n-N}, \ldots, x_{n+M})$ for some function $\Phi: \mathcal{A}^{N+M+1} \to \mathcal{B}$.

Equivalently — and this is the combinatorial characterization — $X$ is sofic iff $\mathcal{L}(X)$ is a *regular language*: accepted by a finite-state automaton.

**Theorem 12.5.2.** Every SFT is sofic (the identity map is a sliding block code), but not every sofic shift is an SFT.

The even shift (Section 12.2) is the canonical counter-example: its language is regular (a finite automaton can check that every run of 0s between consecutive 1s has even length), but no finite set of forbidden words describes it. Any SFT description would need to forbid $10^{2n+1}1$ for all $n$, an infinite list.

**Theorem 12.5.3 (Counter-example: Even Shift).** The even shift is sofic (its language is regular) but is not an SFT (it requires infinitely many forbidden words to describe).

The key structural result for sofic shifts is the existence of a canonical minimal presentation:

**Theorem 12.5.4 (Fischer Cover).** Every irreducible sofic shift $X$ has a canonical minimal deterministic presentation: the *Fischer cover* (also called the *left Krieger cover*). The Fischer cover is the unique minimal edge SFT $X_A$ that maps onto $X$ by a 1-block code, with $|V|$ equal to the number of *follower sets* of words in $\mathcal{L}(X)$.

The follower set of a word $w$ is $\mathcal{F}(w) = \{u \in \mathcal{A}^* : wu \in \mathcal{L}(X)\}$ — the set of all words that can legally follow $w$. Two words $w$ and $w'$ are "equivalent" if they have the same follower set; the equivalence classes are the states of the Fischer cover automaton.

What this is saying is: the Fischer cover is the "canonical form" for a sofic shift — the smallest SFT that surjects onto it. It is the right generalization of the transition matrix from SFTs to sofic shifts, and it is the object you compute when you want to analyze the entropy, periodic orbits, or measure theory of a sofic shift.

In information theory, sofic shifts correspond precisely to hidden Markov models (HMMs): the SFT is the hidden state process, and the sliding block code is the observation function. We spell this out carefully in Section 12.9.
