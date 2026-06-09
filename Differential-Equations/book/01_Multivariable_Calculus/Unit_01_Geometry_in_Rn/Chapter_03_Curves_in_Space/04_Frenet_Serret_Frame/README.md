# Frenet-Serret Frame

The curvature $\kappa$ and torsion $\tau$ of a curve were defined in the previous section as scalar quantities measuring bending and twisting. But a moving point on a curve does not just bend and twist abstractly — it does so in specific directions. The Frenet-Serret frame provides the natural coordinate system adapted to the curve at each point: three mutually perpendicular unit vectors that travel with the curve and whose rates of change are governed by $\kappa$ and $\tau$ through the Frenet-Serret formulas. This moving frame is one of the most beautiful constructions in classical differential geometry, and it demonstrates how the derivative — applied repeatedly to a vector-valued function — encodes complete geometric information.

## The Three Frame Vectors

Let $\mathbf{r}(s)$ be a unit-speed curve in $\mathbb{R}^3$ with $\kappa(s) > 0$ everywhere. Three vectors are defined at each point:

**Unit tangent vector:** $\mathbf{T}(s) = \mathbf{r}'(s)$, the unit vector pointing in the direction of motion.

**Principal normal vector:** Since $\|\mathbf{T}\| = 1$ is constant, $\mathbf{T}'$ is perpendicular to $\mathbf{T}$. Define $\mathbf{N}(s) = \mathbf{T}'(s)/\|\mathbf{T}'(s)\| = \mathbf{T}'(s)/\kappa(s)$. The principal normal points toward the center of the osculating circle — the direction in which the curve is bending.

**Binormal vector:** $\mathbf{B}(s) = \mathbf{T}(s)\times\mathbf{N}(s)$. Since $\mathbf{T}$ and $\mathbf{N}$ are unit vectors and orthogonal, $\mathbf{B}$ is also a unit vector, and is perpendicular to both.

The ordered triple $(\mathbf{T}, \mathbf{N}, \mathbf{B})$ forms a right-handed orthonormal basis for $\mathbb{R}^3$ at each point of the curve. This is the **Frenet-Serret frame** (also called the **TNB frame**).

## Geometric Meaning of the Three Planes

The frame defines three planes at each point:
- The **osculating plane** is spanned by $\mathbf{T}$ and $\mathbf{N}$. It is the plane that best approximates the curve locally (to second order). The curve curves within the osculating plane.
- The **normal plane** is spanned by $\mathbf{N}$ and $\mathbf{B}$. It is perpendicular to the tangent.
- The **rectifying plane** is spanned by $\mathbf{T}$ and $\mathbf{B}$.

A plane curve lies in a fixed plane — its osculating plane is the same at every point.

## The Frenet-Serret Formulas

**Theorem (Frenet-Serret).** For a unit-speed curve with curvature $\kappa > 0$ and torsion $\tau$,

$$\mathbf{T}' = \kappa\mathbf{N},$$
$$\mathbf{N}' = -\kappa\mathbf{T} + \tau\mathbf{B},$$
$$\mathbf{B}' = -\tau\mathbf{N}.$$

**Proof.**
- $\mathbf{T}' = \kappa\mathbf{N}$: This is just the definition of $\mathbf{N}$ and $\kappa$.
- $\mathbf{B}' = -\tau\mathbf{N}$: Differentiate $\mathbf{B} = \mathbf{T}\times\mathbf{N}$: $\mathbf{B}' = \mathbf{T}'\times\mathbf{N} + \mathbf{T}\times\mathbf{N}' = \kappa\mathbf{N}\times\mathbf{N} + \mathbf{T}\times\mathbf{N}'$. Since $\mathbf{N}\times\mathbf{N} = \mathbf{0}$, we have $\mathbf{B}' = \mathbf{T}\times\mathbf{N}'$. Since $\|\mathbf{B}\| = 1$, $\mathbf{B}'\perp\mathbf{B}$. Also $\mathbf{B}\perp\mathbf{T}$ always, so $\mathbf{B}'$ is perpendicular to both $\mathbf{T}$ and $\mathbf{B}$, hence parallel to $\mathbf{N}$. We write $\mathbf{B}' = -\tau\mathbf{N}$ as the definition of torsion $\tau$.
- $\mathbf{N}' = -\kappa\mathbf{T} + \tau\mathbf{B}$: Differentiate $\mathbf{N} = \mathbf{B}\times\mathbf{T}$ (since $\mathbf{B} = \mathbf{T}\times\mathbf{N}$ implies $\mathbf{N} = \mathbf{B}\times\mathbf{T}$): $\mathbf{N}' = \mathbf{B}'\times\mathbf{T} + \mathbf{B}\times\mathbf{T}' = (-\tau\mathbf{N})\times\mathbf{T} + \mathbf{B}\times(\kappa\mathbf{N}) = \tau(\mathbf{T}\times\mathbf{N}) + \kappa(\mathbf{B}\times\mathbf{N}) = \tau\mathbf{B} - \kappa\mathbf{T}$.

The Frenet-Serret formulas can be written compactly in matrix form:

$$\frac{d}{ds}\begin{pmatrix}\mathbf{T}\\\mathbf{N}\\\mathbf{B}\end{pmatrix} = \begin{pmatrix}0 & \kappa & 0 \\ -\kappa & 0 & \tau \\ 0 & -\tau & 0\end{pmatrix}\begin{pmatrix}\mathbf{T}\\\mathbf{N}\\\mathbf{B}\end{pmatrix}.$$

The matrix is antisymmetric, reflecting the fact that the frame vectors maintain mutual orthonormality as they evolve.

## Worked Example: The Helix

For $\mathbf{r}(s) = \left(a\cos\frac{s}{c}, a\sin\frac{s}{c}, \frac{b}{c}s\right)$ with $c = \sqrt{a^2+b^2}$ (arc length parameterization from the previous section):

$$\mathbf{T}(s) = \left(-\frac{a}{c}\sin\frac{s}{c},\; \frac{a}{c}\cos\frac{s}{c},\; \frac{b}{c}\right).$$

$$\mathbf{T}'(s) = \left(-\frac{a}{c^2}\cos\frac{s}{c},\; -\frac{a}{c^2}\sin\frac{s}{c},\; 0\right) = \frac{a}{c^2}\left(-\cos\frac{s}{c},\; -\sin\frac{s}{c},\; 0\right).$$

Since $\kappa = a/c^2 = a/(a^2+b^2)$ (computed previously), $\mathbf{N} = \mathbf{T}'/\kappa = (-\cos(s/c), -\sin(s/c), 0)$.

$$\mathbf{B} = \mathbf{T}\times\mathbf{N} = \left(-\frac{a}{c}\sin\frac{s}{c},\;\frac{a}{c}\cos\frac{s}{c},\;\frac{b}{c}\right)\times\left(-\cos\frac{s}{c},\;-\sin\frac{s}{c},\;0\right).$$

Computing: $\mathbf{B} = \left(\frac{b}{c}\sin\frac{s}{c},\;-\frac{b}{c}\cos\frac{s}{c},\;\frac{a}{c}\right)$. (This can be verified by checking that $\mathbf{B}$ is a unit vector perpendicular to both $\mathbf{T}$ and $\mathbf{N}$.)

One can verify $\mathbf{B}'(s) = -\tau\mathbf{N}$ with $\tau = b/c^2 = b/(a^2+b^2)$, confirming the formula from the previous section.

## The Fundamental Theorem of Curves

The Frenet-Serret formulas form a system of nine first-order linear ODEs (three for each of the three vector equations, nine components in total). Given initial values $\mathbf{T}(0)$, $\mathbf{N}(0)$, $\mathbf{B}(0)$ forming an orthonormal frame, and given $\kappa(s)$ and $\tau(s)$, the theory of ODEs guarantees a unique solution. Integrating $\mathbf{r}'(s) = \mathbf{T}(s)$ then gives the curve itself (up to the initial point $\mathbf{r}(0)$).

**Fundamental Theorem of Space Curves.** Given smooth functions $\kappa(s) > 0$ and $\tau(s)$ for $s \in [0, L]$, there exists a unique (up to rigid motion) unit-speed curve $\mathbf{r}: [0, L] \to \mathbb{R}^3$ with curvature $\kappa$ and torsion $\tau$.

This is the complete solution to the curve classification problem: $\kappa$ and $\tau$ are the invariants that determine a curve's shape.

## Connection to Physics and Differential Equations

The Frenet-Serret system $\begin{pmatrix}\mathbf{T}'\\\mathbf{N}'\\\mathbf{B}'\end{pmatrix} = A(s)\begin{pmatrix}\mathbf{T}\\\mathbf{N}\\\mathbf{B}\end{pmatrix}$ is a linear matrix ODE with coefficient matrix $A(s)$. This is the first appearance in the course of a system of differential equations, and it has the exact form studied in the unit on linear systems. The fact that $A$ is antisymmetric means the solution matrix is orthogonal for all $s$, which is the mathematical expression of the frame remaining orthonormal.

In physics, equations of the same form govern the precession of a spinning object's axis under a torque — the body frame of the spinning object plays the role of the Frenet-Serret frame.
