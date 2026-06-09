# Chapter 38 — Exercises

## Important Figures

- **Henri Poincaré (1854–1912)** — invented combinatorial (simplicial) homology in *Analysis Situs* (1895–1904); Betti numbers; the Euler characteristic; algebraic topology begins here
- **Emmy Noether (1882–1935)** — reformulated Betti numbers as ranks of homology groups (c. 1925–1926); gave the chain complex its group-theoretic interpretation
- **Heinz Hopf (1894–1971)** — Hopf invariant; connections between homotopy and homology; algebraic topology in the 1930s–40s
- **Henri Cartan (1904–2008) & Samuel Eilenberg (1913–1998)** — *Homological Algebra* (1956): abstract chain complex formalism; derived functors

## References and Primary Sources

- **H. Poincaré, "Analysis Situs" (1895)** — *J. Éc. Polytechn.* (2) 1 — birth of algebraic topology
- **H. Cartan & S. Eilenberg, *Homological Algebra* (Princeton, 1956)** — definitive treatment
- **C. Weibel, *An Introduction to Homological Algebra* (Cambridge, 1994)**
- **J. Rotman, *An Introduction to Homological Algebra* (2nd ed., Springer, 2009)**

## Examples, Applications, and Thought Experiments

- **Simplicial homology of $S^1$** — triangulate $S^1$ with 3 vertices and 3 edges; boundary maps: $\partial_1(e_{ij}) = v_j - v_i$; $H_1(S^1) = \ker \partial_1 / \text{im} \partial_2 = \mathbb{Z}$ (one independent loop); $H_0(S^1) = \mathbb{Z}$ (connected); the algebra detects topology
- **Euler characteristic** — $\chi = \sum (-1)^i \text{rank}(H_i) = \sum (-1)^i \text{rank}(C_i)$; for the torus $T^2$: $\chi = 1 - 2 + 1 = 0$; for the sphere: $\chi = 2$; a topological invariant computable algebraically; the alternating sum cancels "internal" contributions
- **Chain homotopy** — two chain maps $f, g: C_* \to D_*$ that are chain homotopic (there exist $h_n: C_n \to D_{n+1}$ with $f - g = \partial h + h\partial$) induce equal maps on homology; the algebraic analogue of homotopic continuous maps; this is why homology is a homotopy invariant
- **Thought experiment: homology as "algebra remembers shape"** — a chain complex is an algebraic object; its homology groups capture global topological features (holes of each dimension); the remarkable fact is that this purely algebraic computation recovers topological invariants; this is the bridge Poincaré and Noether built between topology and algebra

## Exercises

1. Let $C_\bullet$ be the chain complex $0 \to \mathbb{Z}^3 \xrightarrow{\partial_2} \mathbb{Z}^3 \xrightarrow{\partial_1} \mathbb{Z} \to 0$ where $\partial_2$ and $\partial_1$ are the boundary maps for a triangulation of $S^1$ using 3 vertices $v_0, v_1, v_2$ and 3 edges $e_{01}, e_{12}, e_{02}$. Write the matrices of $\partial_1$ and $\partial_2$ explicitly (with rows labeled by the target basis). Compute $\ker \partial_1$, $\operatorname{im} \partial_2$, and both $H_0(C_\bullet)$ and $H_1(C_\bullet)$.

2. Consider the chain complex $0 \to \mathbb{Z} \xrightarrow{2} \mathbb{Z} \xrightarrow{3} \mathbb{Z} \to 0$ concentrated in degrees 2, 1, 0. Compute all homology groups. Then replace the map in degree 1 by $6$ (so the complex is $0 \to \mathbb{Z} \xrightarrow{2} \mathbb{Z} \xrightarrow{6} \mathbb{Z} \to 0$) and verify that $\partial \circ \partial = 0$ fails if the maps are changed inconsistently. What is the condition on maps $a, b: \mathbb{Z} \to \mathbb{Z}$ (multiplication by integers) for $0 \to \mathbb{Z} \xrightarrow{a} \mathbb{Z} \xrightarrow{b} \mathbb{Z} \to 0$ to be a chain complex?

3. Prove that if $0 \to A_\bullet \xrightarrow{f} B_\bullet \xrightarrow{g} C_\bullet \to 0$ is a short exact sequence of chain complexes (meaning each term $0 \to A_n \to B_n \to C_n \to 0$ is exact), then there exists a natural connecting homomorphism $\delta_n: H_n(C_\bullet) \to H_{n-1}(A_\bullet)$ and the resulting sequence $\cdots \to H_n(A_\bullet) \to H_n(B_\bullet) \to H_n(C_\bullet) \xrightarrow{\delta_n} H_{n-1}(A_\bullet) \to \cdots$ is exact. (You may assume the snake lemma.)

4. Let $f, g: C_\bullet \to D_\bullet$ be chain maps. Suppose there exist $\mathbb{Z}$-module maps $h_n: C_n \to D_{n+1}$ satisfying $f_n - g_n = \partial_{n+1}^D h_n + h_{n-1} \partial_n^C$ for all $n$. Prove directly from this formula that $f_* = g_*: H_n(C_\bullet) \to H_n(D_\bullet)$ for every $n$.

5. Compute the simplicial homology of the triangulated torus $T^2$ using the standard minimal triangulation (7 vertices, 21 edges, 14 triangles — or use the 9-vertex triangulation if preferred). Verify that $H_0(T^2) \cong \mathbb{Z}$, $H_1(T^2) \cong \mathbb{Z}^2$, $H_2(T^2) \cong \mathbb{Z}$, and check that the Euler characteristic satisfies $\chi = \text{rank}(C_0) - \text{rank}(C_1) + \text{rank}(C_2) = \sum_i (-1)^i \text{rank}(H_i)$.

6. Let $f: C_\bullet \to D_\bullet$ be a chain map such that each $f_n: C_n \to D_n$ is an isomorphism of modules. Prove that $f$ is a chain homotopy equivalence. (Hint: the inverse chain map is the obvious one; show the compositions are chain homotopic to the identity using the zero homotopy.)

7. Define a cochain complex $C^\bullet$ by $C^n = \mathbb{Z}/2\mathbb{Z}$ for $0 \leq n \leq 3$ and $C^n = 0$ otherwise, with all differentials equal to the identity map $\mathbb{Z}/2\mathbb{Z} \to \mathbb{Z}/2\mathbb{Z}$. Compute all cohomology groups $H^n(C^\bullet)$. What does the vanishing of cohomology in most degrees tell you about the "shape" this complex is measuring?

8. (Challenge) Let $C_\bullet$ and $D_\bullet$ be chain complexes and define the tensor product complex $(C \otimes D)_n = \bigoplus_{i+j=n} C_i \otimes D_j$ with differential $\partial(x \otimes y) = \partial^C(x) \otimes y + (-1)^{|x|} x \otimes \partial^D(y)$. Verify that $\partial^2 = 0$. If $C_\bullet$ is the complex $0 \to \mathbb{Z} \xrightarrow{2} \mathbb{Z} \to 0$ (in degrees 1 and 0) and $D_\bullet = C_\bullet$, compute $(C \otimes D)_\bullet$ explicitly and find all homology groups of the tensor product complex.
