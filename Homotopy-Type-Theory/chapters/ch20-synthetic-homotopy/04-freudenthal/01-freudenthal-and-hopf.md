# 4.1 Freudenthal Suspension and the Hopf Fibration

## Stability and the Freudenthal Theorem

When you suspend a space, its homotopy groups shift: $\pi_k(X)$ becomes $\pi_{k+1}(\Sigma X)$. But this shift is only an isomorphism in a certain "stable range" — above that range, the suspension might introduce new homotopy information.

The Freudenthal suspension theorem makes this precise:

**Theorem 4.1 (Freudenthal Suspension, HoTT version).** If $A$ is $n$-connected (i.e., $\pi_k(A) = 0$ for $k \leq n$), then the suspension unit map $\eta : A \to \Omega \Sigma A$ is $2n$-connected.

**Corollary 4.2 (Stability of homotopy groups).** If $A$ is $(n-1)$-connected, then for $k \leq 2n - 2$:
$$\pi_k(A) \cong \pi_{k+1}(\Sigma A)$$

**What this says.** For spheres: $S^m$ is $(m-1)$-connected (trivial homotopy groups below dimension $m$). So for $k \leq 2m - 2$:
$$\pi_k(S^m) \cong \pi_{k+1}(S^{m+1})$$

The homotopy groups of spheres "stabilize" as $m \to \infty$. The *stable homotopy groups* $\pi_k^s = \pi_{k+n}(S^n)$ for large $n$ form the subject of stable homotopy theory.

## Proof Sketch of Freudenthal

The full proof uses the *Blakers-Massey theorem* (excision in HoTT). Here's the high-level argument:

**The suspension-loop adjunction.** The natural map $\eta : A \to \Omega \Sigma A$ comes from the suspension-loop adjunction:
$$[\Sigma A, Y] \simeq [A, \Omega Y]$$

for any pointed type $Y$. The map $\eta$ sends $a : A$ to the loop "go north along $\mathsf{merid}(a)$, then go south along $\mathsf{merid}(*)^{-1}$" — where $* : A$ is the basepoint.

**The fiber of $\eta$.** Consider the fiber $\mathsf{fib}_\eta(*)$ of $\eta$ at the basepoint loop. Freudenthal says this fiber is $2n$-connected. To show this, we use the join construction and the Blakers-Massey theorem.

**Blakers-Massey gives Freudenthal.** The key step: the pushout square for $\Sigma A$ (gluing north and south hemispheres) is a homotopy pushout where each map $* \to \Sigma A$ is $(n+1)$-connected. Blakers-Massey then gives that the map $A \to \Omega \Sigma A$ is $2n$-connected, which is the Freudenthal statement.

The full proof in HoTT was given by multiple groups (Brunerie, van Doorn, Lumsdaine; Favonia and Shulman). It's one of the significant formalization achievements in synthetic homotopy theory.

## Consequences of Freudenthal

**The stable range.** From Freudenthal:
- $\pi_k(S^n) \cong \pi_{k+1}(S^{n+1})$ for $k \leq 2n - 2$

In particular:
- $\pi_1(S^1) = \mathbb{Z}$ and $\pi_2(S^2) = \mathbb{Z}$ (same group, by Freudenthal for $n=1$)
- $\pi_2(S^2) = \mathbb{Z}$ and $\pi_3(S^3) = \mathbb{Z}$
- $\pi_n(S^n) = \mathbb{Z}$ for all $n \geq 1$ (the degree computation)

**The first unstable computation.** $\pi_3(S^2) = \mathbb{Z}$ is the first "unstable" computation — outside the stable range for $S^2$. This requires the Hopf fibration.

## The Hopf Fibration

The Hopf fibration is a fiber bundle $S^1 \to S^3 \to S^2$ — the 3-sphere fibers over the 2-sphere with $S^1$ fibers. It was discovered by Hopf in 1931 and generates $\pi_3(S^2) = \mathbb{Z}$.

In HoTT, the Hopf fibration is constructed synthetically using the *join* and the group structure of $S^1$.

**Step 1: The H-space structure on $S^1$.** The circle $S^1$ is an H-space: it has a multiplication $\mu : S^1 \times S^1 \to S^1$ that is "unital up to homotopy" (an H-space structure). The multiplication comes from the group structure of $\pi_1(S^1) = \mathbb{Z}$: since $S^1 \simeq K(\mathbb{Z}, 1)$, it has a group-like multiplication.

More precisely: Define $\mu : S^1 \times S^1 \to S^1$ using the $S^1$ recursion:
$$\mu(\mathsf{base}, -) :\equiv \mathsf{id}_{S^1}$$
$$\mu(\mathsf{loop}, -) :\equiv \text{the map rotating by the basepoint loop}$$

**Step 2: The Hopf construction.** Given an H-space $(A, \mu)$, there is a canonical fibration:
$$A \to A * A \xrightarrow{h_\mu} \Sigma A$$

called the *Hopf construction*. The map $h_\mu : A * A \to \Sigma A$ is defined using the join structure:
$$h_\mu(\mathsf{inl}(a)) :\equiv \mathsf{N} \quad h_\mu(\mathsf{inr}(a)) :\equiv \mathsf{S}$$
$$h_\mu(\mathsf{join}(a, b)) :\equiv \mathsf{merid}(\mu(a, b)) : \mathsf{N} = \mathsf{S}$$

For $A = S^1$:
- $A * A = S^1 * S^1 \simeq S^3$ (join of two circles is the 3-sphere)
- $\Sigma A = \Sigma S^1 \simeq S^2$ (suspension of the circle is the 2-sphere)

So the Hopf construction gives $S^3 \to S^2$ with fiber $S^1$.

**Theorem 4.3 (Hopf Fibration).** The Hopf construction $h_\mu : S^3 \to S^2$ has fiber $S^1$.

*Proof sketch.* The fiber of $h_\mu$ over $\mathsf{N} : S^2$ consists of preimages under $h_\mu$:
$$\mathsf{fib}_{h_\mu}(\mathsf{N}) = \sum_{x : S^3} h_\mu(x) = \mathsf{N}$$

For $x = \mathsf{inl}(a)$: $h_\mu(\mathsf{inl}(a)) = \mathsf{N}$ always, so $\mathsf{inl}(a)$ is always in the fiber.
For $x = \mathsf{inr}(b)$: $h_\mu(\mathsf{inr}(b)) = \mathsf{S} \neq \mathsf{N}$, so $\mathsf{inr}(b)$ is not in the fiber.
For $x = \mathsf{join}(a, b)$ with path to $\mathsf{N}$: $h_\mu(\mathsf{join}(a, b)) = \mathsf{merid}(\mu(a, b)) : \mathsf{N} = \mathsf{S}$, so this joins the fiber...

The fiber turns out to be $\simeq S^1$ — the join of the left copy $\{\mathsf{inl}(a) \mid a : S^1\} \simeq S^1$ and the "homotopically trivial" contribution from the join paths. $\square$

## $\pi_3(S^2) = \mathbb{Z}$

**Theorem 4.4.** $\pi_3(S^2) = \mathbb{Z}$.

*Proof using the Hopf fibration and long exact sequence.*

The Hopf fibration $S^1 \to S^3 \to S^2$ gives a long exact sequence (from the fibration long exact sequence):

$$\cdots \to \pi_3(S^1) \to \pi_3(S^3) \xrightarrow{h_*} \pi_3(S^2) \to \pi_2(S^1) \to \pi_2(S^3) \to \pi_2(S^2) \to \pi_1(S^1) \to \cdots$$

Now substitute known values:
- $\pi_k(S^1) = 0$ for $k \geq 2$ (S^1 is a 1-type, proved in HoTT)
- $\pi_k(S^3) = 0$ for $k \leq 2$ (S^3 is 2-connected)
- $\pi_3(S^3) = \mathbb{Z}$ (by the Freudenthal-derived result that $\pi_n(S^n) = \mathbb{Z}$)
- $\pi_2(S^2) = \mathbb{Z}$ (by Freudenthal)

The exact sequence simplifies to:
$$0 = \pi_3(S^1) \to \pi_3(S^3) \xrightarrow{h_*} \pi_3(S^2) \to \pi_2(S^1) = 0$$

So $h_* : \pi_3(S^3) \to \pi_3(S^2)$ is an isomorphism:
$$\pi_3(S^2) \cong \pi_3(S^3) = \mathbb{Z}$$

And the generator is the Hopf map $h : S^3 \to S^2$. $\square$

## Brunerie's Theorem: $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$

In 2016, Guillaume Brunerie formalized a proof that $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$.

**The proof strategy:**
1. Use the long exact sequence of the Hopf fibration $S^1 \to S^3 \to S^2$ to show $\pi_4(S^3) \cong \pi_4(S^2)$.
2. Compute $\pi_4(S^2)$ using:
   - The Gysin sequence (a cohomology computation)
   - The cohomology ring structure of $K(\mathbb{Z}, 2) = \mathbb{CP}^\infty$
   - Cup products in HoTT cohomology

3. Show $\pi_4(S^2) = \mathbb{Z}/2\mathbb{Z}$.

**The Brunerie number.** The integer "2" in $\mathbb{Z}/2\mathbb{Z}$ appears as a specific element $n : \mathbb{Z}$ defined in the proof. Brunerie *conjectured* that $n = 2$, but the proof term was so complex that it could only be verified by computer. 

In 2022, Ljungström and Mörtberg gave a simplification of the proof that could compute $n$ in reasonable time, verifying $n = 2$.

**What this demonstrates.** Synthetic homotopy theory can handle the same level of computation as classical algebraic topology — including results like $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$ that are genuinely difficult. And the proofs have *computational content* — Brunerie's number is not just an assertion but a computed value.

## The Power of Synthetic Methods

**1. Proofs without point-set topology.** No open sets, no point-net arguments, no compactness arguments. Every step is type-theoretic.

**2. Fully constructive.** The Hopf fibration, the long exact sequence, the Freudenthal theorem — all constructive, all with computable witnesses.

**3. Machine-verified.** Every theorem in this chapter has been or can be formalized in a proof assistant.

**4. New insights.** The synthetic proof of Blakers-Massey (excision) by Anel, Biedermann, Finster, Joyal uses a new "orthogonal factorization system" approach that's cleaner than the classical proof and reveals new structure.

Synthetic homotopy theory is not just "homotopy theory with a type-theoretic flavor." It's a new approach to the subject that often gives simpler, more illuminating proofs — and sometimes reveals theorems and structures that the classical approach obscures.
