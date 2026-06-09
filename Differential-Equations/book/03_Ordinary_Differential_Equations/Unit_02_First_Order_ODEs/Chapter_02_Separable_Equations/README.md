# Chapter 2: Separable Equations

A separable first-order ODE is one in which the right-hand side can be factored into a product of a function of $x$ alone and a function of $y$ alone. This structural property allows the equation to be separated into two independent integrals, one in each variable, making it the most directly solvable class of first-order equations.

## The Core Idea

Writing the equation as $dy/dx = g(x)h(y)$, one divides both sides by $h(y)$ (where $h(y) \neq 0$) to get $dy/h(y) = g(x)\,dx$, then integrates both sides. The result is an implicit equation relating $y$ to $x$, from which an explicit formula for $y$ may or may not be extractable. The method is straightforward when it applies, but care must be taken about constant solutions (where $h(y) = 0$) and about domains.

## Chapter Contents

This chapter covers four topics. The first section develops the method of separation of variables in full generality, with attention to the implicit form of solutions and the treatment of constant solutions. The second examines autonomous equations, a particularly important subclass where $g(x) = 1$ and the equation $y' = f(y)$ depends only on $y$. The third applies separation of variables to the classical models of exponential growth and decay, including radioactive decay, Newton's law of cooling, and compound interest. The fourth develops the logistic equation in depth, analyzing it both analytically (by separation and partial fractions) and qualitatively (via the phase line).

## Why This Chapter Matters

Separation of variables is the gateway technique in the subject. It introduces the habit of manipulating differentials, an informal but powerful shorthand for the rigorous operation of dividing both sides by $h(y)$ and integrating. The applications, growth, decay, and logistic dynamics, are among the most important in mathematical biology, physics, and economics. And the logistic equation provides a first encounter with the contrast between local linearization (exponential growth near the zero equilibrium) and global nonlinear behavior (saturation at the carrying capacity), a contrast that recurs throughout the subject.
