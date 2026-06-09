# Unit 3: Second-Order Linear Ordinary Differential Equations

Second-order linear ODEs occupy the center of gravity of the classical theory. They describe mechanical vibrations, electrical oscillations, quantum mechanical wave functions, and acoustic phenomena. Their solution theory is complete, elegant, and rich with connections to linear algebra and spectral theory.

## Why Second Order?

Newton's second law $F = ma$ is fundamentally a second-order equation in position: acceleration is the second derivative of position. The spring-mass-dashpot system, the pendulum, the RLC circuit, and the Euler-Bernoulli beam equation are all governed by second-order linear equations or systems of them. These physical origins motivated the development of the theory, and the mathematical results in turn illuminate the physics with great precision.

Second-order equations are also the simplest setting in which all the important structural phenomena of linear ODE theory appear: the two-dimensional solution space, the Wronskian, the method of undetermined coefficients, variation of parameters for systems of equations, and resonance. Higher-order linear equations extend this theory by dimension but introduce no fundamentally new phenomena.

## Organization

The unit is organized into four chapters. Chapter 1 develops the structural theory: the superposition principle, the Wronskian criterion for linear independence, Abel's formula, and reduction of order. These results are the analogs, for linear operators, of the basic linear algebra of vector spaces and bases.

Chapter 2 solves the constant-coefficient homogeneous equation $ay'' + by' + cy = 0$ completely by the characteristic equation. The three cases, real distinct roots, repeated roots, and complex conjugate roots, give qualitatively different solution behaviors: exponential growth/decay, critically damped behavior with a polynomial factor, and oscillations.

Chapter 3 addresses nonhomogeneous equations $ay'' + by' + cy = g(x)$. Two methods are developed: undetermined coefficients (for special forcing functions) and variation of parameters (for general forcing). The principle of superposition allows decomposition of complex forcing into simpler components.

Chapter 4 applies the theory to mechanical vibrations (free, damped, and forced) and to the RLC electrical circuit, developing the physics of resonance and beats and revealing the mathematical parallel between mechanical and electrical systems.

## Central Themes

The most important conceptual thread of this unit is the interplay between the algebraic structure of the solution space and the analytic behavior of solutions. The fact that solutions form a two-dimensional vector space is not just an abstract statement; it means that every solution is a linear combination of two fundamentally different behaviors (determined by the roots of the characteristic equation), and understanding those two behaviors gives complete information about all solutions.

The phenomenon of resonance, occurring when the natural frequency of the homogeneous equation matches the driving frequency of the forcing, illustrates how the structure of the solution space interacts with the forcing in a physically dramatic way.
