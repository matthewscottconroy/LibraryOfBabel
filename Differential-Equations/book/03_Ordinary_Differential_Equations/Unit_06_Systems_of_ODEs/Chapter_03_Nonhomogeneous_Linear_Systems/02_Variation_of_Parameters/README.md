# Variation of Parameters for Systems

Variation of parameters extends to systems of ODEs with no restriction on the form of the nonhomogeneous term $\mathbf{g}(t)$. Where undetermined coefficients requires $\mathbf{g}$ to have a specific exponential-polynomial form, variation of parameters applies whenever $\mathbf{g}$ is continuous (or merely integrable), making it the more powerful and general of the two techniques. The cost is an integral that may not be expressible in closed form, but the theoretical structure is clean and the connection to the fundamental matrix and Green's function is illuminating.

## Setup and Derivation

Consider the system $\mathbf{x}' = A(t)\mathbf{x} + \mathbf{g}(t)$, where $A(t)$ is an $n \times n$ matrix function and $\mathbf{g}(t)$ is an $n$-vector function, both continuous on an interval $I$. Let $\Phi(t)$ be a fundamental matrix for the homogeneous system $\mathbf{x}' = A(t)\mathbf{x}$, so that $\Phi'(t) = A(t)\Phi(t)$ and $\det\Phi(t) \neq 0$ on $I$.

The homogeneous general solution is $\mathbf{x}_h = \Phi(t)\mathbf{c}$ for an arbitrary constant vector $\mathbf{c} \in \mathbb{R}^n$. To find a particular solution, we replace the constant vector $\mathbf{c}$ by an unknown vector function $\mathbf{v}(t)$ and write $\mathbf{x}_p = \Phi(t)\mathbf{v}(t)$. Differentiating:

$$\mathbf{x}_p' = \Phi'(t)\mathbf{v}(t) + \Phi(t)\mathbf{v}'(t) = A(t)\Phi(t)\mathbf{v}(t) + \Phi(t)\mathbf{v}'(t).$$

Substituting into $\mathbf{x}' = A(t)\mathbf{x} + \mathbf{g}(t)$:

$$A(t)\Phi(t)\mathbf{v}(t) + \Phi(t)\mathbf{v}'(t) = A(t)\Phi(t)\mathbf{v}(t) + \mathbf{g}(t).$$

The $A(t)\Phi(t)\mathbf{v}(t)$ terms cancel, leaving the key equation:

$$\Phi(t)\mathbf{v}'(t) = \mathbf{g}(t).$$

Since $\Phi(t)$ is invertible (its determinant is nonzero), this can be solved:

$$\mathbf{v}'(t) = \Phi(t)^{-1}\mathbf{g}(t).$$

Integrating:

$$\mathbf{v}(t) = \int_{t_0}^t \Phi(s)^{-1}\mathbf{g}(s)\,ds.$$

The particular solution is then:

$$\mathbf{x}_p(t) = \Phi(t)\int_{t_0}^t \Phi(s)^{-1}\mathbf{g}(s)\,ds.$$

The general solution to the nonhomogeneous system is:

$$\mathbf{x}(t) = \Phi(t)\mathbf{c} + \Phi(t)\int_{t_0}^t \Phi(s)^{-1}\mathbf{g}(s)\,ds.$$

## The State Transition Form and Green's Function

For constant coefficient systems, it is natural to use the principal fundamental matrix $\Phi(t) = e^{At}$, normalized at $t = 0$. In this case $\Phi(s)^{-1} = e^{-As}$, and the general solution formula becomes:

$$\mathbf{x}(t) = e^{At}\mathbf{c} + \int_0^t e^{A(t-s)}\mathbf{g}(s)\,ds.$$

For the initial value problem with $\mathbf{x}(t_0) = \mathbf{x}_0$, the constant $\mathbf{c}$ is determined by $\mathbf{c} = \Phi(t_0)^{-1}\mathbf{x}_0$, and the solution formula becomes:

$$\mathbf{x}(t) = \Phi(t)\Phi(t_0)^{-1}\mathbf{x}_0 + \Phi(t)\int_{t_0}^t \Phi(s)^{-1}\mathbf{g}(s)\,ds.$$

The matrix $G(t,s) = \Phi(t)\Phi(s)^{-1}$ is called the **state transition matrix** of the system. It describes how the state at time $s$ propagates to time $t$ under the homogeneous dynamics. For constant coefficients, $G(t,s) = e^{A(t-s)}$, which depends only on the elapsed time $t - s$.

The particular solution can be written compactly as:

$$\mathbf{x}_p(t) = \int_{t_0}^t G(t,s)\mathbf{g}(s)\,ds.$$

The kernel $G(t,s)$ is the matrix analogue of the scalar Green's function. This representation has a natural physical interpretation: the forcing $\mathbf{g}(s)\,ds$ at time $s$ produces a response $G(t,s)\mathbf{g}(s)\,ds$ at time $t$, and the total response is the superposition (integral) of all these individual impulse responses. This is the principle of superposition for continuous forcing.

## Theorem: Existence and Uniqueness via Variation of Parameters

**Theorem.** Let $A(t)$ and $\mathbf{g}(t)$ be continuous on an interval $I$ containing $t_0$, and let $\Phi(t)$ be a fundamental matrix for $\mathbf{x}' = A(t)\mathbf{x}$. Then the unique solution to the initial value problem $\mathbf{x}' = A(t)\mathbf{x} + \mathbf{g}(t)$, $\mathbf{x}(t_0) = \mathbf{x}_0$ is:

$$\mathbf{x}(t) = \Phi(t)\Phi(t_0)^{-1}\mathbf{x}_0 + \int_{t_0}^t \Phi(t)\Phi(s)^{-1}\mathbf{g}(s)\,ds, \qquad t \in I.$$

The proof is immediate from the derivation above combined with the existence and uniqueness theorem for linear systems. Uniqueness follows because the difference of two solutions satisfies the homogeneous system with zero initial condition, so it is identically zero.

## Worked Example

Solve $\mathbf{x}' = \begin{pmatrix}0 & 1 \\ -1 & 0\end{pmatrix}\mathbf{x} + \begin{pmatrix}\sec t \\ 0\end{pmatrix}$, $\mathbf{x}(0) = \mathbf{0}$.

The matrix $A = \begin{pmatrix}0&1\\-1&0\end{pmatrix}$ has eigenvalues $\pm i$. The fundamental matrix is:

$$\Phi(t) = \begin{pmatrix}\cos t & \sin t \\ -\sin t & \cos t\end{pmatrix}.$$

This can be verified: $\Phi'(t) = \begin{pmatrix}-\sin t & \cos t \\ -\cos t & -\sin t\end{pmatrix} = A\Phi(t)$, and $\det\Phi = \cos^2 t + \sin^2 t = 1$. The inverse is:

$$\Phi(t)^{-1} = \begin{pmatrix}\cos t & -\sin t \\ \sin t & \cos t\end{pmatrix}.$$

(For a $2\times 2$ matrix $\begin{pmatrix}a&b\\c&d\end{pmatrix}$ with determinant 1, the inverse is $\begin{pmatrix}d&-b\\-c&a\end{pmatrix}$.)

Now compute $\Phi(s)^{-1}\mathbf{g}(s)$:

$$\Phi(s)^{-1}\begin{pmatrix}\sec s \\ 0\end{pmatrix} = \begin{pmatrix}\cos s & -\sin s \\ \sin s & \cos s\end{pmatrix}\begin{pmatrix}\sec s \\ 0\end{pmatrix} = \begin{pmatrix}\cos s \cdot \sec s \\ \sin s \cdot \sec s\end{pmatrix} = \begin{pmatrix}1 \\ \tan s\end{pmatrix}.$$

Integrate from $0$ to $t$:

$$\mathbf{v}(t) = \int_0^t \begin{pmatrix}1 \\ \tan s\end{pmatrix}ds = \begin{pmatrix}t \\ -\ln|\cos t|\end{pmatrix}.$$

The particular solution is:

$$\mathbf{x}_p(t) = \Phi(t)\mathbf{v}(t) = \begin{pmatrix}\cos t & \sin t \\ -\sin t & \cos t\end{pmatrix}\begin{pmatrix}t \\ -\ln|\cos t|\end{pmatrix}.$$

Carrying out the multiplication:

$$x_{p,1} = t\cos t - (\ln|\cos t|)\sin t,$$
$$x_{p,2} = -t\sin t - (\ln|\cos t|)\cos t.$$

Since $\mathbf{x}(0) = \mathbf{0}$ and $\mathbf{x}_p(0) = \mathbf{0}$ (as $t=0$ gives $0 \cdot 1 - 0 \cdot 0 = 0$ and $0 - 0 = 0$), the homogeneous part has $\mathbf{c} = \mathbf{0}$, and $\mathbf{x} = \mathbf{x}_p$.

The forcing $\sec t$ is not of exponential-polynomial type, so undetermined coefficients cannot handle this example. The particular solution involving $\ln|\cos t|$ arises naturally and correctly from the integration.

## Comparison with Undetermined Coefficients

Variation of parameters is more general than undetermined coefficients: it works for any continuous forcing, for variable-coefficient systems, and handles resonance automatically without requiring a modification rule. However, for exponential-polynomial forcing with constant-coefficient systems, undetermined coefficients is often faster because it reduces to solving a linear system rather than computing matrix inverses and performing vector integration.

The deeper unification is through the formula $\mathbf{x}_p(t) = \int_{t_0}^t e^{A(t-s)}\mathbf{g}(s)\,ds$: in the scalar case with $A = a$ and $\mathbf{g}(s) = be^{\alpha s}$, this integral recovers exactly the particular solution found by undetermined coefficients (with the appropriate modification when $\alpha$ is an eigenvalue). The variation of parameters formula thus provides the theoretical foundation that unifies all particular solution methods.
