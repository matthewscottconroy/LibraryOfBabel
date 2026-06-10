# Thought Experiments: Homotopy Theory

## 1. The Algebraic Fingerprint

Every topological space has a sequence of groups attached to it: $\pi_0(X)$ (its path-components), $\pi_1(X, x_0)$ (its fundamental group), $\pi_2(X, x_0)$, $\pi_3(X, x_0)$, and so on. This sequence is the algebraic fingerprint of the space's homotopy type.

Consider the following thought experiment. You are handed a black box — a topological space you cannot directly examine — but you are allowed to ask oracle queries: for any $n \geq 0$, you can ask "what is $\pi_n(X, x_0)$?" The oracle answers with a group. Your task: identify $X$ up to homotopy equivalence.

*Questions:*
- Could you identify $X$ from finitely many queries? (In general, no: the full Postnikov tower requires all $\pi_n$, and there is no bound on how high you must look.)
- What if you know $X$ is a surface (compact, connected, 2-manifold)? (Then $\pi_n = 0$ for $n \geq 2$ for aspherical surfaces, and $\pi_1$ uniquely determines the surface by the classification theorem.)
- What if you know $X$ is a sphere? (Then knowing $\pi_n(X) = \mathbb{Z}$ for some $n$ and $\pi_k(X) = 0$ for $k < n$ tells you $X \simeq S^n$, by Whitehead's theorem and the Hurewicz theorem.)
- Could two different spaces have the same $\pi_n$ for all $n$ but be different homotopy types? (Yes: a counterexample involves the lens spaces $L(7,1)$ and $L(7,2)$, which have the same homotopy groups but are not homotopy equivalent. Algebraic K-theory and Reidemeister torsion are needed to distinguish them.)

The thought experiment reveals a fundamental truth: the homotopy groups are powerful invariants but do not completely determine the homotopy type of a space. The Postnikov tower — which includes the k-invariants relating successive stages — carries additional information beyond the individual groups.

## 2. The Fundamental Group as Explorer

Imagine you are a very small creature living inside a 2-dimensional surface, unable to see beyond it. You can walk along paths, and you can detect when two paths are homotopic (you can deform one into the other within the surface). Your goal: determine the topology of the surface without going outside it.

Your main tool: loops. Start at a point $x_0$ and explore loops. If every loop can be contracted to a point, you know you live on a simply-connected surface — either the sphere $S^2$ or the disk $D^2$ (if you have a boundary). If some loop cannot be contracted, you have found a "hole."

*The experiment:*
- You find a loop $a$ that cannot be contracted, and a loop $b$ that also cannot be contracted, and $ab = ba$ (they commute in $\pi_1$). This suggests $\pi_1 = \mathbb{Z}^2$ — you might be on a torus.
- You find a loop $a$ with $a^2 = e$ (going around twice returns you to the basepoint) and no simpler relation. This suggests $\pi_1 = \mathbb{Z}/2\mathbb{Z}$ — you might be on $\mathbb{RP}^2$.
- You find a loop $a$ and a loop $b$ with $aba^{-1}b^{-1} \neq e$ — they don't commute. This means you're on a genus-$\geq 2$ surface.

The thought experiment reveals how the fundamental group encodes the "hole structure" of a surface, and why the classification of surfaces by $\pi_1$ (via the classification theorem) is the right result.

## 3. The Hopf Fibration: A Tale of Two Spheres

Here is a strange fact: you can wrap the 3-sphere around the 2-sphere in a non-trivial way, with circles as fibers. This is the Hopf fibration.

Try to visualize it. The 3-sphere $S^3$ can be written as a union of two solid tori glued along their boundary (a Heegaard splitting). Each solid torus $D^2 \times S^1$ is foliated by circles — the fibers of the Hopf fibration. The two solid tori are attached along their boundary torus in a way that gives the 2-sphere as the base.

Alternatively: the 2-sphere is divided by its equator into two disks (northern and southern hemispheres). The preimage of each hemisphere under the Hopf map is a solid torus $D^2 \times S^1$; the preimage of the equator is a torus $S^1 \times S^1$. The two solid tori are glued together along this torus, but with a twist — and this twist is what makes the Hopf fibration non-trivial.

*The conceptual question:* The Hopf fibration $\eta : S^3 \to S^2$ generates $\pi_3(S^2) = \mathbb{Z}$. The element $n \cdot [\eta] \in \pi_3(S^2)$ corresponds to a map $S^3 \to S^2$ that "wraps the Hopf fibration $n$ times." What does this mean geometrically? (Answer: these maps have Hopf invariant $n^2$, not $n$ — the Hopf invariant distinguishes maps of different "linking numbers" between fibers.)

## 4. Covering Spaces as Symmetry

The Galois correspondence between covering spaces and subgroups of $\pi_1$ is one of the most beautiful theorems in mathematics. Here is a way to make it visceral.

Consider the figure-eight $X = S^1 \vee S^1$ with fundamental group $F_2 = \langle a, b \rangle$ (the free group on two generators). Every subgroup of $F_2$ corresponds to a covering space, and every covering space is a graph.

*Explore the correspondence:*
- The trivial subgroup $\{e\}$ corresponds to the universal cover, which is the Cayley graph of $F_2$ with generators $a$ and $b$ — an infinite 4-regular tree.
- The subgroup $\langle a \rangle$ (generated by $a$) corresponds to a covering where: one base 0-cell lifts to infinitely many copies (one for each coset $n$ of $\langle a \rangle$ in $F_2$); the loop $a$ lifts to edges connecting copies, and $b$ lifts to edges going "off" in another direction.
- The subgroup $\langle a^2, b^2, ab \rangle$ (of index 2) corresponds to a 2-sheeted covering, which is another graph.

The thought experiment: given a covering space (described as a labeled graph with two types of edges, one for $a$ and one for $b$), reconstruct the corresponding subgroup of $F_2$ by determining which loops in the base lift to loops in the cover.

## 5. The Eckmann-Hilton Argument: Why Higher Groups Must Be Abelian

The Eckmann-Hilton argument is one of those mathematical results that seems to come from nowhere and then seems completely obvious in retrospect. Here is the key insight expressed as a thought experiment.

Suppose you have a set $G$ with two binary operations $*$ and $\bullet$, both of which have the same two-sided identity element $e$. Suppose further that they "commute with each other" in the sense that $(a * b) \bullet (c * d) = (a \bullet c) * (b \bullet d)$.

Claim: $*$ and $\bullet$ are the same operation, and this operation is commutative.

*Proof:* To compute $a * b$: write $a = a \bullet e$ and $b = e \bullet b$. Then $a * b = (a \bullet e) * (e \bullet b) = (a * e) \bullet (e * b) = a \bullet b$. So $* = \bullet$. To see that this operation is commutative: $a * b = (e \bullet a) * (b \bullet e) = (e * b) \bullet (a * e) = b \bullet a = b * a$.

This argument applies directly to $\pi_2(X, x_0)$: the two "operations" are concatenation in the first coordinate and concatenation in the second coordinate of the square $[0,1]^2$. Both have the constant loop at $x_0$ as their identity. The interchange law holds by a planar homotopy argument. Therefore $\pi_2$ is abelian.

*The HoTT version:* In HoTT, the same argument shows that $\pi_2(A, a) = \pi_1(\Omega A, \mathsf{refl}_a)$ is abelian for any type $A$ and any point $a : A$. The proof is a sequence of path manipulations: $(p \cdot q) = (p \cdot \mathsf{refl}) \cdot (\mathsf{refl} \cdot q) = \ldots = q \cdot p$. This is perhaps the most beautiful computation in the HoTT book.
