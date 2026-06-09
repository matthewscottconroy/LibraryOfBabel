# Chapter 1: Introduction to Vector Fields

Consider the wind. At every point above the earth's surface, the air moves with some speed and in some direction. This situation — a direction and magnitude associated with each point in a region of space — is precisely what a vector field captures. The same structure appears in the force exerted by gravity or by an electric charge, in the flow of heat through a solid, and in the velocity of a fluid in motion. Vector fields are the fundamental objects of multivariable calculus, and understanding them well is prerequisite to everything that follows.

## Chapter Overview

This chapter builds the conceptual and computational foundation for vector fields. It proceeds through four closely linked sections.

**Section 1: Definition and Examples** gives the precise mathematical definition of a vector field and works through a range of concrete examples in two and three dimensions. Special attention is paid to physically motivated examples — gravitational fields, radial fields, rotational fields — since these will recur throughout the module and serve as test cases for every new concept introduced.

**Section 2: Visualization and Flow Lines** addresses how we see vector fields. An arrow diagram attaches a scaled vector to a grid of sample points, giving a coarse but informative picture. More informative still are flow lines (also called field lines or integral curves): curves whose tangent vectors at every point agree with the vector field at that point. These are the curves that a particle would trace if carried along by the field, and finding them reduces to solving a system of differential equations — an early and important connection to the broader course.

**Section 3: Conservative Fields** introduces one of the most important structural distinctions in vector calculus. A vector field $\mathbf{F}$ is conservative if there exists a scalar function $f$ such that $\mathbf{F} = \nabla f$. Conservative fields enjoy a special geometric property: their flow lines are orthogonal to the level curves (in two dimensions) or level surfaces (in three dimensions) of the potential function $f$. More importantly for computation, line integrals of conservative fields depend only on the endpoints of the path — a fact that drastically simplifies calculation. The criterion for conservativity in two dimensions involves checking whether $\partial P / \partial y = \partial Q / \partial x$; the analogous three-dimensional condition involves the curl.

**Section 4: Potential Functions** develops systematic methods for finding the potential function $f$ when $\mathbf{F}$ is known to be conservative. The procedure is a structured integration that recovers $f$ from its partial derivatives, and it requires careful bookkeeping to ensure consistency across components.

## The Geometric Picture

Perhaps the most important skill this chapter develops is the ability to move between the formula for a vector field and its geometric behavior. Given $\mathbf{F}(x,y) = -y\,\mathbf{i} + x\,\mathbf{j}$, can you see immediately that the field rotates counterclockwise and that each vector is perpendicular to the position vector? Given the gravitational field $\mathbf{F}(\mathbf{r}) = -GM/|\mathbf{r}|^2 \cdot \hat{\mathbf{r}}$, can you see that vectors point toward the origin with magnitude that decays as the square of the distance?

This geometric fluency pays dividends when we pass to integration. Knowing that a field circulates in loops, for instance, suggests immediately that the curl will be nonzero, that circulation integrals will be nonzero, and that the field cannot be conservative.

## Connection to Differential Equations

Flow lines deserve special emphasis as a connection to the rest of the course. Given a vector field $\mathbf{F}(x, y) = (P(x,y),\, Q(x,y))$, a flow line is a parametric curve $\mathbf{r}(t) = (x(t), y(t))$ satisfying

$$\frac{d\mathbf{r}}{dt} = \mathbf{F}(\mathbf{r}(t)).$$

This is a system of autonomous ordinary differential equations. The qualitative behavior of its solutions — whether they spiral, converge, diverge, circulate — is the subject of dynamical systems theory, and it is described entirely in terms of properties of the vector field: equilibria correspond to zeros of $\mathbf{F}$, stability is governed by the Jacobian matrix at those zeros, and global behavior is constrained by topological properties of the field.
