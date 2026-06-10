# Important Figures

## Peter LeFanu Lumsdaine (dates not public)
*Type theorist; co-author of the formal semantics of HITs; contributor to the simplicial model.*

Peter LeFanu Lumsdaine completed his PhD at Dalhousie University and has held positions at Stockholm University and elsewhere. As noted in Chapter 18, his work spans the simplicial set model of univalent foundations and the structural theory of identity types. For this chapter, his central contribution is the 2020 paper "Semantics of higher inductive types" (with Michael Shulman), which resolved the longstanding open problem of giving a rigorous semantic account of HITs.

The difficulty was substantial: HITs are defined by elimination principles with both point and path cases, but there was no general framework specifying which HIT signatures are valid and what their intended models are. Lumsdaine and Shulman introduced the notion of a *globular sum* — a type formed by iterated pushouts along sphere-boundary inclusions — and proved that any HIT definable by such a signature has a model in any $(\infty,1)$-topos satisfying mild technical conditions. This established the legitimacy of the HITs used throughout HoTT: the circle, suspension, pushout, propositional truncation, and Eilenberg-MacLane spaces are all globular sums, and the semantics theorem guarantees their existence and their correct elimination principles. Lumsdaine has also contributed to the HoTT Book and to the development of HITs in Lean and Agda, and he remains active in the formal semantics of dependent type theory.

---

## Michael Shulman (1980–present)
*Type theorist and category theorist; co-developer of HITs semantics; contributor to every major area of HoTT.*

Michael Shulman received his PhD from the University of Chicago in 2009 and is currently a professor at the University of San Diego. He is one of the most broadly active contributors to HoTT, with research spanning the categorical semantics of type theory, synthetic homotopy theory, cohesive and modal HoTT, linear type theory, and the formalization of mathematics in proof assistants. His technical range — from abstract $\infty$-category theory to machine-checked proofs in Agda — makes him unusual among researchers in the field.

For this chapter, Shulman's contributions are multiple. With Lumsdaine, he proved the semantic completeness theorem for HITs as described above. With Licata, he developed the encode-decode method for computing $\pi_1(S^1) = \mathbb{Z}$ (Chapter 20 discusses this in detail, but the method originated in work for Chapter 19's HITs). He also contributed to the HoTT Book, particularly the chapters on HITs and synthetic homotopy theory, and to the formalization of the van Kampen theorem and Freudenthal suspension theorem. More recently, Shulman has developed *real cohesion* — a modal extension of HoTT that can distinguish between the discrete integers $\mathbb{Z}$ (the fundamental group of the circle, computed combinatorially) and the topological real line $\mathbb{R}$ (the universal cover of the circle, modeled analytically), and he proved that every $(\infty,1)$-topos models Book HoTT with universes, which settled a major open problem.

---

## Daniel R. Licata (dates not public)
*Type theorist; developer of the encode-decode method; contributor to the formalization of synthetic homotopy theory.*

Daniel Licata received his PhD from Carnegie Mellon University under Robert Harper and is currently a professor at Wesleyan University. His research focuses on the computational content of HoTT proofs, particularly the use of HITs to compute homotopy groups, and on the design of type theories with good computational properties for HoTT.

Licata's main contribution to this chapter is his development (with Shulman) of the encode-decode method, first applied to prove $\pi_1(S^1) = \mathbb{Z}$ and formalized in Agda in 2013. The key insight — that the loop space of a HIT can be computed by constructing a "code family" that lands in a known type, and then showing that encode and decode are inverse — requires using the HIT's elimination principle to define the code family and transport to compute what the encode map does. Licata was the first to carry this out in full generality and to formalize it machine-checkably. He subsequently applied the encode-decode method to compute $\pi_2(S^2)$ (using the Hopf fibration HIT) and contributed to the formalization of the Seifert-van Kampen theorem. He also developed the "2D type theory" (a directed type theory extending MLTT with two-dimensional cells) and has worked on modal type theory for cohesion.

---

## Guillaume Brunerie (1988–present)
*Type theorist; author of the first computation of $\pi_4(S^3)$ in synthetic homotopy theory; pioneer of Brunerie numbers.*

Guillaume Brunerie completed his PhD at the Université Nice Sophia-Antipolis in 2016 under Carlos Simpson and Thierry Coquand, and has held positions at the Institute for Advanced Study and Stockholm University. His PhD thesis, "On the Homotopy Groups of Spheres in Homotopy Type Theory," is the central document of the synthetic homotopy theory program (discussed in depth in Chapter 20), but his work on HITs provides the foundation on which that thesis rests.

Brunerie's contributions relevant to this chapter are primarily in the use of HITs to construct and compute with higher-dimensional spaces. The Hopf fibration, constructed as a map $S^3 \to S^2$ using a HIT-based description of $S^3$ as a join, is central to his computation of $\pi_3(S^2) = \mathbb{Z}$. His thesis also develops the theory of the James construction as a HIT (the free associative multiplication on a pointed type), which is used to set up the EHP sequence for computing $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$. The result $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$ was formally stated in his thesis but depended on a numerical computation that Brunerie identified as a specific integer (the "Brunerie number," which he conjectured equals 2) but could not verify by hand — it was later verified computationally in Cubical Agda by Ljungström and Mörtberg.

---

## Kristina Sojakova (dates not public)
*Type theorist; developer of the universal property characterization of HITs; contributor to synthetic homotopy theory.*

Kristina Sojakova completed her PhD at Carnegie Mellon University under Steve Awodey, and has held positions at INRIA Paris and Cornell University. Her research focuses on the semantics of HITs — specifically, on giving them clean universal property characterizations that are analogous to the initial algebra characterization of ordinary inductive types.

Sojakova's 2015 POPL paper "Higher Inductive Types as Homotopy-Initial Algebras" proposes that HITs should be characterized as *homotopy-initial algebras*: algebras in the $\infty$-categorical sense that are initial, meaning any other algebra of the same signature receives a unique (up to homotopy) map from them. This is the correct $\infty$-categorical analog of the standard initial algebra semantics for ordinary inductive types. She applied this framework to prove that the circle $S^1$, defined as a HIT, is the homotopy-initial $(\mathbb{Z}, +1)$-algebra, which gives a clean characterization of the circle independent of any particular HIT specification. Sojakova also formalized the Seifert-van Kampen theorem in Agda, proving that the fundamental group of a pushout is the amalgamated free product of the fundamental groups of the parts, using the pushout HIT and the encode-decode method.

---

## Nicolai Kraus (dates not public)
*Type theorist; developer of the metatheory of truncation and h-levels in the context of HITs.*

Nicolai Kraus received his PhD from the University of Nottingham under Thorsten Altenkirch and is currently at the University of Nottingham. His research focuses on the metatheory of HoTT, particularly on truncation levels, the behavior of HITs under truncation, coherence problems in HoTT, and the relationship between homotopy-theoretic and set-theoretic aspects of type theory.

Kraus's contributions most relevant to this chapter are in the theory of truncation as a HIT. The propositional truncation $\|A\|$ and the $n$-truncations $\|A\|_n$ are HITs, and understanding their elimination principles (in particular, the general elimination principle for $\|A\|$ into a type that need not be a proposition) requires delicate reasoning about coherence. Kraus proved the "general universal property of propositional truncation" — characterizing maps out of $\|A\|$ into arbitrary types using coherent families of proofs — and also developed the theory of *weakly constant functions* (functions that send all equal inputs to equal outputs, even if the function itself is not a section of a truncation). He has also worked on the metatheory of higher truncations, on the Kraus-Escardó-Coquand-Altenkirch theorem characterizing the homotopy types that arise as $\|A\|_n$, and on the question of how much of HoTT can be done without the full propositional truncation.

---

## Favonia (Kuen-Bang Hou Cheng, dates not public)
*Type theorist; contributor to the formalization of the Seifert-van Kampen theorem and covering spaces in HoTT.*

Favonia (who publishes under the name Kuen-Bang Hou) received a PhD from Carnegie Mellon University under Robert Harper and is currently at the University of Minnesota. Their research focuses on the formalization of algebraic topology in HoTT, with a particular emphasis on covering space theory, the Seifert-van Kampen theorem, and synthetic algebraic topology.

Favonia's primary contribution to the HITs chapter is the joint work with Michael Shulman formalizing the Seifert-van Kampen theorem in HoTT. The theorem, which computes the fundamental group of a pushout as an amalgamated free product, is proved using the pushout HIT and a careful encode-decode argument tailored to the pushout's universal property. The formalization required developing a substantial library of results about fundamental groupoids and their functoriality, all within the HoTT framework. Favonia has also worked on covering spaces in HoTT — showing that covering spaces of a type $X$ are classified by actions of the fundamental group $\pi_1(X)$ on sets, proved synthetic-homotopy-theoretically using the code/fiber construction — and on the formalization of the cohomology of spaces using HITs for Eilenberg-MacLane spaces.
