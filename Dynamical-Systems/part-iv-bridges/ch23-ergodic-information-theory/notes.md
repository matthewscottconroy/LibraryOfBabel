# Chapter Notes — Chapter 23

## On the Shannon-McMillan-Breiman theorem

The theorem bears three names because it was proved in three stages. Shannon (1948) established convergence in the i.i.d. case, essentially as a corollary of the AEP. McMillan (1953) proved $L^1$ convergence for stationary ergodic processes — a significant generalization, but still not the almost-sure result. Breiman (1957) proved pointwise a.e. convergence using the martingale convergence theorem, completing the story. The proof using Birkhoff's ergodic theorem as the organizing principle is the modern version, presented cleanly in:

- **Walters**, *An Introduction to Ergodic Theory*, Chapter 8 — the standard treatment
- **Shields**, *The Ergodic Theory of Discrete Sample Paths* — a more detailed account

The connection between SMB and Birkhoff's theorem is explicit in Walters' presentation and makes clear why ergodicity is needed: without ergodicity, Birkhoff's theorem gives convergence to a random variable (the conditional expectation with respect to the invariant $\sigma$-algebra), not to a constant. For non-ergodic sources, the "entropy rate" is a random variable, and the SMB theorem holds conditionally on the ergodic component.

## On universal source coding via Lempel-Ziv

The original LZ papers are worth reading:

- **Ziv and Lempel**, "A universal algorithm for sequential data compression," *IEEE Trans. Inf. Theory* 23, 1977 — this is LZ77
- **Ziv and Lempel**, "Compression of individual sequences via variable-rate coding," *IEEE Trans. Inf. Theory* 24, 1978 — this is LZ78

The proof of asymptotic optimality for stationary ergodic sources is in:

- **Wyner and Ziv**, "The sliding-window Lempel-Ziv algorithm is asymptotically optimal," *Proc. IEEE* 82, 1994

The gap between the original (1977-78) and the optimality proof (1994) is sixteen years — a measure of how subtle the ergodic analysis is.

## On the hierarchy of limit theorems

The clean presentation of the AEP → SMB → Birkhoff hierarchy is in Cover and Thomas's *Elements of Information Theory* (Chapter 16), which also has the best exposition of the AEP for Markov chains and stationary sources. Particularly recommended is the discussion of the "type" of a sequence — the empirical distribution — as a bridge between the combinatorial and measure-theoretic views.

## On LZ complexity as an entropy estimator

For the convergence theorem (Theorem 23.5.2), see:

- **Lempel and Ziv**, "On the complexity of finite sequences," *IEEE Trans. Inf. Theory* 22, 1976

For applications to neuroscience and genomics, the Lempel-Ziv complexity has a large literature. A starting point is:

- **Kaspar and Schuster**, "Easily calculable measure for the complexity of spatiotemporal patterns," *Physical Review A* 36, 1987

For careful finite-sample analysis of LZ-based entropy estimators, the recent work of Verdú and collaborators gives tight bounds.
