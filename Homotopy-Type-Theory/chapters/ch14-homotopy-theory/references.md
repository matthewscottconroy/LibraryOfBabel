# References and Primary Sources

## Foundational Texts

**A. Hatcher.** *Algebraic Topology.* Cambridge University Press, 2002. (Free PDF at https://pi.math.cornell.edu/~hatcher/)
The most widely used modern text for algebraic topology. Chapters 1–3 cover the fundamental group, homology, and cohomology with great geometric clarity. Chapter 4 (homotopy theory: higher homotopy groups, fibrations, Hurewicz theorem, Whitehead's theorem) is the primary reference for the core material of Chapter 14. The writing style emphasizes geometric intuition before formal verification and is the right companion to read alongside this chapter.

**J.P. May.** *A Concise Course in Algebraic Topology.* University of Chicago Press, 1999. (Free PDF at https://math.uchicago.edu/~may/CONCISE/ConciseRevised.pdf)
Covers the essentials of algebraic topology — homotopy, homology, cohomology, fibrations, spectral sequences — concisely and rigorously. The chapters on CW complexes, fibrations, and the Hurewicz theorem are models of clarity. Essential supplementary reading for the material in Chapter 14.

**R. Mosher and M. Tangora.** *Cohomology Operations and Applications in Homotopy Theory.* Harper and Row, 1968. (Dover reprint available.)
Covers the machinery of cohomology operations (Steenrod squares, Steenrod powers) and their application to computing homotopy groups of spheres, including the Adams spectral sequence. More technical than Hatcher or May; the right next step after mastering Chapter 14.

**R. Switzer.** *Algebraic Topology: Homotopy and Homology.* Die Grundlehren der mathematischen Wissenschaften 212, Springer, 1975. (Springer Classics reprint, 2002.)
A comprehensive graduate-level treatment of classical homotopy theory: CW complexes, exact sequences, obstruction theory, spectra, and the Adams spectral sequence. More detailed than May and more systematic than Hatcher. The standard reference for results that fall between the elementary texts and research papers.

**G.W. Whitehead.** *Elements of Homotopy Theory.* Graduate Texts in Mathematics 61, Springer, 1978.
A thorough graduate reference covering homotopy groups, CW complexes, fibrations, the Hurewicz theorem, and more advanced topics. Written by one of the founders of modern homotopy theory; authoritative but dense.

---

## Seminal Papers

**H. Hopf.** "Über die Abbildungen der dreidimensionalen Sphäre auf die Kugelfläche." *Mathematische Annalen* 104 (1931), 637–665.
The paper introducing the Hopf fibration: $\eta : S^3 \to S^2$, a continuous map from the 3-sphere to the 2-sphere that is not null-homotopic. Hopf proved that $\pi_3(S^2) \neq 0$ and described the map by associating to each point on $S^3$ (viewed as a subset of $\mathbb{C}^2$) a point on $S^2$ (viewed as $\mathbb{CP}^1$). This was the first indication that homotopy groups of spheres do not vanish above the dimension of the sphere — a shocking result that opened the modern era of homotopy theory.

**J.-P. Serre.** "Homologie singulière des espaces fibrés." *Annals of Mathematics* 54 (1951), 425–505.
Serre's doctoral thesis, one of the most influential papers in 20th-century mathematics. Introduced the Serre spectral sequence for computing the homology of a fibration from the homology of base and fiber, and used it to determine $\pi_n(S^2)$ for small $n$ and to prove that $\pi_n(S^n) = \mathbb{Z}$ and $\pi_{n+1}(S^n) = \mathbb{Z}/2\mathbb{Z}$ for $n \geq 3$. Established the method of mod-$p$ Serre spectral sequences, showing that almost all homotopy groups of spheres are finite.

**W. Hurewicz.** "Beiträge zur Topologie der Deformationen I–IV." *Proceedings of the Koninklijke Akademie van Wetenschappen te Amsterdam* 38–39 (1935–36).
The series of papers where Hurewicz introduced **homotopy groups** $\pi_n(X)$ for $n \geq 2$ as an algebraic invariant of topological spaces, proved their abelian character for $n \geq 2$, established the **Hurewicz theorem** (the first nonzero homotopy group coincides with the first nonzero homology group for simply connected spaces), and proved the **Hurewicz isomorphism** $\pi_n(X) \cong H_n(X)$ when $\pi_k(X) = 0$ for $k < n$.

**J.H.C. Whitehead.** "Combinatorial Homotopy I, II." *Bulletin of the American Mathematical Society* 55 (1949), 213–245, 453–496.
Introduced **CW complexes** — cell complexes built by attaching cells of increasing dimension — and proved **Whitehead's theorem**: a map between CW complexes that induces isomorphisms on all homotopy groups is a homotopy equivalence. CW complexes are now the standard spaces in algebraic topology (every space is weakly homotopy equivalent to a CW complex), and Whitehead's theorem is the fundamental criterion for homotopy equivalence.

**F. Adams.** "On the Non-Existence of Elements of Hopf Invariant One." *Annals of Mathematics* 72 (1960), 20–104.
Proved that the only spheres that are $H$-spaces (have a continuous multiplication with unit) are $S^0$, $S^1$, $S^3$, and $S^7$, corresponding to the real numbers, complex numbers, quaternions, and octonions. Adams introduced the **Adams spectral sequence** for computing stable homotopy groups and the **Adams operations** in K-theory, two of the most powerful tools in modern homotopy theory.

**H. Hopf.** "Über die Topologie der Gruppen-Mannigfaltigkeiten und ihre Verallgemeinerungen." *Annals of Mathematics* 42 (1941), 22–52.
Proved the **Hopf theorem**: any compact, connected Lie group has the rational homotopy type of a product of odd-dimensional spheres, with the multiplication map inducing a Hopf algebra structure on the cohomology ring. This paper connected Lie group theory to the emerging algebraic topology and established cohomological methods as a tool for studying continuous group actions.

---

## Textbooks and Modern Treatments

**J. Milnor.** *Morse Theory.* Annals of Mathematics Studies 51, Princeton University Press, 1963.
Develops Morse theory — the study of smooth functions on manifolds — and uses it to prove the **Bott periodicity theorem** (the homotopy groups of the unitary group satisfy $\pi_n(U) \cong \pi_{n+2}(U)$). Chapter 14 provides background for understanding the CW complex structure that Morse theory produces; Milnor's treatment is the standard reference.

**D. Husemoller.** *Fibre Bundles.* 3rd edition, Graduate Texts in Mathematics 20, Springer, 1994.
A comprehensive treatment of fiber bundles, principal bundles, vector bundles, and classifying spaces. Essential for understanding the Hopf fibration in its full generality (as a principal $S^1$-bundle over $S^2$) and for the connections between fibrations and physics (gauge theory).

**J. McCleary.** *A User's Guide to Spectral Sequences.* 2nd edition, Cambridge Studies in Advanced Mathematics 58, Cambridge University Press, 2001.
The standard accessible reference for spectral sequences — the primary computational tool for homotopy groups beyond those accessible by elementary methods. Essential for going beyond Chapter 14 into the calculation of $\pi_n(S^k)$.

**M. Arkowitz.** *Introduction to Homotopy Theory.* Universitext, Springer, 2011.
A modern, concise graduate introduction to homotopy theory, covering all the material of Chapter 14 (homotopy of maps, higher homotopy groups, fibrations, CW complexes, Whitehead's theorem) in a single self-contained text. Good for a reader who wants to move quickly through the foundational material.

---

## Online Resources and Lecture Notes

**A. Hatcher.** *Algebraic Topology.* Free PDF at https://pi.math.cornell.edu/~hatcher/
Indispensable; see the Foundational Texts entry above.

**J.P. May.** *A Concise Course in Algebraic Topology.* Free PDF at https://math.uchicago.edu/~may/CONCISE/ConciseRevised.pdf
Also indispensable; see above.

**The Homotopy Type Theory book (HoTT book).** https://homotopytypetheory.org/book/
Chapters 2 and 6 provide the internal (HoTT) perspective on paths, homotopies, fibrations, and higher inductive types. Chapter 8 gives the HoTT proofs of $\pi_n(S^n) = \mathbb{Z}$ and the Freudenthal suspension theorem.

**nLab: "homotopy groups of spheres."** https://ncatlab.org/nlab/show/homotopy+groups+of+spheres
A comprehensive survey of known homotopy groups of spheres, with references to proofs and computational methods.

**J. Lurie.** "Topics in Geometric Topology." Lecture notes, available at https://www.math.ias.edu/~lurie/
Lurie's lecture notes on various geometric topology topics, including the Hopf fibration and its relationship to the Dirac monopole in mathematical physics.

---

## Historical Context

Homotopy theory as an independent discipline emerged from the work of the 1930s–1950s. Before 1935, algebraic topology was primarily concerned with homology — the study of "holes" via chain complexes and Betti numbers (Poincaré, Noether, Alexandrov, Veblen). The fundamental group $\pi_1$ had been studied since Poincaré, but higher homotopy groups were unknown. Witold Hurewicz's papers of 1935–36 changed this: he defined $\pi_n(X)$ for all $n$, proved that $\pi_n$ is abelian for $n \geq 2$, and proved the Hurewicz theorem relating homotopy to homology. But Hurewicz's tools were insufficient to compute $\pi_n(S^k)$ for $k > 1$ — the problem that drove the next twenty years of development.

The decisive advance was Heinz Hopf's 1931 discovery of the Hopf fibration $\eta : S^3 \to S^2$, which showed that $\pi_3(S^2)$ is nontrivial — there exist essential maps between spheres of "wrong" dimension. This was deeply surprising: all maps $S^n \to S^m$ for $n < m$ are null-homotopic (trivially), but the Hopf map showed that $\pi_3(S^2) \neq 0$. The computation $\pi_3(S^2) = \mathbb{Z}$ was understood only with Serre's spectral sequences in 1951.

Jean-Pierre Serre's 1951 thesis was the next watershed. Introducing spectral sequences for fibrations, he computed $\pi_n(S^k)$ for many values and proved that for $k \geq 2$, all groups $\pi_n(S^k)$ are finitely generated, and all but finitely many are finite (i.e., the rational homotopy of spheres is simple). Serre shared the Fields Medal in 1954 partly for this work. Simultaneously, J.H.C. Whitehead's CW complexes (1949) provided the correct category of spaces for homotopy theory: spaces built by attaching cells, with a Whitehead theorem ensuring that isomorphisms on all $\pi_n$ imply homotopy equivalence.

The subsequent decades saw systematic attacks on the stable homotopy groups of spheres (the groups $\pi_{n+k}(S^n)$ for large $n$, which stabilize): Adams's spectral sequence (1958–1960), Milnor's exotic spheres (1956), the Adams conjecture (1970), and Quillen's formalization via model categories (1967). By 1980 the field had matured into modern algebraic topology, and its connection to HoTT was beginning to be glimpsed in the work on simplicial sets and Kan complexes that would lead to Voevodsky's program.
