# Chapter 12: Further Reading

## Primary Sources

**Jaeger, H. (2014). Controlling recurrent neural networks by conceptors. *arXiv preprint arXiv:1403.3369*.**
[Jaeger2014]

The complete reference on conceptors. At 110 pages, this is a monograph rather than a paper. The first five sections develop the core theory covered in this chapter; the remaining sections present applications to cognitive modeling, continual learning, and multi-pattern generation. The appendix contains detailed proofs. Reading the monograph with pencil and paper — working through the derivations — is the recommended approach.

**Jaeger, H. (2017). Using conceptors to manage neural long-term memories for temporal patterns. *Journal of Machine Learning Research*, 18(13), 1–43.**
[Jaeger2017]

A more focused, journal-length treatment that emphasizes the memory management application: storing many patterns in a single reservoir, incrementally loading new patterns without catastrophic forgetting, and selective recall. This is a gentler entry point than the 2014 monograph, with less mathematical density and more practical focus.

## Background

**Birkhoff, G. (1940). *Lattice Theory*. American Mathematical Society.**
[Birkhoff1940]

The standard reference on lattice theory. The connection to conceptors is that the set of conceptors forms a bounded distributive lattice. Chapter 1 of Birkhoff's book covers the definitions and basic theorems needed to understand the lattice structure proof in Section 12.3.7.

**MacLennan, B. J. (1988). Field computation: A framework for parallel, continuous computation. Technical Report CS-88-87, University of Tennessee.**
[MacLennan1988]

An early paper on computing with continuous-valued "fields" rather than symbolic logic. Conceptors can be viewed as a sophisticated implementation of field computation over reservoir state spaces.

**Jaeger, H., Noheda, B., & van der Wiel, W. G. (2023). Toward a formal theory for computing machines made out of whatever physics offers. *Nature Communications*, 14, 4911.**
[Jaeger2023]

Jaeger's recent attempt to formalize the broader reservoir computing program, including conceptors, in a way that applies to physical implementations. Represents the current frontier of the theoretical program that conceptors are part of.

## Related Work on Neural Memory

**Hopfield, J. J. (1982). Neural networks and physical systems with emergent collective computational abilities. *Proceedings of the National Academy of Sciences*, 79(8), 2554–2558.**
[Hopfield1982]

The Hopfield network paper. Classical attractor-based memory — the complement to conceptors' subspace-based memory. Understanding both approaches clarifies the advantages of each: Hopfield networks are simple and well-analyzed but scale poorly with the number of stored patterns; conceptors scale better but require the reservoir substrate.

**Ramsauer, H., Schäfl, B., Lehner, J., Seidl, P., Widrich, M., Gruber, L., ... & Hochreiter, S. (2021). Hopfield networks is all you need. In *International Conference on Learning Representations*.**
[Ramsauer2021]

The modern update to Hopfield networks, showing that attention mechanisms in transformers are equivalent to a continuous Hopfield network with exponential interaction. This connection between Hopfield networks and transformers suggests potential connections to conceptors as well — an active research direction.
