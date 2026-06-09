# Chapter 1: Theory of Linear Equations

The second-order linear ODE $y'' + p(x)y' + q(x)y = g(x)$ is not merely an equation to be solved; it is an instance of a linear operator acting on a function space, and the theory governing its solutions is a piece of linear algebra. This chapter develops the structural results that underlie all solution methods: the superposition principle, the characterization of linear independence via the Wronskian, Abel's identity for the Wronskian's evolution, and the reduction of order technique for finding second solutions.

## Chapter Contents

The first section establishes the superposition principle for homogeneous equations, shows that the solution set is a two-dimensional vector space, and states that the general solution is a linear combination of any two linearly independent solutions.

The second section introduces the Wronskian $W(y_1, y_2) = y_1 y_2' - y_1' y_2$ as the determinant test for linear independence. The Wronskian is either identically zero (if the solutions are dependent) or never zero (if they are independent), with no intermediate possibility, a remarkable dichotomy.

The third section presents Abel's identity: the Wronskian satisfies $W' = -p(x)W$, so $W(x) = W(x_0)e^{-\int_{x_0}^x p(t)\,dt}$. This formula determines the Wronskian without knowing the individual solutions.

The fourth section develops the reduction of order technique: given one solution $y_1$ of the homogeneous equation, a second linearly independent solution $y_2$ can be found by seeking $y_2 = v(x)y_1$ and solving a first-order linear equation for $v'$.

## Key Theorems

The existence and uniqueness theorem for second-order linear equations (Picard applied to the equivalent first-order system) guarantees that every IVP has a unique solution on the entire interval where $p$ and $q$ are continuous.

The Wronskian characterization: $W(y_1, y_2)(x) \neq 0$ for some $x$ if and only if $y_1$ and $y_2$ form a fundamental set. Combined with Abel's identity, this means that checking the Wronskian at a single point determines independence everywhere.
