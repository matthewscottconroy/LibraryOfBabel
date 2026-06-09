# Chapter Notes — Chapter 12

## The Primary Text

The standard reference for symbolic dynamics is Lind and Marcus, *An Introduction to Symbolic Dynamics and Coding* (Cambridge University Press). It is excellent: written at exactly the right level, with clear proofs, good examples, and a thorough treatment of both the dynamical and coding-theoretic perspectives. If you are going to read one book from this chapter's bibliography, read this one.

The original paper by Morse and Hedlund (1938) — *Symbolic Dynamics* in the *American Journal of Mathematics* — is worth reading for historical perspective. They introduced the term and established the foundational results on complexity functions and periodic sequences. It is entirely accessible and surprisingly readable.

## Hidden Markov Models and Information Theory

The sofic shift / hidden Markov model correspondence is made explicit in Ephraim and Merhav's survey *Hidden Markov Processes* (*IEEE Transactions on Information Theory*, 2002). This is the definitive treatment from the information-theoretic perspective, and it is the right bridge between what you learn in this chapter and what you need to know for Chapter 24.

The key open problem on the information-theoretic side: computing the entropy rate of a hidden Markov process is hard. There is no closed-form formula in general; it requires either approximation or special structure. This connects symbolic dynamics to questions in statistical mechanics (transfer matrices, Ising models) and to estimation theory.

## Automorphisms and Undecidability

The automorphism group of the full shift (Section 12.8) is studied in depth by Boyle, Lind, and Rudolph in their 1988 paper *The Automorphism Group of a Shift of Finite Type* in *Transactions of the AMS*. It is a long and technical paper, but Section 1 is a beautiful overview of what is known and unknown.

The undecidability of strong shift equivalence over $\mathbb{Z}$ (Kim-Roush, 1992) connects symbolic dynamics to the undecidability of the word problem for groups — a connection explored in the model theory of dynamical systems and in Chapter 27.

## Perron-Frobenius and Applications

The Perron-Frobenius theorem appears constantly in symbolic dynamics, information theory, and Markov chain theory. The clean reference for all three perspectives is Seneta's *Non-Negative Matrices and Markov Chains* (Springer). If you find yourself repeatedly using Perron-Frobenius without a good reference for the subtle cases (reducible matrices, boundary eigenvalues), Seneta is the place to go.

## Research Entry Points

The live research areas most accessible from this chapter: (1) complexity functions of minimal subshifts and their relationship to topological and algebraic invariants; (2) the automorphism group of SFTs, where many basic questions remain open; (3) the information-theoretic properties of sofic and beyond-sofic processes (Chapter 24 is the bridge). The survey by Boyle and Schmieding (*New Directions in Symbolic Dynamics*, 2021) gives a good picture of the current frontier.
