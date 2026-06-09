# Chapter 1: Linear Systems

A linear first-order system has the form $\mathbf{x}' = A(t)\mathbf{x} + \mathbf{g}(t)$, where $\mathbf{x}(t)$ is a vector of unknown functions, $A(t)$ is a given matrix of coefficient functions, and $\mathbf{g}(t)$ is a vector forcing function. This chapter develops the structural theory: what makes the solution space a vector space, how to characterize linear independence of solutions, and what the fundamental matrix is.

## The Matrix Equation

Writing the system $x_1' = a_{11}x_1 + \cdots + a_{1n}x_n + g_1$, ..., $x_n' = a_{n1}x_1 + \cdots + a_{nn}x_n + g_n$ in matrix form:

$$\mathbf{x}' = A(t)\mathbf{x} + \mathbf{g}(t), \qquad \mathbf{x}(t_0) = \mathbf{x}_0.$$

When $\mathbf{g} = \mathbf{0}$: **homogeneous** system. When $\mathbf{g} \neq \mathbf{0}$: **nonhomogeneous**.

## Superposition and Solution Space

By the same argument as for single equations, the set of solutions of the homogeneous system $\mathbf{x}' = A(t)\mathbf{x}$ forms a vector space. For continuous $A(t)$ on an interval $I$, this space has dimension exactly $n$ (the size of the system).

## The Wronskian and Fundamental Matrix

Given $n$ solutions $\mathbf{x}_1(t), \ldots, \mathbf{x}_n(t)$, the **Wronskian** is $W(t) = \det[\mathbf{x}_1(t) \mid \cdots \mid \mathbf{x}_n(t)]$ (the determinant of the matrix whose columns are the solutions). By the analog of Abel's identity:

$$W(t) = W(t_0)\exp\!\left(\int_{t_0}^t \mathrm{tr}\,A(s)\,ds\right),$$

(Liouville's formula). As before, $W$ is either always zero or never zero. When $W \neq 0$, the $n$ solutions are linearly independent and form a **fundamental set**. The $n \times n$ matrix $\Phi(t) = [\mathbf{x}_1(t) \mid \cdots \mid \mathbf{x}_n(t)]$ is the **fundamental matrix**: every solution is $\mathbf{x}(t) = \Phi(t)\mathbf{c}$ for some constant vector $\mathbf{c}$.

The unique solution of the IVP $\mathbf{x}' = A\mathbf{x}$, $\mathbf{x}(t_0) = \mathbf{x}_0$ is $\mathbf{x}(t) = \Phi(t)\Phi(t_0)^{-1}\mathbf{x}_0$.
