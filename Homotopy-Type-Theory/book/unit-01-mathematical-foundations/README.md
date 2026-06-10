# Unit 01: Mathematical Foundations

## Why Start Here

There is a temptation, when beginning a curriculum on Homotopy Type Theory, to plunge directly into type theory. After all, HoTT is the destination. Why not go there at once?

Here is why. HoTT is a theory about mathematical structures — about what it means for two things to be equal, about how proofs carry computational content, about spaces and paths and deformations. To understand what the theory achieves, you need to already know what it is responding to. The mathematical foundations in this unit are not background decoration. They are the problem space that HoTT was invented to solve.

We cover four interconnected domains: logic and proof, set theory, abstract algebra, and real analysis. Each one contributes something irreplaceable to the story that follows.

## What Logic and Proof Contributes

Mathematical proof is not rhetorical persuasion. It is a formal structure: a sequence of statements, each following from the previous by an explicit rule. Chapter 0 builds this machinery from scratch.

But there is a deeper point, one that will resonate throughout the curriculum. Every proof in classical mathematics can be written as a program. This is the Curry-Howard correspondence, and it is not a curiosity — it is the foundational insight behind proof assistants like Lean and Coq. When you learn to write a proof by introducing a conjunction, you are doing the same thing as constructing a pair in a programming language. When you reason by cases, you are pattern-matching. When you apply modus ponens, you are calling a function.

We do not develop this correspondence fully in Chapter 0. But the proof techniques we introduce — natural deduction, quantifiers, induction — are precisely the ones whose computational interpretation we will later unpack. Study them here with an eye toward their structure, not just their use.

## What Set Theory Contributes

For most of the twentieth century, Zermelo-Fraenkel set theory with the Axiom of Choice (ZFC) was the official foundation of mathematics. Everything — numbers, functions, spaces, structures — was supposed to be a set. Chapter 1 surveys this foundation: its axioms, its constructions, its reach.

But ZFC has problems that set theorists themselves acknowledge. The biggest one is identity. In ZFC, two groups can be isomorphic — structurally indistinguishable — and yet be different sets. The mathematicians treat them as the same; the foundation says they are not. This gap between mathematical practice and official foundation is exactly what the Univalence Axiom resolves.

Understanding ZFC prepares you to understand what Voevodsky was reacting against when he developed HoTT. The axioms we study are the ones we will ultimately transcend.

## What Abstract Algebra Contributes

Groups are the mathematics of symmetry. A group is what you get when you ask: what structure do rotations, permutations, and reversible transformations all share? The axioms are spare — associativity, identity, inverses — and from them an enormous theory unfolds.

But there is a specific reason group theory matters for HoTT: the free group. A free group is built from symbols and their inverses, with no imposed relations. Its elements are strings — words — that compose by concatenation. This is precisely the structure of paths in a topological space: a path that goes through point A, then B, then backtracks through B, is exactly a word in a free group.

In HoTT, this correspondence becomes a definition. The identity type — the type of proofs that two things are equal — is modeled by paths. Paths compose like words in a free group. The algebraic structure of Chapter 2 is not merely analogous to the type-theoretic structure of equality; it is the same structure.

## What Real Analysis Contributes

Analysis makes precise the notion of continuity: what it means for a function to have no gaps, no jumps, no sudden breaks. The central objects are metric spaces — sets with a notion of distance — and the theorems about them are theorems about how continuous structure behaves under limits, convergence, and deformation.

A path in a topological space is a continuous function from the interval [0,1] into the space. This simple definition carries enormous content. Two paths that can be continuously deformed into each other — while keeping their endpoints fixed — are called homotopic. The collection of homotopy classes of loops at a point forms a group: the fundamental group.

The fundamental group is where algebra meets topology. It measures the "shape" of a space by algebraic means. And in HoTT, the shape of the identity type is exactly what the fundamental group measures. The analytic notion of homotopy and the type-theoretic notion of equality are — by design, not by accident — the same.

## How the Chapters Fit Together

The four chapters are not independent modules to be sampled in any order. They build on each other in a specific way.

Logic and proof (Chapter 0) is the grammar of all mathematical argument. Without it, nothing else can be stated precisely. Set theory (Chapter 1) is the classical foundation that we will both use and critique. Abstract algebra (Chapter 2) provides the algebraic vocabulary — groups, homomorphisms, quotients — that makes the topological and type-theoretic structures tractable. Real analysis (Chapter 3) provides the geometric intuition — paths, continuity, homotopy — that HoTT formalizes.

By the end of this unit, you will have the mathematical vocabulary to understand both what HoTT is saying and why it is the right thing to say. The remaining units will develop the formalism. This unit develops the ground.

## A Note on Style

The writing in this unit treats you as a colleague who is new to some of the material, not as a student who needs to be protected from difficulty. We move quickly, we prove things, and we connect every topic to the larger story. When a theorem is hard, we say so. When an intuition is imprecise, we flag it.

The goal is not to cover the material but to understand it — to reach the point where you could explain to someone else not just what the theorems say, but why they are true and why they matter.

That goal is achievable. Let's begin.
