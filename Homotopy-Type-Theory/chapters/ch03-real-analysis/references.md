# References and Primary Sources

## Foundational Texts

**Augustin-Louis Cauchy, *Cours d'analyse de l'École Royale Polytechnique* (1821).** The first rigorous treatment of limits, continuity, and convergence, introducing the concept of infinitely small quantities in a precise setting. Cauchy's definitions of convergent sequences and continuous functions laid the groundwork for the epsilon-delta formalism that followed.

**Richard Dedekind, *Stetigkeit und irrationale Zahlen* (Continuity and Irrational Numbers, 1872).** The monograph in which Dedekind introduces his construction of the real numbers as "cuts" in the rationals. Dedekind gives a clear, elementary account of why the rational line is "incomplete" and how to fill in the gaps; this remains the most conceptually transparent construction of the reals.

**Georg Cantor, *Über die Ausdehnung eines Satzes aus der Theorie der trigonometrischen Reihen* (1872).** Cantor's rival construction of the real numbers via equivalence classes of Cauchy sequences of rationals, published in the same year as Dedekind's cut construction. The coincidence of publication date is not accidental: both men were working independently to settle the same foundational gap.

**Henri Poincaré, *Analysis Situs* (1895) and subsequent papers (1899–1904).** The founding documents of algebraic topology. Poincaré defines the fundamental group, introduces homology groups, and poses the Poincaré conjecture. The concept of path, homotopy, and the fundamental group as this chapter develops them are Poincaré's inventions.

**The Univalent Foundations Program, *Homotopy Type Theory: Univalent Foundations of Mathematics* (2013).** The "HoTT Book," written collectively by participants in the 2012–13 IAS special year. Chapter 2 develops the analogy between identity types and path spaces systematically; Chapters 6–8 compute fundamental groups of types using higher inductive types.

---

## Seminal Papers

**Richard Dedekind, "Stetigkeit und irrationale Zahlen" (1872).** See above. As a paper-length monograph, this is directly readable; it is remarkable for its directness and for Dedekind's own commentary on the conceptual problem he was solving.

**Georg Cantor, "Über die Ausdehnung eines Satzes aus der Theorie der trigonometrischen Reihen," *Mathematische Annalen* 5 (1872), pp. 123–132.** Introduces the Cauchy-sequence construction of the reals as an offshoot of Cantor's work on trigonometric series. The paper is historically important for showing how the completeness of the reals is needed to state theorems about Fourier series correctly.

**Karl Weierstrass, lecture notes transcribed by students, Berlin 1861–1886 (published posthumously).** Weierstrass never wrote a treatise on analysis, but his Berlin lectures — transcribed by students and circulated widely — are the source of the rigorous epsilon-delta definition of limits, the Weierstrass M-test for uniform convergence, and many theorems on continuous functions (the extreme value theorem and the approximation theorem).

**Eduard Heine, "Über trigonometrische Reihen," *Journal für die reine und angewandte Mathematik* 71 (1870), pp. 353–365.** Contains an early version of what is now called the Heine-Cantor theorem: a continuous function on a closed bounded interval is uniformly continuous. Heine's paper, together with Cantor's response, began the rigorous treatment of uniform continuity.

**Maurice Fréchet, "Sur quelques points du calcul fonctionnel," *Rendiconti del Circolo Matematico di Palermo* 22 (1906), pp. 1–74.** Introduces the concept of a metric space (under the name "classe (V)"), generalizing distance from Euclidean space to an abstract setting. This paper is the origin of the modern definition of metric space.

**Henri Poincaré, "Analysis Situs," *Journal de l'École Polytechnique* (1895), pp. 1–121.** The founding paper of algebraic topology. Defines the fundamental group $\pi_1(X, x_0)$, introduces homology, and begins the systematic study of topological invariants of manifolds.

**Heinz Hopf, "Über die Abbildungen der dreidimensionalen Sphäre auf die Kugelfläche," *Mathematische Annalen* 104 (1931), pp. 637–665.** Constructs the Hopf fibration $S^3 \to S^2$ and proves $\pi_3(S^2) = \mathbb{Z}$. This was the first computation of a higher homotopy group of a sphere, showing that the higher homotopy groups are not trivially zero and inaugurating the field of homotopy theory proper.

---

## Textbooks and Modern Treatments

**Walter Rudin, *Principles of Mathematical Analysis*, 3rd ed. (McGraw-Hill, 1976).** The standard graduate-level real analysis text in English, universally known as "Baby Rudin." Rigorous, terse, and comprehensive; covers metric spaces, sequences, series, continuity, differentiation, and Riemann-Stieltjes integration. The compactness theory and the Arzelà-Ascoli theorem are presented with characteristic elegance. Best read after a first analysis course.

**Charles Chapman Pugh, *Real Mathematical Analysis*, 2nd ed. (Springer, 2015).** A more geometric and discursive alternative to Rudin, aimed at undergraduates seeing analysis rigorously for the first time. Pugh includes many pictures and emphasizes the meaning of theorems before the proofs. The treatment of compactness and connectedness in metric spaces is particularly clear; the book also covers multivariable calculus and Lebesgue measure.

**Elias Stein and Rami Shakarchi, *Real Analysis: Measure Theory, Integration, and Hilbert Spaces* (Princeton, 2005).** Volume 3 of the Princeton Lectures in Analysis. Develops Lebesgue measure, $L^p$ spaces, and abstract Hilbert spaces. Essential for understanding the functional-analytic aspects of the chapter (signal processing applications, $L^2$ spaces). The treatment of approximations by smooth functions and of Fourier analysis is outstanding.

**Allen Hatcher, *Algebraic Topology* (Cambridge, 2001; freely available at author's website).** The standard modern introduction to algebraic topology. Chapters 0–1 cover the fundamentals of homotopy theory and the fundamental group. A thorough treatment of covering spaces, van Kampen's theorem, and the computation $\pi_1(S^1) = \mathbb{Z}$. Chapter 4 covers higher homotopy groups. Highly readable with a geometric emphasis.

**Tammo tom Dieck, *Algebraic Topology* (EMS, 2008).** A more advanced alternative to Hatcher, closer in style to modern categorical treatments. Contains careful accounts of cofibrations, fibrations, and the Seifert-van Kampen theorem. Useful for the reader who wants to understand how the homotopy theory from this chapter connects to the $\infty$-categorical perspective underlying HoTT.

---

## Online Resources and Lecture Notes

**Terence Tao, *Analysis I* and *Analysis II* (lecture notes, freely available at terrytao.wordpress.com).** Tao's books, based on his UCLA courses, develop real analysis from first principles in an unusually accessible way, starting from the Peano axioms and constructing the rationals and reals before proceeding to metric space topology. The books are the basis of his widely-read blog, where he posts supplementary material and solutions.

**Paul Halmos, *Naive Set Theory* (Springer, 1974; van Nostrand 1960) — combined with his lecture notes on functional analysis.** Halmos's writing style is among the clearest in mathematics. His *Naive Set Theory* provides the foundational background assumed by this chapter; his *Introduction to Hilbert Space and the Theory of Spectral Multiplicity* connects to the $L^2$ material.

**MIT OpenCourseWare 18.100B, "Analysis I" (available at ocw.mit.edu).** Lecture notes, problem sets, and exams from MIT's standard real analysis course, using Rudin. Freely downloadable. Useful for additional exercises on metric spaces and uniform continuity.

**The HoTT Book online (homotopytypetheory.org/book).** The complete text of the HoTT Book, freely available as a PDF and in source form. Chapters 2 and 6 are directly relevant to the paths-and-homotopy material in Section 7 of this chapter.

**nLab (ncatlab.org).** The collaborative wiki for higher category theory and mathematical foundations. Entries on "homotopy," "fundamental group," "metric space," "Cauchy completion," and "univalence axiom" are technically precise and include references to primary sources. Best used as a reference for specific definitions and theorems, not as an introduction.

---

## Historical Context

The rigorization of analysis was one of the central projects of 19th-century mathematics. Before Cauchy, the informal infinitesimal calculus of Newton and Leibniz — brilliant but imprecise — had led to genuine confusions, particularly in the theory of infinite series and the foundations of the integral. Cauchy's *Cours d'analyse* (1821) introduced the limit as the primary concept, defining continuity and convergence in terms of limits without appealing to infinitesimals. Cauchy's definitions were a major advance, but they still contained informal elements: his notion of "variable approaching a limit" was not yet fully symbolic. The final step was taken by Weierstrass, who replaced variable-approach language with the $\varepsilon$-$\delta$ formulation in his Berlin lectures (1860s–1880s). Weierstrass also identified pathological examples — continuous nowhere-differentiable functions, for instance — that showed the need for precision. The construction of the real numbers themselves was settled independently by Dedekind (cuts) and Cantor (Cauchy sequences) in 1872. Both constructions define the reals not as primitive objects but as certain mathematical entities built from the rationals; both yield a complete ordered field, and Dedekind's uniqueness argument showed any two complete ordered fields are isomorphic.

The path from analysis to topology was pioneered by Poincaré at the end of the 19th century. Poincaré introduced the fundamental group in his *Analysis Situs* (1895) as an invariant of topological spaces — a group that captured something about the "loops" in a space that cannot be contracted. The group operation on loops (concatenation) and the notion of homotopy (continuous deformation) both come from this paper. The subsequent development of algebraic topology in the 20th century — covering spaces, higher homotopy groups, homology and cohomology — built on Poincaré's framework. When Grothendieck and then Quillen reformulated homotopy theory in categorical terms in the 1960s–1970s, the stage was set for the conceptual identification of spaces with types that defines HoTT. Voevodsky's discovery of the univalence axiom (around 2006) and the collective development of HoTT (culminating in the 2013 book) completed the arc: a concept originating in the epsilon-delta rigour of 19th-century analysis had become a foundational axiom of 21st-century type theory.
