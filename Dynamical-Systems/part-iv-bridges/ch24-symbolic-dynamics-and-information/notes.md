# Chapter Notes — Chapter 24

## The central reference

**Lind and Marcus**, *An Introduction to Symbolic Dynamics and Coding* (Cambridge, 1995) is the definitive text for this chapter. Part III (Chapters 7–13) covers the connection to coding theory in depth: the Curtis-Hedlund-Lyndon theorem, the classification of SFTs via shift equivalence, and the constrained coding theorem. It is unusually readable for a mathematics text, and the exercises are excellent.

## On hidden Markov processes and sofic shifts

The identification of HMM outputs with sofic processes is implicit in the work of Weiss (who introduced sofic shifts in 1973) and made explicit in:

- **Ephraim and Merhav**, "Hidden Markov Processes," *IEEE Transactions on Information Theory* 48 (2002) — a comprehensive survey

The difficulty of computing HMM entropy rates is discussed in:

- **Ordentlich and Weissman**, "On the optimality of symbol-by-symbol filtering and denoising," *IEEE Trans. Inf. Theory* 52 (2006)
- **Jacquet, Seroussi, and Szpankowski**, "On the entropy of a hidden Markov process," *Theoretical Computer Science* 395 (2008)

The connection to Lyapunov exponents of random matrix products is made precise in Hürzeler and Künsch (1995). Computing Lyapunov exponents of random matrix products is generically undecidable (Blondel et al.), which explains why HMM entropy rates are hard.

## On constrained coding for magnetic recording

The Adler-Coppersmith-Hassner state-splitting algorithm is in:

- **Adler, Coppersmith, and Hassner**, "Algorithms for sliding block codes," *IEEE Trans. Inf. Theory* 29 (1983)

The engineering application to magnetic recording is surveyed in:

- **Marcus, Roth, and Siegel**, *Constrained Systems and Coding for Recording Channels* (1996) — available as a book chapter from the authors' websites

The $(d, k)$ runlength limited codes used in hard drives are fully analyzed in this reference. The IBM researchers who developed the practical theory (Marcus, Roth, and Siegel) were working directly with the mathematical theory of SFTs.

## On the automorphism group

The automorphism group of the full shift is enormously complicated. Key results:

- **Boyle, Franks, and Kitchens**, "The automorphism groups of subshifts," *Publications Mathématiques de l'IHÉS* (1990) — shows the automorphism group of the full 2-shift contains every finite group and a copy of every countable locally finite group
- **Kim, Roush, and Wagoner** — the conjugacy problem for SFTs and the connection to K-theory

This is an active research area. The classification of SFTs up to conjugacy (beyond topological entropy) involves algebraic invariants related to the shift equivalence class of the transition matrix.
