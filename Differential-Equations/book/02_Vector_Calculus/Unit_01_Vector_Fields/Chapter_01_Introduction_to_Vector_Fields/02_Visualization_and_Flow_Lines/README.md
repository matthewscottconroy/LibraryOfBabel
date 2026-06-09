# Visualization and Flow Lines

A formula for a vector field contains all the information about the field, but it does not make the field's behavior transparent. The field $\mathbf{F}(x,y) = (x - y)\,\mathbf{i} + (x + y)\,\mathbf{j}$ is perfectly well-defined, but to understand whether it spirals, radiates, or circulates requires either computation or a picture. This section develops two complementary visual tools: arrow diagrams and flow lines.

## Arrow Diagrams

The most direct way to visualize a vector field is to evaluate it at a grid of sample points and draw the resulting vectors as arrows anchored at those points. In practice, the vectors are often scaled down by a common factor to prevent overlap; the scaling does not change direction and does not distort the qualitative picture.

**Reading an arrow diagram.** The direction of each arrow shows which way the field points at that location. The length of each arrow (before scaling) represents the field's magnitude. Regions where arrows are long and densely aligned suggest strong, coherent flow; regions where arrows are short or point in conflicting directions suggest cancellation or near-zero magnitude.

**Example.** For the rotational field $\mathbf{F}(x,y) = -y\,\mathbf{i} + x\,\mathbf{j}$, all arrows are tangent to circles centered at the origin and point counterclockwise. The magnitude at $(x,y)$ is $\sqrt{x^2+y^2}$, so arrows grow longer with distance from the origin. For the gradient field $\mathbf{F} = \nabla(x^2 + y^2) = 2x\,\mathbf{i} + 2y\,\mathbf{j}$, all arrows point away from the origin and their length grows linearly with distance. These two fields are everywhere perpendicular to each other — a fact that is not coincidence but a consequence of the general orthogonality between a gradient field and the level curves of its potential.

## Flow Lines

Arrow diagrams give a static snapshot. A **flow line** (also called a field line, integral curve, or streamline) gives a dynamic picture: it is the path that a particle would follow if it moved with velocity equal to the vector field at each point.

**Formal definition.** Let $\mathbf{F}: D \to \mathbb{R}^2$ be a vector field. A flow line is a differentiable curve $\mathbf{r}(t) = (x(t), y(t))$ satisfying

$$\frac{d\mathbf{r}}{dt} = \mathbf{F}(\mathbf{r}(t)),$$

that is,

$$\frac{dx}{dt} = P(x(t), y(t)), \qquad \frac{dy}{dt} = Q(x(t), y(t)).$$

This is a system of autonomous ordinary differential equations. Each solution curve is one flow line; the collection of all flow lines is called the **phase portrait** of the field.

**Existence and uniqueness.** If $\mathbf{F}$ is $C^1$, then by the Picard-Lindelof theorem, through each point of $D$ there passes exactly one flow line (locally). Flow lines therefore partition the domain into non-intersecting curves. This non-intersection is not just a theorem but a geometric principle: if two flow lines crossed at a point $\mathbf{p}$, then $\mathbf{F}(\mathbf{p})$ would point in two directions simultaneously, which is impossible for a well-defined field.

## Computing Flow Lines

Finding flow lines reduces to solving the ODE system above. Depending on the field, this may be straightforward or require the techniques of later chapters.

**Example 1: Radial field.** For $\mathbf{F}(x,y) = x\,\mathbf{i} + y\,\mathbf{j}$, the system is

$$\frac{dx}{dt} = x, \qquad \frac{dy}{dt} = y.$$

These equations decouple: $x(t) = x_0 e^t$, $y(t) = y_0 e^t$. The flow line through $(x_0, y_0)$ is the ray $\{(x_0 e^t, y_0 e^t) : t \in \mathbb{R}\}$. Eliminating the parameter gives $y/x = y_0/x_0$ (constant), confirming that flow lines are rays through the origin.

**Example 2: Rotational field.** For $\mathbf{F}(x,y) = -y\,\mathbf{i} + x\,\mathbf{j}$, the system is

$$\frac{dx}{dt} = -y, \qquad \frac{dy}{dt} = x.$$

This can be written in matrix form as $\dot{\mathbf{r}} = A\mathbf{r}$ with $A = \begin{pmatrix} 0 & -1 \\ 1 & 0 \end{pmatrix}$. The eigenvalues of $A$ are $\pm i$, giving solutions $x(t) = r_0 \cos(t + \phi_0)$, $y(t) = r_0 \sin(t + \phi_0)$. Flow lines are circles centered at the origin, traversed counterclockwise.

**Example 3: Spiral field.** Consider $\mathbf{F}(x,y) = (x - y)\,\mathbf{i} + (x + y)\,\mathbf{j}$. The matrix is $A = \begin{pmatrix} 1 & -1 \\ 1 & 1 \end{pmatrix}$ with eigenvalues $1 \pm i$. Solutions spiral outward: $r(t) = r_0 e^t$, $\theta(t) = \theta_0 + t$. The flow lines are outward spirals, and the exponential growth reflects the fact that the real part of the eigenvalue is positive.

## Eliminating the Parameter

When finding flow lines by hand, it is sometimes more convenient to find a curve equation $y = g(x)$ directly rather than a parametrization. Dividing the two equations gives

$$\frac{dy}{dx} = \frac{Q(x,y)}{P(x,y)},$$

which is a first-order ODE in $x$ and $y$. Solving it (by separation of variables, integrating factor, or other techniques) yields the family of flow lines as level curves of some conserved quantity.

For the rotational field, $dy/dx = x/(-y)$ gives $y\,dy = -x\,dx$, so $x^2 + y^2 = C$ — confirming that flow lines are circles.

## Flow Lines versus Level Curves

For a gradient field $\mathbf{F} = \nabla f$, the flow lines are always perpendicular to the level curves $\{f = c\}$. This is because the gradient at any point is normal to the level surface through that point. The flow lines of $\nabla f$ are sometimes called **gradient lines** or **lines of steepest ascent**. This orthogonality is geometrically pleasing and computationally useful: if you know the level curves of $f$, you can sketch the flow lines without solving any ODE, simply by drawing curves that cross each level curve at right angles.

## Physical Interpretation

In fluid mechanics, $\mathbf{F}$ represents the velocity field of a fluid, and the flow lines are the actual paths of fluid particles (in steady, i.e., time-independent, flow). In electrostatics, the flow lines of the electric field $\mathbf{E}$ are the lines along which a positive test charge would accelerate. In heat conduction, the flow lines of the heat flux $\mathbf{q} = -k\nabla T$ are the paths along which heat energy propagates.

The convergence or divergence of flow lines signals compression or expansion of the fluid: lines that converge toward a region indicate a sink (fluid is accumulating or being absorbed); lines that diverge from a region indicate a source. This qualitative behavior is quantified precisely by the divergence operator, introduced in Chapter 2.

## Summary

Arrow diagrams and flow lines provide complementary pictures of a vector field. Arrow diagrams show direction and magnitude at individual points; flow lines trace the global structure of how the field propagates. Computing flow lines reduces to solving a first-order ODE system, and the qualitative behavior of solutions — spiraling, radiating, circulating — directly reflects the algebraic structure of the field. The tools developed here will be essential for understanding the integral theorems later in the module.
