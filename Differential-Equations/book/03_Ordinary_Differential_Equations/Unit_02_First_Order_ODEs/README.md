# Unit 2: First-Order Ordinary Differential Equations

First-order ODEs occupy a central place in the subject both because they arise constantly in applications and because the full range of solution techniques, from the purely geometric to the rigorously algebraic, can be developed and illustrated in this relatively simple setting. The tools built here will be extended and generalized in every subsequent unit.

## Structure of This Unit

The unit is organized into five chapters, each addressing a major technique or class of first-order equations.

Chapter 1 develops the geometric and qualitative approach: direction fields, phase lines, and stability analysis. This perspective requires no explicit formulas and applies even to equations that cannot be solved in closed form. It cultivates the habit of extracting qualitative information, whether solutions grow or decay, whether equilibria attract or repel, from the structure of the equation itself.

Chapter 2 treats separable equations, the simplest class amenable to exact integration. Autonomous equations (where the right-hand side depends only on $y$) are a particularly important subclass. Applications to exponential growth and decay and to the logistic model connect the mathematics to biology, physics, and economics.

Chapter 3 addresses the first-order linear equation $y' + p(x)y = q(x)$, introducing the integrating factor method. This technique reduces any first-order linear equation to a direct integration, and the resulting formula is one of the most used in all of applied mathematics.

Chapter 4 develops the theory of exact equations. The condition for exactness is a partial differential equation criterion, and the solution method involves finding a potential function. Integrating factors for non-exact equations extend the reach of the method considerably.

Chapter 5 covers substitution methods: homogeneous equations (solved by the substitution $v = y/x$), Bernoulli equations (linearized by $v = y^{1-n}$), Riccati equations (linearized when one particular solution is known), and general strategies for recognizing productive substitutions.

## Why First-Order Methods Matter

Every second-order equation can be converted to a system of two first-order equations. Numerical methods for ODEs of all orders are implemented through first-order systems. The Laplace transform converts differential equations into algebraic equations whose solution, when inverted, often involves the same partial fractions and exponentials that appear in first-order exact solutions. The methods in this unit are therefore not merely preliminary exercises; they are the computational bedrock of the entire subject.

## The Interplay of Technique and Theory

Throughout this unit, solution techniques are accompanied by the theoretical context that explains why they work. The integrating factor works because multiplying through by the right function converts a non-exact equation into an exact one. Separation of variables works because the equation can be written as $g(y)\,dy = f(x)\,dx$, making each side an integral in a single variable. This conceptual clarity, not just procedural fluency, is the goal.
