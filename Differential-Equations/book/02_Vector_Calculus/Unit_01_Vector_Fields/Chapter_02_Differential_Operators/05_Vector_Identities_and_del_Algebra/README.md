# Vector Identities and del Algebra

The del operator $\nabla$ obeys an algebraic system that parallels ordinary calculus — product rules, chain rules, and commutativity properties — while simultaneously respecting the distinct natures of scalar and vector fields. These identities are not merely computational shortcuts. They encode the deep structural relationships between gradient, divergence, and curl, and they are the primary tools used in deriving the integral theorems of Units 2 through 4 and in simplifying the equations of mathematical physics.

This section presents the principal identities systematically, with proofs of the most important ones and worked applications of the rest.

## Notation

Throughout, $f$ and $g$ denote $C^2$ scalar fields; $\mathbf{F}$ and $\mathbf{G}$ denote $C^2$ vector fields. All identities hold on suitable domains (open subsets of $\mathbb{R}^3$).

## Identities Involving Gradient

**Product rule for gradient:**

$$\nabla(fg) = f\nabla g + g\nabla f.$$

*Proof.* The $x$-component is $\partial(fg)/\partial x = f\,\partial g/\partial x + g\,\partial f/\partial x = f(\nabla g)_x + g(\nabla f)_x$. Similarly for $y$ and $z$.

**Gradient of a dot product:**

$$\nabla(\mathbf{F} \cdot \mathbf{G}) = (\mathbf{F} \cdot \nabla)\mathbf{G} + (\mathbf{G} \cdot \nabla)\mathbf{F} + \mathbf{F} \times (\nabla \times \mathbf{G}) + \mathbf{G} \times (\nabla \times \mathbf{F}).$$

This identity is more involved; it is proved by expanding both sides in components.

**Chain rule:**

$$\nabla(h \circ f) = h'(f)\nabla f$$

for a $C^1$ scalar function $h: \mathbb{R} \to \mathbb{R}$.

## Identities Involving Divergence

**Linearity:**

$$\nabla \cdot (\mathbf{F} + \mathbf{G}) = \nabla \cdot \mathbf{F} + \nabla \cdot \mathbf{G}, \qquad \nabla \cdot (c\mathbf{F}) = c\,\nabla \cdot \mathbf{F}.$$

**Product rule for divergence:**

$$\nabla \cdot (f\mathbf{F}) = f\,\nabla \cdot \mathbf{F} + \mathbf{F} \cdot \nabla f.$$

*Proof.* $\nabla \cdot (f\mathbf{F}) = \partial(fP)/\partial x + \partial(fQ)/\partial y + \partial(fR)/\partial z$. Expanding each term: $\partial(fP)/\partial x = f\,\partial P/\partial x + P\,\partial f/\partial x$. Summing: $f(\partial P/\partial x + \partial Q/\partial y + \partial R/\partial z) + (P\,\partial f/\partial x + Q\,\partial f/\partial y + R\,\partial f/\partial z) = f\,\nabla\cdot\mathbf{F} + \mathbf{F}\cdot\nabla f$.

**Divergence of a cross product:**

$$\nabla \cdot (\mathbf{F} \times \mathbf{G}) = \mathbf{G} \cdot (\nabla \times \mathbf{F}) - \mathbf{F} \cdot (\nabla \times \mathbf{G}).$$

## The Two Vanishing Identities

These are the most important identities in del algebra — they encode topological information about the structure of vector fields.

**Identity 1: Curl of a gradient is zero.**

$$\nabla \times (\nabla f) = \mathbf{0}.$$

*Proof.* The $\mathbf{i}$-component is $\partial^2 f/\partial y\,\partial z - \partial^2 f/\partial z\,\partial y = 0$ by Clairaut's theorem. The $\mathbf{j}$ and $\mathbf{k}$ components vanish similarly.

**Consequence.** Any conservative field $\mathbf{F} = \nabla f$ is irrotational: $\nabla \times \mathbf{F} = \mathbf{0}$.

**Identity 2: Divergence of a curl is zero.**

$$\nabla \cdot (\nabla \times \mathbf{F}) = \mathbf{0}.$$

*Proof.* Let $\mathbf{G} = \nabla \times \mathbf{F}$. Then

$$\nabla \cdot \mathbf{G} = \frac{\partial G_x}{\partial x} + \frac{\partial G_y}{\partial y} + \frac{\partial G_z}{\partial z} = \frac{\partial}{\partial x}\left(\frac{\partial R}{\partial y} - \frac{\partial Q}{\partial z}\right) + \frac{\partial}{\partial y}\left(\frac{\partial P}{\partial z} - \frac{\partial R}{\partial x}\right) + \frac{\partial}{\partial z}\left(\frac{\partial Q}{\partial x} - \frac{\partial P}{\partial y}\right).$$

Expanding and collecting terms, each second partial derivative appears once with a plus sign and once with a minus sign, yielding zero (by Clairaut's theorem for $C^2$ fields).

**Consequence.** Any field of the form $\mathbf{F} = \nabla \times \mathbf{G}$ is automatically solenoidal: $\nabla \cdot \mathbf{F} = 0$.

These two identities form a chain:

$$\text{scalar } f \xrightarrow{\nabla} \text{vector } \nabla f \xrightarrow{\nabla \times} \mathbf{0}, \qquad \text{vector } \mathbf{G} \xrightarrow{\nabla \times} \text{vector } \nabla \times \mathbf{G} \xrightarrow{\nabla \cdot} 0.$$

In the language of differential forms, these are the statement $d \circ d = 0$ (the exterior derivative of an exterior derivative is zero), which will be developed in Unit 4, Chapter 4.

## Identities Involving Curl

**Linearity:**

$$\nabla \times (\mathbf{F} + \mathbf{G}) = \nabla \times \mathbf{F} + \nabla \times \mathbf{G}.$$

**Product rule for curl:**

$$\nabla \times (f\mathbf{F}) = f(\nabla \times \mathbf{F}) + (\nabla f) \times \mathbf{F}.$$

**Curl of a cross product:**

$$\nabla \times (\mathbf{F} \times \mathbf{G}) = \mathbf{F}(\nabla \cdot \mathbf{G}) - \mathbf{G}(\nabla \cdot \mathbf{F}) + (\mathbf{G} \cdot \nabla)\mathbf{F} - (\mathbf{F} \cdot \nabla)\mathbf{G}.$$

Here $(\mathbf{G} \cdot \nabla)\mathbf{F}$ denotes the directional derivative of $\mathbf{F}$ in the direction of $\mathbf{G}$:

$$(\mathbf{G} \cdot \nabla)\mathbf{F} = \left(G_x\frac{\partial P}{\partial x} + G_y\frac{\partial P}{\partial y} + G_z\frac{\partial P}{\partial z}\right)\mathbf{i} + \cdots$$

## Second-Order Identities

**Curl of a curl:**

$$\nabla \times (\nabla \times \mathbf{F}) = \nabla(\nabla \cdot \mathbf{F}) - \nabla^2\mathbf{F}.$$

This is perhaps the most practically important second-order identity. In free space, Maxwell's equations give $\nabla \cdot \mathbf{E} = 0$ (in the absence of charges). Then:

$$\nabla \times (\nabla \times \mathbf{E}) = \nabla(\underbrace{\nabla \cdot \mathbf{E}}_{=0}) - \nabla^2\mathbf{E} = -\nabla^2\mathbf{E}.$$

Faraday's law gives $\nabla \times \mathbf{E} = -\partial\mathbf{B}/\partial t$, and Ampere's law gives $\nabla \times \mathbf{B} = \mu_0\varepsilon_0\,\partial\mathbf{E}/\partial t$. Combining:

$$-\nabla^2\mathbf{E} = -\frac{\partial}{\partial t}(\nabla \times \mathbf{B}) = -\mu_0\varepsilon_0\frac{\partial^2\mathbf{E}}{\partial t^2},$$

yielding the wave equation $\nabla^2\mathbf{E} = \mu_0\varepsilon_0\,\partial^2\mathbf{E}/\partial t^2$, from which the speed of light $c = 1/\sqrt{\mu_0\varepsilon_0}$ follows.

**Green's first identity:**

$$\int_D g\,\nabla^2 f\,dV = \oint_{\partial D} g\,\nabla f \cdot d\mathbf{S} - \int_D \nabla g \cdot \nabla f\,dV.$$

This follows from applying the Divergence Theorem to $\mathbf{F} = g\,\nabla f$ using the product rule $\nabla \cdot (g\nabla f) = g\,\nabla^2 f + \nabla g \cdot \nabla f$.

**Green's second identity (Green's theorem in the symmetric form):**

$$\int_D (g\,\nabla^2 f - f\,\nabla^2 g)\,dV = \oint_{\partial D} (g\,\nabla f - f\,\nabla g)\cdot d\mathbf{S}.$$

Subtract Green's first identity with $f$ and $g$ interchanged.

## Summary Table

| Identity | Formula |
|---|---|
| Grad of product | $\nabla(fg) = f\nabla g + g\nabla f$ |
| Div of scalar times vector | $\nabla\cdot(f\mathbf{F}) = f\,\nabla\cdot\mathbf{F} + \mathbf{F}\cdot\nabla f$ |
| Curl of scalar times vector | $\nabla\times(f\mathbf{F}) = f\,\nabla\times\mathbf{F} + (\nabla f)\times\mathbf{F}$ |
| Div of cross product | $\nabla\cdot(\mathbf{F}\times\mathbf{G}) = \mathbf{G}\cdot(\nabla\times\mathbf{F}) - \mathbf{F}\cdot(\nabla\times\mathbf{G})$ |
| Curl of gradient | $\nabla\times(\nabla f) = \mathbf{0}$ |
| Div of curl | $\nabla\cdot(\nabla\times\mathbf{F}) = 0$ |
| Curl of curl | $\nabla\times(\nabla\times\mathbf{F}) = \nabla(\nabla\cdot\mathbf{F}) - \nabla^2\mathbf{F}$ |

These identities form the complete grammar of vector calculus. Every derivation in Units 2 through 4 — and in the physics that follows — draws on some combination of them.
