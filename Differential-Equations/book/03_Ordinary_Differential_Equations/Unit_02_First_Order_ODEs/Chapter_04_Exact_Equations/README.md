# Chapter 4: Exact Equations

An exact differential equation is one that can be recognized as the total differential of some function $F(x, y)$. Solving it amounts to finding $F$, and the general solution is simply the family of level curves $F(x, y) = C$. The exactness condition is a clean partial derivative criterion, and the method for finding $F$ is systematic. When an equation is not exact, integrating factors can sometimes restore exactness, extending the method considerably.

## Chapter Contents

The first section presents the exactness condition: the equation $M\,dx + N\,dy = 0$ is exact if and only if $\partial M/\partial y = \partial N/\partial x$ (on a simply connected domain). This condition is the ODE analog of the curl-free condition for a conservative vector field in calculus. The second section develops the algorithm for finding the potential function $F$, and the third addresses the strategy of finding integrating factors that convert a non-exact equation into an exact one.

## The Core Geometric Idea

The equation $M(x,y)\,dx + N(x,y)\,dy = 0$ says that along any solution curve, the differential form $M\,dx + N\,dy$ equals zero. If this form is the total differential of some function $F$, then $dF = M\,dx + N\,dy$, and $dF = 0$ along solution curves, meaning $F$ is constant. Thus the solution curves are the level sets of $F$.

This connection between differential equations and potential theory makes exact equations a natural bridge between ODEs and the multivariable calculus concepts of gradient fields and line integrals.

## Connections

The exactness condition $M_y = N_x$ is the integrability condition of classical calculus. The same condition appears in the theory of differential forms: the 1-form $\omega = M\,dx + N\,dy$ is exact (is the differential of a function) if and only if it is closed (has zero exterior derivative, i.e., $M_y = N_x$). When the domain is simply connected, these conditions are equivalent by Poincare's lemma. The ODE chapter on exact equations is therefore a concrete introduction to ideas that generalize to differential geometry and topology.
