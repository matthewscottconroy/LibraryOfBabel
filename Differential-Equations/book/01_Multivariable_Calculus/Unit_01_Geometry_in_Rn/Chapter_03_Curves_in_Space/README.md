# Chapter 03 Curves in Space

A curve in space is the trace left by a moving point. Whether one is tracking a planet's orbit, the path of a roller coaster, the trajectory of a charged particle in a magnetic field, or the shape of a strand of DNA, the mathematical object involved is the same: a smooth map from an interval of real numbers (time, or arc length, or any convenient parameter) into $\mathbb{R}^3$. This chapter develops the calculus of such maps, building from the basic definition to a complete local description of a curve's geometric shape.

## What This Chapter Covers

**Section 1 (Parametric Curves)** establishes the fundamental definition: a curve in $\mathbb{R}^n$ is a continuous (and, when needed, differentiable) function $\mathbf{r}: [a, b] \to \mathbb{R}^n$. The **velocity vector** $\mathbf{r}'(t)$ is the derivative, computed componentwise: if $\mathbf{r}(t) = (x(t), y(t), z(t))$, then $\mathbf{r}'(t) = (x'(t), y'(t), z'(t))$. The velocity vector is tangent to the curve at each point. The section discusses regularity (the condition $\mathbf{r}'(t) \neq \mathbf{0}$, ensuring the curve has a well-defined tangent direction everywhere), and reviews how various familiar curves — circles, helices, ellipses — are represented parametrically.

**Section 2 (Arc Length)** addresses the question of measuring distance along a curve. The arc length from $t = a$ to $t = b$ is $\int_a^b \|\mathbf{r}'(t)\|\,dt$, the integral of the speed. This formula reduces to the one-variable arc length formula when $n = 2$ and $\mathbf{r}(t) = (t, f(t))$. The section introduces the **arc length parameterization**: by reparameterizing so that the parameter $s$ measures distance along the curve, one obtains $\|\mathbf{r}'(s)\| = 1$ everywhere, which simplifies the formulas for curvature and torsion considerably.

**Section 3 (Curvature and Torsion)** introduces the two intrinsic geometric quantities associated with a space curve. The **curvature** $\kappa$ measures how rapidly the direction of the curve is changing — how sharply it bends. It is defined as $\kappa = \|\mathbf{T}'(s)\|$ where $\mathbf{T}(s) = \mathbf{r}'(s)$ is the unit tangent vector in the arc length parameterization. In a general parameterization, $\kappa = \|\mathbf{r}' \times \mathbf{r}''\|/\|\mathbf{r}'\|^3$. For a circle of radius $R$, $\kappa = 1/R$: tight curves have large curvature. The **torsion** $\tau$ measures how much the curve twists out of the plane of curvature. A plane curve has $\tau = 0$; a helix has constant nonzero torsion.

**Section 4 (Frenet-Serret Frame)** synthesizes the preceding sections into an elegant local coordinate system that moves with the curve. At each point, three mutually perpendicular unit vectors are defined: the **unit tangent** $\mathbf{T}$, the **principal normal** $\mathbf{N}$ (pointing toward the center of curvature), and the **binormal** $\mathbf{B} = \mathbf{T}\times\mathbf{N}$. These satisfy the **Frenet-Serret formulas**:

$$\mathbf{T}' = \kappa \mathbf{N}, \quad \mathbf{N}' = -\kappa\mathbf{T} + \tau\mathbf{B}, \quad \mathbf{B}' = -\tau\mathbf{N},$$

where derivatives are with respect to arc length. These three equations encode all the geometric information about the curve: given $\kappa(s)$ and $\tau(s)$, the curve is determined up to a rigid motion (the Fundamental Theorem of Curves).

## How the Sections Build on Each Other

The sections form a logical chain. Parametric curves provide the basic objects; arc length provides the natural parameter; curvature and torsion are defined cleanly in terms of the arc length parameterization; and the Frenet-Serret frame packages curvature and torsion into a complete system. The cross product from Chapter 1 is used to define the binormal $\mathbf{B}$ and appears in the computation formula for curvature. The dot product appears throughout in verifying orthogonality of the frame vectors.

## How This Chapter Fits into the Course

The differential geometry of curves is not merely a beautiful detour; it connects directly to the material that follows. The derivative of a vector-valued function — the central operation here — is the same operation that appears in the Jacobian matrix in Unit 2. The arc length integral is the simplest example of a line integral, the theory of which is fully developed in the next stage of the course (vector calculus). The Frenet-Serret frame is a moving frame, a concept that generalizes to surfaces and manifolds and underlies much of modern differential geometry. Students who master the material in this chapter develop a concrete sense of what it means to differentiate in $\mathbb{R}^n$, which is the foundation for everything that follows.
