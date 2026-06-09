# Chapter 03: Conformal Mapping

A conformal map is an analytic function whose derivative is nonvanishing, and the key property it possesses is the preservation of angles: if two smooth curves meet at a point with a certain angle, their images under the map meet at the same angle. This geometric fact, combined with the rich algebraic structure of analytic functions, makes conformal maps extraordinarily useful for solving boundary value problems, analyzing fluid flows, and classifying domains.

## Section 01: Preservation of Angles

At a point where $f'(z_0) \neq 0$, the map $f$ is locally a rotation by $\arg f'(z_0)$ combined with scaling by $|f'(z_0)|$. Both operations preserve angles. This angle-preservation holds for both the magnitude and the sense (orientation) of angles — conformal maps are directly conformal, not just isogonal.

At a critical point where $f'(z_0) = 0$, angles are multiplied by the order of the zero of $f'(z_0)$: a zero of order $k-1$ (so a zero of order $k$ of $f - f(z_0)$) multiplies angles by $k$.

## Section 02: Mobius Transformations

Mobius (or fractional linear) transformations $w = (az + b)/(cz + d)$ with $ad - bc \neq 0$ are the most important family of conformal maps. They:
- Are bijections of the Riemann sphere $\hat{\mathbb{C}}$ to itself.
- Map circles and lines to circles and lines (circles on the Riemann sphere to circles).
- Are determined by three points: a Mobius transformation is uniquely specified by where it sends three distinct points.
- Form a group under composition, isomorphic to $\mathrm{PSL}(2, \mathbb{C})$.

## Section 03: The Schwarz-Christoffel Transformation

The Schwarz-Christoffel formula gives an explicit conformal map from the upper half-plane (or unit disk) to a polygon with prescribed vertices and interior angles. It is expressed as an integral:
$$f(z) = A + C\int_{z_0}^z \prod_k (w - x_k)^{\alpha_k - 1}\, dw,$$
where $x_k$ are preimages of the vertices and $\alpha_k\pi$ are the interior angles. This formula underlies much of conformal mapping in applications to engineering and physics.

## Section 04: The Riemann Mapping Theorem

**Theorem.** Any simply connected proper open subset of $\mathbb{C}$ is conformally equivalent to the open unit disk $\mathbb{D}$.

This theorem — proved without an explicit formula — is one of the deepest results in complex analysis. It guarantees that all simply connected domains (other than $\mathbb{C}$ itself) have the same conformal type, and that the unit disk is a universal model.

## Section 05: Applications to PDEs and Fluid Flow

Conformal maps solve Laplace's equation on complicated domains by pulling back to the disk or half-plane. In fluid mechanics, the complex potential $\Omega = \phi + i\psi$ (velocity potential plus stream function) satisfies $\Delta\phi = 0$, and conformal maps transform one flow pattern into another. This allows the solution for flow past a circle to be transformed into the solution for flow past a more complicated airfoil (Joukowski transform).

## Learning Objectives

After this chapter, a student should be able to:

- Verify conformality of a given analytic map and determine where it fails.
- Find Mobius transformations mapping specified triples of points to specified triples.
- Determine whether a Mobius transformation is elliptic, hyperbolic, parabolic, or loxodromic.
- Use the Schwarz-Christoffel formula to set up (and in simple cases evaluate) conformal maps to polygons.
- State the Riemann Mapping Theorem and its hypotheses precisely.
- Apply conformal mapping to solve Laplace's equation on a given domain.
