# Unit 01 Geometry in Rn

Before one can differentiate or integrate a function of several variables, one needs a working language for the space in which that function lives. This unit develops that language systematically, beginning with the algebraic and geometric structure of $\mathbb{R}^n$ itself and ending with the differential geometry of smooth curves in space.

## What This Unit Covers

The unit is organized around three chapters that form a natural progression from the simplest geometric objects (points and vectors) through classical analytic geometry (lines, planes, and quadric surfaces) to the calculus of curves.

**Chapter 1: Vectors in $\mathbb{R}^n$** introduces the vector space $\mathbb{R}^n$ and the operations that give it geometric meaning. Vector addition and scalar multiplication are the algebraic backbone, but the dot product and cross product are what make $\mathbb{R}^n$ feel like space rather than just a set of $n$-tuples. The dot product encodes lengths and angles; the cross product (defined in $\mathbb{R}^3$) encodes perpendicularity and oriented area. The chapter closes with projections, which appear in decomposing forces, computing distances, and later in understanding orthogonal bases.

**Chapter 2: Analytic Geometry** applies vector language to describe the classical geometric objects. A line in $\mathbb{R}^n$ is most naturally described not by a slope-intercept equation but by a point and a direction vector, leading to parametric equations. Planes in $\mathbb{R}^3$ are characterized by a normal vector. The chapter then introduces quadric surfaces — ellipsoids, hyperboloids, paraboloids — which are the natural three-dimensional analogues of conic sections and which appear as level surfaces of quadratic functions throughout the course. It closes with a treatment of polar, cylindrical, and spherical coordinate systems, which simplify both geometry and integration in many important situations.

**Chapter 3: Curves in Space** is where geometry meets calculus. A curve in $\mathbb{R}^n$ is a vector-valued function of a single real parameter, and the derivative of that function is a tangent vector. Arc length is computed by integrating the magnitude of the velocity vector, and the arc-length parameterization provides a natural way to describe a curve independent of how fast one traverses it. From the unit tangent vector one defines curvature, measuring how sharply the curve bends, and torsion, measuring how much it twists out of a plane. These quantities are captured elegantly by the Frenet-Serret frame: three mutually perpendicular unit vectors — the tangent $\mathbf{T}$, the normal $\mathbf{N}$, and the binormal $\mathbf{B}$ — that travel with the curve and satisfy the Frenet-Serret formulas.

## How the Chapters Build on Each Other

Chapter 1 is foundational for everything that follows. The dot product is needed to define the angle in Chapter 2 and the curvature in Chapter 3; the cross product provides the normal vector to a plane in Chapter 2 and the binormal vector in Chapter 3. Chapter 2 introduces the geometric objects that will appear as domains of integration in Unit 3 and as constraint sets in optimization problems in Unit 2. Chapter 3 provides the first real example of calculus in $\mathbb{R}^n$: differentiating a vector-valued function and extracting geometric information from the result.

## How This Unit Fits into the Course

This unit serves primarily as preparation. Its algebraic and geometric tools appear everywhere in later material: the gradient of a function is a vector, and its geometric interpretation (as normal to a level set) depends on the dot product; the change-of-variables formula for multiple integrals uses the determinant of the Jacobian matrix, which is related to the volume of a parallelepiped spanned by vectors. The Frenet-Serret frame is the first instance of a moving frame, a concept that returns in the theory of differential equations on manifolds.

Students who feel comfortable working with vectors, visualizing planes and surfaces in $\mathbb{R}^3$, and computing arc lengths and curvatures of space curves will find the later units significantly more accessible.
