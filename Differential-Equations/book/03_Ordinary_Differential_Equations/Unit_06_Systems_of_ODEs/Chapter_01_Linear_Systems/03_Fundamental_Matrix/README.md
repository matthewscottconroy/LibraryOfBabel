# The Fundamental Matrix

The **fundamental matrix** $\Phi(t)$ is any $n \times n$ matrix whose columns are $n$ linearly independent solutions of $\mathbf{x}' = A(t)\mathbf{x}$. It satisfies the matrix differential equation $\Phi'(t) = A(t)\Phi(t)$ with $\det\Phi(t) \neq 0$ for all $t \in I$.

## Properties

Every solution of $\mathbf{x}' = A\mathbf{x}$ has the form $\mathbf{x}(t) = \Phi(t)\mathbf{c}$ for some constant vector $\mathbf{c}$. The solution of the IVP $\mathbf{x}(t_0) = \mathbf{x}_0$ is

$$\mathbf{x}(t) = \Phi(t)\Phi(t_0)^{-1}\mathbf{x}_0.$$

The matrix $\Phi(t)\Phi(t_0)^{-1}$ is called the **state transition matrix** (or evolution operator) and is often denoted $\Phi(t, t_0)$. It maps the initial state $\mathbf{x}_0$ at time $t_0$ to the state $\mathbf{x}(t)$ at time $t$.

## The Principal Fundamental Matrix

The fundamental matrix normalized so that $\Phi(t_0) = I$ (the identity) is the **principal fundamental matrix** at $t_0$. For constant $A$, the principal fundamental matrix is the **matrix exponential** $e^{A(t-t_0)}$.

## Nonhomogeneous Systems: Variation of Parameters

For $\mathbf{x}' = A(t)\mathbf{x} + \mathbf{g}(t)$, the variation of parameters formula gives the particular solution:

$$\mathbf{x}_p(t) = \Phi(t)\int_{t_0}^t \Phi(\tau)^{-1}\mathbf{g}(\tau)\,d\tau.$$

The complete solution of the IVP is $\mathbf{x}(t) = \Phi(t)\Phi(t_0)^{-1}\mathbf{x}_0 + \mathbf{x}_p(t)$.

## Computing the Fundamental Matrix

For constant $A$, the fundamental matrix is $e^{At} = \sum_{k=0}^\infty A^k t^k/k!$. For variable $A$, explicit formulas are rare; one typically finds $n$ independent solutions by the methods of Chapter 2 and assembles them as columns of $\Phi$.

The Wronskian $W(t) = \det\Phi(t)$ satisfies Liouville's formula $W(t) = W(t_0)e^{\int_{t_0}^t\mathrm{tr}A\,ds}$, providing a check on computed fundamental matrices without knowing individual entries.
