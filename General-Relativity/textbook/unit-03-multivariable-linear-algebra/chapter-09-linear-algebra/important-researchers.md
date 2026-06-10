# Chapter 9: Important Researchers

---

## Founders of Linear Algebra

**Arthur Cayley (1821–1895)**
English mathematician who spent 14 years as a lawyer before obtaining a professorship at Cambridge. Cayley's 1858 "Memoir on the Theory of Matrices" introduced matrices as mathematical objects — not just abbreviations for systems of equations — and defined matrix multiplication, inversion, and the characteristic polynomial. He proved the Cayley-Hamilton theorem (every matrix satisfies its own characteristic equation) and began the study of matrix groups. Cayley's work, together with that of his friend Sylvester, founded the British school of linear algebra. He published nearly 1,000 mathematical papers.

**James Joseph Sylvester (1814–1897)**
English mathematician who coined the word "matrix" and much of the language of linear algebra. Sylvester proved Sylvester's law of inertia (the signature of a symmetric bilinear form is independent of the basis — the theorem that tells us a metric with signature (−,+,+,+) cannot be made into (+,+,+,+) by a change of basis). He also introduced the terms "discriminant," "invariant," and "covariant," and worked on the theory of algebraic forms. Sylvester spent periods at Johns Hopkins and at Oxford, having been denied a Cambridge fellowship because he was Jewish.

**Hermann Grassmann (1809–1877)**
German mathematician and schoolteacher who independently discovered the theory of vector spaces and the exterior algebra (Grassmann algebra) in his 1844 *Ausdehnungslehre* — 15 years before anyone recognized his work. Grassmann's exterior product captures the geometry of oriented areas and volumes: a ∧ b represents the oriented parallelogram spanned by a and b, with |a ∧ b| = area. His work, ignored in his lifetime, is the foundation of differential forms, the Grassmannian, and spinor theory. Grassmann died believing his mathematical work to be a failure; within a decade of his death, the community recognized its depth.

**Giuseppe Peano (1858–1932)**
Italian mathematician who gave the first fully modern axioms for a vector space in 1888, building on Grassmann's ideas. Peano's axioms are essentially the ones used today. He also devised a curve that fills a square (the Peano curve, 1890), contributed to mathematical logic, and developed an international language called Latino sine flexione (Latin without inflections). His rigorous axiomatization of the natural numbers — the Peano axioms — is in every foundations-of-mathematics course.

**Camille Jordan (1838–1922)**
French mathematician who proved the Jordan canonical form theorem: every square matrix over an algebraically closed field is similar to a matrix in Jordan normal form (diagonal blocks with 1s on the superdiagonal). The Jordan form completely classifies linear maps up to similarity, and is the algebraic model for the Petrov classification of the Weyl tensor. Jordan also proved the Jordan curve theorem (a simple closed curve in the plane divides it into exactly two regions — a theorem that seems obvious but is surprisingly difficult to prove rigorously).

---

## The Theory of Bilinear Forms and Metrics

**Carl Gustav Jacob Jacobi (1804–1851)**
See Chapter 7. In the context of this chapter: Jacobi developed the theory of quadratic forms (the diagonalization of symmetric matrices via congruence transformations) and the Jacobi method for computing eigenvalues of symmetric matrices.

**James Joseph Sylvester (1814–1897)**
[listed above] Sylvester's law of inertia is the fundamental theorem about the metric: the signature (number of positive, negative, zero eigenvalues) of a symmetric bilinear form is a basis-independent invariant. This is why the Minkowski metric has signature (−,+,+,+) in every coordinate system.

**Bernhard Riemann (1826–1866)**
See Chapter 4 and the unit introduction. In the context of this chapter: Riemann introduced the metric tensor gᵢⱼ as a (0,2) symmetric tensor on each tangent space, defining the inner product ⟨u,v⟩ = gᵢⱼ uⁱ vʲ. This is the inner product of Section 9.4 generalized to the tangent spaces of a curved manifold. Riemann's insight that curvature is intrinsic — measurable from within the manifold — is the foundation of GR.

---

## Tensor Calculus

**Gregorio Ricci-Curbastro (1853–1925)**
Italian mathematician who developed the "absolute differential calculus" — tensor calculus — beginning in the 1880s. Ricci's tensor calculus introduced the systematic use of upper and lower indices to track transformation properties, the covariant derivative ∇_μ, the Riemann curvature tensor R^ρ_{σμν}, and contraction. His 1901 paper with Levi-Civita (listed above) was the paper Einstein studied intensively in 1912 to formulate GR. Ricci's name survives in: the Ricci tensor (R_{μν} = R^ρ_{μρν}), the Ricci scalar (R = g^{μν} R_{μν}), and the Ricci flow (Hamilton's equation for how a Riemannian metric evolves).

**Tullio Levi-Civita (1873–1941)**
Italian mathematician, student of Ricci, who co-authored the foundational 1901 paper on tensor calculus and developed the theory further. Levi-Civita introduced the concept of parallel transport on a Riemannian manifold (the Levi-Civita connection), the totally antisymmetric Levi-Civita symbol ε_{μνρσ} (and corresponding tensor √(-g) ε_{μνρσ}), and many other tools. He also worked on fluid mechanics and celestial mechanics. Levi-Civita corresponded directly with Einstein during the development of GR, pointing out errors in Einstein's work on gravitational waves. He was removed from his professorship under Mussolini's antisemitic laws in 1938.

---

## Spectral Theory

**David Hilbert (1862–1943)**
German mathematician, the dominant figure in mathematics in the early twentieth century. Hilbert developed the infinite-dimensional spectral theorem for integral operators (1904), which is the mathematical foundation of quantum mechanics (where the Hamiltonian is an operator on a Hilbert space — the space named after him). Hilbert posed 23 problems at the 1900 International Congress of Mathematicians that drove much of twentieth-century mathematical research. His program to formalize all of mathematics was answered — negatively — by Gödel's incompleteness theorems in 1931.

**John von Neumann (1903–1957)**
Hungarian-American mathematician and polymath who formalized the mathematical foundations of quantum mechanics in his *Mathematical Foundations of Quantum Mechanics* (1932), placing Dirac's bra-ket formalism in the rigorous framework of operators on Hilbert spaces. Von Neumann proved the spectral theorem for unbounded self-adjoint operators — the infinite-dimensional version of the real symmetric matrix theorem. He also made foundational contributions to game theory, mathematical economics, computer architecture, and nuclear physics.

---

## The Modern View: Modules and Categories

**Emmy Noether (1882–1935)**
German mathematician, described by Einstein as "the most significant creative mathematical genius thus far produced." Noether's revolutionary contributions to abstract algebra — the theory of rings and modules — provided the language in which modern algebra is written. Her 1915 theorems (proven in 1918) relating continuous symmetries to conservation laws (Noether's theorem, Chapter 14) are among the most important results in theoretical physics. She was denied a professorship at Göttingen because of her gender, emigrated to the US to escape Nazi persecution in 1933, and died of cancer in 1935. Every chapter of this textbook uses her legacy.

**Saunders Mac Lane (1909–2005) and Samuel Eilenberg (1913–1998)**
American mathematicians who invented category theory in 1945. Category theory provides the most general framework for thinking about mathematical structures: objects, morphisms between them, and the universal properties that characterize constructions like direct sums, tensor products, and dual spaces. The tensor product V ⊗ W (which underlies the tensor fields of GR) is most cleanly defined by its universal property in category theory.
