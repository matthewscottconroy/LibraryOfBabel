# Chapter 1: Derivation and Properties of Harmonic Functions

A function $u$ satisfying Laplace's equation $\Delta u = 0$ in a domain $\Omega$ is called **harmonic**. Harmonic functions are among the most studied objects in all of mathematics: they arise naturally in potential theory, complex analysis (where they are the real and imaginary parts of analytic functions), probability (as the expected values of Brownian motion at stopping times), and physics (as electrostatic, gravitational, and fluid-flow potentials).

The properties of harmonic functions are extraordinary: they are infinitely smooth, satisfy the mean value property (the value at any point equals the average over any surrounding sphere), obey a maximum principle, and are real-analytic (equal to their Taylor series) in their domain. These properties make harmonic functions far more regular than merely smooth functions and give elliptic PDE theory its distinctive character.

## Structure of This Chapter

**Section 1: Physical Origins** reviews the derivation of Laplace's equation from the steady-state heat equation, electrostatics, and fluid mechanics. The physical origins motivate the boundary conditions (Dirichlet: prescribed potential; Neumann: prescribed flux) and the interpretation of solutions.

**Section 2: Harmonic Functions** defines harmonic functions precisely and establishes the basic examples: polynomials satisfying $\Delta u = 0$ (harmonic polynomials), the fundamental solution $\Phi(x) = -\log|x|/(2\pi)$ in 2D and $\Phi(x) = 1/(4\pi|x|)$ in 3D, and functions of the form $u = \text{Re}(f(z))$ for analytic $f$ in the 2D case.

**Section 3: The Mean Value Property** proves the theorem that the value of a harmonic function at any point equals its average over any ball (or sphere) centered at that point:

$$u(\mathbf{x}_0) = \frac{1}{|B_r|}\int_{B_r(\mathbf{x}_0)}u\,d\mathbf{x} = \frac{1}{|\mathcal{S}_{r}|}\int_{\partial B_r(\mathbf{x}_0)}u\,dS.$$

This is one of the most elegant results in analysis and has profound consequences.

**Section 4: The Maximum Principle** proves that a harmonic function on a bounded domain $\Omega$ attains its maximum (and minimum) on the boundary $\partial\Omega$. The maximum principle is an immediate consequence of the mean value property and is the cornerstone of elliptic PDE theory.

## Key Theorems Previewed

- **Harnack's inequality:** If $u \geq 0$ is harmonic on $B_{2r}(\mathbf{x}_0)$, then $\sup_{B_r} u \leq C \inf_{B_r} u$ for a constant $C$ depending only on dimension — harmonic functions cannot vary too rapidly.

- **Removable singularity theorem:** If $u$ is harmonic in $\Omega\setminus\{\mathbf{x}_0\}$ and bounded near $\mathbf{x}_0$, then $u$ extends to a harmonic function on all of $\Omega$.

- **Weyl's lemma:** Any distributional solution of $\Delta u = 0$ is actually smooth — there are no weak harmonic functions that are not classical.
