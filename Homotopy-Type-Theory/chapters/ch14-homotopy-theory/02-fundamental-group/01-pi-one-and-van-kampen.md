# 2.1 The Fundamental Group and Van Kampen's Theorem

## Loops and the Idea of "Going Around"

Imagine standing at a point $x_0$ on a topological space $X$ and drawing a loop — a continuous path that starts and ends at $x_0$. The question is: can you continuously deform this loop to the trivial (constant) loop? If not, the loop "sees" some non-trivial topology of $X$.

For the real plane $\mathbb{R}^2$: any loop can be shrunk to a point (the plane has no holes). For the circle $S^1$: a loop that winds around once cannot be shrunk to a point. For the plane with a hole $\mathbb{R}^2 \setminus \{0\}$: loops that encircle the hole cannot be shrunk.

The *fundamental group* formalizes this by collecting all "topologically distinct" loops at a basepoint into a group.

## Paths and Homotopy Relative Endpoints

**Definition 2.1 (Path).** A *path* from $x_0$ to $x_1$ in $X$ is a continuous function $\gamma : [0,1] \to X$ with $\gamma(0) = x_0$ and $\gamma(1) = x_1$.

**Definition 2.2 (Path homotopy / homotopy relative endpoints).** Two paths $\gamma_0, \gamma_1 : [0,1] \to X$ from $x_0$ to $x_1$ are *path-homotopic* if there is a continuous map $H : [0,1] \times [0,1] \to X$ with:
- $H(s, 0) = \gamma_0(s)$ and $H(s, 1) = \gamma_1(s)$ for all $s$ (deforms $\gamma_0$ to $\gamma_1$)
- $H(0, t) = x_0$ and $H(1, t) = x_1$ for all $t$ (fixes the endpoints)

This is a homotopy that keeps the endpoints fixed throughout the deformation. The equivalence class of $\gamma$ under path-homotopy is written $[\gamma]$.

## The Group Structure

The fundamental group has three operations:
1. **Concatenation:** "Go along $\gamma_1$, then go along $\gamma_2$"
2. **Inversion:** "Go along $\gamma$ backwards"
3. **Identity:** "Stay at $x_0$"

**Path concatenation:** For $\gamma_1 : x_0 \to x_1$ and $\gamma_2 : x_1 \to x_2$:
$$(\gamma_1 \cdot \gamma_2)(s) = \begin{cases} \gamma_1(2s) & 0 \leq s \leq 1/2 \\ \gamma_2(2s-1) & 1/2 \leq s \leq 1 \end{cases}$$

This traverses $\gamma_1$ in the first half of the time interval and $\gamma_2$ in the second half.

**Path reversal:** $\bar\gamma(s) = \gamma(1-s)$ (run the path backwards).

**Constant path:** $c_{x_0}(s) = x_0$ for all $s$.

**Theorem 2.3.** The above operations respect path-homotopy: if $\gamma_0 \simeq \gamma_0'$ (rel endpoints) and $\gamma_1 \simeq \gamma_1'$, then $\gamma_0 \cdot \gamma_1 \simeq \gamma_0' \cdot \gamma_1'$ (rel endpoints).

**Definition 2.4 (Fundamental Group).** The *fundamental group* $\pi_1(X, x_0)$ is the set of path-homotopy classes of loops at $x_0$ (paths from $x_0$ to $x_0$), with group operation $[\gamma_1] \cdot [\gamma_2] = [\gamma_1 \cdot \gamma_2]$.

**Verification that this is a group:**
- *Well-defined:* By Theorem 2.3
- *Associativity:* $[(\gamma_1 \cdot \gamma_2) \cdot \gamma_3] = [\gamma_1 \cdot (\gamma_2 \cdot \gamma_3)]$ — proved by the reparametrization homotopy $H(s,t) = \gamma_1 \cdot \gamma_2 \cdot \gamma_3$ with varying speed. Explicitly:
$$H(s,t) = \begin{cases} \gamma_1\!\left(\frac{4s}{1+t}\right) & 0 \leq s \leq \frac{1+t}{4} \\ \gamma_2(4s - 1 - t) & \frac{1+t}{4} \leq s \leq \frac{2+t}{4} \\ \gamma_3\!\left(\frac{4s - 2 - t}{2 - t}\right) & \frac{2+t}{4} \leq s \leq 1 \end{cases}$$
- *Identity:* $[c_{x_0}] \cdot [\gamma] = [\gamma]$ and $[\gamma] \cdot [c_{x_0}] = [\gamma]$ — by reparametrization homotopies
- *Inverses:* $[\gamma] \cdot [\bar\gamma] = [c_{x_0}]$ — the homotopy contracts $\gamma \cdot \bar\gamma$ to the constant loop

## The Fundamental Group as a Functor

The fundamental group is functorial: a continuous map $f : (X, x_0) \to (Y, y_0)$ (with $f(x_0) = y_0$) induces a group homomorphism:
$$f_* : \pi_1(X, x_0) \to \pi_1(Y, y_0), \quad f_*([\gamma]) = [f \circ \gamma]$$

**Theorem 2.5.** $f_*$ is a group homomorphism, and:
- $(\mathsf{id}_X)_* = \mathsf{id}_{\pi_1(X)}$
- $(g \circ f)_* = g_* \circ f_*$

So $\pi_1 : \mathbf{Top}_* \to \mathbf{Grp}$ is a functor from pointed topological spaces to groups.

**Corollary 2.6.** If $f : X \to Y$ is a homotopy equivalence (even without preserving basepoints in the strict sense), then $f_* : \pi_1(X) \to \pi_1(Y)$ is an isomorphism.

This is why $\pi_1$ is a homotopy invariant: homotopy equivalent spaces have isomorphic fundamental groups.

## Computing $\pi_1(S^1) = \mathbb{Z}$

The fundamental example: the circle $S^1$ has fundamental group $\mathbb{Z}$.

**The generator.** The loop $\omega : [0,1] \to S^1$ defined by $\omega(s) = e^{2\pi i s}$ winds around the circle once. The loop $\omega^n$ (defined by composing $n$ copies) winds around $n$ times. So we have a map $\mathbb{Z} \to \pi_1(S^1)$ given by $n \mapsto [\omega^n]$.

**Theorem 2.7.** $\pi_1(S^1, 1) \cong \mathbb{Z}$, with generator $[\omega]$.

**Proof sketch (via covering spaces).** The universal cover of $S^1$ is $\mathbb{R}$, with covering map $p : \mathbb{R} \to S^1$, $p(t) = e^{2\pi i t}$.

By the path-lifting theorem, any loop $\gamma$ in $S^1$ based at $1$ lifts uniquely to a path $\tilde\gamma$ in $\mathbb{R}$ starting at $0$. The endpoint $\tilde\gamma(1) \in p^{-1}(1) = \mathbb{Z}$ is an integer — the *winding number* of $\gamma$.

- This defines a map $\phi : \pi_1(S^1, 1) \to \mathbb{Z}$, $\phi([\gamma]) = \tilde\gamma(1)$
- $\phi$ is a group homomorphism (lifting concatenated loops gives the sum of winding numbers)
- $\phi$ is injective: if $\tilde\gamma(1) = 0$, then $\tilde\gamma$ is a path from $0$ to $0$ in $\mathbb{R}$; since $\mathbb{R}$ is simply connected, $\tilde\gamma \simeq c_0$ rel endpoints; projecting down, $\gamma \simeq c_1$ rel endpoints
- $\phi$ is surjective: the loop $\omega^n$ has winding number $n$ $\square$

## Seifert-Van Kampen Theorem

The fundamental group of a union is computed by the van Kampen theorem — one of the most powerful tools in algebraic topology.

**Theorem 2.8 (Seifert-Van Kampen).** Let $X = U \cup V$ where $U, V$ are open and path-connected, and $U \cap V$ is also path-connected (and non-empty). Choose basepoint $x_0 \in U \cap V$. Then:
$$\pi_1(X, x_0) \cong \pi_1(U, x_0) *_{\pi_1(U \cap V, x_0)} \pi_1(V, x_0)$$

The right side is the *amalgamated free product*: the pushout in the category of groups.

**The amalgamated free product.** Given group homomorphisms $\phi_1 : H \to G_1$ and $\phi_2 : H \to G_2$, the amalgamated free product $G_1 *_H G_2$ is the group generated by $G_1$ and $G_2$, with the relations $\phi_1(h) = \phi_2(h)$ for all $h \in H$.

Explicitly: $G_1 *_H G_2 = (G_1 * G_2)/N$ where $N$ is the normal subgroup generated by $\{\phi_1(h)\phi_2(h)^{-1} : h \in H\}$.

**Van Kampen for the circle.** Decompose $S^1$ as two open arcs:
- $U = S^1 \setminus \{-1\}$ (all except the "left" point): homeomorphic to $\mathbb{R}$, so $\pi_1(U) = 0$
- $V = S^1 \setminus \{+1\}$ (all except the "right" point): homeomorphic to $\mathbb{R}$, so $\pi_1(V) = 0$
- $U \cap V = S^1 \setminus \{-1, +1\}$: two disjoint arcs...

But $U \cap V$ is not path-connected (two components)! The standard van Kampen requires path-connected intersection. We need a modified version (the groupoid version).

A cleaner computation uses *slightly overlapping hemispheres*: let $U$ and $V$ be two open arcs covering $S^1$ that overlap at the top and bottom. Then $U \cap V$ is two small arcs (each contractible), and van Kampen gives... well, it's a bit involved. The cleanest proof still uses covering spaces.

**Van Kampen for the torus.** The torus $T^2$ has a CW structure: one 0-cell $v$, two 1-cells $a$ and $b$, one 2-cell with attaching map $aba^{-1}b^{-1}$.

Decompose $T^2 = U \cup V$ where:
- $V$ = the interior of the 2-cell (an open disk, contractible, $\pi_1(V) = 0$)
- $U$ = complement of a point in the 2-cell (deformation retracts to the 1-skeleton $S^1 \vee S^1$)
- $U \cap V$ = the punctured disk, homotopy equivalent to $S^1$, $\pi_1(U \cap V) = \mathbb{Z}$

The inclusion $U \cap V \hookrightarrow U$ sends the generator of $\pi_1(U \cap V) = \mathbb{Z}$ to the word $aba^{-1}b^{-1}$ in $\pi_1(U) = \pi_1(S^1 \vee S^1) = \mathbb{Z} * \mathbb{Z} = F_2$.

Van Kampen:
$$\pi_1(T^2) = F_2 *_{\mathbb{Z}} \{e\} = F_2 / \langle aba^{-1}b^{-1} \rangle = \langle a, b \mid aba^{-1}b^{-1} = e \rangle = \mathbb{Z} \times \mathbb{Z} = \mathbb{Z}^2$$

The last equality: $ab = ba$ (from the relation), so $a$ and $b$ commute, giving the free abelian group on two generators.

**Van Kampen for the free product.** If $X = U \cup V$ and $U \cap V$ is contractible (so $\pi_1(U \cap V) = 0$), then:
$$\pi_1(X) = \pi_1(U) * \pi_1(V) \quad \text{(free product)}$$

This applies to wedge sums: $\pi_1(S^1 \vee S^1) = \mathbb{Z} * \mathbb{Z} = F_2$ (the free group on two generators).

## Van Kampen in HoTT

The van Kampen theorem has a beautiful HoTT formulation: it's the statement that the pushout of types computes the fundamental group as the pushout of groups.

**HoTT van Kampen:** For a pushout type $C = A \sqcup_D B$ (where $D$ maps to both $A$ and $B$):
$$\pi_1(C, c_0) \cong \pi_1(A, a_0) *_{\pi_1(D, d_0)} \pi_1(B, b_0)$$

This is not just an analog — it follows directly from the universal property of pushouts, combined with the fact that $\pi_1$ is a left adjoint (it preserves pushouts).

## Summary

| Space | $\pi_1$ | Method |
|---|---|---|
| $\mathbb{R}^n$, contractible | trivial | Straight-line homotopy |
| $S^1$ | $\mathbb{Z}$ | Covering space / winding number |
| $S^n$ for $n \geq 2$ | trivial | No non-trivial loops |
| $T^2 = S^1 \times S^1$ | $\mathbb{Z}^2$ | Van Kampen |
| $\mathbb{RP}^2$ | $\mathbb{Z}/2$ | Universal cover $S^2$ |
| $S^1 \vee S^1$ | $F_2$ (free group) | Van Kampen |

The fundamental group is the first and most computable homotopy invariant. It classifies spaces "up to 1-dimensional homotopy" and is completely determined by van Kampen's theorem and the structure of the space's 2-skeleton.
