# Applications: Higher Inductive Types

## Application 1: Formal Verification — Correctness of Distributed Systems

Distributed systems — networks of computers communicating by message passing — must achieve *consensus*: all nodes must agree on a single value, despite failures and delays. The correctness of consensus protocols (Paxos, Raft, PBFT) is notoriously difficult to verify.

Higher inductive types provide the right language for specifying and verifying such protocols. A distributed system can be modeled as a type with point constructors (states of the system) and path constructors (transitions between states, corresponding to message events). The path constructors encode the protocol rules: from state s₁ and state s₂, if certain conditions hold, there is a transition (path) to a new combined state.

The consensus property — that all nodes eventually agree — corresponds to a path (or truncated path) between all "agreement" states. The path constructor machinery ensures that all paths to agreement states compose coherently: if node A and node B both reach agreement, and both follow the protocol paths, their agreement states are connected by a 2-path that witnesses the consistency of the protocol.

The TLA+ (Temporal Logic of Actions) formalism for verifying distributed systems can be embedded into HoTT using HITs: TLA+ states become types, TLA+ actions become path constructors, and TLA+ safety properties become propositions about paths. The HIT formalization provides machine-checkable correctness proofs.

## Application 2: Topological Data Analysis and Persistent Homology

Persistent homology is the main tool of topological data analysis. Given a dataset (a finite set of points in R^n), persistent homology tracks how topological features (connected components, loops, voids) appear and disappear as we vary a scale parameter r.

The key construction: the *Vietoris-Rips complex* VR(X, r) is the simplicial complex whose k-simplices are sets of (k+1) points in X with pairwise distance ≤ r. As r increases, VR(X, r) grows, and its topology changes.

In HoTT, the Vietoris-Rips complex can be modeled as a HIT: points of X are point constructors, and for each pair (x,y) with d(x,y) ≤ r, a path constructor connects them. Higher-dimensional features (loops, voids) appear as 2-cells and higher.

Persistent homology corresponds to tracking how the path structure of these HITs changes with r. The persistence diagram — the main output of TDA — records the birth and death of path-components (h-level 0 features), loops (h-level 1), and higher features. This is precisely the h-level hierarchy applied dynamically.

Machine learning applications: persistent homology is used to classify data by shape (brain networks, protein structures, financial time series). The HoTT formalization gives rigorous foundations for these applications and, via cubical type theory, computationally efficient implementations.

## Application 3: Homotopy-Based Machine Learning — Graph Neural Networks

Graph neural networks (GNNs) process data on graph-structured inputs. The key challenge: GNNs must be equivariant under graph automorphisms — the same graph, drawn differently, should give the same output.

The HIT formulation of graphs makes this automatic. A graph can be defined as a HIT with:
- Vertex constructors: v : G for each vertex
- Edge constructors: e : v₁ = v₂ for each edge (v₁,v₂)
- Relations: higher path constructors for any additional structure

Graph automorphisms are self-equivalences of this HIT. By the Univalence Axiom and the transport machinery, any type-theoretically expressible function on graphs is automatically equivariant — invariant under graph automorphisms.

This provides the formal foundation for the *equivariant machine learning* program: designing neural networks that respect symmetries of the input data. The HIT formulation ensures equivariance is not a design choice but a consequence of the foundational framework.

## Application 4: Synthetic Algebraic Geometry

Grothendieck's approach to algebraic geometry uses *schemes* — geometric objects built by gluing algebraic patches (affine schemes). The gluing is a colimit construction — precisely a pushout.

Higher inductive types provide a synthetic foundation for algebraic geometry, where schemes are defined directly as HITs:
- An affine scheme Spec(R) is a type with point constructors given by prime ideals of the ring R.
- Gluing of affine schemes along open subsets is a pushout of HITs.
- A scheme is the resulting pushout type.

The great advantage: in the HIT formulation, many properties of schemes that are theorems in classical algebraic geometry become *definitional* — they follow from the HIT structure without proof. For example, the fact that functions on a glued scheme are functions on the pieces that agree on the overlap is exactly the universal property of the pushout.

This program — *spectral algebraic geometry* in HoTT — is being actively developed. The Condensed Mathematics program of Clausen and Scholze, which reformulates algebraic geometry using condensed sets, may have a HoTT incarnation using HITs for the condensed structure.

## Application 5: Quantum Computing — Topological Quantum Computing

Topological quantum computing (TQC) uses *anyons* — quasi-particles in 2-dimensional quantum systems that have non-abelian statistics. When anyons are braided (one moved around another), the quantum state transforms by a unitary matrix, and these matrices form a representation of the braid group.

The braid group is the fundamental group of the configuration space of n points in the plane: π₁(Conf_n(R^2)) = Bₙ (the braid group on n strands).

In HoTT, the configuration space Conf_n(R^2) can be modeled as a HIT, and its fundamental group is the braid group. The path constructors correspond to the generators of the braid group (elementary braids), and the path-of-path constructors correspond to the braid relations.

The quantum computation performed by braiding anyons is exactly the representation of the fundamental group — transport along the path (braid) acts on the quantum state (an element of the Hilbert space fiber). The correctness of a topological quantum computation — that it implements the desired unitary — is a statement about paths in the configuration space HIT, verifiable by type-checking.

This connection suggests that HoTT, with its HITs, provides the right foundational framework for topological quantum computing — making the topology of the computation explicit and checkable.
