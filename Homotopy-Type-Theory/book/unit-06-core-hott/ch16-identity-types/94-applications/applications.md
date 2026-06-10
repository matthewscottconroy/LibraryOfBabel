# Applications: Identity Types and Paths

## Application 1: Formal Verification of Security Protocols

Security protocols depend critically on the notion of equality — two messages are authentic iff they match an expected value; two cryptographic keys are the same iff they produce the same outputs; a hash is valid iff it equals the expected digest. In formal verification of security protocols, identity types provide the language for stating and proving these equalities.

The path-theoretic view of equality is directly relevant. In a security protocol, there may be *multiple proofs* that a message is authentic — one per valid certificate chain. In classical logic, these would all be collapsed to a single "yes." In HoTT, the different certificate chains are different elements of the identity type, and we can reason about their relationship.

Concretely: in the Tamarin Prover and ProVerif, security protocols are verified by reasoning about equality in symbolic models. The path-level reasoning of HoTT translates to reasoning about different derivations of the same security property, which is precisely what security analysis requires. The ap and transport operations correspond to substitution of equal values in security contexts — transport ensures that substituting an authenticated value preserves the security property in a type-safe way.

## Application 2: Topological Data Analysis and Machine Learning

Topological Data Analysis (TDA) uses the shape of data — its connected components, loops, and higher topological features — to extract invariants robust to noise. The identity types of HoTT provide the correct foundational language for TDA.

In particular: persistent homology, the main tool of TDA, tracks how topological features (connected components = paths, loops = 1-cycles, etc.) appear and disappear as a parameter varies. The "paths" tracked by persistent homology are exactly the paths of HoTT identity types, classified by the h-level hierarchy of Chapter 17.

Machine learning applications: neural network architectures that respect topological symmetries use the fact that their loss functions are invariant under certain equivalences. The Univalence Axiom and its consequences (that equivalent structures are equal) provide the formal foundation for "invariant learning" — learning functions that treat equivalent inputs identically. The ap operation — that functions act on paths — corresponds to the statement that a learning model preserves topological symmetries.

## Application 3: Formal Mathematics — The Kepler Conjecture

The Flyspeck project (Formal Proof of the Kepler Conjecture), completed in 2014, is one of the most complex formal verification efforts in history. The Kepler Conjecture — that the densest packing of spheres in R³ is the face-centered cubic packing — was proved by Hales in 1998 using extensive computer calculation, and the Flyspeck project verified this proof in HOL Light and Isabelle.

The identity-type machinery of HoTT is directly relevant to such formalization efforts. Every equality in the proof — between real numbers, between geometric configurations, between volumes — must be formally witnessed. The path-theoretic view makes explicit which equalities require proof by computation (definitional equality) and which require genuinely non-trivial argument (propositional equality).

More directly: the transport operation corresponds to "rewriting under context" — substituting one equal value for another in a complex expression. The correctness of this substitution, in a context-dependent (dependent type) setting, requires exactly the transport machinery of this chapter. Future formalizations of complex mathematical proofs will benefit from the clean type-theoretic account of equality presented here.

## Application 4: Type-Safe Software and Verified Compilers

In software, equality bugs are everywhere. Two values that *should* be equal are compared with the wrong equality predicate; two types that *should* be the same type are not recognized as such by the type checker; two representations of the same data are treated as incompatible.

The identity type and its associated machinery — transport, ap, and funext — provide the tools to prevent these bugs. A verified compiler, for example, must prove that:

1. Two intermediate representations of the same program are semantically equivalent (a path in the type of programs).
2. An optimization preserves program behavior (transport of a property along the equivalence path).
3. The composition of two optimizations is correct (the concatenation of their equivalence paths is the path of the composed optimization).

The CompCert verified C compiler uses these ideas implicitly. The HoTT-inspired reformulation makes them explicit and checkable: every program transformation carries a proof of correctness in the form of a path, and these paths compose and transport correctly by the theorems of this chapter.

## Application 5: Homotopy-Invariant Mathematical Foundations

The long-term application of the identity type machinery is to provide a foundation for mathematics that is automatically invariant under equivalence. Classical mathematics has this property informally — mathematicians routinely say "up to isomorphism" and "without loss of generality" — but the formal foundations (ZFC set theory) do not enforce it.

With identity types and the Univalence Axiom (Chapter 18), the formal system enforces equivalence-invariance: any theorem proved about a type A automatically holds for any type B equivalent to A, because A = B (by Univalence) and transport carries the theorem from A's proof to B's. This is the formal version of "without loss of generality."

Applications include: algebraic geometry (where isomorphic schemes should be treated identically), category theory (where equivalent categories should be interchangeable), and mathematical physics (where symmetry-equivalent configurations should satisfy the same laws). The identity type is the technical foundation for a mathematics that is finally as invariant as it always claimed to be.
