# Chapter 5 — Exercises

These exercises develop facility with spectral theory from the concrete (Jordan form computations) to the abstract (Koopman operators, entanglement). The final exercise connects the chapter to the ergodic theory developed in Part II.

---

**Exercise 5.1.** Find the Jordan canonical form of $A = \begin{pmatrix} 3 & 1 & 0 \\ 0 & 3 & 1 \\ 0 & 0 & 3 \end{pmatrix}$ and compute $e^{tA}$.

**Exercise 5.2.** Let $A$ be real symmetric. Show all eigenvalues are real. Show eigenvectors for distinct eigenvalues are orthogonal.

**Exercise 5.3.** Compute the SVD of $A = \begin{pmatrix} 1 & 1 \\ 0 & 1 \\ 1 & 0 \end{pmatrix}$. Find the rank-1 approximation.

**Exercise 5.4.** Let $H = L^2([0,1])$ and $T: H \to H$ the Volterra operator $Tf(x) = \int_0^x f(t)\,dt$. Show $T$ is compact. Is $T$ self-adjoint? What is $\sigma(T)$?

**Exercise 5.5.** Let $U: \ell^2 \to \ell^2$ be the bilateral shift: $U(e_n) = e_{n+1}$ for all $n \in \mathbb{Z}$. Show $U$ is unitary and compute its spectrum.

**Exercise 5.6.** (Perron-Frobenius) Let $A = \begin{pmatrix} 1 & 2 \\ 1 & 0 \end{pmatrix}$ (transition matrix for a 2-state system). Find the Perron eigenvalue and eigenvector. If $A$ is the transition matrix of a subshift of finite type, what is its topological entropy?

**Exercise 5.7.** (Tensor product and quantum information) In quantum mechanics, the state space of a system composed of subsystems $A$ and $B$ is $H_A \otimes H_B$. Show that not every state in $H_A \otimes H_B$ can be written as a pure product state $v_A \otimes v_B$. Such states are *entangled*. For $H_A = H_B = \mathbb{C}^2$, exhibit a maximally entangled state (the Bell state $(\ket{00} + \ket{11})/\sqrt{2}$) and compute the entanglement entropy.

**Exercise 5.8.** The *Koopman operator* for the doubling map $f: x \mapsto 2x \pmod{1}$ on $[0,1]$ acts on $L^2([0,1])$ by $U_f \varphi = \varphi \circ f$. Compute $U_f(e^{2\pi i k x})$ for each $k \in \mathbb{Z}$ and describe the spectrum of $U_f$.
