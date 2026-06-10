# Chapter 2: Abstract Algebra — Groups, Symmetry, and the Seeds of Topology

## What This Chapter Is About

Mathematics has a recurring trick: take a complicated, specific thing — the rotations of a cube, the shuffling of a deck of cards, the modular arithmetic your clock uses — and ask what all those things have *in common*. Strip away everything particular, keep only the structure, and study that structure directly. What you get is called *abstract algebra*, and it turns out to be extraordinarily powerful.

In this chapter, the central object is a **group**. A group is an abstraction of the idea of *symmetry*: things that can be combined, undone, and done nothing with. That sounds vague, and it is at first. But the precision comes from axioms, and the power comes from the fact that once you prove something about groups *in general*, you've proven it about every particular case simultaneously.

There are two reasons this chapter matters deeply for the rest of the curriculum.

**First:** Group theory is the algebraic backbone of topology. When you study the shape of a space — whether it has holes, how loops can be wound around those holes, how spaces can be continuously deformed into one another — the fundamental questions are answered in the language of groups. The *fundamental group* of a topological space is literally a group, and understanding what it is and how to compute it is most of what algebraic topology is about.

**Second:** *Free groups* — groups built from pure symbols with no imposed relations — are the algebraic shadow of paths in a space. When a loop in a topological space goes around one hole, then another, then backtracks — that's exactly the structure of a word in a free group. And in Homotopy Type Theory, this becomes the *definition*: the identity type (the type of proofs that two things are equal) is modeled by paths, which are modeled by free group elements. The algebra you learn here is not just preparation — it is literally the algebra of equality proofs.

## The Style of Algebraic Reasoning

Before diving in, it's worth appreciating the *style* of algebra. In earlier mathematics, you might prove things by computation: plug in numbers, manipulate expressions, get an answer. Algebra takes a different approach. You prove things by *reasoning about the axioms directly*, without any specific example in mind. The proofs feel more like logic puzzles than calculations.

This style — sometimes called *axiomatic* or *structural* mathematics — is also the style of type theory and proof assistants. When you write a proof in Lean or Coq, you're doing exactly this: reasoning from hypotheses and rules, without reference to any "underlying reality." Abstract algebra is a beautiful place to practice this mode of thought.

## Roadmap

Here's what we'll cover, and why each piece matters:

1. **Groups** — the definition, key consequences, and a zoo of examples. We prove things like "the identity is unique" directly from the axioms, which is a gentle warm-up for structural reasoning.

2. **Subgroups, Cosets, and Lagrange's Theorem** — a group can contain smaller groups, and these smaller pieces carve the big group into equal-sized chunks. Lagrange's theorem is the first deep result: it constrains the sizes of subgroups dramatically.

3. **Normal Subgroups and Quotient Groups** — when a subgroup is "symmetric enough," you can collapse the big group by it to get a new, smaller group. This is the algebraic version of "modding out by an equivalence relation."

4. **Homomorphisms and the Isomorphism Theorems** — the structure-preserving maps between groups. The isomorphism theorems describe how maps and quotients interact. These are among the most used tools in all of algebra.

5. **Free Groups** — groups with generators and no relations. They are the most "unconstrained" groups, and their structure directly corresponds to paths in spaces.

6. **Group Presentations** — how to describe any group as a quotient of a free group. Every group has a presentation, and the presentations of topological spaces' fundamental groups directly encode the topology.

7. **Group Actions** — groups acting on sets, which gives a unified framework for understanding symmetry. Cayley's theorem shows every group is a symmetry group.

8. **Rings and Fields** — a brief look at algebraic structures with two operations. They appear when we need to count things (homology groups), or work over a field (cohomology rings).

9. **The Bridge to Homotopy** — a preview of how everything connects. This table becomes a *definition* in HoTT.

Let's begin.
