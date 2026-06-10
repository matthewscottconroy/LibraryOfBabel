# References and Primary Sources

## Foundational Texts

**Guillaume Brunerie.** *On the Homotopy Groups of Spheres in Homotopy Type Theory*. PhD thesis, Université Nice Sophia-Antipolis, 2016. Available at guillebrunerie.com. The central document of the synthetic homotopy theory program: proves $\pi_1(S^1) = \mathbb{Z}$, $\pi_2(S^2) = \mathbb{Z}$, $\pi_3(S^2) = \mathbb{Z}$ (via the Hopf fibration), and states and partially verifies $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$. This thesis established synthetic homotopy theory as a mature research program and contains the deepest original mathematics in HoTT up to its date.

**The Univalent Foundations Program.** *Homotopy Type Theory: Univalent Foundations of Mathematics*. Institute for Advanced Study, 2013. Chapter 8 ("Homotopy Theory") contains the HoTT Book's treatment of synthetic homotopy: the encode-decode method, $\pi_1(S^1) = \mathbb{Z}$, the Seifert-van Kampen theorem, the Freudenthal suspension theorem, and the Blakers-Massey theorem. Available free at homotopytypetheory.org/book.

**Allen Hatcher.** *Algebraic Topology*. Cambridge University Press, 2002. Available free at pi.math.cornell.edu/~hatcher/AT/ATpage.html. The standard modern textbook for classical algebraic topology; essential for understanding the classical statements of the theorems proved synthetically in this chapter (fundamental group, Freudenthal, Hopf fibration, homotopy groups of spheres).

**Robert Mosher and Martin Tangora.** *Cohomology Operations and Applications in Homotopy Theory*. Harper & Row, 1968. A classical reference for the stable homotopy theory computations that synthetic homotopy theory is beginning to formalize; relevant background for understanding the EHP sequence and the Brunerie number.

---

## Seminal Papers

**Daniel R. Licata and Michael Shulman.** "Calculating the Fundamental Group of the Circle in Homotopy Type Theory." In *28th Annual ACM/IEEE Symposium on Logic in Computer Science (LICS 2013)*, 223–232, 2013. The foundational paper: the first machine-checked proof of $\pi_1(S^1) = \mathbb{Z}$ in HoTT, introducing the encode-decode method in full generality. Every subsequent computation of homotopy groups in HoTT uses the technique introduced here. The paper is short and highly readable.

**Mathieu Anel, Georg Biedermann, Eric Finster, and André Joyal.** "A Generalized Blakers-Massey Theorem." *Journal of Topology*, 13(4):1521–1553, 2020 (preprint 2017). Proves a synthetic version of the Blakers-Massey excision theorem in any $\infty$-topos, using HoTT-style reasoning about modalities; this is the version used to prove the Freudenthal suspension theorem synthetically, and it demonstrates that synthetic methods give cleaner proofs of classical theorems.

**Guillaume Brunerie.** "The James Construction and $\pi_4(S^3)$ in Homotopy Type Theory." *Journal of Automated Reasoning*, 63(2):255–284, 2019 (journal version of thesis material). Contains the key constructions for $\pi_4(S^3)$: the James construction (the free $A_\infty$-space on a pointed type, defined as a HIT), the EHP sequence, and the identification of the Brunerie number. The journal version is more accessible than the thesis.

**Axel Ljungström and Anders Mörtberg.** "Formalizing π₄(S³) ≅ ℤ/2ℤ and Computing a Brunerie Number in Cubical Agda." In *Proceedings of the 38th Annual ACM/IEEE Symposium on Logic in Computer Science (LICS 2023)*, 2023. The definitive verification of Brunerie's result: a complete, machine-checked proof that $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$ in Cubical Agda, including a computation of the Brunerie number showing it equals 2. This is the state-of-the-art synthetic homotopy computation.

**Egbert Rijke.** "The join construction." Preprint, arXiv:1701.07538, 2017. Develops the theory of the join (and the Hopf fibration as a special case) in HoTT; Rijke also has related work on the Freudenthal suspension theorem and on connectivity, including the sequential colimit approach to the Freudenthal theorem.

**Carlo Angiuli, Kuen-Bang Hou (Favonia), and Robert Harper.** "Cartesian Cubical Computational Type Theory: Constructive Reasoning with Paths and Equalities." In *Computer Science Logic (CSL 2018)*, 2018. Develops the theoretical basis for cubical type theory as implemented in RedTT and related systems, with specific attention to synthetic homotopy computations; the constructive Freudenthal suspension theorem appears as a worked example.

---

## Textbooks and Modern Treatments

**Egbert Rijke.** *Introduction to Homotopy Type Theory*. Cambridge University Press (to appear; preprint arXiv:2212.11082, 2022). The most up-to-date textbook treatment of synthetic homotopy theory, including full coverage of the encode-decode method, the Seifert-van Kampen theorem, Freudenthal, and the Hopf fibration. The presentation is cleaner than the HoTT Book and reflects more recent developments.

**J. P. May.** *A Concise Course in Algebraic Topology*. University of Chicago Press, 1999. Available free at math.uchicago.edu/~may/CONCISE/ConciseRevised.pdf. A terse and rigorous classical treatment of algebraic topology including the Hopf fibration, fibration sequences, and the long exact sequence of a fibration — the tools that synthetic homotopy theory formalizes.

**Douglas Ravenel.** *Complex Cobordism and Stable Homotopy Groups of Spheres*. Academic Press, 1986 (2nd ed. AMS Chelsea, 2004). The authoritative classical reference for the stable homotopy groups of spheres; relevant as background for understanding why the computations in this chapter ($\pi_1, \pi_2, \pi_3, \pi_4$ of spheres) are interesting and how they fit into the broader program.

**Floris van Doorn.** *On the Formalization of Higher Inductive Types and Synthetic Homotopy Theory*. PhD thesis, Carnegie Mellon University, 2018. Available at florisvandoorn.com. Covers many of the same theorems as this chapter (Freudenthal, Hopf) but in a Lean 2 formalization; useful as a companion showing how the proofs are organized in a proof assistant.

---

## Online Resources and Formalization Code

**The `agda/cubical` library, Homotopy/ subdirectory.** Available at github.com/agda/cubical. The primary repository for formalized synthetic homotopy theory; contains the circle ($\pi_1(S^1)$ proof), spheres, Hopf fibration, Brunerie number computation, cohomology, and more. The `Cubical.Homotopy.Group.Pi4S3` module contains the Ljungström-Mörtberg verification of $\pi_4(S^3)$.

**Guillaume Brunerie's Agda formalization.** Available at github.com/guillaumebrunerie/HoTT-Agda. The original Agda formalization of the Brunerie thesis results, including the James construction and the EHP sequence. Historically significant as the first attempt at $\pi_4(S^3)$ in HoTT.

**The RedTT proof assistant.** Available at github.com/RedPRL/redtt. An experimental proof assistant for Cartesian cubical type theory, developed at Carnegie Mellon by Angiuli, Harper, and collaborators; several synthetic homotopy theory results are formalized in RedTT, and the source files are useful for understanding the cubical proof techniques.

**nLab: Synthetic homotopy theory.** Available at ncatlab.org/nlab/show/synthetic+homotopy+theory. A maintained reference connecting the HoTT approach to classical algebraic topology; contains precise statements of the main theorems with links to their formalizations and original papers.

**The Brunerie number.** Available at github.com/agda/cubical/blob/master/Cubical/Homotopy/Group/Pi4S3/Summary.agda. A single Cubical Agda file whose normalization produces the integer 2, verifying that the Brunerie number equals 2 and therefore $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$. A remarkable piece of formalized mathematics.

---

## Historical Context

Synthetic homotopy theory emerged as soon as researchers realized that the combination of HITs and univalence gives type theory genuine homotopy-theoretic power. The first signal was the $\pi_1(S^1) = \mathbb{Z}$ proof by Licata and Shulman (2013), which showed that the encode-decode method — turning a loop space computation into a type equivalence — could be carried out formally in Agda. This was followed rapidly by computations of $\pi_2(S^2)$, $\pi_3(S^2) = \mathbb{Z}$ (via the Hopf fibration), and the Seifert-van Kampen and Freudenthal theorems, all proved synthetically during the period 2013–2016.

The Brunerie thesis (2016) represented a qualitative leap: Brunerie developed the synthetic proof of $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$, a classical result that requires substantial classical homotopy theory (the James construction, the EHP sequence, cohomological computations). His proof reduced the result to the calculation of a specific integer — the Brunerie number $\beta$ — defined as the "attaching map" of a certain 4-cell in a type, and conjectured that $\beta = 2$. The proof was complete modulo this numerical computation. The computation was eventually carried out by Ljungström and Mörtberg in Cubical Agda in 2022–2023, normalizing the Brunerie number to 2 and thus completing the proof machine-checkably. This was the first major synthetic homotopy computation to require a computer to verify even a numerical step — foreshadowing the future of the field, where proof assistants and synthetic methods collaborate to push the computational frontier.

The Blakers-Massey theorem, proved synthetically by Anel, Biedermann, Finster, and Joyal (2017), demonstrated that synthetic proofs can be not merely translations of classical arguments but genuinely new and often simpler arguments, illuminating the mathematics from a fresh angle. The synthetic proof uses the language of modalities (which HoTT provides naturally) and is shorter and more conceptual than the classical proof.
