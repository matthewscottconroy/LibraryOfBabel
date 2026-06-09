# Definition and Examples of Vector Fields

Suppose you want to describe the gravitational force acting on a unit mass placed at an arbitrary point in space near a planet. At each point $\mathbf{r} = (x, y, z)$, the force has a specific direction (toward the planet's center) and a specific magnitude (proportional to $1/|\mathbf{r}|^2$). Capturing this assignment — a vector for each point — requires a new kind of mathematical object: a vector field. This section defines vector fields precisely, establishes the notation used throughout the module, and develops geometric intuition through a sequence of worked examples.

## Definition

Let $D$ be an open subset of $\mathbb{R}^n$ (we work primarily with $n = 2$ or $n = 3$). A **vector field** on $D$ is a function

$$\mathbf{F}: D \to \mathbb{R}^n$$

that assigns to each point $\mathbf{p} \in D$ a vector $\mathbf{F}(\mathbf{p}) \in \mathbb{R}^n$.

In two dimensions, a vector field is typically written as

$$\mathbf{F}(x, y) = P(x, y)\,\mathbf{i} + Q(x, y)\,\mathbf{j},$$

where $P$ and $Q$ are scalar-valued functions called the **component functions** of $\mathbf{F}$. In three dimensions,

$$\mathbf{F}(x, y, z) = P(x, y, z)\,\mathbf{i} + Q(x, y, z)\,\mathbf{j} + R(x, y, z)\,\mathbf{k}.$$

We say $\mathbf{F}$ is **continuous** (resp. **differentiable**, $C^1$, etc.) if each of its component functions is continuous (resp. differentiable, $C^1$, etc.). Unless stated otherwise, all vector fields in this module are assumed to be at least $C^1$.

## Fundamental Examples

Working through concrete examples carefully is not a preliminary chore before reaching the "real" material — it is the substance of learning to think in vector fields.

### Example 1: Constant Field

The simplest vector field is constant: $\mathbf{F}(x, y) = \mathbf{i} + 2\mathbf{j}$ for all $(x, y)$. Every point is assigned the same vector. This models, approximately, the gravitational field near the earth's surface, where the direction and magnitude of $g$ are nearly constant over small regions.

### Example 2: Radial Fields

Define $\mathbf{F}(x, y) = x\,\mathbf{i} + y\,\mathbf{j}$. At each point $(x, y)$, the vector field equals the position vector itself. The vectors point directly away from the origin, and their magnitude $|\mathbf{F}| = \sqrt{x^2 + y^2}$ increases linearly with distance. This is the archetypal **outward radial field**.

More generally, radial fields in $\mathbb{R}^3$ have the form

$$\mathbf{F}(\mathbf{r}) = f(|\mathbf{r}|)\,\hat{\mathbf{r}},$$

where $f$ is a scalar function of distance and $\hat{\mathbf{r}} = \mathbf{r}/|\mathbf{r}|$ is the unit radial vector. The gravitational field of a point mass $M$ at the origin is

$$\mathbf{F}(\mathbf{r}) = -\frac{GM}{|\mathbf{r}|^2}\,\hat{\mathbf{r}} = -\frac{GM}{|\mathbf{r}|^3}\,\mathbf{r},$$

where the minus sign indicates attraction toward the origin. This field is defined on $\mathbb{R}^3 \setminus \{\mathbf{0}\}$ — the origin is excluded because the force is singular there.

### Example 3: Rotational Field

Define $\mathbf{F}(x, y) = -y\,\mathbf{i} + x\,\mathbf{j}$. At any point $(x, y)$, the vector $(-y, x)$ is perpendicular to the position vector $(x, y)$ (since their dot product is $-xy + yx = 0$) and has the same magnitude $\sqrt{x^2 + y^2}$. The field therefore rotates counterclockwise around the origin, with speed proportional to distance. This is the velocity field of a rigid body rotating about the $z$-axis with unit angular velocity.

### Example 4: Gradient Field

Given a scalar function $f(x, y) = x^2 + y^2$, its gradient is

$$\nabla f = \frac{\partial f}{\partial x}\,\mathbf{i} + \frac{\partial f}{\partial y}\,\mathbf{j} = 2x\,\mathbf{i} + 2y\,\mathbf{j}.$$

This is precisely twice the outward radial field of Example 2. The gradient field points in the direction of steepest increase of $f$, which here is directly away from the origin (where $f$ has its minimum). The level curves of $f$ are circles, and the gradient vectors are everywhere perpendicular to these circles — a general fact about gradient fields.

### Example 5: Inverse-Square Law in Three Dimensions

The electric field of a point charge $q$ at the origin is

$$\mathbf{E}(\mathbf{r}) = \frac{q}{4\pi\varepsilon_0}\cdot\frac{\mathbf{r}}{|\mathbf{r}|^3}.$$

This field is radial and repulsive for positive $q$. It turns out to be a gradient field (conservative), with potential function

$$f(\mathbf{r}) = -\frac{q}{4\pi\varepsilon_0\,|\mathbf{r}|}.$$

Verifying this requires computing $\nabla(1/|\mathbf{r}|)$, which is a calculation every student of vector calculus should carry out at least once: with $r = \sqrt{x^2+y^2+z^2}$,

$$\frac{\partial}{\partial x}\left(\frac{1}{r}\right) = -\frac{x}{r^3},$$

and similarly for $y$ and $z$, giving $\nabla(1/r) = -\mathbf{r}/r^3$.

## Scalar Fields versus Vector Fields

A **scalar field** is simply a function $f: D \to \mathbb{R}$ — it assigns a number (not a vector) to each point. Temperature, pressure, and potential energy are scalar fields. The gradient operator turns scalar fields into vector fields. Divergence and curl (introduced later) turn vector fields into scalar and vector fields, respectively. Keeping track of the type of object you are working with at each step — scalar or vector — prevents many errors.

## Notation and Conventions

Throughout this module, boldface letters denote vectors and vector fields: $\mathbf{F}$, $\mathbf{v}$, $\mathbf{r}$. Ordinary letters denote scalars: $f$, $P$, $Q$, $R$. The standard basis vectors in $\mathbb{R}^3$ are $\mathbf{i} = (1,0,0)$, $\mathbf{j} = (0,1,0)$, $\mathbf{k} = (0,0,1)$; some texts use $\mathbf{e}_1, \mathbf{e}_2, \mathbf{e}_3$.

The domain $D$ of a vector field matters. The gravitational field $\mathbf{F} = -GM\mathbf{r}/|\mathbf{r}|^3$ is perfectly smooth on $\mathbb{R}^3 \setminus \{\mathbf{0}\}$, but the presence of the puncture at the origin has topological consequences that affect the existence of potential functions and the behavior of integrals. We will return to this point when discussing simply connected regions.

## Summary

A vector field assigns a vector to each point in its domain. The most important families to internalize are: constant fields, radial fields, rotational fields, and gradient fields. Each has a distinctive geometric signature — arrow direction and length pattern — that should be recognizable immediately from the formula. In the sections that follow, we build the tools needed to analyze, integrate, and classify these fields.
