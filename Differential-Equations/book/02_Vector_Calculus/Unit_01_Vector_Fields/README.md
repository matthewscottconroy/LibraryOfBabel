# Unit 1: Vector Fields and Differential Operators

A vector field is the natural mathematical object for describing any physical situation in which each point in space is associated with a direction and a magnitude: the velocity of air at each location in a room, the force that gravity exerts on a test mass placed at each point in space, the electric field surrounding a charged particle. Before we can integrate such objects — before we can compute work done, flux through a surface, or circulation around a loop — we need a precise understanding of what vector fields are and how they behave under differentiation.

## Unit Overview

This unit develops the foundational vocabulary and tools that will be used throughout the rest of the module. It consists of two chapters.

**Chapter 1: Introduction to Vector Fields** establishes the basic definition and builds geometric intuition. A vector field on a region $D \subseteq \mathbb{R}^n$ is a function $\mathbf{F}: D \to \mathbb{R}^n$ that assigns a vector to each point. In two dimensions we write $\mathbf{F}(x, y) = P(x,y)\,\mathbf{i} + Q(x,y)\,\mathbf{j}$; in three dimensions, $\mathbf{F}(x, y, z) = P\,\mathbf{i} + Q\,\mathbf{j} + R\,\mathbf{k}$. The chapter covers visualization via arrow diagrams and flow lines, introduces the special class of conservative vector fields (those that arise as gradients of scalar functions), and develops the notion of a potential function. Conservative fields are particularly tractable: the work done against them is path-independent, and computing line integrals reduces to evaluating a potential at two endpoints.

**Chapter 2: Differential Operators** introduces the three operators — gradient, divergence, and curl — that form the backbone of vector calculus. These are collected under the del operator $\nabla$, which acts differently depending on what it is applied to and how. The gradient $\nabla f$ of a scalar field $f$ is a vector field pointing in the direction of most rapid increase. The divergence $\nabla \cdot \mathbf{F}$ of a vector field is a scalar measuring the rate at which the field spreads from a point. The curl $\nabla \times \mathbf{F}$ is a vector measuring the rotational tendency of the field. The Laplacian $\nabla^2 f = \nabla \cdot \nabla f$ is a scalar operator of central importance in physics: a function satisfying $\nabla^2 f = 0$ is called harmonic and arises throughout electrostatics and fluid mechanics.

## Why Differential Operators Matter

The power of the del notation comes partly from the algebraic identities it satisfies. These identities — $\nabla \times (\nabla f) = \mathbf{0}$, $\nabla \cdot (\nabla \times \mathbf{F}) = 0$, and many others — are not mere curiosities. They encode deep structural facts about vector fields. For instance, the identity $\nabla \times (\nabla f) = \mathbf{0}$ says that the curl of any gradient is zero, which is one direction of the characterization of conservative fields. The identity $\nabla \cdot (\nabla \times \mathbf{F}) = 0$ says that divergence-free fields can sometimes be expressed as curls — a fact that underlies the theory of magnetic vector potentials in electrodynamics.

Fluency with these operators and their identities is not optional background. The integral theorems of Units 2 through 4 are statements about these operators, and the partial differential equations you will encounter in later modules are written entirely in this language.

## Prerequisites and Goals

You should arrive at this unit comfortable with partial derivatives and with the algebra of vectors in $\mathbb{R}^3$ (dot product, cross product, magnitude). By the end of the unit, you should be able to:

- Sketch or interpret arrow diagrams of vector fields in two dimensions.
- Determine whether a given vector field is conservative, and if so find its potential function.
- Compute the gradient of a scalar field, the divergence and curl of a vector field, and the Laplacian of a scalar field.
- Apply del algebra to prove or verify vector identities.
- Recognize the physical meaning of divergence and curl in the context of fluid flow and electromagnetic fields.
