# 5.1 Fibrations and the Long Exact Sequence

## Fibrations: The Right Notion of Projection

A *fibration* is a map that "behaves like a projection" in the homotopy-theoretic sense. The key property: you can always lift homotopies.

Think of a fiber bundle $p : E \to B$ (like a cylinder $S^1 \times [0,1] \to S^1$ projecting onto the circle). Given a homotopy $H_t : Y \to B$ and a starting lift $f : Y \to E$ over $H_0$, there should be a way to lift the entire homotopy to $E$. Fibrations are exactly the maps with this lifting property.

**Definition 5.1 (Hurewicz Fibration).** A map $p : E \to B$ is a *fibration* (or Hurewicz fibration) if it has the *homotopy lifting property (HLP)*: for any space $Y$, any map $f : Y \to E$, and any homotopy $H : Y \times [0,1] \to B$ with $H(-,0) = p \circ f$, there exists a lift $\tilde{H} : Y \times [0,1] \to E$ with:
- $p \circ \tilde{H} = H$
- $\tilde{H}(-,0) = f$

$$\begin{array}{ccc}
Y & \xrightarrow{f} & E \\
\downarrow_{i_0} & \nearrow_{\tilde{H}} & \downarrow_p \\
Y \times [0,1] & \xrightarrow{H} & B
\end{array}$$

Here $i_0(y) = (y, 0)$.

**Intuition:** You're given a map into $E$ and a homotopy of the projection into $B$. You can always find a way to "follow" the homotopy in $E$, lifting it from $B$ back to $E$.

## Serre vs. Hurewicz Fibrations

There are two main flavors:
- **Hurewicz fibration:** HLP holds for all spaces $Y$
- **Serre fibration:** HLP holds for $Y = D^n$ (disks) for all $n$

Serre fibrations are weaker (more maps qualify) but sufficient for computing homotopy groups. Hurewicz fibrations have nicer theoretical properties.

For our purposes, we'll work with Hurewicz fibrations and note that all the key examples satisfy both conditions.

## Key Examples

**Covering maps are fibrations.** A covering map $p : \tilde{X} \to X$ satisfies HLP (this is the homotopy lifting theorem from covering space theory). For covering maps, the lift is unique.

**Fiber bundles are fibrations.** A fiber bundle (locally trivial fibration) is always a Hurewicz fibration. Examples: the tangent bundle of a manifold, the Hopf fibration $S^1 \to S^3 \to S^2$.

**Path-loop fibration.** For any space $X$ with basepoint $x_0$:
$$p : PX \to X, \quad p(\gamma) = \gamma(1)$$
where $PX = \{\gamma : [0,1] \to X \mid \gamma(0) = x_0\}$ is the *based path space*. The fiber $p^{-1}(x_0) = \Omega X$ (the based loop space).

The path space $PX$ is contractible (contract each path to the constant path). So the path-loop fibration is:
$$\Omega X \hookrightarrow PX \to X, \quad \text{with } PX \simeq \{*\}$$

## The Long Exact Sequence of a Fibration

The most powerful tool for computing homotopy groups:

**Theorem 5.2 (Long Exact Sequence).** For a fibration $p : E \to B$ with fiber $F = p^{-1}(b_0)$ (over the basepoint $b_0 \in B$) and chosen basepoint $e_0 \in F$:

$$\cdots \to \pi_n(F, e_0) \xrightarrow{i_*} \pi_n(E, e_0) \xrightarrow{p_*} \pi_n(B, b_0) \xrightarrow{\partial} \pi_{n-1}(F, e_0) \to \cdots$$
$$\cdots \to \pi_1(B, b_0) \xrightarrow{\partial} \pi_0(F) \to \pi_0(E) \to \pi_0(B)$$

where $i : F \hookrightarrow E$ is the inclusion of the fiber, and $\partial$ is the *connecting homomorphism*.

**How to use it:** If you know 2 out of 3 of $\pi_n(F)$, $\pi_n(E)$, $\pi_n(B)$, you can often compute the third.

## The Connecting Homomorphism

The connecting map $\partial : \pi_n(B, b_0) \to \pi_{n-1}(F, e_0)$ is defined as follows:

Given a map $\alpha : S^n \to B$ (representing an element of $\pi_n(B)$):
1. View $S^n$ as the suspension of $S^{n-1}$: $S^n = \Sigma S^{n-1} = (S^{n-1} \times [-1,1]) / (S^{n-1} \times \{-1\} \sim N, S^{n-1} \times \{1\} \sim S)$
2. The map $\alpha$ restricted to the "upper hemisphere" can be lifted (since the upper hemisphere is contractible, hence any map into $B$ from it lifts to $E$)
3. The lift at the equator $S^{n-1}$ lands in the fiber $F$, defining a map $S^{n-1} \to F$

This map represents an element of $\pi_{n-1}(F)$ — that's $\partial(\alpha)$.

## Applications

### The Path-Loop Fibration

Apply the long exact sequence to $\Omega X \hookrightarrow PX \to X$ with $PX$ contractible:

$$\cdots \to \pi_n(PX) \to \pi_n(X) \xrightarrow{\partial} \pi_{n-1}(\Omega X) \to \pi_{n-1}(PX) \to \cdots$$

Since $PX$ is contractible, $\pi_n(PX) = 0$ for all $n$. The long exact sequence reduces to:
$$0 \to \pi_n(X) \xrightarrow{\partial} \pi_{n-1}(\Omega X) \to 0$$

So $\partial$ is an isomorphism: $\pi_n(X) \cong \pi_{n-1}(\Omega X)$.

This gives: **$\pi_n(X) \cong \pi_{n-1}(\Omega X)$** — the homotopy groups of $X$ are the homotopy groups of its loop space, shifted by 1.

### The Hopf Fibration

The Hopf fibration $S^1 \hookrightarrow S^3 \to S^2$:

$$\cdots \to \pi_3(S^1) \to \pi_3(S^3) \to \pi_3(S^2) \xrightarrow{\partial} \pi_2(S^1) \to \pi_2(S^3) \to \pi_2(S^2) \to \pi_1(S^1) \to \pi_1(S^3) \to \pi_1(S^2) \to \cdots$$

Known values:
- $\pi_k(S^1) = 0$ for $k \geq 2$, $\pi_1(S^1) = \mathbb{Z}$
- $\pi_1(S^3) = \pi_2(S^3) = 0$ (sphere of dimension $\geq 2$ is simply connected with trivial $\pi_2$)
- $\pi_3(S^3) = \mathbb{Z}$

The sequence (around $n = 3$):
$$0 = \pi_3(S^1) \to \pi_3(S^3) = \mathbb{Z} \to \pi_3(S^2) \xrightarrow{\partial} \pi_2(S^1) = 0$$

So $\mathbb{Z} \to \pi_3(S^2) \to 0$ is exact, meaning $\pi_3(S^2)$ surjects onto $0$ and the map from $\mathbb{Z}$ is surjective... let's also use the $n=2$ part:

$$0 = \pi_2(S^1) \to \pi_2(S^3) = 0 \to \pi_2(S^2) = \mathbb{Z} \xrightarrow{\partial} \pi_1(S^1) = \mathbb{Z} \to \pi_1(S^3) = 0$$

The map $\pi_2(S^2) = \mathbb{Z} \to \pi_1(S^1) = \mathbb{Z}$ must be an isomorphism (its kernel = image of $0$, its cokernel = kernel of $0$). So $\partial$ here is an isomorphism.

Going back to $n = 3$:
$$0 \to \pi_3(S^3) = \mathbb{Z} \xrightarrow{\eta_*} \pi_3(S^2) \to 0$$

(The last $0$ is because $\partial : \pi_3(S^2) \to \pi_2(S^1) = 0$ is trivial.) So $\pi_3(S^2) = \mathbb{Z}$, generated by the Hopf map $\eta : S^3 \to S^2$. 

This is one of the most elegant applications of the long exact sequence.

### $\pi_2(S^2) = \mathbb{Z}$

From the Hopf fibration's long exact sequence at $n = 2$:
$$\pi_2(S^3) = 0 \to \pi_2(S^2) \to \pi_1(S^1) = \mathbb{Z} \to \pi_1(S^3) = 0$$

The middle map is an isomorphism (kernel = image of 0, cokernel = kernel of 0), so $\pi_2(S^2) = \mathbb{Z}$. ✓

## Fibrations in HoTT

In HoTT, fibrations are modeled by *type families* (dependent types):

**Type family as fibration:** A type family $B : A \to \mathsf{Type}$ gives a fibration:
- Total space: $E = \sum_{a:A} B(a)$ (the dependent sum)
- Base: $A$
- Projection: $\pi_1 : E \to A$, $\pi_1(a, b) = a$
- Fiber: $B(a)$ = the type over $a$

The HLP in HoTT: given $b : B(a)$ and a path $p : a = a'$ in $A$, we can transport $b$ to $\mathsf{transport}^B(p, b) : B(a')$. Transport is the lifting of paths.

**Long exact sequence in HoTT:** For a fibration (type family) $B : A \to \mathsf{Type}$, there is a long exact sequence of homotopy groups. This follows from the fact that $\sum_{a:A} B(a) \to A$ is a fibration in the model-category sense, and the usual long exact sequence applies.

**The path-loop fibration in HoTT:**
$$\Omega A \hookrightarrow PA \to A$$
where $PA = \sum_{a:A} (a_0 = a)$ is contractible (it's a "based path space" type). This gives $\pi_n(A) \cong \pi_{n-1}(\Omega A)$ — the loop space adjunction in HoTT.

## Summary

| Concept | Classical | HoTT |
|---|---|---|
| Fibration | Map with HLP | Type family $B : A \to \mathsf{Type}$ |
| Fiber over $b_0$ | $p^{-1}(b_0)$ | $B(b_0)$ |
| HLP / lifting | Homotopy lifting | Transport $\mathsf{transport}^B$ |
| Long exact sequence | $\pi_n(F) \to \pi_n(E) \to \pi_n(B) \to \pi_{n-1}(F)$ | Follows from J rule |
| Hopf fibration | $S^1 \to S^3 \to S^2$, $\pi_3(S^2) = \mathbb{Z}$ | Constructable as a HIT map |

Fibrations are the central tool for computing homotopy groups. The long exact sequence is the computational workhorse of homotopy theory. In HoTT, fibrations are native (type families), and the long exact sequence follows from the structure of the identity types — the same theorem, expressed in the internal language of ∞-toposes.
