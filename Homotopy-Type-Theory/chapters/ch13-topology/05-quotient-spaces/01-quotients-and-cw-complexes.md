# 5.1 Quotient Spaces and CW Complexes

## Gluing Spaces Together

Some of the most important topological spaces are built by taking simpler spaces and *gluing* parts of them together. The circle is built by identifying the two endpoints of an interval. The torus is built by identifying opposite edges of a square. Projective space is built by identifying antipodal points of a sphere.

All of these are instances of *quotient spaces* — and this construction is central to both classical topology and HoTT (where it's axiomatized by higher inductive types).

## The Quotient Construction

**Definition 5.1 (Quotient Space).** Given a topological space $X$ and an equivalence relation $\sim$ on $X$:
- The *quotient set* $X/\sim$ is the set of equivalence classes $[x] = \{y \in X : y \sim x\}$
- The *quotient map* $q : X \to X/\sim$, $q(x) = [x]$, is surjective
- The *quotient topology* on $X/\sim$: a set $V \subseteq X/\sim$ is open iff $q^{-1}(V)$ is open in $X$

The quotient topology is the *final topology* with respect to $q$: it's the finest topology making $q$ continuous.

**Universal property.** A function $f : X/\sim \to Y$ is continuous iff $f \circ q : X \to Y$ is continuous and $x \sim y \Rightarrow f(x) = f(y)$.

This is the key property: to define a continuous function *out of* a quotient, you just need a continuous function out of $X$ that respects the equivalence relation.

**Quotient topology is not always Hausdorff.** Even if $X$ is Hausdorff, $X/\sim$ may not be. Example: $\mathbb{R}/\mathbb{Q}$ (identifying rationals) is an example of a non-Hausdorff quotient.

## Key Examples

### The Circle

$S^1 = [0,1]/(0 \sim 1)$: identify the two endpoints of $[0,1]$.

The quotient map $q : [0,1] \to S^1$ sends $t$ to $e^{2\pi i t}$ (in the complex number interpretation). It's a continuous surjection, and the quotient topology gives $S^1$ its standard topology (homeomorphic to the unit circle in $\mathbb{R}^2$).

### The Torus

$T^2 = [0,1]^2/\sim$ where $(x,0) \sim (x,1)$ (identify top and bottom edges) and $(0,y) \sim (1,y)$ (identify left and right edges).

Equivalently: $T^2 = S^1 \times S^1$. This is a compact surface — the "donut."

### The Möbius Band

$[0,1]^2/\sim$ where $(0,y) \sim (1, 1-y)$ (identify left and right edges with a flip). This is a non-orientable surface with boundary.

### Real Projective Space

$\mathbb{RP}^n = S^n/(x \sim -x)$: identify antipodal points.

$\mathbb{RP}^1 \cong S^1$ (the circle). $\mathbb{RP}^2$ is the real projective plane — a non-orientable closed surface. $\pi_1(\mathbb{RP}^n) = \mathbb{Z}/2\mathbb{Z}$ for $n \geq 2$.

### Suspensions

The *suspension* of a space $X$ is $\Sigma X = X \times [-1,1]/\sim$ where all of $X \times \{1\}$ is identified to a single point (the "north pole") and all of $X \times \{-1\}$ is identified to a single point (the "south pole").

$\Sigma S^n \cong S^{n+1}$: the suspension of the $n$-sphere is the $(n+1)$-sphere. This is a fundamental operation in homotopy theory, corresponding to the suspension HIT in HoTT.

## CW Complexes

CW complexes are the "right" spaces for homotopy theory. They're built inductively by attaching cells of increasing dimension.

**Definition 5.2 (CW Complex).** A *CW complex* is a space $X$ built inductively:
- **$X^0$ (0-skeleton):** A discrete set of points (0-cells)
- **$X^n$ ($n$-skeleton):** Attach $n$-cells $e^n_\alpha$ (copies of $D^n$, the closed $n$-disk) via *attaching maps* $\phi_\alpha : S^{n-1} \to X^{n-1}$

The space $X^n$ is the pushout:
$$X^{n-1} \xleftarrow{\sqcup \phi_\alpha} \sqcup_\alpha S^{n-1} \hookrightarrow \sqcup_\alpha D^n$$

Then $X = \bigcup_n X^n$ with the weak topology (a set is closed iff its intersection with each $X^n$ is closed).

**Why "CW"?** C = "closure-finite" (each cell's closure meets only finitely many other cells); W = "weak topology."

**Examples:**
- $S^n$: one 0-cell and one $n$-cell (attached by the constant map $S^{n-1} \to \{*\}$)
- $T^2$: one 0-cell, two 1-cells, one 2-cell
- $\mathbb{RP}^n$: one cell in each dimension 0 through $n$
- Any simplicial complex (triangulated space) is a CW complex

**Why CW complexes are "right" for homotopy theory:**
1. Every topological space is weakly homotopy equivalent to a CW complex (CW approximation)
2. The category of CW complexes has very good homotopy-theoretic properties
3. Homotopy groups are easy to compute inductively by Mayer-Vietoris and cellular chain complexes
4. CW complexes are exactly the spaces that can be built step by step, which mirrors inductive type definitions

## Pushouts and Homotopy Pushouts

The key categorical construction underlying CW complexes is the *pushout*.

**Definition 5.3 (Pushout).** Given maps $f : A \to X$ and $g : A \to Y$, the pushout $X \sqcup_A Y$ is the quotient:
$$X \sqcup_A Y = (X \sqcup Y)/(f(a) \sim g(a) \text{ for all } a \in A)$$

This "glues" $X$ and $Y$ together along $A$, identifying the images of $f$ and $g$.

Attaching an $n$-cell is a special pushout: $X^n = X^{n-1} \sqcup_{S^{n-1}} D^n$ (along the attaching map $S^{n-1} \to X^{n-1}$).

**Homotopy pushout.** In homotopy theory, the "right" notion of pushout is the *homotopy pushout*, which is homotopy invariant. For CW complexes, pushouts and homotopy pushouts agree (up to homotopy equivalence).

**In HoTT:** Pushouts are axiomatized as higher inductive types. The circle HIT is the pushout of two contractible spaces along two maps from the 0-sphere $S^0 = \{0, 1\}$... or more directly: one point generator and one path generator. This is the type-theoretic analog of the CW complex construction.

## The Connection to Higher Inductive Types

The key insight: **CW complexes in topology correspond to higher inductive types in HoTT.**

The correspondences:
- 0-cells (points) ↔ point constructors
- 1-cells (paths) ↔ path constructors
- 2-cells (homotopies) ↔ homotopy constructors
- $n$-cells ↔ $n$-dimensional generators

**The circle:**
- Classical: $S^1 = D^0 \cup_\phi D^1$ (one point, one edge glued to itself)
- HoTT: $S^1 :=$ a type with $\mathsf{base} : S^1$ and $\mathsf{loop} : \mathsf{base} = \mathsf{base}$

**The 2-sphere:**
- Classical: $S^2 = D^0 \cup D^2$ (one point, one 2-cell attached by a map $S^1 \to \{*\}$)
- HoTT: $S^2 :=$ a type with $\mathsf{north}, \mathsf{south} : S^2$, $\mathsf{merid}(x) : \mathsf{north} = \mathsf{south}$ for $x : S^1$, and a 2-cell filling...

**Suspension:**
- Classical: $\Sigma X = (X \times I) / (X \times \{0\} \sim * = X \times \{1\})$
- HoTT: $\Sigma A :=$ a type with $\mathsf{N}, \mathsf{S} : \Sigma A$ and $\mathsf{merid}(a) : \mathsf{N} = \mathsf{S}$ for each $a : A$

The HIT construction is a synthetic version of the CW construction. In HoTT, you don't need to specify open sets or attaching maps — you just declare the generators (constructors) and the induction principle takes care of the rest.

## Quotient Types in HoTT

More generally, quotient spaces in classical topology correspond to *quotient types* (or *set quotients*) in HoTT.

**Set quotient (HIT):** Given a type $A$ and a relation $R : A \to A \to \mathsf{Prop}$, the quotient $A/R$ is a HIT with:
- $q : A \to A/R$ (the quotient map)
- $\mathsf{eq} : \prod_{a,b:A} R(a,b) \to q(a) = q(b)$ (related elements are equal)
- $\mathsf{set} : \mathsf{isSet}(A/R)$ (the quotient is a set)

The recursion principle: to define $f : A/R \to B$ (for a set $B$), provide $f_0 : A \to B$ with $\prod_{a,b} R(a,b) \to f_0(a) = f_0(b)$.

This is the HoTT analog of the universal property of quotient spaces. It's constructive and homotopy-coherent.

## Summary

| Topological Concept | HoTT Analog |
|---|---|
| Quotient space $X/\sim$ | Quotient type (set quotient HIT) |
| Identifying a point to a point | Path constructor |
| Attaching a 1-cell | Path between constructors |
| Attaching a 2-cell | 2-path (homotopy) constructor |
| CW complex | Higher inductive type |
| Suspension $\Sigma X$ | Suspension type |
| Pushout $X \sqcup_A Y$ | Pushout HIT |

Quotient spaces are the key construction in both classical topology and HoTT. In classical topology, you specify the equivalence relation set-theoretically. In HoTT, you specify the generators (constructors) and get the equivalence relation "for free" from the path structure. HITs are quotient spaces done synthetically and homotopy-coherently.
