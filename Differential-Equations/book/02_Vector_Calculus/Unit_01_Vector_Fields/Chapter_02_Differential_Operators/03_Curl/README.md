# Curl

Place a tiny paddle wheel in a flowing fluid and allow it to spin freely. If the fluid flow has a net rotational character near the wheel's location, the wheel will rotate; if the flow is purely translational or perfectly symmetric, the wheel will not spin. The **curl** of a vector field is the operator that measures exactly this tendency toward rotation: it captures the infinitesimal rotational behavior of the field at each point, producing a vector whose direction is the axis of rotation and whose magnitude gives the angular speed.

## Definition

Let $\mathbf{F} = P\,\mathbf{i} + Q\,\mathbf{j} + R\,\mathbf{k}$ be a $C^1$ vector field on $D \subseteq \mathbb{R}^3$. The **curl** of $\mathbf{F}$ is the vector field

$$\nabla \times \mathbf{F} = \begin{vmatrix} \mathbf{i} & \mathbf{j} & \mathbf{k} \\ \partial/\partial x & \partial/\partial y & \partial/\partial z \\ P & Q & R \end{vmatrix}$$

Expanding this determinant:

$$\nabla \times \mathbf{F} = \left(\frac{\partial R}{\partial y} - \frac{\partial Q}{\partial z}\right)\mathbf{i} - \left(\frac{\partial R}{\partial x} - \frac{\partial P}{\partial z}\right)\mathbf{j} + \left(\frac{\partial Q}{\partial x} - \frac{\partial P}{\partial y}\right)\mathbf{k}.$$

The determinant notation is a mnemonic, not a literal matrix determinant, since the first row contains unit vectors and the second contains operators. Nevertheless, it gives the correct formula when expanded along the first row.

## The Two-Dimensional Case

In two dimensions, $\mathbf{F} = P\,\mathbf{i} + Q\,\mathbf{j}$ (with $R = 0$ and no $z$-dependence), the curl reduces to a scalar (the $z$-component of the three-dimensional curl):

$$(\nabla \times \mathbf{F})_z = \frac{\partial Q}{\partial x} - \frac{\partial P}{\partial y}.$$

This quantity is precisely the expression that appears in the exactness condition for conservative fields: $\mathbf{F}$ is conservative (on a simply connected domain) if and only if $\partial Q/\partial x - \partial P/\partial y = 0$, i.e., $\nabla \times \mathbf{F} = \mathbf{0}$. The two-dimensional curl thus measures "how far $\mathbf{F}$ is from being conservative."

## Geometric Interpretation

The curl at a point $\mathbf{p}$ can be characterized as follows. For a small disk of area $A$ with unit normal $\hat{\mathbf{n}}$ centered at $\mathbf{p}$ with boundary circle $C$:

$$(\nabla \times \mathbf{F})(\mathbf{p}) \cdot \hat{\mathbf{n}} = \lim_{A \to 0} \frac{1}{A}\oint_C \mathbf{F} \cdot d\mathbf{r}.$$

The line integral $\oint_C \mathbf{F} \cdot d\mathbf{r}$ is the **circulation** of $\mathbf{F}$ around $C$. So the component of the curl in the $\hat{\mathbf{n}}$ direction equals the circulation per unit area around an infinitesimal loop perpendicular to $\hat{\mathbf{n}}$.

This characterization shows that curl is independent of coordinates — it measures physical rotation, regardless of the coordinate system used to compute it.

**Irrotational fields.** A field with $\nabla \times \mathbf{F} = \mathbf{0}$ is called **irrotational**. The curl is zero at every point, meaning there is no net rotation anywhere. Conservative fields are always irrotational (on any domain); on simply connected domains, the converse holds too.

## Worked Examples

**Example 1: Rigid rotation.** Consider the velocity field of a rigid body rotating with angular velocity $\omega$ about the $z$-axis:

$$\mathbf{v}(x,y,z) = -\omega y\,\mathbf{i} + \omega x\,\mathbf{j}.$$

$$\nabla \times \mathbf{v} = \left(\frac{\partial 0}{\partial y} - \frac{\partial(\omega x)}{\partial z}\right)\mathbf{i} - \left(\frac{\partial 0}{\partial x} - \frac{\partial(-\omega y)}{\partial z}\right)\mathbf{j} + \left(\frac{\partial(\omega x)}{\partial x} - \frac{\partial(-\omega y)}{\partial y}\right)\mathbf{k} = (0)\mathbf{i} - (0)\mathbf{j} + (2\omega)\mathbf{k} = 2\omega\,\mathbf{k}.$$

The curl is constant, pointing along the rotation axis, with magnitude $2\omega$. If $\boldsymbol{\omega} = \omega\,\mathbf{k}$ is the angular velocity vector, then $\nabla \times \mathbf{v} = 2\boldsymbol{\omega}$. This is not a coincidence: for any rigid rotation $\mathbf{v} = \boldsymbol{\omega} \times \mathbf{r}$, one can show $\nabla \times \mathbf{v} = 2\boldsymbol{\omega}$.

**Example 2: Irrotational field.** Let $\mathbf{F}(x,y,z) = (2xz)\,\mathbf{i} + (y^2)\,\mathbf{j} + (x^2)\,\mathbf{k}$.

$$\nabla \times \mathbf{F} = \left(\frac{\partial(x^2)}{\partial y} - \frac{\partial(y^2)}{\partial z}\right)\mathbf{i} - \left(\frac{\partial(x^2)}{\partial x} - \frac{\partial(2xz)}{\partial z}\right)\mathbf{j} + \left(\frac{\partial(y^2)}{\partial x} - \frac{\partial(2xz)}{\partial y}\right)\mathbf{k}$$

$$= (0 - 0)\mathbf{i} - (2x - 2x)\mathbf{j} + (0 - 0)\mathbf{k} = \mathbf{0}.$$

The curl is zero, confirming this field is irrotational. (You can verify it is conservative by finding the potential $f = x^2 z + y^3/3$.)

**Example 3: Non-trivial curl.** Let $\mathbf{F} = y\,\mathbf{i} - x\,\mathbf{j} + z\,\mathbf{k}$.

$$\nabla \times \mathbf{F} = \left(\frac{\partial z}{\partial y} - \frac{\partial(-x)}{\partial z}\right)\mathbf{i} - \left(\frac{\partial z}{\partial x} - \frac{\partial y}{\partial z}\right)\mathbf{j} + \left(\frac{\partial(-x)}{\partial x} - \frac{\partial y}{\partial y}\right)\mathbf{k} = (0)\mathbf{i} - (0)\mathbf{j} + (-1-1)\mathbf{k} = -2\,\mathbf{k}.$$

The curl points downward along the $z$-axis with constant magnitude 2. The field circulates clockwise when viewed from above.

## Curl and Angular Velocity: Deeper Connection

For any $C^1$ vector field, define the "angular velocity" at a point as the axis and rate of rotation that a small rigid body would acquire if embedded in the field. The curl gives exactly twice this angular velocity. This is why $\nabla \times \mathbf{v} = 2\boldsymbol{\omega}$ for rigid rotation: the factor of 2 arises from the antisymmetry in the curl formula.

## Curl of a Gradient is Zero

**Theorem.** For any $C^2$ scalar field $f$,

$$\nabla \times (\nabla f) = \mathbf{0}.$$

**Proof.** Computing directly, the $\mathbf{k}$-component of $\nabla \times (\nabla f)$ is

$$\frac{\partial}{\partial x}\left(\frac{\partial f}{\partial y}\right) - \frac{\partial}{\partial y}\left(\frac{\partial f}{\partial x}\right) = \frac{\partial^2 f}{\partial x\,\partial y} - \frac{\partial^2 f}{\partial y\,\partial x} = 0$$

by Clairaut's theorem (mixed partials commute for $C^2$ functions). The other components vanish similarly.

This theorem gives the necessary condition for conservativity: if $\mathbf{F} = \nabla f$, then $\nabla \times \mathbf{F} = \nabla \times (\nabla f) = \mathbf{0}$.

## Curl in Cylindrical Coordinates

In cylindrical coordinates, the curl is more complex due to scale factors. For $\mathbf{F} = F_r\,\hat{\mathbf{r}} + F_\theta\,\hat{\boldsymbol{\theta}} + F_z\,\hat{\mathbf{k}}$:

$$\nabla \times \mathbf{F} = \left(\frac{1}{r}\frac{\partial F_z}{\partial \theta} - \frac{\partial F_\theta}{\partial z}\right)\hat{\mathbf{r}} + \left(\frac{\partial F_r}{\partial z} - \frac{\partial F_z}{\partial r}\right)\hat{\boldsymbol{\theta}} + \frac{1}{r}\left(\frac{\partial(rF_\theta)}{\partial r} - \frac{\partial F_r}{\partial \theta}\right)\hat{\mathbf{k}}.$$

## Summary

The curl measures the rotational tendency of a vector field at each point. It is a vector: its direction indicates the axis of rotation, and its magnitude gives the angular speed (times two). Conservative fields are irrotational ($\nabla \times \mathbf{F} = \mathbf{0}$); on simply connected domains this is also sufficient for conservativity. The curl of a gradient is always zero. In Unit 4, Stokes' Theorem will show that the circulation of a field around a closed curve equals the flux of the curl through any surface bounded by that curve — making the curl the bridge between local rotation and global circulation.
