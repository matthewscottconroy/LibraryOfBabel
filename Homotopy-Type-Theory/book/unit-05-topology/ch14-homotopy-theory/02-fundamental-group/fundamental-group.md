# The Fundamental Group

## Loops and Their Deformations

Fix a point $x_0$ in a topological space $X$ — the *basepoint*. A *loop based at $x_0$* is a continuous function $\gamma : [0,1] \to X$ with $\gamma(0) = \gamma(1) = x_0$: a path that starts and ends at $x_0$. Two loops $\gamma$ and $\delta$ are *homotopic rel basepoint* if there is a homotopy $H : [0,1] \times [0,1] \to X$ with $H(s, 0) = \gamma(s)$, $H(s, 1) = \delta(s)$, and $H(0, t) = H(1, t) = x_0$ for all $t$. The homotopy keeps the basepoint fixed throughout.

This is the right notion of equivalence for loops: we deform one loop into another, but we keep the basepoint pinned. The homotopy class $[\gamma]$ is the deformation class of the loop $\gamma$.

The *loop space* $\Omega(X, x_0)$ is the set of all loops based at $x_0$, with the compact-open topology. The *fundamental group* $\pi_1(X, x_0)$ is the set of homotopy classes of loops based at $x_0$, equipped with a group structure that we now define.

## The Group Structure

**Concatenation.** Given loops $\gamma$ and $\delta$ based at $x_0$, define their *concatenation* $\gamma \cdot \delta$ by:
$$(\gamma \cdot \delta)(s) = \begin{cases} \gamma(2s) & 0 \leq s \leq 1/2 \\ \delta(2s-1) & 1/2 \leq s \leq 1 \end{cases}$$

This is a loop: it starts at $x_0$ (since $\gamma(0) = x_0$), transitions to the basepoint at $s = 1/2$ (since $\gamma(1) = x_0 = \delta(0)$), and ends at $x_0$ (since $\delta(1) = x_0$). The pasting lemma ensures continuity.

This operation descends to homotopy classes: if $\gamma \simeq \gamma'$ and $\delta \simeq \delta'$ (rel basepoint), then $\gamma \cdot \delta \simeq \gamma' \cdot \delta'$. So we get a well-defined operation $[\gamma] \cdot [\delta] = [\gamma \cdot \delta]$ on $\pi_1(X, x_0)$.

**The group axioms:**
- *Associativity:* $[\gamma] \cdot ([\delta] \cdot [\varepsilon]) = ([\gamma] \cdot [\delta]) \cdot [\varepsilon]$. Not on the nose — $\gamma \cdot (\delta \cdot \varepsilon)$ and $(\gamma \cdot \delta) \cdot \varepsilon$ are different loops (different parametrizations) — but they are homotopic rel basepoint via a reparametrization homotopy.
- *Unit:* The constant loop $c_{x_0}(s) = x_0$ is the identity: $[\gamma] \cdot [c_{x_0}] = [\gamma]$ and $[c_{x_0}] \cdot [\gamma] = [\gamma]$ (up to homotopy).
- *Inverses:* The reverse loop $\overline{\gamma}(s) = \gamma(1-s)$ provides the inverse: $[\gamma] \cdot [\overline{\gamma}] = [c_{x_0}]$ (up to homotopy). The homotopy contracts the concatenation $\gamma \cdot \overline{\gamma}$ to the constant loop by "retracting" the path along itself.

So $\pi_1(X, x_0)$ with the concatenation operation is a group: the *fundamental group* of $X$ at $x_0$.

## The First Great Theorem: $\pi_1(S^1) = \mathbb{Z}$

The fundamental group of the circle is the integers. This is the prototype of all fundamental group computations and the foundation of the entire theory.

**Theorem.** $\pi_1(S^1, 1) \cong \mathbb{Z}$.

The isomorphism is the *winding number*: a loop $\gamma$ in $S^1$ based at $1$ wraps around the circle some integer number of times (possibly negative, for counterclockwise wrapping). This integer is the winding number and is a homotopy invariant. Every integer arises as the winding number of some loop (the loop that goes around $n$ times), and two loops with the same winding number are homotopic.

*Proof sketch via covering spaces.* Consider the covering map $p : \mathbb{R} \to S^1$ given by $p(t) = e^{2\pi i t}$. Every loop $\gamma : [0,1] \to S^1$ based at $1$ lifts uniquely to a path $\tilde{\gamma} : [0,1] \to \mathbb{R}$ starting at $0$. The endpoint $\tilde{\gamma}(1)$ is an integer (since $p(\tilde{\gamma}(1)) = \gamma(1) = 1$ and $p^{-1}(1) = \mathbb{Z}$). The map $[\gamma] \mapsto \tilde{\gamma}(1)$ is the winding number; it is a group isomorphism $\pi_1(S^1) \cong \mathbb{Z}$.

This computation is the key result that unlocks the entire theory. In HoTT, this theorem has a synthetic proof using the encode-decode method, and it is one of the central results of the HoTT book. The proof is non-trivial in both settings, and understanding it fully requires understanding covering spaces (Section 3).

## Van Kampen's Theorem

The Seifert-van Kampen theorem is the main computational tool for fundamental groups. It computes the fundamental group of a union $X = U \cup V$ from the fundamental groups of $U$, $V$, and $U \cap V$.

**Theorem (Seifert-van Kampen).** Let $X = U \cup V$ where $U$, $V$, and $U \cap V$ are path-connected open sets, and let $x_0 \in U \cap V$. Then $\pi_1(X, x_0)$ is the *pushout* (amalgamated free product) of $\pi_1(U, x_0)$ and $\pi_1(V, x_0)$ over $\pi_1(U \cap V, x_0)$:
$$\pi_1(X, x_0) \cong \pi_1(U, x_0) *_{\pi_1(U \cap V, x_0)} \pi_1(V, x_0)$$

In generators and relations: if $\pi_1(U) = \langle S_U \mid R_U \rangle$, $\pi_1(V) = \langle S_V \mid R_V \rangle$, and $\pi_1(U \cap V) = \langle S_{U \cap V} \mid R_{U \cap V} \rangle$, then:
$$\pi_1(X) = \langle S_U, S_V \mid R_U, R_V, \{i_U(g) = i_V(g) : g \in S_{U \cap V}\} \rangle$$
where $i_U : \pi_1(U \cap V) \to \pi_1(U)$ and $i_V : \pi_1(U \cap V) \to \pi_1(V)$ are the inclusion-induced maps.

**Applications:**

*The circle.* Take $U = S^1 \setminus \{-1\}$ and $V = S^1 \setminus \{1\}$. Both $U$ and $V$ are contractible (homeomorphic to open intervals, hence to $\mathbb{R}$). Their intersection $U \cap V = S^1 \setminus \{-1, 1\}$ has two components, each contractible. Van Kampen gives $\pi_1(S^1) = \mathbb{Z} * \mathbb{Z} / \langle ab = 1 \rangle = \mathbb{Z}$.

*The torus.* $T^2$ has a CW structure with one 0-cell, two 1-cells $a$ and $b$, and one 2-cell attached via $aba^{-1}b^{-1}$. The 1-skeleton is $S^1 \vee S^1$ with $\pi_1 = F_2 = \langle a, b \rangle$ (free group). Attaching the 2-cell kills $aba^{-1}b^{-1}$, giving $\pi_1(T^2) = \langle a, b \mid aba^{-1}b^{-1} = e \rangle = \mathbb{Z}^2$.

*The Klein bottle.* CW structure with one 0-cell, two 1-cells, one 2-cell attached via $abab^{-1}$. This gives $\pi_1(K) = \langle a, b \mid abab^{-1} = e \rangle$, which is non-abelian.

## Change of Basepoint and the Fundamental Groupoid

The fundamental group depends on the basepoint: $\pi_1(X, x_0)$ and $\pi_1(X, x_1)$ for different basepoints $x_0$ and $x_1$ need not be equal. But if $X$ is path-connected, they are *isomorphic*: any path $\alpha$ from $x_0$ to $x_1$ induces an isomorphism $\phi_\alpha : \pi_1(X, x_0) \to \pi_1(X, x_1)$ by $[\gamma] \mapsto [\overline{\alpha} \cdot \gamma \cdot \alpha]$.

The isomorphism depends on the homotopy class of $\alpha$: if $\alpha$ and $\beta$ are homotopic paths from $x_0$ to $x_1$, they give the same isomorphism. So the fundamental group is well-defined up to isomorphism for path-connected spaces.

For non-path-connected spaces, or when you want to track basepoints carefully, the *fundamental groupoid* $\Pi_1(X)$ is more natural: its objects are points of $X$, and its morphisms from $x_0$ to $x_1$ are homotopy classes of paths from $x_0$ to $x_1$. The fundamental groupoid captures all the path-connectivity information simultaneously.

In HoTT, this is the identity type structure: the fundamental groupoid of a type $A$ is the groupoid with objects $a : A$ and morphisms $a =_A b$. Path concatenation is composition; identity paths are identity morphisms; path reversal is the groupoid inverse. HoTT makes explicit what homotopy theory knew implicitly: spaces are groupoids, and the identity types encode the groupoid structure.

## Functoriality

A continuous map $f : (X, x_0) \to (Y, f(x_0))$ of based spaces induces a group homomorphism $f_* : \pi_1(X, x_0) \to \pi_1(Y, f(x_0))$ by $f_*([\gamma]) = [f \circ \gamma]$ (compose the loop with $f$). This is functorial: $(g \circ f)_* = g_* \circ f_*$ and $(\mathsf{id}_X)_* = \mathsf{id}_{\pi_1(X)}$.

If $f$ is a homotopy equivalence, then $f_*$ is an isomorphism. So $\pi_1$ is a functor from the homotopy category of based spaces to the category of groups.

This functoriality is why $\pi_1$ is a useful invariant: any topological construction that is functorial (preserved by maps) is also preserved by homotopy equivalences, and $\pi_1$ detects differences between spaces that maps cannot preserve.
