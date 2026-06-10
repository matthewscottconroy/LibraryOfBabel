# 94 — Applications

## How Research-Level HoTT Connects to the World

The previous chapters have been about what HoTT is and what it can prove. This chapter is about what it *does* — how research-level HoTT connects to active work in cryptography, quantum computing, AI alignment, programming languages, algebraic geometry, physics, and the long-term project of machine-verified science.

These connections are not metaphors and not aspirations. Each of the following applications is the subject of active research, with named people, specific papers, and concrete open problems. The connections range from well-established (formal verification of cryptographic protocols) to speculative but well-posed (the M-theory formalization program). In each case, the path from HoTT to the application is described concretely enough that you can assess whether it is a research program worth joining.

---

## Application 1: Formal Verification of Cryptographic Protocols

**The connection.** Cryptographic protocols — the protocols that secure internet communication, blockchain transactions, and authentication systems — are notoriously difficult to prove correct. The attack surface is large: a protocol might be individually correct (each step follows the rules) but fail under composition (two correct protocols, run together, create a vulnerability). The history of cryptography is full of protocols that were "obviously correct" until someone found the attack.

Formal verification attacks this by proving protocol correctness mathematically, in a proof assistant, from explicit axioms. The verification checks not just that the protocol does what it is supposed to do, but that it does *nothing else* — no information leaks, no unexpected side channels.

**Where HoTT fits.** Protocol correctness is fundamentally about *types*: what information is available to each party, what can be derived from what, and what cannot be derived from what (security). Type theory is the natural framework for reasoning about this, because types literally encode what information is present at each point in the computation.

HoTT adds specific value in two ways:

First, *quotient types and information hiding.* A cryptographic key is not just a bit string; it is a bit string modulo the equivalence relation of cryptographic indistinguishability (two keys are equivalent if no computationally bounded adversary can distinguish them). In HoTT, quotient types are first-class: the type of keys can be the quotient type, and the elimination principle ensures that any function on keys is automatically invariant under the equivalence. This is not possible in classical type theory (without quotients) and awkward in set-theoretic foundations.

Second, *proofs of non-derivability.* HoTT's identity types reason about what can and cannot be proved equal. Security proofs often require showing that a value cannot be derived from given information — formally, that there is no term of a certain type. In HoTT, this is a statement about the homotopy type of the function space, which is exactly the kind of reasoning HoTT is designed for.

**Current state.** The EasyCrypt proof system (used by Microsoft Research and academic cryptographers) is not based on HoTT, but ongoing work in the type theory community (including work by Benedikt Ahrens and collaborators) is developing HoTT-based frameworks for protocol verification. The connection between dependent types and cryptographic security has been explored in the academic literature (Swamy et al., "Dependent Types for Secure Information Flow," among others).

**The next step.** A formal verification of the TLS 1.3 handshake protocol in Cubical Agda, using HoTT's quotient type machinery for the cryptographic equivalences. This is a concrete, achievable project that would demonstrate HoTT's value for real-world security verification.

---

## Application 2: Type Theories for Quantum Computing

**The connection.** Quantum computing requires a fundamentally different model of computation: operations are unitary transformations, states are superpositions, and measurement is irreversible. Classical type systems (even Haskell's) cannot enforce these constraints — a classical type system cannot prevent you from copying a quantum state (violating the no-cloning theorem) or from applying a non-unitary operation.

**Linear types and quantum computation.** The no-cloning theorem in quantum mechanics says that quantum states cannot be copied. In type theory, this corresponds exactly to linearity: a linear type system enforces that each variable is used exactly once. If quantum states have linear types, then copying is a type error.

HoTT's connection to this: the path algebra of HoTT can be interpreted as encoding the reversibility of quantum operations. A path p : a = b represents a reversible transition from state a to state b; the inverse path p⁻¹ : b = a represents the reverse operation. The homotopy-theoretic structure (path concatenation, naturality of paths) corresponds to the group structure of unitary operators.

**Homotopy-theoretic quantum protocols.** Some quantum protocols are naturally described in terms of homotopy: the topological quantum computing approach uses anyons — particles in 2+1 dimensional space-time whose exchange statistics are described by braid groups, which are fundamental groups of configuration spaces. The path space of a configuration of n anyons is the n-strand braid group. A quantum computation is a path in this configuration space.

In HoTT, this is literally the situation HoTT is designed to handle: the fundamental group of a type, computed synthetically. A type theory for topological quantum computing would have the braid groups as fundamental groups of configuration types, and quantum computations as paths.

**Current work.** Pablo Andres-Martinez, Chris Heunen, and collaborators have been developing type theories for quantum circuits. Brent Yorgey and colleagues have worked on functional quantum programming (Quipper). The connection to HoTT is recognized but not yet formalized.

**The open question.** Can HoTT's linear types (or a graded version of HoTT that enforces linearity at the type level) give a formal foundation for quantum programming that enforces no-cloning, unitarity, and reversibility by the type system? What would the elimination principle for quantum types look like?

---

## Application 3: Formal Specification in AI Alignment

**The connection.** AI alignment is the problem of specifying, formally and precisely, what behavior we want an AI system to exhibit, and then verifying that a given system actually exhibits that behavior. The difficulty is that natural language specifications are ambiguous, and informal mathematical specifications have gaps that an intelligent system can exploit.

**Why type theory.** Type theory provides a language for formal specification: types are specifications, and a program of that type is a proof that the specification is met. If you can write the desired behavior of an AI system as a type, then verifying the system's behavior reduces to type-checking.

**HoTT's specific contribution.** Many desirable properties of AI systems are *invariance properties*: the system should behave the same way on equivalent inputs (e.g., two phrasings of the same question should give equivalent answers). In classical type theory, "equivalent" means "definitionally equal," which is too strong (different phrasings are not definitionally equal). In HoTT, "equivalent" can mean "equal up to the relevant equivalence relation," with the equivalence relation specified as a HIT or a quotient type. The elimination principle then ensures that the system's behavior is automatically invariant.

**Reward hacking and univalence.** One form of reward hacking is when a system achieves a high reward score by exploiting a difference between the specification (the reward function) and the intended behavior (what the reward function was supposed to capture). In type-theoretic terms, this is the gap between the type (the reward function's type) and the intended semantics (what the type was supposed to capture). Univalence, which identifies equivalent types, could in principle close this gap — but only if the equivalence relation captures the intended semantics correctly.

This is speculative but not unprincipled. The connection between univalence and the invariance of specification is real; making it precise enough to be useful for alignment is the research challenge.

**Current work.** The MATS program (Machine Learning Alignment Theory Scholars) and related organizations are exploring formal methods for AI alignment. Dmitri Zaitsev and colleagues at the Alignment Forum have written about type-theoretic approaches to alignment specification. The field is very early; a graduate student entering now could shape its direction.

---

## Application 4: Future Programming Languages Based on HoTT

**The connection.** The Curry-Howard correspondence (propositions as types, proofs as programs) has driven the design of functional programming languages for decades: Haskell's type classes, Agda's dependent types, Coq's inductive types, Lean's tactic language. HoTT extends this correspondence with the homotopy-theoretic content of identity types.

**What HoTT-based programming languages would offer.** A programming language based on HoTT would provide:

*Automatic transportation.* If you have a program that works on type A and you know A ≃ B (an equivalence), transport gives you a program that works on B. In current practice, this requires manual boilerplate. HoTT's transport is the formal guarantee that no such boilerplate is needed — the program transfers automatically along the equivalence.

*Quotient types as first-class citizens.* Many programming abstractions are naturally quotient types: abstract data types (the implementation is hidden behind an interface — i.e., the type is quotiented by implementation equivalence), database views (rows are equivalent if they match the view), and protocol sessions (message sequences are equivalent if they have the same effect). HoTT's quotient HITs would give a clean foundation for all of these.

*Correct-by-construction proofs.* A program with a proof that it satisfies its specification is correct by construction — you cannot separate the implementation from the proof. Current practice relies on tests and review. HoTT-based languages would make the proof part of the program itself.

**Where this is happening.** The Cubical Agda language is itself a programming language (programs are terms, evaluation is computation). The HoTT-based work on Arend (a proof assistant developed at JetBrains), on the Andromeda type theory (Bauer et al.), and on MLTT extensions (Nuprl, RedPRL, cooltt) all push in this direction. Specific efforts to design programming languages with HoTT-style quotients include work by Conor McBride (on "ornaments" and type-directed development) and others.

**The key open problem.** A practical programming language based on HoTT needs computationally efficient quotient types — efficient enough that everyday programming does not pay a performance penalty for the formal guarantees. The CCHM cubical computation rules are a step in this direction; efficient implementation of HITs in a programming language context is an active research area.

---

## Application 5: Formalizing Modern Algebraic Geometry

**The connection.** Modern algebraic geometry (post-Grothendieck, post-Deligne) is built on a hierarchy of structures: sheaves, stacks, ∞-stacks, derived algebraic geometry, and (most recently) condensed mathematics (Scholze). These structures are notoriously difficult to formalize: the definition of a scheme requires understanding sheaves of rings on topological spaces, which requires understanding the site structure, which requires category theory, and so on through layers of abstraction.

**HoTT's approach.** In HoTT, a scheme could be defined synthetically: instead of building it from sets and functions and topology, you define it as a type with appropriate properties (locally isomorphic to Spec(R) for a ring R, where "locally" is defined via a Grothendieck topology formulated as a modality). This is the cohesive HoTT approach: the smooth or algebraic geometry is encoded in the modalities, and the scheme is a type with the right relationship to these modalities.

**Current work.** David Jaz Myers' cohesive HoTT framework (2021) provides the axioms. The connection to algebraic geometry has been developed in outline by Shulman and Myers for the smooth setting. The algebraic setting (schemes, stacks) is less developed but the framework is there.

**Condensed mathematics and Lean 4.** The "Liquid Tensor Experiment" (Scholze, Commelin, and the Lean community, 2020–2021) formalized a key theorem from condensed mathematics in Lean 4 — a major achievement that demonstrated the feasibility of formalizing cutting-edge algebraic geometry. This used classical foundations (Lean 4 is not HoTT). The question of whether cohesive HoTT provides a cleaner foundation for the same mathematics is an active research program.

---

## Application 6: Schreiber's Program — Formalizing M-Theory

**The connection.** Urs Schreiber has been developing, since approximately 2012, a formalization of M-theory (the conjectured 11-dimensional theory that unifies the various string theories) using cohesive higher-categorical mathematics. His claim is that the geometric structures of M-theory — higher gauge fields, M-branes, flux quantization — are most naturally described using cohesive ∞-toposes, and that HoTT with appropriate cohesion axioms provides a type-theoretic foundation for this description.

**What Schreiber's program entails.** M-theory involves:
- Higher gauge fields: connections on higher principal bundles (not just U(1) connections as in electromagnetism, but connections on bundles with fiber an ∞-group)
- Flux quantization: the quantization conditions on these higher gauge fields, which are conditions about cohomology classes
- M-branes: extended objects (membranes) on which higher gauge fields can end
- The Green-Schwarz anomaly cancellation: a consistency condition connecting the gravitational and gauge degrees of freedom

In Schreiber's formulation, these structures are all expressed using cohesive HoTT:
- Higher gauge fields are morphisms in the cohesive ∞-groupoid of connections
- Flux quantization is a statement about the shape modality applied to the space of connections
- The anomaly cancellation is a homotopy-theoretic identity in the cohesive ∞-category of fields

**Current status.** Schreiber has published detailed papers developing this framework at a mathematical level (in the journal Annals of Physics and on the arXiv). The formalization in a proof assistant remains largely undone — the framework is developed informally, and the translation to Cubical Agda or any other proof assistant is a major open project.

**What formalization would require.**
- A Cubical Agda formalization of smooth cohesive HoTT (partly available in Myers' work)
- Definitions of higher principal bundles and connections in Cubical Agda
- Formalization of the flux quantization conditions
- Verification that the Green-Schwarz mechanism gives the correct anomaly cancellation

This is a long-term program — years of work for a research team. But the mathematical framework is specified precisely enough that it is not vague. Schreiber's nLab pages provide hundreds of pages of mathematical content that would need to be formalized.

---

## Application 7: Machine-Verified Mathematics and Science

**The long view.** What would it mean for all of mathematics to be machine-verified?

Not just theorems that have been checked by a computer — we already have thousands of those. But a state of affairs where:
(a) Every published mathematical result is accompanied by a machine-checkable proof
(b) The proof assistant can check any new result that builds on previously verified results
(c) The foundation is rich enough that any current area of mathematics can be expressed in it

This is not a fantasy. It is the direction in which mathematics is moving. The question is the foundation.

**Why HoTT is positioned well.** HoTT has two properties that make it especially suitable for this goal:
- *Alignment with mathematical practice.* Mathematicians routinely identify isomorphic structures; HoTT makes this formally valid via univalence. This means HoTT proofs are closer to mathematical practice than ZFC proofs, and formalization is correspondingly less painful.
- *Constructive content.* In HoTT, every proof is a computation. The logical content of a theorem (that something exists, that something is possible) corresponds to an actual program. This means formalized mathematics produces not just verified theorems but verified algorithms.

**What machine-verified science would look like.** If mathematics is machine-verified, scientific models built on that mathematics could be machine-verified too. A model of climate change, a drug simulation, a financial stress test — all built on verified mathematical foundations, with verified computational implementations. The gap between "we believe the model is correct" and "we can verify the model is correct" is exactly the gap that machine-verified mathematics closes.

This is a decades-long project. It begins with what the HoTT community is already doing: formalizing algebraic topology, one theorem at a time. Each formalized theorem is a permanent, checkable artifact — part of the permanent mathematical record. The rate at which this record grows depends on how many people join the work.

The door is open. The work is real. The beginning is the Cubical Agda library, the Zulip, the HoTTEST seminar, and the problem you decide to work on first.
