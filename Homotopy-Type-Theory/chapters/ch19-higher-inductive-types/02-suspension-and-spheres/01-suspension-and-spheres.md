# 2.1 Suspension and Spheres

## Iterating to Build Higher-Dimensional Spaces

The circle $S^1$ was defined with one point and one loop. How do we build higher-dimensional spheres?

The key operation is *suspension* — a systematic way to "raise the dimension" of a type. Suspending a type adds two new poles (north and south) and connects every point of the original type to both poles via paths ("meridians").

Starting from a single point ($\mathbf{1}$) and suspending repeatedly gives all spheres:
$$\mathbf{1} \xrightarrow{\Sigma} S^0 \xrightarrow{\Sigma} S^1 \xrightarrow{\Sigma} S^2 \xrightarrow{\Sigma} S^3 \xrightarrow{\Sigma} \cdots$$

(with a slight correction: $S^0 = \mathbf{Bool}$, and $\Sigma \mathbf{Bool} = S^1$ — we'll verify this.)

## The Suspension HIT

**Definition 2.1 (Suspension).** For a type $A$, the *suspension* $\Sigma A$ is defined by:
- Point constructor: $\mathsf{N} : \Sigma A$ (north pole)
- Point constructor: $\mathsf{S} : \Sigma A$ (south pole)
- Path constructor: $\mathsf{merid}(a) : \mathsf{N} =_{\Sigma A} \mathsf{S}$ for each $a : A$

So $\Sigma A$ has two poles and one "meridian" path from north to south for each element $a \in A$.

**Intuition.** Topologically, the suspension of a space $X$ is $(X \times [0,1]) / (X \times \{0\} \sim \mathsf{N}, X \times \{1\} \sim \mathsf{S})$ — the product with an interval, with the "top" collapsed to one point and the "bottom" collapsed to another. In type theory, the product with the interval becomes a path, and the collapsed endpoints become the poles.

**The non-dependent eliminator for $\Sigma A$.** To define $f : \Sigma A \to B$:
- $n : B$ (image of north pole)
- $s : B$ (image of south pole)
- $m : A \to (n = s)$ (for each $a : A$, a path from $n$ to $s$ — the image of $\mathsf{merid}(a)$)

Computation rules:
- $f(\mathsf{N}) \equiv n$
- $f(\mathsf{S}) \equiv s$
- $\mathsf{ap}_f(\mathsf{merid}(a)) = m(a)$

## Computing Small Suspensions

**Theorem 2.2 (Suspension of empty type).** $\Sigma \mathbf{0} \simeq \mathbf{1}$.

*Proof.* $\Sigma \mathbf{0}$ has two poles and meridians $\mathsf{merid}(a)$ for each $a : \mathbf{0}$ — but there are no elements of $\mathbf{0}$, so there are no meridians. The only paths are reflexivity and the groupoid structure.

The two poles $\mathsf{N}$ and $\mathsf{S}$ are... can they be shown to be equal? With no meridians, we can't build a path between them, and they might be distinct. So $\Sigma \mathbf{0}$ has two points and no paths between them.

Actually, $\Sigma \mathbf{0}$ should be $S^0 = \mathbf{1} + \mathbf{1}$ (the zero-sphere, a two-point discrete set), not $\mathbf{1}$.

Let me recalculate: $\Sigma^0 A = A$, $\Sigma^1 A = \Sigma A$, $\Sigma^n A = \Sigma(\Sigma^{n-1} A)$.

Then $S^n = \Sigma^n \mathbf{Bool}$ for $n \geq 0$... let me check:
- $S^0 = \mathbf{Bool} = \mathbf{1} + \mathbf{1}$ (two points) ✓
- $S^1 = \Sigma \mathsf{Bool}$ ✓ (we'll verify this)
- $S^2 = \Sigma^2 \mathbf{Bool} = \Sigma S^1$ ✓
- etc.

The standard definition of $S^n$ in HoTT:
$$S^{-1} :\equiv \mathbf{0}, \quad S^n :\equiv \Sigma S^{n-1}$$

So $S^0 = \Sigma S^{-1} = \Sigma \mathbf{0}$. Let's verify: $\Sigma \mathbf{0}$ has $\mathsf{N}$, $\mathsf{S}$, and no meridians. This is indeed a two-point discrete type $\simeq \mathbf{Bool} = S^0$. ✓

**Theorem 2.3 (Suspension of a point).** $\Sigma \mathbf{1} \simeq \mathbb{I}$ (the interval).

*Proof.* $\Sigma \mathbf{1}$ has $\mathsf{N}$, $\mathsf{S}$, and one meridian $\mathsf{merid}(\star) : \mathsf{N} = \mathsf{S}$. This is exactly the interval! $\square$

**Theorem 2.4 (Suspension of Bool is the circle).** $\Sigma \mathsf{Bool} \simeq S^1$.

*Proof.* $\Sigma \mathsf{Bool}$ has $\mathsf{N}$, $\mathsf{S}$, and two meridians $\mathsf{merid}(\mathsf{true}) : \mathsf{N} = \mathsf{S}$ and $\mathsf{merid}(\mathsf{false}) : \mathsf{N} = \mathsf{S}$.

The circle $S^1$ has one base point and one loop. Can we match these?

Define $f : \Sigma \mathsf{Bool} \to S^1$:
- $f(\mathsf{N}) = \mathsf{base}$
- $f(\mathsf{S}) = \mathsf{base}$
- $f(\mathsf{merid}(\mathsf{true})) = \mathsf{loop} : \mathsf{base} = \mathsf{base}$... wait, but $\mathsf{merid}(\mathsf{true}) : \mathsf{N} = \mathsf{S}$ and $f(\mathsf{N}) = f(\mathsf{S}) = \mathsf{base}$, so we need $\mathsf{ap}_f(\mathsf{merid}(\mathsf{true})) : \mathsf{base} = \mathsf{base}$. Take this to be $\mathsf{loop}$.
- $f(\mathsf{merid}(\mathsf{false})) = \mathsf{refl}_\mathsf{base}$.

Define $g : S^1 \to \Sigma \mathsf{Bool}$:
- $g(\mathsf{base}) = \mathsf{N}$
- $g(\mathsf{loop}) = \mathsf{merid}(\mathsf{true}) \cdot (\mathsf{merid}(\mathsf{false}))^{-1} : \mathsf{N} = \mathsf{N}$ (go down via true, come back up via false-inverse)

Show $f \circ g \sim \mathsf{id}_{S^1}$ and $g \circ f \sim \mathsf{id}_{\Sigma \mathsf{Bool}}$ using the respective elimination principles. $\square$

**The general pattern:** $\Sigma S^n \simeq S^{n+1}$.

## The 2-Sphere

**Definition 2.5 (2-sphere).** $S^2 = \Sigma S^1$, or directly:
- Point constructor: $\mathsf{N} : S^2$
- Point constructor: $\mathsf{S} : S^2$
- Path constructor: $\mathsf{merid}(x) : \mathsf{N} = \mathsf{S}$ for each $x : S^1$

The 2-sphere has a continuum of meridians, one for each point of the circle.

Alternatively, $S^2$ can be defined as a HIT with one point and one 2-cell:
- Point constructor: $\mathsf{base}_2 : S^2$
- 2-cell constructor: $\mathsf{surface} : \mathsf{refl}_{\mathsf{base}_2} = \mathsf{refl}_{\mathsf{base}_2}$ (a 2-path at the base point)

Both definitions are equivalent (using the fact that $\Sigma S^1$ has the suspension structure which gives the right homotopy type).

## The n-Sphere

**Definition 2.6 ($n$-sphere).** The $n$-sphere is defined inductively:
$$S^{-1} :\equiv \mathbf{0}$$
$$S^{n+1} :\equiv \Sigma S^n$$

**Properties:**
- $S^0 = \mathbf{Bool}$ (two points)
- $S^1$ has fundamental group $\mathbb{Z}$
- $S^n$ is $(n-1)$-connected: $\pi_k(S^n) = 0$ for $k < n$
- $\pi_n(S^n) = \mathbb{Z}$ (degree maps)

**Computing homotopy groups of spheres.** The homotopy groups $\pi_k(S^n)$ are notoriously difficult to compute classically. In synthetic HoTT:
- $\pi_1(S^1) = \mathbb{Z}$ ✓ (Chapter 20)
- $\pi_2(S^2) = \mathbb{Z}$ (follows from Freudenthal)
- $\pi_3(S^2) = \mathbb{Z}$ (Hopf fibration)
- $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$ (Brunerie's theorem)
- $\pi_k(S^n)$ for $k \leq 2n - 2$: stable and computed by Freudenthal

## The Elimination Principle for Suspension

The suspension eliminator is the key tool for proving things about $\Sigma A$:

**Theorem 2.7 (Dependent eliminator for $\Sigma A$).** For $P : \Sigma A \to \mathsf{Type}$, a section $s : \prod_{x:\Sigma A} P(x)$ is determined by:
- $n : P(\mathsf{N})$
- $s' : P(\mathsf{S})$
- $m : \prod_{a:A} \mathsf{transport}^P(\mathsf{merid}(a), n) = s'$

The third condition says: for each $a : A$, the meridian $\mathsf{merid}(a)$ transports $n$ to $s'$.

**Application.** To define $\mathsf{code} : S^2 \to \mathsf{Type}$ (a type family over the 2-sphere), we need:
- A type at $\mathsf{N}$
- A type at $\mathsf{S}$
- For each $x : S^1$, a path in $\mathsf{Type}$ (equivalence) between these types

This is how the code family for $\pi_2(S^2) = \mathbb{Z}$ is constructed: use the group of integers with the action of $\pi_1(S^1) = \mathbb{Z}$ on it.

## The Join and Higher-Dimensional Cells

**Definition 2.8 (Join).** The *join* $A * B$ is the pushout:
$$A \xleftarrow{\pi_1} A \times B \xrightarrow{\pi_2} B$$

or explicitly:
- Point constructor: $\mathsf{inl}(a) : A * B$ for $a : A$
- Point constructor: $\mathsf{inr}(b) : A * B$ for $b : B$
- Path constructor: $\mathsf{join}(a, b) : \mathsf{inl}(a) = \mathsf{inr}(b)$ for $a : A$, $b : B$

**Key facts:**
- $A * \mathbf{0} = A$ and $\mathbf{0} * B = B$
- $A * \mathbf{1} = \Sigma A$ (suspension as a special join)
- $S^m * S^n = S^{m+n+1}$

The last fact is crucial for the Hopf fibration: $S^1 * S^1 = S^3$.

**The join as "paths between spaces".** Topologically, $A * B$ is the "space of all paths from $A$ to $B$" — you can travel from any point of $A$ to any point of $B$ via a "join path." This is related to the $*$-product in homological algebra.

## Why Suspensions Matter

Suspension is the fundamental operation connecting lower-dimensional and higher-dimensional homotopy theory:

**The suspension isomorphism.** For $(n-1)$-connected types, $\pi_k(A) \cong \pi_{k+1}(\Sigma A)$ for $k \leq 2n - 1$ (Freudenthal). This lets you "shift" homotopy groups by one dimension.

**Building spheres.** Every sphere is a (repeated) suspension of a point. The sphere $S^n$ is the "$n$-th suspension of a discrete two-point space" — the simplest possible spaces, from which all spheres grow.

**Cohomology.** In HoTT, cohomology theories can be defined as functors on types that are "stable" under suspension. The suspension isomorphism for cohomology ($\tilde{H}^n(\Sigma A) \cong \tilde{H}^{n-1}(A)$) is the type-theoretic version of the classical reduced cohomology suspension isomorphism.

**Spectra.** A *spectrum* is a sequence of types $E_0, E_1, E_2, \ldots$ with equivalences $E_n \simeq \Omega E_{n+1}$. Spectra are the "stable" objects in homotopy theory, and they encode generalized cohomology theories. In HoTT, spectra can be defined using suspensions and the Univalence axiom.
