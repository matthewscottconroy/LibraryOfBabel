# Chapter 46 — Exercises

## Important Figures

- **Sophus Lie (1842–1899)** — founded Lie group theory to systematize the symmetries of differential equations; *Theorie der Transformationsgruppen* (3 vols., 1888–1893)
- **Wilhelm Killing (1847–1923)** — classified simple Lie algebras (with errors corrected by Cartan); introduced root systems
- **Élie Cartan (1869–1951)** — classified real forms of Lie groups; symmetric spaces; structure theory of semisimple Lie groups; completed Killing's program
- **Hermann Weyl (1885–1955)** — compact Lie groups; Weyl character formula; global structure; Weyl group; *The Classical Groups* (1939)

## References and Primary Sources

- **S. Lie, *Theorie der Transformationsgruppen* (3 vols., Teubner, 1888–1893)** — founding texts
- **B. Hall, *Lie Groups, Lie Algebras, and Representations* (2nd ed., Springer, 2015)** — accessible matrix Lie group introduction
- **A. Knapp, *Lie Groups Beyond an Introduction* (2nd ed., Birkhäuser, 2002)** — comprehensive treatment of structure and representation theory

## Examples, Applications, and Thought Experiments

- **$SO(2) \cong S^1$** — the rotation group of the plane: matrices $\begin{bmatrix}\cos\theta & -\sin\theta \\ \sin\theta & \cos\theta\end{bmatrix}$; diffeomorphic to the circle; Lie algebra $\mathfrak{so}(2) \cong \mathbb{R}$; exponential map $\exp(t) = e^{it} \in S^1$; the simplest compact connected Lie group
- **$SU(2)$ double covers $SO(3)$** — the map $SU(2) \to SO(3)$ has kernel $\{\pm I\}$; a $360°$ rotation in $SO(3)$ corresponds to a $-I$ transformation in $SU(2)$; a $720°$ rotation returns to the identity; this explains spinors in quantum mechanics: a spin-$\frac{1}{2}$ particle needs a $720°$ rotation to return to its original state
- **$GL_n(\mathbb{R})$ and its subgroups** — the general linear group contains $SL_n$ (determinant 1), $O_n$ (orthogonal), $SO_n$ (special orthogonal), each a Lie subgroup; the inclusions $SO_n \subset O_n \subset GL_n$ reflect geometric constraints (orientation, isometry, invertibility)
- **The rotation group $SO(3)$ and rigid body motion** — the configuration space of a rigid body with one fixed point is $SO(3)$; Euler's theorem: any rotation is a rotation by some angle about some axis; the Lie algebra $\mathfrak{so}(3) \cong \mathbb{R}^3$ with the cross product as Lie bracket; angular velocity is an element of $\mathfrak{so}(3)$

## Exercises

1. Verify directly that $SL_n(\mathbb{R}) = \{A \in GL_n(\mathbb{R}) : \det A = 1\}$ is a smooth submanifold of $GL_n(\mathbb{R})$ of dimension $n^2 - 1$ by applying the regular value theorem to the smooth map $\det: GL_n(\mathbb{R}) \to \mathbb{R}^{\times}$. Conclude that $SL_n(\mathbb{R})$ is a Lie group.

2. Show that the orthogonal group $O_n(\mathbb{R}) = \{A \in GL_n(\mathbb{R}) : A^T A = I\}$ is a compact Lie group of dimension $\frac{n(n-1)}{2}$. Determine which connected components contain $I$ and describe the component group $O_n / SO_n$.

3. Let $U_n = \{A \in GL_n(\mathbb{C}) : A^\dagger A = I\}$ be the unitary group. Compute $\dim_{\mathbb{R}} U_n$ and verify that $SU_n = \ker(\det: U_n \to S^1)$ is a Lie subgroup of dimension $n^2 - 1$. Show that $U_1 \cong S^1$ as Lie groups.

4. For the matrix exponential $\exp(A) = \sum_{k=0}^{\infty} \frac{A^k}{k!}$, prove that $\det(\exp(A)) = e^{\text{tr}(A)}$ using the Jordan normal form (or alternatively by proving both sides satisfy the same differential equation). Deduce that $\exp$ maps $\mathfrak{sl}_n = \{A : \text{tr}(A) = 0\}$ into $SL_n$.

5. A one-parameter subgroup of a Lie group $G$ is a smooth group homomorphism $\gamma: \mathbb{R} \to G$. Show that every one-parameter subgroup of $GL_n(\mathbb{R})$ has the form $\gamma(t) = e^{tA}$ for some matrix $A \in \mathfrak{gl}_n(\mathbb{R})$. What is $A$ in terms of $\gamma$?

6. Construct explicitly the covering homomorphism $\phi: SU(2) \to SO(3)$ by having $SU(2)$ act on the space of traceless $2 \times 2$ Hermitian matrices (which is a real three-dimensional space) by conjugation. Identify the kernel and verify it equals $\{\pm I\}$.

7. Let $G$ and $H$ be Lie groups and $f: G \to H$ a Lie group homomorphism. Prove that the image $f(G)$ is a Lie subgroup of $H$ (not merely an abstract subgroup), and that the kernel $\ker f$ is a closed normal Lie subgroup of $G$. Give an example showing that $f(G)$ need not be closed in $H$.

8. (Challenge) Let $G$ be a connected Lie group with Lie algebra $\mathfrak{g}$, and let $H \subset G$ be a connected closed subgroup corresponding to a Lie subalgebra $\mathfrak{h} \subset \mathfrak{g}$. Prove that the coset space $G/H$ is a smooth manifold of dimension $\dim \mathfrak{g} - \dim \mathfrak{h}$, and give two explicit examples illustrating the correspondence between Lie subalgebras of $\mathfrak{so}(3)$ and homogeneous spaces of $SO(3)$.
