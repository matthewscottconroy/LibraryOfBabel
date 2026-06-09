# Vector Calculus

Vector calculus extends the ideas of single-variable differentiation and integration into the multi-dimensional settings demanded by physics, engineering, and geometry. Where ordinary calculus studies how scalar quantities change along a line, vector calculus asks how quantities — some scalar, some vector-valued — vary across surfaces, through volumes, and along curves in two and three dimensions. The subject is not merely a generalization for its own sake: it is the mathematical language of electromagnetism, fluid mechanics, heat flow, gravitational theory, and much of modern differential geometry.

## What This Module Covers

This module develops vector calculus from its conceptual foundations through its deepest unifying theorems. The treatment assumes you are comfortable with single-variable calculus (derivatives, integrals, the Fundamental Theorem of Calculus) and with the basics of linear algebra (vectors, dot products, cross products, matrices). Multivariable calculus — partial derivatives, multiple integrals — will be reviewed and extended as needed, but some prior exposure is helpful.

The module is organized into four units that build on each other in a deliberate sequence.

**Unit 1: Vector Fields and Differential Operators.** Before integrating over curves or surfaces, we must understand the objects being integrated. A vector field assigns a vector to each point in space, encoding phenomena like wind velocity, gravitational force, or electric field intensity. We introduce the three fundamental differential operators — gradient, divergence, and curl — and develop their algebraic and geometric interpretations. The gradient turns a scalar field into a vector field pointing in the direction of steepest ascent. The divergence measures how much a vector field spreads out from a point. The curl measures rotation. These operators are not independent: they satisfy a rich algebra encoded in identities involving the del operator $\nabla$, and understanding those identities is prerequisite to understanding the integral theorems that close the module.

**Unit 2: Line Integrals.** Integration along curves generalizes the single-variable definite integral in two distinct ways. A scalar line integral accumulates a scalar function along a curve, weighted by arc length. A vector line integral — also called a work integral — measures the cumulative effect of a vector field along the direction of travel. The distinction matters physically: dragging a box across a rough surface involves a scalar line integral of friction force magnitude, while computing the work done by a conservative gravitational field involves a vector line integral. This unit also introduces path independence and the Fundamental Theorem for Line Integrals, which shows that for conservative fields, the integral between two points depends only on the endpoints, not on the path taken.

**Unit 3: Surface Integrals.** The step from curves to surfaces introduces the challenge of parametrization: describing a two-dimensional surface embedded in three-dimensional space. Once parametrized, we can integrate scalar functions over a surface (obtaining, for instance, total mass if we know surface density) or integrate vector fields through a surface (obtaining flux, the net flow of fluid or field lines through the surface). Orientation plays a central role: the sign of a flux integral depends on which side of the surface we designate as positive.

**Unit 4: The Fundamental Theorems.** The climax of the module is a suite of three theorems — Green's Theorem, Stokes' Theorem, and the Divergence Theorem — that relate integrals over a region to integrals over its boundary. Each theorem is a higher-dimensional analogue of the Fundamental Theorem of Calculus: knowing a derivative-like quantity throughout a region is equivalent to knowing certain boundary values. The module closes by introducing differential forms, which reveal that all three theorems are instances of a single master theorem, the Generalized Stokes' Theorem, expressed in the language of exterior calculus.

## Mathematical Maturity and Prerequisites

The material is rigorous but not pedantic. Theorems are stated precisely and proofs (or proof sketches) are provided, but geometric intuition is developed alongside formal definitions. You should be comfortable reading and writing $\epsilon$-$\delta$ arguments, though most of the work here is at the level of computing and reasoning about explicit formulas.

Concrete prerequisites: partial derivatives and the chain rule in several variables; double and triple integrals; the cross product and its geometric meaning; parametric curves.

## Connections to Differential Equations

Vector calculus is not tangential to the study of differential equations — it is central to it. The partial differential equations that govern heat conduction, wave propagation, and electrostatics are all written in the language of gradient, divergence, curl, and Laplacian. The Divergence Theorem converts between the integral and differential forms of conservation laws. Stokes' Theorem underlies the structure of Maxwell's equations. By the end of this module, you will have the tools needed to read and derive the fundamental equations of mathematical physics.
