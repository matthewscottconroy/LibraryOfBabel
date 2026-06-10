# The Limits of Set Theory

## What Set Theory Gets Right

Before criticizing ZFC, we should acknowledge what it achieves. It is an extraordinary intellectual accomplishment.

ZFC is *universal*: every mathematical structure can be encoded as a set. Groups, rings, topological spaces, graphs, functions, real numbers — all are sets. This universality means that ZFC provides a single language in which all of mathematics can be expressed. If you can state your theorem in the language of set theory, ZFC can (in principle) adjudicate it.

ZFC is *relatively consistent* in the best available sense: Gödel showed that if there is any consistent set theory, ZFC is consistent. More precisely: ZFC is as consistent as any set theory we know how to write down, and much more likely to be consistent than it is to be inconsistent. (We cannot prove its consistency from within, by Gödel's second incompleteness theorem, but this is a limitation of any formal system, not a defect of ZFC specifically.)

ZFC provides a *cumulative hierarchy* that organizes the universe of sets into stages, giving a clear intuitive picture. Every set lives at some level Vα. The universe is well-founded. There are no circular sets.

For these reasons, ZFC was — and remains — the dominant foundation of mathematics. The working mathematician who never thinks about foundations is implicitly working in ZFC.

## The Identity Problem

Yet ZFC has a problem that becomes visible the moment you look carefully.

**What is the number 3?**

In the von Neumann encoding: 3 = {∅, {∅}, {∅, {∅}}} = {0, 1, 2}.

In the Zermelo encoding: 3 = {{∅}} = {{{∅}}}.

Both encodings satisfy Peano's axioms. Both support all the arithmetic we need. But they are *different sets*. The element {∅} is in the von Neumann 3 but not in the Zermelo 3. If you ask "is {∅} an element of 3?", the answer depends on which encoding you chose.

This is Benacerraf's problem, posed sharply in his 1965 paper "What Numbers Could Not Be": if numbers are sets, which sets are they? And if the question has no principled answer, then numbers are not sets — or the question of which set is which number is meaningless. Either way, the formal foundation (ZFC) and mathematical practice (the unique natural numbers ℕ) come apart.

Mathematicians handle this by working *up to isomorphism*. Two groups are "the same" if they are isomorphic, regardless of their underlying sets. Two constructions of ℝ (Dedekind cuts and Cauchy sequences) are "the same" because they yield isomorphic ordered fields. The formal identity of the underlying sets is irrelevant.

But this informal convention is in tension with the formal foundation. In ZFC, two isomorphic groups are *not* the same object — they are two distinct sets that happen to have the same group structure. The mathematician says "treat them as the same"; the formal system says "they are different." The gap between practice and foundation is real.

## The Problem of Transport

The identity problem has a more specific form in mathematical practice: *transport*.

Suppose I prove a theorem about the group ℤ/6ℤ. I want to use this theorem about ℤ/2ℤ × ℤ/3ℤ, which is isomorphic to ℤ/6ℤ (by the Chinese Remainder Theorem). In practice: I just say "since the groups are isomorphic, the theorem applies." This is entirely valid mathematically. But in ZFC, the two groups are different sets — ℤ/6ℤ = {{0},{0,1},...} (with some encoding) while ℤ/2ℤ × ℤ/3ℤ = {((0,0), (0,1),...}. The theorem I proved is literally about the first set, not the second. Applying it to the second requires a formal isomorphism, and every use of a theorem about an isomorphic structure requires building a new proof.

In practice, mathematicians skip this — the transport is obvious and the formal gap is ignored. But in a formal proof assistant based on ZFC (like Metamath), every such transport must be made explicit, and proofs become much longer and less readable.

The Univalence Axiom in HoTT resolves this. Univalence says: equivalent types are equal. Two equivalent groups — groups connected by a group isomorphism that is also a type equivalence — are *literally* the same type. Transporting theorems from one to the other is not a mathematical operation you have to perform; it is trivial, because the types are definitionally equal.

## Proof Irrelevance and Computational Content

ZFC has no notion of *computational content* for proofs. A proof in ZFC is a finite sequence of formal symbols, and once you know the theorem is proved, the proof can be discarded. "There exists a prime greater than 10^100" is a theorem, and the formal proof establishes it, but the proof does not tell you which prime. The existence is shown non-constructively.

This is acceptable for classical mathematics. But for several applications — especially in computer science — we want proofs that carry computational content.

**Program extraction.** In a constructive proof of "for every n, there exists a prime p > n," the proof is itself an algorithm for finding p given n. In ZFC, this is not guaranteed: the proof might use the Axiom of Choice in a way that leaves no algorithmic trace.

**Proof assistants.** When we verify software in a proof assistant, we often want to *extract* a program from a proof of its correctness. This extraction works cleanly only in a system where proofs carry computational content — a constructive type theory like Lean or Coq.

**The distinction between proof and computation.** In type theory, a proof of a proposition *is* a program of the corresponding type. The proof-term carries information. In ZFC, proofs are external justifications, not internal objects.

## The Type-Safety Problem

ZFC is a *single-sorted* theory: there is one kind of thing (sets), and every object is a set. This means you can ask "is 3 ∈ 4?" — and in the von Neumann encoding, the answer is yes (3 ∈ {0,1,2,3} = 4). You can ask "is ℕ ∈ ℝ?" — a question that is formally meaningful in ZFC but mathematically absurd.

In a typed system, "is 3 ∈ ℕ?" has a sensible answer, but "is 3 ∈ ℝ?" is a type error. The type system prevents category mistakes at the level of syntax.

Type theory is multi-sorted by design: every term has a type, and operations are only well-formed when applied to terms of the appropriate type. This prevents the category errors that ZFC allows but practice forbids.

## Why HoTT

Homotopy Type Theory addresses all three of these problems simultaneously.

**Identity.** The Univalence Axiom says: equivalence implies identity. Two equivalent types are the same type. Isomorphic mathematical structures are literally equal in HoTT, not just "treated as" equal by informal convention. The gap between mathematical practice and formal foundation closes.

**Computational content.** HoTT is based on constructive type theory. Every proof is a program. Every existence proof carries a witness. Every theorem can be run as a computation. The content of proofs is intrinsic, not incidental.

**Type safety.** HoTT is a typed theory. Operations are only well-formed for terms of the right type. Category errors are caught at the level of syntax. The formal system enforces the informal conventions that ZFC leaves unenforceable.

HoTT also adds something new: *higher-dimensional structure*. In ZFC, equality is a proposition (either equal or not). In HoTT, equality is a type that can have multiple elements — different proofs of equality, which are themselves structured as paths. This higher-dimensional equality is what connects HoTT to topology and allows the study of mathematical structures "up to higher equivalence."

ZFC opened mathematics to the transfinite and gave it a universal language. It served well for a century. Its limitations are not failures of ingenuity but structural constraints: it was designed before the Curry-Howard correspondence was discovered, before constructive type theory was developed, before mathematicians realized they wanted to reason about the *identity of mathematical structures*. HoTT is designed with all of this in mind.

We do not abandon set theory. We use it throughout this curriculum. But we understand it now as a stepping stone — an extraordinary achievement that points beyond itself.
