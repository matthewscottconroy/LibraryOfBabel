# Unit 01: Complex Numbers and Functions

The foundation of complex analysis is the number system $\mathbb{C}$ itself. This unit builds that foundation carefully, moving from the algebraic definition of complex numbers through their rich geometric structure and on to the first notions of complex-valued functions. The material here is elementary in the sense that it requires no calculus, but it is not trivial: the geometric content of complex arithmetic is deep, and the subtleties of multivalued functions will reappear throughout the course.

## Chapter 01: The Complex Number System

A complex number is an expression $z = x + iy$ where $x, y \in \mathbb{R}$ and $i^2 = -1$. The set $\mathbb{C}$ of all such numbers forms a field under the natural operations of addition and multiplication. The first chapter studies this field from three perspectives: algebraic, geometric, and polar.

Algebraically, $\mathbb{C}$ extends $\mathbb{R}$ while preserving all field axioms, and the introduction of $i$ allows every polynomial over $\mathbb{C}$ to factor completely — a consequence that will be proved rigorously via Liouville's theorem in Unit 03.

Geometrically, identifying $z = x + iy$ with the point $(x, y)$ in the plane endows $\mathbb{C}$ with a Euclidean topology. The modulus $|z| = \sqrt{x^2 + y^2}$ measures distance from the origin, and the triangle inequality $|z + w| \leq |z| + |w|$ follows immediately from the Cauchy-Schwarz inequality applied in $\mathbb{R}^2$.

The polar form $z = r e^{i\theta}$ — whose legitimacy is established once the complex exponential is properly defined — encodes multiplication as a combination of scaling and rotation. De Moivre's theorem and the theory of $n$-th roots of unity emerge naturally from this representation.

## Chapter 02: Complex Functions

Once $\mathbb{C}$ is understood as a number system, we ask what it means for a rule $f : \mathbb{C} \to \mathbb{C}$ to be a function. In principle, any $f(z) = u(x,y) + i v(x,y)$ where $u$ and $v$ are real-valued functions of two real variables defines a complex function. The challenge is to identify which such functions deserve to be called analytic — the central concept of the entire subject, developed fully in Unit 02.

This chapter lays the groundwork: limits and continuity for complex functions, phrased in terms of the metric $|z - w|$. The definitions mirror those from real analysis, but the plane topology introduces a new phenomenon: a limit point can be approached from uncountably many directions, which places far stronger constraints on limits than in the real case.

The chapter closes with one of the most important and subtle topics in elementary complex analysis: multivalued functions and branch cuts. The complex logarithm, defined by $\log z = \ln|z| + i \arg z$, is not single-valued because the argument $\arg z$ is determined only up to integer multiples of $2\pi$. To obtain a genuine function, one must restrict the domain by introducing a branch cut — a curve in the plane that the variable $z$ is forbidden to cross. The choice of branch cut is a matter of convention, but it has real computational consequences.

## Learning Objectives

By the end of this unit, a student should be able to:

- Perform arithmetic with complex numbers in both rectangular and polar form, and convert freely between the two representations.
- State and apply de Moivre's theorem to compute powers and roots of complex numbers.
- Find all $n$-th roots of a complex number and describe their geometric distribution on a circle.
- Give the $\varepsilon$-$\delta$ definition of a limit of a complex function and determine limits from first principles.
- Explain the origin of multivalued behavior in complex functions and define a principal branch of the logarithm and power functions.
- Describe what a branch cut is and why it is necessary.

## Connections Forward

The geometric intuition built in this unit pays dividends immediately in Unit 02. Understanding multiplication as rotation and scaling makes the Cauchy-Riemann equations geometrically transparent. The branch cut discussion is a preview of the global topological considerations that arise in the theory of analytic continuation and the study of Riemann surfaces.
