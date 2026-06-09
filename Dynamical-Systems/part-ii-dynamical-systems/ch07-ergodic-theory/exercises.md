# Exercises — Chapter 7

These exercises develop computational and proof-writing fluency across the core topics of ergodic theory. Several build toward important applications in number theory and information theory.

---

**Exercise 7.1.** Prove that the Gauss map $G(x) = \{1/x\}$ preserves the Gauss measure $\mu_G = \frac{1}{\ln 2} \frac{dx}{1+x}$. (*Hint:* Compute $\mu_G(G^{-1}([a,b]))$ directly.)

**Exercise 7.2.** Prove the Mean Ergodic Theorem in the following form: if $U$ is a unitary operator on a Hilbert space $H$, then $\frac{1}{N}\sum_{n=0}^{N-1} U^n \varphi \to P\varphi$ in $H$, where $P$ is the orthogonal projection onto $\ker(U - I)$.

**Exercise 7.3.** Let $(X, \mu, f)$ be ergodic. Show that for any $\varphi, \psi \in L^2(\mu)$:
$$\lim_{N \to \infty} \frac{1}{N} \sum_{n=0}^{N-1} \langle U_f^n \varphi, \psi \rangle = \langle \varphi, 1 \rangle \langle 1, \psi \rangle = \left(\int \varphi\right)\left(\int \psi\right).$$

**Exercise 7.4.** (Ergodicity of the Doubling Map) Prove that the doubling map $f(x) = 2x \pmod{1}$ is ergodic with respect to Lebesgue measure, using Fourier analysis on $[0,1]$.

**Exercise 7.5.** (Entropy Computation) Compute the KS entropy of the $\frac{1}{3}$-$\frac{2}{3}$ Bernoulli shift (where $p_0 = 1/3$, $p_1 = 2/3$). Compare to the KS entropy of the fair coin shift ($p_0 = p_1 = 1/2$).

**Exercise 7.6.** Show that $h_\mu(f^n) = n \cdot h_\mu(f)$ for any MPT $f$ and $n \geq 1$.

**Exercise 7.7.** (Ornstein connection) The Arnold cat map $A = \begin{pmatrix} 2 & 1 \\ 1 & 1 \end{pmatrix}$ on ${\mathbb T}^2$ has eigenvalues $\lambda = \frac{3 \pm \sqrt{5}}{2}$. Compute the KS entropy using Pesin's formula. Conclude that the cat map is Bernoulli.

**Exercise 7.8.** Prove the Poincaré Recurrence Theorem directly from Birkhoff's theorem: if $\mu(A) > 0$, apply Birkhoff to $\varphi = \mathbf{1}_A$ and show $\varphi^*(x) = \mu(A) > 0$ for a.e. $x \in A$, which implies infinitely many returns.
