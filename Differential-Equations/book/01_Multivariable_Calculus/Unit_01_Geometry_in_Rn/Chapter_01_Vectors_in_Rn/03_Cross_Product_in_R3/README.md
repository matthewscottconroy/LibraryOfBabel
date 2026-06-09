# Cross Product in R3

The dot product takes two vectors and produces a scalar that encodes angle information. But many geometric and physical problems require a third vector that is simultaneously perpendicular to two given vectors: the normal to a plane spanned by two directions, the torque produced by a force about an axis, the magnetic force on a moving charge. The cross product is the tool built for this purpose, and it exists in exactly the right form only in three dimensions.

## Definition

Given $\mathbf{u} = (u_1, u_2, u_3)$ and $\mathbf{v} = (v_1, v_2, v_3)$ in $\mathbb{R}^3$, their **cross product** is the vector

$$\mathbf{u} \times \mathbf{v} = \begin{vmatrix} \mathbf{i} & \mathbf{j} & \mathbf{k} \\ u_1 & u_2 & u_3 \\ v_1 & v_2 & v_3 \end{vmatrix} = (u_2 v_3 - u_3 v_2)\,\mathbf{i} - (u_1 v_3 - u_3 v_1)\,\mathbf{j} + (u_1 v_2 - u_2 v_1)\,\mathbf{k}.$$

Written in components:

$$\mathbf{u} \times \mathbf{v} = \bigl(u_2 v_3 - u_3 v_2,\; u_3 v_1 - u_1 v_3,\; u_1 v_2 - u_2 v_1\bigr).$$

The determinant notation is a mnemonic, not an actual determinant with vector entries, but it produces the correct formula and is universally used.

## Fundamental Properties

**Theorem.** The cross product satisfies the following:

1. **Perpendicularity:** $\mathbf{u} \times \mathbf{v}$ is orthogonal to both $\mathbf{u}$ and $\mathbf{v}$: $\mathbf{u} \cdot (\mathbf{u} \times \mathbf{v}) = 0$ and $\mathbf{v} \cdot (\mathbf{u} \times \mathbf{v}) = 0$.

2. **Antisymmetry:** $\mathbf{v} \times \mathbf{u} = -(\mathbf{u} \times \mathbf{v})$.

3. **Magnitude and area:** $\|\mathbf{u} \times \mathbf{v}\| = \|\mathbf{u}\|\|\mathbf{v}\|\sin\theta$, where $\theta \in [0,\pi]$ is the angle between $\mathbf{u}$ and $\mathbf{v}$. Equivalently, $\|\mathbf{u} \times \mathbf{v}\|$ equals the area of the parallelogram spanned by $\mathbf{u}$ and $\mathbf{v}$.

4. **Direction (right-hand rule):** $\mathbf{u} \times \mathbf{v}$ points in the direction that the right hand's fingers curl from $\mathbf{u}$ toward $\mathbf{v}$ when the thumb points away from the hand.

5. **Bilinearity:** The cross product is linear in each argument separately, i.e., $(\mathbf{u}_1 + \mathbf{u}_2) \times \mathbf{v} = \mathbf{u}_1 \times \mathbf{v} + \mathbf{u}_2 \times \mathbf{v}$ and $(c\mathbf{u}) \times \mathbf{v} = c(\mathbf{u} \times \mathbf{v})$.

**Proof of perpendicularity.** Direct computation: $\mathbf{u} \cdot (\mathbf{u} \times \mathbf{v}) = u_1(u_2 v_3 - u_3 v_2) + u_2(u_3 v_1 - u_1 v_3) + u_3(u_1 v_2 - u_2 v_1)$. Expanding: $u_1 u_2 v_3 - u_1 u_3 v_2 + u_2 u_3 v_1 - u_1 u_2 v_3 + u_1 u_3 v_2 - u_2 u_3 v_1 = 0$. Every term cancels.

**Proof of the magnitude formula.** Using the identity $\|\mathbf{u} \times \mathbf{v}\|^2 + (\mathbf{u}\cdot\mathbf{v})^2 = \|\mathbf{u}\|^2\|\mathbf{v}\|^2$ (verified by direct expansion of both sides), and substituting $\mathbf{u}\cdot\mathbf{v} = \|\mathbf{u}\|\|\mathbf{v}\|\cos\theta$:

$$\|\mathbf{u}\times\mathbf{v}\|^2 = \|\mathbf{u}\|^2\|\mathbf{v}\|^2 - \|\mathbf{u}\|^2\|\mathbf{v}\|^2\cos^2\theta = \|\mathbf{u}\|^2\|\mathbf{v}\|^2\sin^2\theta.$$

Since $\sin\theta \geq 0$ for $\theta \in [0,\pi]$, taking the square root gives $\|\mathbf{u}\times\mathbf{v}\| = \|\mathbf{u}\|\|\mathbf{v}\|\sin\theta$.

## The Cross Products of Standard Basis Vectors

The cross products of the standard basis vectors follow the cyclic rule:

$$\mathbf{i}\times\mathbf{j} = \mathbf{k}, \quad \mathbf{j}\times\mathbf{k} = \mathbf{i}, \quad \mathbf{k}\times\mathbf{i} = \mathbf{j},$$

and reversing the order changes the sign: $\mathbf{j}\times\mathbf{i} = -\mathbf{k}$, etc. Also $\mathbf{i}\times\mathbf{i} = \mathbf{0}$ (and similarly for $\mathbf{j}$ and $\mathbf{k}$), since parallel vectors span zero area.

## The Scalar Triple Product

The **scalar triple product** of three vectors is $\mathbf{u}\cdot(\mathbf{v}\times\mathbf{w})$. This equals the determinant

$$\mathbf{u}\cdot(\mathbf{v}\times\mathbf{w}) = \begin{vmatrix} u_1 & u_2 & u_3 \\ v_1 & v_2 & v_3 \\ w_1 & w_2 & w_3 \end{vmatrix}.$$

Its absolute value is the volume of the **parallelepiped** spanned by $\mathbf{u}$, $\mathbf{v}$, $\mathbf{w}$: the base is the parallelogram spanned by $\mathbf{v}$ and $\mathbf{w}$ (area $= \|\mathbf{v}\times\mathbf{w}\|$), and the height is the component of $\mathbf{u}$ perpendicular to the base (which equals $|\mathbf{u}\cdot\hat{n}|$ where $\hat{n} = (\mathbf{v}\times\mathbf{w})/\|\mathbf{v}\times\mathbf{w}\|$). The sign of the scalar triple product records whether the ordered triple $(\mathbf{u}, \mathbf{v}, \mathbf{w})$ is right-handed or left-handed.

Three vectors are **coplanar** if and only if their scalar triple product is zero, since zero volume means the parallelepiped has collapsed to a flat figure.

## What the Cross Product Is Not

The cross product is **not** associative: in general, $\mathbf{u}\times(\mathbf{v}\times\mathbf{w}) \neq (\mathbf{u}\times\mathbf{v})\times\mathbf{w}$. Instead, one has the **vector triple product identity**:

$$\mathbf{u}\times(\mathbf{v}\times\mathbf{w}) = (\mathbf{u}\cdot\mathbf{w})\mathbf{v} - (\mathbf{u}\cdot\mathbf{v})\mathbf{w}.$$

This identity (sometimes called the BAC-CAB rule) is extremely useful in vector calculus identities involving the curl.

The cross product is also specific to $\mathbb{R}^3$. There is no binary cross product in $\mathbb{R}^n$ for $n \neq 3$ (and $n \neq 7$, where a related but less used product exists). In higher dimensions, the role of the cross product is played by exterior products, which associate an $(n-2)$-vector to a pair of vectors in $\mathbb{R}^n$.

## Worked Example

Find a vector normal to the plane through the points $P = (1, 0, 0)$, $Q = (0, 2, 0)$, $R = (0, 0, 3)$.

Form two vectors in the plane: $\mathbf{u} = Q - P = (-1, 2, 0)$ and $\mathbf{v} = R - P = (-1, 0, 3)$.

$$\mathbf{n} = \mathbf{u}\times\mathbf{v} = \begin{vmatrix}\mathbf{i} & \mathbf{j} & \mathbf{k} \\ -1 & 2 & 0 \\ -1 & 0 & 3\end{vmatrix} = \mathbf{i}(2\cdot3 - 0\cdot0) - \mathbf{j}((-1)\cdot3 - 0\cdot(-1)) + \mathbf{k}((-1)\cdot0 - 2\cdot(-1))$$
$$= \mathbf{i}(6) - \mathbf{j}(-3) + \mathbf{k}(2) = (6, 3, 2).$$

Verify: $(-1,2,0)\cdot(6,3,2) = -6+6+0 = 0$ and $(-1,0,3)\cdot(6,3,2) = -6+0+6 = 0$. The plane equation is $6x + 3y + 2z = 6$ (substituting $P$: $6(1) = 6$, confirmed).

## Physical Applications

In mechanics, the **torque** produced by a force $\mathbf{F}$ about a pivot, when the force is applied at displacement $\mathbf{r}$ from the pivot, is $\boldsymbol{\tau} = \mathbf{r}\times\mathbf{F}$. The direction of $\boldsymbol{\tau}$ is the axis of rotation, and its magnitude $\|\mathbf{r}\|\|\mathbf{F}\|\sin\theta$ is greatest when the force is perpendicular to the lever arm.

In electromagnetism, the **Lorentz force** on a charge $q$ moving with velocity $\mathbf{v}$ in magnetic field $\mathbf{B}$ is $\mathbf{F} = q\mathbf{v}\times\mathbf{B}$, always perpendicular to both the velocity and the field.
