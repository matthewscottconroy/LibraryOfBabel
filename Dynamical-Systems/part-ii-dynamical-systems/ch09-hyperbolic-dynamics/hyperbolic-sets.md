# 9.2 Hyperbolic Sets

The horseshoe worked because the map contracted in one direction and expanded in another, and these directions were preserved under iteration. That's the key abstraction: a *hyperbolic set* is an invariant set where every tangent vector is either exponentially contracted or exponentially expanded.

**Definition 9.2.1.** Let $f: M \to M$ be a $C^1$ diffeomorphism and $\Lambda \subseteq M$ a compact $f$-invariant set. $\Lambda$ is a *hyperbolic set* if there exists a $Df$-invariant splitting $T_xM = E^s(x) \oplus E^u(x)$ for each $x \in \Lambda$, and constants $C > 0$, $0 < \lambda < 1$ such that for all $n \geq 0$ and $x \in \Lambda$:
$$\|Df^n(x) v\| \leq C\lambda^n \|v\| \quad \text{for } v \in E^s(x)$$
$$\|Df^{-n}(x) v\| \leq C\lambda^n \|v\| \quad \text{for } v \in E^u(x).$$

The stable/unstable bundles $E^s, E^u$ vary continuously and are preserved by $Df$.

Let's read this carefully. At each point $x \in \Lambda$, the tangent space splits into two subspaces: $E^s(x)$ (stable, contracted by $Df$) and $E^u(x)$ (unstable, contracted by $Df^{-1}$ — equivalently, expanded by $Df$). Vectors in $E^s(x)$ decay exponentially as you apply $Df$; vectors in $E^u(x)$ grow exponentially. The splitting is $Df$-invariant: applying $Df$ carries $E^s(x)$ to $E^s(f(x))$ and $E^u(x)$ to $E^u(f(x))$.

This is the precise formalization of "the system stretches in some directions and compresses in others, uniformly along all orbits."

**Examples:**
- The horseshoe $\Lambda$ is a hyperbolic set. ($E^s$ = vertical direction, $E^u$ = horizontal direction, $\lambda$ = contraction rate, $1/\mu$ = expansion-inverse rate.)
- Repelling fixed points are hyperbolic sets (with $E^u = T_xM$, $E^s = 0$).
- Uniformly expanding maps (all eigenvalues $> 1$) have hyperbolic invariant sets.
- The invariant set of an Anosov diffeomorphism is $\Lambda = M$ (the whole manifold).

The range of examples suggests that hyperbolicity is a genuinely common phenomenon, not a special case. Repelling fixed points and expanding maps are hyperbolic in a degenerate sense (no stable directions, or no unstable directions). The interesting cases are those with both stable and unstable directions — like the horseshoe and the Anosov diffeomorphisms.

The stable and unstable distributions $x \mapsto E^s(x)$ and $x \mapsto E^u(x)$ are continuous (this follows from the uniform bounds in the definition). They integrate to stable and unstable *foliations* $\mathcal{W}^s$ and $\mathcal{W}^u$ — families of submanifolds that the dynamics contracts or expands. These foliations are the geometric heart of hyperbolic theory.

In the next section, we consider the extreme case: Anosov diffeomorphisms, where $\Lambda = M$.
