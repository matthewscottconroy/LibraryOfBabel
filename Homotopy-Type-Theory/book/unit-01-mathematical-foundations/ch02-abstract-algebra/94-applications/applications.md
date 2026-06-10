# Applications: Abstract Algebra

## 1. Cryptography: Groups as the Basis of Encryption

Modern public-key cryptography is built on group theory. The security of RSA, Diffie-Hellman key exchange, and elliptic curve cryptography all rest on the computational difficulty of certain group-theoretic problems.

**RSA** uses the multiplicative group (ℤ/nℤ)* for n = pq (product of two large primes). The security rests on the difficulty of factoring n — which is equivalent to computing the order of the group, which allows decryption.

**Diffie-Hellman** uses a cyclic group of large order. Alice and Bob want to establish a shared secret over an insecure channel. They agree on a group G with generator g. Alice picks secret integer a, sends g^a. Bob picks secret integer b, sends g^b. Both compute g^{ab} = (g^b)^a = (g^a)^b. An eavesdropper sees g^a and g^b but must solve the *discrete logarithm problem* (find a from g and g^a) to recover the secret.

**Elliptic curve cryptography (ECC)** replaces the cyclic group ℤ/pℤ with the group of points on an elliptic curve over 𝔽_p. Elliptic curve groups have the same abstract structure (abelian groups) but are harder to compute in — smaller keys suffice for the same security level.

Group homomorphisms are central: the encryption and decryption operations are group homomorphisms, and their security relies on the computational hardness of inverting specific maps.

## 2. Coding Theory: Groups and Error Correction

When you transmit data over a noisy channel, some bits will be flipped. *Error-correcting codes* add redundancy so that the original message can be recovered even with some errors.

A *linear code* is a subgroup of (𝔽₂)ⁿ (binary vectors of length n under componentwise addition mod 2). The Hamming distance between two codewords is the number of positions where they differ. A code can correct t errors if the minimum Hamming distance between any two codewords is at least 2t + 1.

The structure theory of linear codes uses the theory of modules over 𝔽₂ (linear algebra over a field with 2 elements). Generator matrices, parity check matrices, and the coding/decoding algorithms are all linear algebra operations.

More sophisticated codes — BCH codes, Reed-Solomon codes (used in CDs, DVDs, QR codes) — use the structure of finite fields and polynomial rings. A Reed-Solomon code encodes a message as a polynomial over 𝔽_q (a finite field), sends its values at multiple points, and recovers the polynomial from enough values even with errors. The algebraic structure ensures efficient encoding and decoding.

## 3. Symmetry in Physics: The Standard Model

The fundamental particles and forces of nature are organized by symmetry groups. The *Standard Model* of particle physics is built on the Lie group G = SU(3) × SU(2) × U(1).

- SU(3): the symmetry group of the strong nuclear force (quantum chromodynamics). Quarks come in three "colors," and SU(3) symmetry dictates how they interact.
- SU(2): the symmetry group of the weak nuclear force. The Higgs mechanism breaks this symmetry, giving particles their mass.
- U(1): the symmetry group of electromagnetism.

The particles of the Standard Model are classified by *representations* of G — the ways G can act as a group of linear transformations on a vector space. Each particle is characterized by its representation (its "quantum numbers"). The force-carrying bosons are the generators of the Lie algebras of these groups.

This is group theory applied to the deepest level of physical reality: the specific group structure of G determines what particles exist, what forces there are, and how they interact.

## 4. Crystallography: Space Groups and Material Science

The internal structure of a crystal — the arrangement of its atoms in a periodic lattice — has the symmetry of one of exactly 230 *space groups* (in 3 dimensions). Space groups are subgroups of the Euclidean symmetry group that leave a lattice invariant.

This classification — completed in 1891 by Fedorov, Schönflies, and Barlow, independently — determines what kinds of crystalline structures are possible. Every known crystal has its symmetry described by one of these 230 groups. The classification uses the theory of group extensions, lattice theory, and the representation theory of finite groups.

In material science, the symmetry group of a crystal determines its physical properties: which directions transmit light differently, which axes are electrically neutral, whether the material is ferroelectric or piezoelectric. Symmetry breaking (phase transitions) corresponds to passing from a higher-symmetry group to a subgroup.

## 5. Robotics and Computer Vision: Lie Groups and Motion

The motion of a rigid body in 3-dimensional space is described by the special Euclidean group SE(3) = SO(3) ⋉ ℝ³ — a Lie group combining rotations (SO(3)) and translations (ℝ³). Elements of SE(3) are pairs (R, t) where R is a rotation matrix and t is a translation vector.

In robotics, a robot arm's *configuration space* (the space of all possible arm positions) is a Lie group or a manifold acted on by a Lie group. Path planning — finding a continuous motion from one configuration to another — is a problem in the topology of this configuration space. Fundamental groups and covering spaces appear naturally: whether a planned motion can be continuously deformed to avoid obstacles.

In computer vision, *structure from motion* (reconstructing a 3D scene from 2D photographs) uses the group structure of camera motions. Fundamental matrices and essential matrices encode the relative position of two cameras as elements of specific matrix groups.

## 6. Formal Verification: Algebraic Proofs in Proof Assistants

The Lean Mathematical Library (Mathlib) contains extensive formal algebraic theory: groups, rings, fields, modules, Galois theory, and algebraic number theory, all machine-verified.

A significant challenge in formalizing algebra is that isomorphic structures must be treated as "the same" in practice, but are technically different terms in the type theory. This is exactly the identity problem of Chapter 1. Mathlib handles it through *typeclasses*: a group structure on a type is a typeclass instance, and the same type can have multiple group structure instances.

The Univalence Axiom of HoTT offers a cleaner solution: two groups (group structures on types) that are isomorphic (connected by a type equivalence respecting group structure) are literally equal. Transporting theorems across isomorphisms is trivial, not a separate proof obligation. This makes algebraic formalization in HoTT cleaner and more natural than in classical type theory.

Current HoTT-based proof assistants (like Agda with the HoTT library) can formalize and verify the basic algebraic structures of this chapter. The fundamental group computation π₁(S¹) = ℤ is a milestone formalization result, requiring all the machinery of higher inductive types and the algebraic theory of the integers.
