# Chapter 04: The Z-Transform

Just as the Laplace transform is the natural transform for continuous-time linear systems governed by differential equations, the **Z-transform** is the natural transform for discrete-time linear systems governed by difference equations (recurrences). The Z-transform converts a recurrence relation into an algebraic equation in a complex variable $z$, enabling a clean analysis of stability, frequency response, and transient behavior of discrete-time systems.

The Z-transform is ubiquitous in digital signal processing: every digital filter, every audio codec, every control algorithm running on a microcontroller, is analyzed using Z-transform methods.

## Chapter Overview

**Section 01: Discrete-Time Systems** introduces the Z-transform and its relationship to discrete-time dynamics. The Z-transform of a sequence $(x_n)_{n \geq 0}$ is $X(z) = \sum_{n=0}^\infty x_n z^{-n}$, a formal power series in $z^{-1}$. The key property is that shifting a sequence by one step corresponds to multiplying $X(z)$ by $z^{-1}$: $\mathcal{Z}[(x_{n-1})](z) = z^{-1}X(z)$. This converts the recurrence $y_n = a y_{n-1} + x_n$ into $Y(z) = az^{-1}Y(z) + X(z)$, so $Y(z) = X(z)/(1 - az^{-1}) = H(z)X(z)$ where $H(z) = 1/(1-az^{-1})$ is the transfer function of the system.

**Section 02: Inverse Z-Transform** addresses recovering the sequence $(x_n)$ from $X(z)$. Methods include partial fraction decomposition (directly from tables), the Cauchy residue theorem (interpreting $x_n$ as the coefficient of $z^{n-1}$ in $X(z)$ via contour integration), and long division of power series.

## Connections

The Z-transform relates to the bilateral Laplace transform by $z = e^{sT}$ (where $T$ is the sampling period), and to the DFT by restricting $z$ to the unit circle $|z| = 1$: $X(e^{2\pi ik/N})$ gives the $k$-th DFT output. The poles of $H(z)$ inside the unit circle correspond to stable (decaying) modes, while poles outside the unit circle correspond to unstable (growing) modes — a direct analog of the Laplace stability criterion (poles with negative real part are stable).
