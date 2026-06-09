# 6.4 Minimality

Transitivity gives you one dense orbit. Minimality asks: what if *every* orbit is dense? That's a much stronger condition, and it describes a kind of ultimate homogeneity — no part of the space is dynamically "more important" than any other.

**Definition 6.4.1.** A TDS $(X, f)$ is *minimal* if every orbit is dense: $\overline{\mathcal{O}(x)} = X$ for all $x \in X$. Equivalently, $X$ has no proper closed $f$-invariant subset.

The equivalence is clean and worth understanding. A closed $f$-invariant set $F \subsetneq X$ would be a "sub-system" — the orbits of points in $F$ stay inside $F$ forever. If every orbit is dense in all of $X$, no such sub-system can exist. Minimality means the dynamics are truly irreducible.

**Examples 6.4.2.**
- *Irrational rotations*: $R_\alpha: {\mathbb T} \to {\mathbb T}$, $R_\alpha(x) = x + \alpha \pmod{1}$ for $\alpha \notin {\mathbb Q}$ is minimal. (Proof: orbit of any point equidistributes — see Weyl, Chapter 31.)
- *Minimal subshifts*: Sturmian sequences, Thue-Morse, other combinatorially defined systems.
- *Subshifts of Finite Type* are never minimal (they have periodic points).

The irrational rotation is the prototypical minimal system. Its orbits are dense — Weyl proved this in the early 1900s, and the proof using Fourier analysis is elegant and worth knowing. Sturmian sequences give you something more exotic: combinatorial objects (infinite words) that turn out to carry the same homogeneous dynamics as rotations. We'll see why when we encounter symbolic dynamics.

---

## Every System Contains a Minimal Piece

Not every system is minimal, of course. The doubling map has fixed points — clearly not all orbits are dense. But here is a remarkable fact: even for non-minimal systems, you can always find a minimal piece hiding inside.

**Theorem 6.4.3 (Existence of Minimal Subsystems).** Every compact TDS $(X, f)$ contains a minimal subset (a closed $f$-invariant set on which $f$ is minimal).

*(proof)* Apply Zorn's lemma to the family of nonempty closed $f$-invariant subsets of $X$ (ordered by reverse inclusion). Any chain has a lower bound (the intersection); a minimal element in the ordering is a minimal subset.

This proof is pure abstract nonsense in the best sense — Zorn's lemma applied cleanly. The key insight is that intersections of chains of closed invariant sets are again closed and invariant (and nonempty, by compactness). So Zorn guarantees a minimal element.

**Remark 6.4.4.** The existence of minimal subsystems is a fundamental compactness argument. It is the topological analogue of the existence of ergodic components in measure theory.

The parallel to ergodic decomposition (which we'll see in Chapter 7) is not an accident. In both cases, you're proving that every system contains an "irreducible piece," and the argument in both cases rests on compactness or something like it. Keep this analogy in mind as you read Chapter 7 — the structures mirror each other.

Next, we examine the "tamest" extreme of the spectrum: what happens when the iterates of $f$ are all uniformly close together?
