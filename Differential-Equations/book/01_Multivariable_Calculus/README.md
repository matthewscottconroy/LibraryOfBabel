# Multivariable Calculus

Single-variable calculus answers questions about functions of one real variable: how fast is a quantity changing, and how do we accumulate it over an interval? Multivariable calculus extends both questions to functions of two, three, or arbitrarily many variables. The extension is not merely a matter of carrying extra symbols; genuine new phenomena appear at every turn, and the tools required to handle them require rethinking what differentiation and integration mean at a foundational level.

## Why This Subject Matters

The physical world is described by fields — temperature distributions, gravitational potentials, velocity fields of fluids — that assign a number or a vector to every point in space. Understanding how such objects vary, and how to compute quantities like flux, work, and volume, demands multivariable calculus. The same mathematics underlies optimization in machine learning (where the "space" has millions of dimensions), geometric modeling in computer graphics, and the formulation of every classical field theory in physics.

Differential equations, which form the broader subject this course serves, are almost always equations involving partial derivatives — derivatives with respect to several variables simultaneously. Without a firm command of multivariable differentiation and integration, partial differential equations remain inaccessible. This unit of the course is therefore not optional background; it is the language in which everything that follows is written.

## Prerequisites

Students should arrive with a solid command of single-variable calculus through integral calculus and series: limits, derivatives, the fundamental theorem of calculus, techniques of integration, and Taylor series in one variable. Familiarity with basic linear algebra — vectors, matrices, and linear transformations at the level of a first course — is assumed throughout, particularly in the differentiation unit where the Jacobian matrix plays a central role. Experience with two- and three-dimensional coordinate geometry, including equations of lines and planes, is helpful but will be reviewed in Unit 1.

## Structure of the Course

The course is organized into three units that follow a natural logical progression.

**Unit 1: Geometry in $\mathbb{R}^n$** establishes the geometric language of higher dimensions. Before differentiating or integrating, one must understand what the underlying space looks like. This unit covers vectors and their algebra, the geometric objects — lines, planes, and quadric surfaces — that generalize familiar two-dimensional curves, and the differential geometry of curves in space. The Frenet-Serret frame that closes Unit 1 is a beautiful first example of how calculus and geometry interact: the derivative of the unit tangent vector measures curvature, and from curvature and torsion together one can reconstruct a curve up to rigid motion.

**Unit 2: Differentiation in Several Variables** is the theoretical heart of the course. Differentiation is much subtler in several variables than in one: the existence of partial derivatives does not imply differentiability, and there are infinitely many directions in which a limit can be approached. The unit begins by establishing the correct notion of limit and continuity in $\mathbb{R}^n$, then introduces partial derivatives, and builds up to the total derivative — a linear map, represented by the Jacobian matrix, that captures all directional information simultaneously. The chain rule generalizes accordingly. The gradient and directional derivatives give geometric content to differentiation. Taylor series in several variables, organized by the Hessian matrix, provide the local approximations needed for optimization. The unit closes with the implicit and inverse function theorems, which are among the deepest results in multivariable calculus and serve as the rigorous foundation for the notion of a manifold.

**Unit 3: Integration in Several Variables** develops double and triple integrals, change-of-variables formulas, and a range of physical applications. The key technical tool is Fubini's theorem, which reduces multidimensional integrals to iterated single-variable integrals. The change-of-variables theorem, whose formulation requires the Jacobian determinant, generalizes the substitution rule and makes coordinate systems like polar, cylindrical, and spherical both practically essential and conceptually illuminating.

## How the Units Connect

Each unit depends on the previous. The geometric vocabulary of Unit 1 — especially the dot product, cross product, and the idea of a normal vector to a surface — reappears constantly in Units 2 and 3. The gradient from Unit 2 is perpendicular to level sets, a fact with geometric content that was set up in Unit 1. The Jacobian determinant of Unit 3 is the determinant of the Jacobian matrix introduced in Unit 2. Running throughout is the theme of linear approximation: locally, smooth functions look like linear maps, and nearly every major theorem in the subject exploits this fact.

Students who complete this material will be prepared to study vector calculus — line integrals, surface integrals, and the theorems of Green, Stokes, and Gauss — as well as ordinary and partial differential equations at an intermediate level.
