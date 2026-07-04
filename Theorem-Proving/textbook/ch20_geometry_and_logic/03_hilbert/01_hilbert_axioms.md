# Hilbert's *Grundlagen der Geometrie*

David Hilbert's *Grundlagen der Geometrie* (*Foundations of Geometry*, 1899) is the founding document of modern axiomatics. Where Euclid took *point*, *line*, and *plane* to name antecedently understood things and read order and continuity off the diagram, Hilbert took the primitives to be **implicitly defined by the axioms alone** and closed every gap of [Section 1](../01_euclid/01_euclid_axioms.md) with an explicit sentence. His remark captures the shift: one must be able to say "at all times — instead of *points*, *lines*, and *planes* — *tables*, *chairs*, and *beer mugs*," because a proof must go through under *every* interpretation making the axioms true. The book does not merely repair Euclid; it makes an axiom system itself a mathematical object, proving its **consistency**, the **independence** of its axioms, and its **categoricity**.

## The Five Groups

Hilbert takes three primitive terms — point, line, plane — and three primitive relations: **incidence** ("lies on"), **betweenness** (a ternary order relation on points), and **congruence** (of segments and, separately, of angles). Everything else is *defined*. The axioms fall into five groups.

**Group I — Incidence** (8 axioms). The combinatorics of "lies on": two distinct points determine a unique line (I.1–I.2); every line has at least two points; three non-collinear points exist (I.3); with analogues for planes and space. These fix what meets what, but say nothing of order or distance.

**Group II — Order** (4 axioms). Betweenness $B(A,B,C)$, the notion Euclid lacked. **II.1**: if $B(A,B,C)$ then $A,B,C$ are distinct collinear and $B(C,B,A)$. **II.2**: every segment extends ($\exists B$ with $B(A,C,B)$). **II.3**: of three collinear points exactly one is between the others. **II.4 (Pasch)**: a line entering a triangle through one side, missing the vertices, exits through another side. From these one *proves* what Euclid assumed: betweenness linearly orders a line, and a line separates the plane into two sides.

**Group III — Congruence** (5 axioms). Segment and angle congruence (each an equivalence relation), a segment-transport axiom (III.1), additivity, angle transport, and the decisive **III.5 (SAS)**: if two triangles agree in two sides and the included angle, all remaining parts agree. SAS is exactly the proposition Euclid I.4 "proved" by superposition; Hilbert's diagnosis — confirmed by independence — is that it is *not derivable* and must be posited. "Superposition" is retired for good.

**Group IV — Parallels** (1 axiom). Playfair's form: through a point not on a line there is *at most one* parallel. This is the axiom whose independence [Section 2](../02_non_euclidean/01_parallel_postulate.md) established.

**Group V — Continuity** (2 axioms). **V.1 (Archimedes)**: finitely many copies of any segment exceed any other — no infinitesimal segments. **V.2 (Line completeness)**: the points of a line admit no extension preserving Groups I–III and V.1 — a maximality axiom, the geometric form of Dedekind completeness, filling Euclid's continuity gap.

## Pons Asinorum Without Superposition

A taste of Hilbert's method — and the theorem later mechanized in proof assistants ([Section 5](../05_lean/01_geometry_lean.md)).

**Proposition (Euclid I.5).** In a triangle with $AB \cong AC$, the base angles are congruent: $\angle ABC \cong \angle ACB$.

*Proof.* Consider the correspondence of triangle $ABC$ with *itself* in the order $A\leftrightarrow A$, $B\leftrightarrow C$, $C\leftrightarrow B$. Then $AB \leftrightarrow AC$ (congruent by hypothesis), $AC \leftrightarrow AB$ (likewise), and the included angle $\angle BAC \leftrightarrow \angle CAB$ — the *same* angle, congruent to itself. Two sides and the included angle of $ABC$ thus match those of $ACB$, so by **SAS (III.5)** the correspondence is a congruence; in particular $\angle ABC \cong \angle ACB$. $\square$

No figure is lifted and laid on another: Pappus's proof, revived by Hilbert, uses only SAS applied to a triangle and its mirror labelling, and is short enough to formalize verbatim. That is the point — with the right axioms, Euclid's informal maneuvers become licensed inferences.

## Consistency by an Analytic Model

Are the axioms consistent? Hilbert answers by **building a model out of numbers**: points are pairs $(x,y) \in \mathbb{R}^2$, lines are ratio triples $(u:v:w)$ (solution sets of $ux+vy+w=0$), betweenness is numerical order along a line, and segment congruence is equality of Euclidean distance $\sqrt{(x_1-x_2)^2+(y_1-y_2)^2}$. All of I–V hold in $\mathbb{R}^2$, so:

> If the arithmetic of the real numbers is consistent, so is Euclidean geometry.

This is **relative consistency** by interpretation — the method Section 2 used for the non-Euclidean geometries, and the one by which all later consistency questions were attacked. Hilbert could not prove $\mathbb{R}$ consistent outright (Gödel later showed why no finitary proof exists, Chapter 10), but reducing geometry's consistency to arithmetic's is exactly what a foundation should do.

## Independence by Partial Models

To show no axiom is redundant, Hilbert builds, for a target axiom $\varphi$, a model of *all the others but not $\varphi$*.

- **Parallel axiom (IV).** The Klein or Poincaré disk model of [Section 2](../02_non_euclidean/01_parallel_postulate.md) satisfies I–III, V but refutes IV — the classical independence of the parallel postulate, in one line.
- **Archimedes (V.1).** Coordinatize over a **non-Archimedean ordered field** — e.g. rational functions ordered so an infinitesimal $t$ is below every positive rational. Groups I–IV hold while V.1 fails: distance $1$ is never reached by finitely many copies of length $t$. This is genuine **non-Archimedean geometry**.
- **SAS (III.5).** A distorted, non-symmetric congruence satisfies III.1–III.4 but not III.5 — confirming that side–angle–side must be *assumed*, and Euclid's unease was justified.

## Segment Arithmetic and Coordinatization

Hilbert's deepest discovery reverses the analytic model: instead of *importing* numbers, he *manufactures* an algebra from within the geometry — the **Streckenrechnung**, or calculus of segments. Fix a unit segment; define the sum of segments by laying them end to end and the product by a similar-triangles construction. Order and congruence make the segments an **ordered field**, and which algebraic laws hold is governed by which geometric theorems are available:

- **Desargues's theorem** holds iff the coordinate ring is a **division ring** (associative, possibly non-commutative).
- **Pappus's theorem** holds iff multiplication is **commutative** — a genuine *field* (and Pappus implies Desargues, a geometric shadow of Wedderburn's theorem).
- **Archimedes (V.1)** embeds the field in $\mathbb{R}$; **completeness (V.2)** forces it to be $\mathbb{R}$ exactly.

This is the two-way bridge between geometry and algebra: a plane is essentially a field, its geometric axioms algebraic axioms in disguise. It is precisely this coordinatization — carried out first-order — that [Tarski](../04_tarski/01_tarski_geometry.md) exploits to prove elementary geometry decidable.

## Categoricity and Its Price

**Theorem (Categoricity).** Any two models of the full system (I–V, both continuity axioms) are isomorphic — each to the Cartesian plane $\mathbb{R}^2$ (or $\mathbb{R}^3$ for space).

So there is, up to relabelling, exactly one Euclidean plane: the informal "space" of intuition is captured completely. But the triumph hides a subtlety the next section makes decisive. The completeness axiom V.2 quantifies over *arbitrary* sets of points and is therefore irreducibly **second-order**. Categoricity is a second-order phenomenon, unavailable to any first-order theory (by Löwenheim–Skolem and compactness, Chapter 9, a first-order theory with an infinite model has models of every infinite cardinality). So pinning down the model costs Hilbert the realm where completeness, compactness, and effective proof search apply. Tarski's move — replace V.2 by a first-order schema, surrender categoricity, and gain **completeness of the theory and decidability** — is [Section 4](../04_tarski/01_tarski_geometry.md).

## Exercises
See [problems/ch20_geometry_and_logic/](../../../problems/ch20_geometry_and_logic/)
