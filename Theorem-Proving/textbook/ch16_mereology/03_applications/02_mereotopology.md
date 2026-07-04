# Mereotopology: Parts, Wholes, and Connection

Mereology is blind to shape. In GEM the fusion of two separated islands and a single connected island of the same total area have exactly the same mereological profile — parthood cannot distinguish a continuous whole from a scattered one, nor touching from distance. **Mereotopology** repairs this by adding a topological primitive to the mereological base, and it descends from A. N. Whitehead's programme of *point-free geometry*: take extended **regions**, not points, as primitive — since points are never given in experience — and reconstruct points later as limits of nested regions ("extensive abstraction"). What Whitehead sketched in *Process and Reality* (1929), Clarke and the Region Connection Calculus made formal.

## The Connection Primitive

Add to the language a binary predicate $C(x,y)$, "$x$ is connected to $y$" — intended model: regions of space that touch or overlap (as regular closed sets, $x \cap y \neq \varnothing$). The core axioms:

$$
\begin{aligned}
&\textbf{(C1) Reflexivity:} && \forall x\, C(x,x)\\
&\textbf{(C2) Symmetry:} && \forall x \forall y\,\bigl(C(x,y) \to C(y,x)\bigr)\\
&\textbf{(C3) Monotonicity:} && \forall x \forall y\,\bigl(P(x,y) \to \forall z\,(C(z,x) \to C(z,y))\bigr)
\end{aligned}
$$

C3 links the two primitives: whatever touches a part touches the whole. Clarke (1981) went further and *defined* parthood from connection:

**Definition (Clarke).** $P(x,y) \equiv \forall z\,\bigl(C(z,x) \to C(z,y)\bigr)$.

With this definition C3 becomes a triviality and antisymmetry amounts to an extensionality axiom for $C$: regions connected to exactly the same things are identical. One primitive then suffices for both topology and mereology — a remarkable economy. Note the family resemblance to the overlap criterion of Section 2 ($P(x,y) \leftrightarrow \forall z(O(z,x) \to O(z,y))$ in EM): connection plays the role of overlap, but is strictly more discriminating, since regions can be connected (touching) without overlapping.

## RCC-8

The **Region Connection Calculus** (Randell, Cui, and Cohn, 1992) distills mereotopology into eight relations, all defined from $C$ (with $O(x,y) \equiv \exists z(P(z,x) \land P(z,y))$ and $P$ as in Clarke's definition):

| Relation | Definition | Reading |
|---|---|---|
| $DC(x,y)$ | $\neg C(x,y)$ | disconnected |
| $EC(x,y)$ | $C(x,y) \land \neg O(x,y)$ | externally connected (touch, no overlap) |
| $PO(x,y)$ | $O(x,y) \land \neg P(x,y) \land \neg P(y,x)$ | partial overlap |
| $TPP(x,y)$ | $PP(x,y) \land \exists z\,(EC(z,x) \land EC(z,y))$ | tangential proper part |
| $NTPP(x,y)$ | $PP(x,y) \land \neg\exists z\,(EC(z,x) \land EC(z,y))$ | non-tangential proper part |
| $TPP^{-1}, NTPP^{-1}$ | converses | has tangential / interior proper part |
| $EQ(x,y)$ | $P(x,y) \land P(y,x)$ | equal |

These eight are **jointly exhaustive and pairwise disjoint** (JEPD): any two regions stand in exactly one. Geographic examples: France $EC$ Germany (shared border, no shared territory); Bavaria $TPP$ Germany (part, touching the national border); Berlin $NTPP$ Germany (strictly interior); France $DC$ Australia.

## Reasoning with the Composition Table

RCC-8 supports a calculus of relational composition: given $R(x,y)$ and $S(y,z)$, which base relations may hold between $x$ and $z$? The answers form an $8 \times 8$ **composition table** whose entries are disjunctions. Two entries:

$$NTPP \circ NTPP = \{NTPP\}, \qquad\quad NTPP \circ EC = \{DC\}.$$

**Worked example.** The nucleolus is $NTPP$ the nucleus, the nucleus is $NTPP$ the cell: by the first entry, the nucleolus is $NTPP$ the cell — interiority composes. And from $NTPP(\text{Berlin}, \text{Germany})$ and $EC(\text{Germany}, \text{France})$, the second entry yields $DC(\text{Berlin}, \text{France})$: a region strictly inside Germany cannot touch anything that only touches Germany's boundary. By contrast $EC \circ NTPP = \{PO, TPP, NTPP\}$ — touching something buried deep inside $z$ forces overlap with $z$, but leaves open how much.

A **constraint network** assigns each pair of variables a set of possible base relations; the *path consistency* algorithm repeatedly refines $T(x,z)$ by intersecting it with the composition of $R(x,y)$ and $S(y,z)$ until a fixed point. This is the workhorse of qualitative spatial reasoning.

## Decidability and Complexity

- **RCC-8 constraint satisfaction is NP-complete** (Renz and Nebel, 1999): deciding whether a network of disjunctive RCC-8 constraints is satisfiable is intractable in general.
- **Tractable fragments exist**: for networks over the base relations, path consistency already decides satisfiability in polynomial time; Renz identified the *maximal* tractable fragments — notably $\widehat{\mathcal{H}}_8$, with 148 of the $2^8$ disjunctive relations — such that adding any further relation restores NP-hardness. Practical solvers split a hard network into tractable sub-networks and backtrack.
- **Full first-order mereotopology is undecidable** (Grzegorczyk, 1951): the elementary theory of regions with parthood *and* connection over Euclidean space interprets enough arithmetic to be undecidable. Contrast this with the previous section: pure GEM is decidable, so it is precisely the added topological structure that crosses the line. RCC-8 stays usable by retreating from full quantification to the constraint (quantifier-free, relation-algebraic) fragment.

## Applications

Mereotopology is the rare corner of metaphysics with an industrial user base. **Qualitative spatial reasoning** uses RCC-8 where coordinates are unknown or irrelevant. **Geographic information systems** implement RCC-8 (and the closely related Egenhofer 9-intersection relations) for spatial queries — the GeoSPARQL standard exposes the eight relations as query predicates. **Formal ontologies** build on mereotopological primitives: Basic Formal Ontology (BFO), used by hundreds of biomedical ontologies, axiomatizes parthood and connection for anatomical structure — the nucleolus/cell example above is exactly the kind of inference an anatomy ontology must license automatically.

There is also a direct line to **formal verification** (Chapter 13). Bennett (1996) encoded RCC-8 satisfiability into propositional modal logic (a fragment of S4), reducing spatial consistency to modal satisfiability; modern implementations encode a constraint network into SAT or SMT — one Boolean variable per pair-and-base-relation, with JEPD and the composition table as clauses — and let an off-the-shelf solver do the search. Checking the consistency of a map's topology thereby becomes the same engineering problem as checking a circuit: see [Chapter 13: Formal Verification](../../ch13_formal_verification_and_applications/README.md).

Whitehead's speculative point-free geometry, formalized by logicians, complexity-classified by computer scientists, and shipped in GIS software: mereotopology is the chapter's thesis in miniature — the theory of parts and wholes is formal logic, with all of logic's rigor and all of its reach.

## Exercises
See [problems/ch16_mereology/](../../../problems/ch16_mereology/)
