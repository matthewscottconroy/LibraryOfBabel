# Thought Experiments: Point-Set Topology

## 1. The Blind Topologist

Imagine you are a mathematician who has been locked inside an unknown topological space. You cannot measure distances — no ruler, no protractor, no GPS. But you can detect which sets are "open" (you have a device that tells you, for any subset you describe, whether it is open or not). You can also detect paths: you can move continuously through the space and tell when you have returned to your starting point.

*Questions to ponder:*
- Could you determine whether you are in a circle or a line? (Hint: in the circle, every loop is homotopic to some power of the fundamental loop. In the line, every loop is contractible.)
- Could you determine whether the space is compact? (You would need to determine whether every open cover has a finite subcover — but with only one point of view, this seems difficult.)
- Could you determine whether the space is $\mathbb{R}^2$ or $\mathbb{R}^3$? (Removing a point from $\mathbb{R}^2$ disconnects it; removing a point from $\mathbb{R}^3$ does not. But you are already inside the space.)
- What is the minimum information you would need to reconstruct the homotopy type of the space? (Answer: all homotopy groups $\pi_n$ for all $n$ — that is, the Postnikov tower of the space.)

The thought experiment reveals what topology *can* and *cannot* do. It can detect connectivity, compactness, and algebraic invariants. It cannot detect the "size" of a space (the cardinality of the point set) or its specific metric structure.

## 2. The Gluing Machine

You have a machine that, given a topological space and a subset of its points, can glue those points together (identify them all to a single point). You also have a machine that, given two spaces, can take their product, their disjoint union, or their quotient by any equivalence relation.

Starting from just two spaces — a single point $\{*\}$ and the interval $[0,1]$ — construct the following spaces using only these machines:
- The circle $S^1$.
- The sphere $S^2$.
- The torus $T^2$.
- The Möbius band.
- The figure-eight $S^1 \vee S^1$.

Can you construct every CW complex this way? (Yes: every CW complex is built by iterating the "attach a disk" operation, which is a pushout — a kind of quotient of a disjoint union.) Can you construct every topological space? (No: not every space is a CW complex, and the construction is restricted to spaces that can be built by gluing disks together.)

*The HoTT version:* In HoTT, the "gluing machine" is the higher inductive type mechanism. You declare constructors (the "generators" of the type) and path constructors (the "gluing instructions"). Every HIT is a type built by this machine. The thought experiment is asking: what is the expressive power of HITs?

## 3. The Continuity Challenge

You are given 10 functions from $\mathbb{R}$ to $\mathbb{R}$. For each, decide: is it continuous? If not, where does continuity fail, and what topology on the domain or codomain would make it continuous?

1. $f(x) = \lfloor x \rfloor$ (the floor function).
2. $g(x) = x \sin(1/x)$ for $x \neq 0$, and $g(0) = 0$.
3. $h(x) = 0$ for $x$ irrational, $h(x) = 1/q$ for $x = p/q$ in lowest terms.
4. $k(x) = 1$ for $x > 0$, $k(0) = 0$, $k(x) = -1$ for $x < 0$.
5. $m(x) = 0$ for $x \leq 0$, $m(x) = e^{-1/x}$ for $x > 0$.

*The topological insight:* The floor function is continuous from $\mathbb{R}$ with the Sorgenfrey topology to $\mathbb{Z}$ with the discrete topology (check: preimages of singletons $\{n\}$ are $[n, n+1)$, which are open in the Sorgenfrey topology). The Thomae function $h$ (function 3) is continuous at every irrational and discontinuous at every rational — a dense set of discontinuities. This shows that the set of discontinuities of a function can be dense, even for functions defined on the full real line.

## 4. Compactness as Finiteness

Finite sets have a trivial property: any open cover has a finite subcover (the whole cover is already finite). Compactness extends this to infinite sets. But *how much* does a compact space behave like a finite set?

Consider the following properties of finite sets:
- Every function $f : X \to Y$ is "bounded" in the sense that $f(X)$ is finite, hence contained in a compact set.
- Every sequence has a constant subsequence (since there are finitely many values).
- The intersection of any decreasing sequence of non-empty subsets is non-empty.

Which of these properties does a compact space inherit (in the appropriate topological generalization)?
- Continuous functions on compact spaces have compact (hence closed and bounded) images. ✓
- Every sequence in a compact metric space has a convergent subsequence. ✓
- Every decreasing sequence of non-empty closed subsets of a compact space has non-empty intersection. ✓ (This is the finite intersection property.)

*The insight:* Compact spaces are exactly the spaces where the finite intersection property holds: a collection of closed sets has non-empty intersection if every finite subcollection does. This is dual to the open cover definition and reveals that compactness is about the "coherence" of the space — you cannot "escape to infinity."

## 5. The Separation Hierarchy

The Hausdorff condition ($T_2$) says: distinct points can be separated by open sets. There is a whole hierarchy of separation axioms, from $T_0$ (distinct points are topologically distinguishable) up to completely normal ($T_5$) and perfectly normal ($T_6$).

Consider what would break if you dropped the Hausdorff condition:
- The Zariski topology on $\text{Spec}(R)$ is $T_0$ but not $T_1$: the closure of a prime ideal $\mathfrak{p}$ is the set of all primes containing $\mathfrak{p}$.
- The Sierpiński space is $T_0$ but not $T_1$: the two points are topologically distinguishable ($\{1\}$ is open, $\{0\}$ is not), but you cannot separate them by open sets.
- A non-Hausdorff space can have sequences that converge to multiple limits simultaneously.

*The HoTT angle:* In HoTT, the "Hausdorff" condition on a type is related to its being a *set* (a 0-type): a type is a set if all its identity types are mere propositions. A set in HoTT is like a Hausdorff space in topology: identity/closeness is well-behaved, and "equal" and "not equal" are the only options. Higher types (groupoids, 2-groupoids, ...) are like non-Hausdorff spaces: points can be "connected" in multiple non-equivalent ways, and the connection structure is itself interesting.
