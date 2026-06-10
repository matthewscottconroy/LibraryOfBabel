# CW Complexes and HoTT

## The Syntax of Spaces

There is a sense in which CW complexes give spaces a *syntax*. A space is described by listing its cells — its points, its edges, its faces, its higher-dimensional pieces — and its attaching maps, which specify how higher-dimensional cells are glued onto lower-dimensional ones. From this data you can read off the space's homotopy groups, homology groups, and fundamental group. The space is presented by generators and relations, just like a group.

This syntactic description of spaces is the bridge to type theory. In HoTT, types are described by their constructors — their generators and the relations among them — and those constructors generate all the terms and paths in the type. The structural parallel is not accidental; it is the mathematical substance of the correspondence between topology and type theory.

## CW Complexes: The Full Construction

A CW complex $X$ is built inductively. Start with a set $X^0$ of *0-cells* (points), given the discrete topology. At each stage $n$, form $X^n$ from $X^{n-1}$ by attaching $n$-cells.

An *$n$-cell* is an open $n$-disk $e^n_\alpha$ (an open ball in $\mathbb{R}^n$). To attach it, you specify an *attaching map* $\phi_\alpha : S^{n-1} \to X^{n-1}$ from the boundary of the closed $n$-disk $\overline{e^n_\alpha} = D^n$ into the existing $(n-1)$-skeleton. The *$n$-skeleton* is:
$$X^n = X^{n-1} \cup_{\{\phi_\alpha\}} \bigsqcup_\alpha D^n_\alpha$$
where the disjoint union of disks is glued to $X^{n-1}$ by identifying $x \in S^{n-1} \subset D^n_\alpha$ with $\phi_\alpha(x) \in X^{n-1}$.

This is a pushout in **Top**:
$$\bigsqcup_\alpha S^{n-1} \xrightarrow{\bigsqcup \phi_\alpha} X^{n-1}$$
$$\downarrow \qquad\qquad\qquad \downarrow$$
$$\bigsqcup_\alpha D^n \longrightarrow X^n$$

The CW complex is the colimit $X = \text{colim}_n X^n$ with the weak topology (a set is open iff its intersection with each $X^n$ is open).

## Canonical Examples

**The circle $S^1$:** One 0-cell $*$ and one 1-cell $e^1$. The attaching map $\phi : S^0 \to X^0 = \{*\}$ sends both boundary points of the interval (which form $S^0 = \{-1, 1\}$) to the single 0-cell. The result: the interval $[-1,1]$ with its two endpoints identified — the circle.

**The $n$-sphere $S^n$:** One 0-cell $*$ and one $n$-cell. The attaching map $\phi : S^{n-1} \to \{*\}$ is the constant map. The entire boundary of the $n$-disk is collapsed to a point, giving a sphere.

**The real projective plane $\mathbb{RP}^2$:** One 0-cell, one 1-cell (forming a circle), and one 2-cell. The 2-cell is attached via the attaching map $\phi : S^1 \to S^1$ of degree 2 (wrapping the circle twice around itself). This kills $\pi_1(S^1) = \mathbb{Z}$ by relation $a^2 = 1$, giving $\pi_1(\mathbb{RP}^2) = \mathbb{Z}/2\mathbb{Z}$.

**The torus $T^2$:** One 0-cell $*$, two 1-cells $a$ and $b$ (each with both endpoints at $*$, forming a figure-eight), and one 2-cell attached via the word $aba^{-1}b^{-1}$ (traverse $a$, then $b$, then $a$ backwards, then $b$ backwards). This gives $\pi_1(T^2) = \langle a, b \mid aba^{-1}b^{-1} \rangle = \mathbb{Z}^2$.

## Van Kampen for CW Complexes

For CW complexes, the Seifert-van Kampen theorem takes a particularly clean form. The fundamental group of the 1-skeleton $X^1$ is the free group on the 1-cells (one generator per 1-cell, using the 0-cells as basepoints and spanning tree). Attaching a 2-cell via attaching map $\phi : S^1 \to X^1$ kills the element $[\phi] \in \pi_1(X^1)$: the fundamental group of $X^2$ is $\pi_1(X^1)$ modulo the normal closure of $\{[\phi_\alpha]\}$.

Higher cells (dimension $\geq 3$) do not affect $\pi_1$. So the fundamental group of a CW complex is completely determined by its 2-skeleton:
$$\pi_1(X) \cong \pi_1(X^2)$$

This gives an explicit presentation of $\pi_1$: generators are 1-cells (modulo spanning tree); relations are attaching maps of 2-cells. The fundamental group is a group given by generators and relations — and every group arises this way (every group has a CW complex with that fundamental group, constructed explicitly by taking one 1-cell per generator and one 2-cell per relation).

## Whitehead's Theorem

**Theorem (Whitehead).** A continuous map $f : X \to Y$ between CW complexes is a homotopy equivalence if and only if $f_* : \pi_n(X) \to \pi_n(Y)$ is an isomorphism for all $n \geq 0$.

A map inducing isomorphisms on all homotopy groups is called a *weak homotopy equivalence*. Whitehead's theorem says: between CW complexes, weak homotopy equivalences are genuine homotopy equivalences. (This fails for general spaces — there are spaces that are weakly equivalent but not homotopy equivalent.)

Whitehead's theorem is the justification for working with CW complexes: you can detect homotopy equivalence by purely algebraic means (computing homotopy groups). It also motivates the notion of a weak equivalence in model category theory: the Quillen model structure on topological spaces uses weak homotopy equivalences as weak equivalences, and Whitehead's theorem ensures that these are genuine equivalences between CW complexes.

## The HoTT Dictionary

Here is the precise correspondence between CW complexes and higher inductive types:

| CW Complex | HoTT |
|---|---|
| 0-cell (point) | Point constructor |
| 1-cell with attaching map $\phi : \{0,1\} \to X^0$ | Path constructor: $\phi(0) = \phi(1)$ |
| 2-cell with attaching map $\phi : S^1 \to X^1$ | 2-path constructor: identifying two paths |
| $n$-cell with attaching map $\phi : S^{n-1} \to X^{n-1}$ | $n$-path constructor |
| Pushout of cell attachment | HIT with the constructor |
| Van Kampen computation of $\pi_1$ | Encode-decode method in HoTT |
| Whitehead's theorem | Univalence + encode-decode |

**The circle $S^1$ as a HIT:**
```
data S¹ : Type where
  base : S¹
  loop : base =_{S¹} base
```

This declares: there is a type $S^1$ with one point constructor `base` and one path constructor `loop` (an element of the identity type `base = base`). Any function out of $S^1$ must specify:
- Where `base` goes (the point value).
- Where `loop` goes (a path in the target from the image of `base` to itself).

This is exactly the universal property of the circle as a pushout: a map out of $S^1$ is a map out of $[0,1]$ that respects the identification of the two endpoints. In categorical terms: maps from the HIT $S^1$ to a type $A$ are exactly pairs $(a : A, p : a = a)$.

**The 2-sphere $S^2$:**
```
data S² : Type where
  base : S²
  surf : refl base =_{base = base} refl base
```

The constructor `surf` is a path between two paths — an element of the second-order identity type. This corresponds to the 2-cell of $S^2$, whose attaching map sends $S^1$ to the single point of $S^2$ (since there are no 1-cells in $S^2$, the 1-dimensional loop is trivial, and `surf` is a path between reflexivity and itself — a 2-dimensional filling).

## Suspensions and Joins

Two important operations on CW complexes have direct HIT analogs:

The *suspension* $\Sigma X$ of a space $X$ is the pushout of $X \leftarrow X \times \{0,1\} \rightarrow \{N, S\}$ — two cones glued at their bases. For $X = S^{n-1}$, $\Sigma S^{n-1} = S^n$. The suspension adds a dimension.

As a HIT:
```
data Susp (A : Type) : Type where
  N : Susp A
  S : Susp A
  merid : A → (N = S)
```

The *join* $X * Y$ is a more complex combination. As a HIT, the join of two types has the two types as point constructors and paths between their points as path constructors.

These operations let you build all higher spheres synthetically: $S^0 = \text{Bool}$, $S^1 = \Sigma S^0$, $S^n = \Sigma S^{n-1}$.

## Why This Matters

The HIT-CW correspondence is not just an analogy — it is the mathematical reason why HoTT can prove theorems about classical spaces. When you prove $\pi_1(S^1) = \mathbb{Z}$ in HoTT, you are proving it about the HIT $S^1$, which is the synthetic version of the topological circle. The proof works because the HIT correctly axiomatizes all the information about the circle that matters for homotopy theory — and it does so because HITs are the synthetic version of CW complexes.

This is what it means for HoTT to "internalize" topology. You do not import topological spaces into type theory and then do homotopy theory with them. Instead, the type theory itself provides the language for describing spaces synthetically, through HITs. The CW complex structure is built into the type, not added afterward.
