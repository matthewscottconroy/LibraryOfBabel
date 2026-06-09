# 5.7 Tensor Products and Exterior Algebra

The tensor product and exterior algebra generalize matrix multiplication to multilinear settings. These structures appear in two quite different contexts: quantum mechanics (tensor products of Hilbert spaces model composite quantum systems) and the theory of Lyapunov exponents (exterior powers of the tangent map control volume growth rates).

**Definition 5.7.1.** The *tensor product* $V \otimes W$ of vector spaces is characterized by a universal bilinear map $V \times W \to V \otimes W$, $(v,w) \mapsto v \otimes w$. Concretely, if $\{e_i\}$ is a basis for $V$ and $\{f_j\}$ for $W$, then $\{e_i \otimes f_j\}$ is a basis for $V \otimes W$.

The tensor product $v \otimes w$ is a "pure tensor" — a rank-1 element. Not every element of $V \otimes W$ is a pure tensor; those that aren't are called *entangled* (in the quantum computing context).

**Definition 5.7.2.** The *exterior power* $\bigwedge^k V$ consists of alternating $k$-linear forms on $V^*$ (equivalently, antisymmetric tensors in $V^{\otimes k}$). If $\dim V = n$, then $\dim \bigwedge^k V = \binom{n}{k}$.

The exterior product $v_1 \wedge v_2 \wedge \cdots \wedge v_k$ is antisymmetric: swapping any two factors changes the sign. This antisymmetry means $v \wedge v = 0$ — you can't wedge a vector with itself. Geometrically, $v_1 \wedge v_2$ represents the oriented parallelogram spanned by $v_1$ and $v_2$; its magnitude is the area of that parallelogram.

For $A \in M_n(\mathbb{R})$, the action on $\bigwedge^k \mathbb{R}^n$ has eigenvalues $\lambda_{i_1} \cdots \lambda_{i_k}$ for all $k$-subsets $\{i_1, \ldots, i_k\}$ of the eigenvalues of $A$. In particular, the action on $\bigwedge^n \mathbb{R}^n \cong \mathbb{R}$ has eigenvalue $\det(A) = \lambda_1 \cdots \lambda_n$ — the determinant is the eigenvalue of the action on the top exterior power.

**Application in Dynamics — Lyapunov Exponents.** The Lyapunov exponents (Chapter 11) measure the growth rates of volumes under the tangent map. If $D\Phi_t$ is the derivative of the flow at time $t$, the growth rate of $k$-dimensional volumes is governed by the action of $D\Phi_t$ on $\bigwedge^k T_pM$.

More precisely: the sum of the largest $k$ Lyapunov exponents equals the exponential growth rate of $k$-dimensional volume elements. The action on the top exterior power $\bigwedge^n TM$ gives the volume growth rate, which equals $\int \text{tr}(Df)\,dt$ by the variational equation — this is how Liouville's theorem for Hamiltonian systems is proved (trace of the symplectic matrix is zero, so volume is preserved).

**Application in Quantum Information.** In quantum mechanics, the state space of a system composed of subsystems $A$ and $B$ is $H_A \otimes H_B$. Not every state can be written as a product $v_A \otimes v_B$ — those that can't are *entangled*. The entanglement structure (how much of the state "lives in the correlations" between subsystems) is measured by the Schmidt decomposition, which is just the SVD applied to the coefficient matrix.

For $H_A = H_B = \mathbb{C}^2$, the Bell state $(\ket{00} + \ket{11})/\sqrt{2}$ is maximally entangled — no product state can approximate it. The entanglement entropy of a pure state $\psi \in H_A \otimes H_B$ is the von Neumann entropy of the reduced density matrix $\rho_A = \text{tr}_B(|\psi\rangle\langle\psi|)$, which is the classical Shannon entropy of the squared singular values of the coefficient matrix.

This connection between linear algebra (SVD), quantum information (entanglement), and dynamics (Lyapunov exponents via exterior algebra) illustrates why spectral theory is the thread that runs through the entire mathematical structure of both classical and quantum dynamical systems.
