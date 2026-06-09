# Chapter 12 — Exercises

## Important Figures

- **Hermann Grassmann (1809–1877)** — exterior algebra in *Ausdehnungslehre* (1844); wedge products and linear independence
- **William Kingdon Clifford (1845–1879)** — Clifford algebras (1878): a common generalization of exterior and quaternion algebras; now central to physics
- **Élie Cartan (1869–1951)** — differential forms and moving frames; exterior algebra applied to differential geometry; the calculus of exterior forms
- **Tullio Levi-Civita (1873–1941)** — tensor calculus and the Levi-Civita symbol; formalism of general relativity with Ricci
- **Bernhard Riemann (1826–1866)** — Riemannian metric as a symmetric $(0,2)$-tensor; curvature tensor

## References and Primary Sources

- **H. Grassmann, *Die lineale Ausdehnungslehre* (1844)** — first systematic exterior algebra
- **W. Greub, *Multilinear Algebra* (2nd ed., Springer, 1978)** — thorough algebraic treatment
- **F.W. Warner, *Foundations of Differentiable Manifolds and Lie Groups* (Springer, 1983)** — differential forms and de Rham cohomology
- **R. Penrose & W. Rindler, *Spinors and Space-Time* (2 vols., Cambridge, 1984–1986)** — tensor methods in physics

## Examples, Applications, and Thought Experiments

- **Cross product as exterior product** — in $\mathbb{R}^3$,$\mathbf{u} \times \mathbf{v}$ is the Hodge dual of$\mathbf{u} \wedge \mathbf{v}$; the wedge product captures the oriented area of the parallelogram, and the Hodge star turns this area 2-form into a vector perpendicular to the plane; the cross product is not intrinsically a vector but a pseudovector
- **Determinant as top exterior power** — $\det(A) =$ coefficient of$e_1 \wedge \cdots \wedge e_n$ in$(Ae_1) \wedge \cdots \wedge (Ae_n)$; the determinant is the scalar by which$A$ scales$n$-dimensional volume; this is the clean conceptual definition
- **The stress tensor in continuum mechanics** — the Cauchy stress tensor $\sigma_{ij}$ gives the force per unit area in direction$i$ on a surface with normal in direction$j$; it packages 9 components into a single rank-$(1,1)$ tensor object that transforms correctly under change of frame
- **Differential forms and integration** — a $k$-form is an alternating multilinear map on tangent vectors; integration of$k$-forms over$k$-dimensional submanifolds is coordinate-free; Stokes' theorem$\int_M d\omega = \int_{\partial M} \omega$ unifies all classical integral theorems

## Exercises

1. Let $V = \mathbb{R}^3$ with standard basis $\{e_1, e_2, e_3\}$ and dual basis $\{e^1, e^2, e^3\}$. For the vector $\mathbf{v} = 2e_1 - e_2 + 3e_3$, compute the dual basis evaluation $e^i(\mathbf{v})$ for each $i$. Then write down the unique linear functional $f \in V^*$ satisfying $f(e_1) = 1$, $f(e_2) = -2$, $f(e_3) = 0$, and express $f$ as a linear combination of the dual basis elements.

2. Let $V$ be a finite-dimensional vector space over $F$ and let $\phi : V \to V^{**}$ be the canonical evaluation map $\phi(v)(f) = f(v)$. Prove that $\phi$ is an isomorphism without choosing a basis. (First show $\phi$ is injective: if $f(v) = 0$ for all $f \in V^*$, conclude $v = 0$. Then use dimension counting.)

3. Let $V$ have basis $\{e_1, e_2\}$ and $W$ have basis $\{f_1, f_2, f_3\}$. Write down a basis for $V \otimes W$ and determine $\dim(V \otimes W)$. Express the element $v = e_1 + e_2$ and $w = f_1 - f_3$ as vectors, and compute $v \otimes w$ as a linear combination of basis tensors. Is $v \otimes w$ a "pure" tensor? Explain what it would mean for an element of $V \otimes W$ to be non-pure (not expressible as a single elementary tensor $u \otimes w$).

4. Let $T : V \to V$ be a linear operator on an $n$-dimensional vector space. The operator $T$ acts on $\Lambda^n(V)$ by $T(v_1 \wedge \cdots \wedge v_n) = Tv_1 \wedge \cdots \wedge Tv_n$. Use this to prove that $\det(ST) = \det(S)\det(T)$ for any two operators $S, T : V \to V$, without any reference to matrix entries or permutations.

5. Let $V = \mathbb{R}^3$ and let $\mathbf{u} = e_1 + e_2$, $\mathbf{v} = e_2 + e_3$, $\mathbf{w} = e_1 + e_3$. Compute $\mathbf{u} \wedge \mathbf{v} \wedge \mathbf{w}$ as a multiple of $e_1 \wedge e_2 \wedge e_3$. Determine whether $\{\mathbf{u}, \mathbf{v}, \mathbf{w}\}$ is a linearly independent set using only the wedge product, explaining the criterion.

6. The symmetric algebra $\mathrm{Sym}(V)$ in degree 2 satisfies $\mathrm{Sym}^2(V) \cong V^{\otimes 2} / \langle v \otimes w - w \otimes v \rangle$. For $V = \mathbb{R}^2$ with basis $\{e_1, e_2\}$, write down a basis for $\mathrm{Sym}^2(V)$ and identify it with the space of homogeneous degree-2 polynomials in two variables. Show that $\dim \mathrm{Sym}^2(\mathbb{R}^n) = \binom{n+1}{2}$ by counting the symmetric basis elements.

7. Let $V$ be an $n$-dimensional real vector space and let $\omega \in \Lambda^2(V)$ be a nonzero alternating 2-tensor. Prove that $\omega^k = \omega \wedge \omega \wedge \cdots \wedge \omega$ ($k$ times) lies in $\Lambda^{2k}(V)$. For $V = \mathbb{R}^4$ and $\omega = e^1 \wedge e^2 + e^3 \wedge e^4$, compute $\omega \wedge \omega$ explicitly and show it is a nonzero element of $\Lambda^4(V)$. What does this say about the 2-form $\omega$?

8. (Challenge) A bilinear form $B : V \times V \to F$ determines a linear map $\hat{B} : V \to V^*$ by $\hat{B}(v)(w) = B(v,w)$. The form is called non-degenerate if $\hat{B}$ is an isomorphism. (a) Show that if $B$ is a non-degenerate symmetric bilinear form, then $V$ is canonically isomorphic to $V^*$ via $\hat{B}$, without any choice of basis. (b) For an $n$-dimensional vector space with a non-degenerate symmetric form, use this isomorphism to define a natural inner product on $V^*$ and show that the dual basis is orthonormal if and only if the original basis is orthonormal. (c) Explain how a Riemannian metric $g$ on a manifold gives, at each point, a canonical isomorphism between tangent and cotangent spaces — the "musical isomorphisms" of differential geometry.
