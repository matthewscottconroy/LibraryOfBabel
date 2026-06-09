# Unit 1: Introduction and Classification of PDEs

Before solving any PDE, you must understand what kind of equation you are dealing with. The classification of PDEs into elliptic, parabolic, and hyperbolic types is not a bureaucratic exercise — it determines which solution methods are appropriate, what kinds of boundary or initial conditions make the problem well-posed, and what qualitative behavior to expect from solutions. A method that works beautifully for one class can be utterly wrong for another, and the failure mode is often not an error message but a physically meaningless or numerically unstable answer.

## What This Unit Covers

This unit builds the conceptual and technical foundation for everything that follows. It consists of three chapters.

**Chapter 1: Basic Concepts** defines PDEs precisely, discusses order and linearity, and introduces the central question of well-posedness. A PDE problem is well-posed in the sense of Hadamard if it has a unique solution that depends continuously on the data. This is not merely a theoretical nicety — it is the condition under which computation and physical modeling are meaningful. Ill-posed problems, where small perturbations in data can produce large changes in solutions, require completely different analytical machinery.

**Chapter 2: Classification of Second-Order PDEs** develops the discriminant-based classification of second-order linear PDEs in two variables, reduces equations to canonical form via coordinate changes, and explains the physical meaning of each type. The general second-order linear PDE in two independent variables has the form

$$Au_{xx} + Bu_{xy} + Cu_{yy} + Du_x + Eu_y + Fu = G,$$

and the sign of the discriminant $B^2 - 4AC$ determines whether the equation is hyperbolic ($B^2 - 4AC > 0$), parabolic ($B^2 - 4AC = 0$), or elliptic ($B^2 - 4AC < 0$). Each type has a canonical form: the wave equation, the heat equation, and Laplace's equation respectively. The chapter also introduces the concept of characteristic curves, which are the curves along which discontinuities can propagate and along which the Cauchy problem may fail to have a unique solution.

**Chapter 3: Boundary and Initial Conditions** surveys the different types of auxiliary conditions that supplement a PDE to define a complete problem. Dirichlet conditions specify the solution value on the boundary; Neumann conditions specify the normal derivative; Robin (or mixed) conditions are linear combinations of the two. The chapter examines when each type is appropriate for each class of PDE, and introduces the Cauchy problem — specifying both function values and normal derivatives on a surface — which is the natural initial value problem for hyperbolic equations. Physical interpretations from heat conduction, fluid flow, and electrostatics ground the abstract formulations.

## Central Theme

The unifying theme of this unit is that the mathematical structure of a PDE reflects the physical phenomenon it models. Elliptic equations arise in steady-state problems where equilibrium is reached; they require boundary conditions all around the domain. Hyperbolic equations describe wave propagation, where information travels along characteristics at finite speed; they require initial conditions that specify state and velocity. Parabolic equations describe diffusion, which is in some sense intermediate: information propagates instantaneously (at infinite speed, formally), but the equation has a definite direction of time.

Recognizing these distinctions transforms the study of PDEs from a collection of ad hoc techniques into a coherent theory.
