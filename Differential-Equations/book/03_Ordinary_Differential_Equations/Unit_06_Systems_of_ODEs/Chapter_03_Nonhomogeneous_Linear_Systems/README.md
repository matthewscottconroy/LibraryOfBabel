# Chapter 3: Nonhomogeneous Linear Systems

For the nonhomogeneous system $\mathbf{x}' = A\mathbf{x} + \mathbf{g}(t)$, the general solution is $\mathbf{x} = \mathbf{x}_h + \mathbf{x}_p$ where $\mathbf{x}_h = \Phi(t)\mathbf{c}$ is the general homogeneous solution and $\mathbf{x}_p$ is any particular solution. Two methods for finding $\mathbf{x}_p$ are developed here.

## Undetermined Coefficients

When $A$ is constant and $\mathbf{g}(t)$ is of the exponential-polynomial type, the method of undetermined coefficients adapts directly from the scalar case: guess a trial vector of the same form as $\mathbf{g}$, substitute into the system, and solve for the unknown coefficient vector. The modification rule (multiply by $t$ if the trial form is a homogeneous solution) applies component-wise.

This method is efficient when it applies but requires recognizing the appropriate trial form. For general $\mathbf{g}$, variation of parameters is needed.

## Variation of Parameters for Systems

The variation of parameters formula extends to systems: given a fundamental matrix $\Phi(t)$, the particular solution is

$$\mathbf{x}_p(t) = \Phi(t)\int \Phi(t)^{-1}\mathbf{g}(t)\,dt.$$

For the IVP $\mathbf{x}(t_0) = \mathbf{x}_0$, the unique solution is

$$\mathbf{x}(t) = \Phi(t)\Phi(t_0)^{-1}\mathbf{x}_0 + \Phi(t)\int_{t_0}^t \Phi(\tau)^{-1}\mathbf{g}(\tau)\,d\tau.$$

For constant $A$, $\Phi(t) = e^{At}$ and $\Phi(t)^{-1} = e^{-At}$:

$$\mathbf{x}(t) = e^{A(t-t_0)}\mathbf{x}_0 + \int_{t_0}^t e^{A(t-\tau)}\mathbf{g}(\tau)\,d\tau.$$

The integral term is the vector convolution of the matrix exponential with the forcing, directly analogous to the scalar formula $y_p = h * g$ from Laplace transform theory.
