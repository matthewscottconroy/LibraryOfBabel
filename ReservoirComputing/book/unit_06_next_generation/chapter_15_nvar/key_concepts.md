# Chapter 15 — Key Concepts

---

## NVAR (Next-Generation Reservoir Computer / NG-RC)

The architecture introduced by Gauthier et al. [Gauthier2021] that replaces the reservoir with polynomial features of recent inputs:

$$\hat{y}_t = W^{out} P(u_t, u_{t-1}, \ldots, u_{t-k})$$

where $P$ is a polynomial feature map (all monomials up to degree $d$). The readout is trained by ridge regression. NVAR has no hidden state and no warmup period. It is equivalent to a finite-truncation Volterra series with a polynomial kernel.

---

## Polynomial Feature Map

The function $P: \mathbb{R}^{(k+1)n} \to \mathbb{R}^D$ that constructs all monomials of degree 1 through $d$ from the stacked recent input history $\mathbf{u}_{t:k} = [u_t^\top, \ldots, u_{t-k}^\top]^\top$. The feature dimension is $D = \sum_{j=1}^d \binom{(k+1)n+j-1}{j}$, which grows exponentially with $d$ and polynomially with $k$ and $n$. This exponential growth is the NVAR's principal limitation for high-dimensional systems.

---

## Volterra Series (Chapter 1 connection)

NVAR with history $k$ and degree $d$ is exactly a $d$-th order, $k$-step Volterra series with polynomial basis functions. The Volterra kernels are the coefficients in $W^{out}$ after mapping from monomial to kernel form. The equivalence makes clear that NVAR is not a new class of model but a computationally efficient implementation of a classical temporal series expansion.

---

## Valid Prediction Time (VPT)

The first time step at which the normalized prediction error of a closed-loop (autonomous) prediction exceeds a threshold:

$$T_{VPT} = \min\left\{t : \frac{\|\hat{\mathbf{u}}_t - \mathbf{u}_t\|}{\sigma_u} > \epsilon\right\}$$

Typically expressed in units of the largest Lyapunov exponent $\lambda_1$: $T_{VPT}^{Ly} = \lambda_1 \cdot T_{VPT}$. For the Lorenz system, NVAR achieves approximately 5 Lyapunov times, matching optimally tuned ESNs with 500 neurons [Gauthier2021].

---

## Random Features Theorem (Rahimi & Recht)

The result [Rahimi2007] that for any shift-invariant kernel $k(\mathbf{x}, \mathbf{y}) = k(\mathbf{x} - \mathbf{y})$, drawing $D$ random frequencies from the kernel's Fourier transform and computing cosine features $z_j(\mathbf{x}) = \cos(\boldsymbol{\omega}_j \cdot \mathbf{x} + b_j)$ gives an approximation $\mathbf{z}(\mathbf{x})^\top \mathbf{z}(\mathbf{y}) \approx k(\mathbf{x} - \mathbf{y})$ with error $O(D^{-1/2})$. This connects ESN to kernel methods: the reservoir state is a random feature map approximating an implicit temporal kernel.

---

## Reservoir Kernel

The implicit kernel on input histories $K^{ESN}(\mathbf{u}_{1:T}, \mathbf{v}_{1:T})$ that an ESN computes. Its precise form depends on the activation function, spectral radius, and leaking rate. For linear reservoirs with Gaussian weights, it is a weighted dot product of input time series. For nonlinear reservoirs, it corresponds to a deep arc-cosine or similar kernel. The ESN state can be viewed as an explicit feature map for this kernel.

---

## Polynomial Kernel

The kernel $K_d(\mathbf{u}, \mathbf{v}) = (1 + \mathbf{u} \cdot \mathbf{v})^d$, which corresponds exactly to the dot product of all polynomial features of degree up to $d$. NVAR operates in the RKHS of this kernel. The polynomial kernel is exact for functions that are polynomial in the inputs; it is not shift-invariant and does not admit a Rahimi-Recht random feature approximation.

---

## Curse of Dimensionality (Volterra)

The exponential growth of feature count $D = O(n^d k^d)$ with input dimension $n$, history length $k$, and polynomial degree $d$ in the Volterra/NVAR framework. For $n=64$ (as in the KS equation), $d=2$, $k=2$, the feature count exceeds $10^4$, making training statistically inefficient. This is the primary limitation of NVAR for high-dimensional systems and the reason ESN retains a fundamental advantage in those regimes.

---

## Shift-Invariant Kernel

A kernel $k(\mathbf{x}, \mathbf{y}) = k(\mathbf{x} - \mathbf{y})$ that depends only on the difference between its arguments. Common examples: the RBF (Gaussian) kernel, the Laplace kernel, the Matérn kernel. Shift-invariant kernels admit Fourier transform representations (Bochner's theorem) and random feature approximations (Rahimi-Recht). The implicit kernel of a linear ESN with Gaussian weights is approximately shift-invariant, making the random features interpretation exact in the linear case.

---

## Lyapunov Time

The characteristic timescale for error amplification in a chaotic system: $T_{Ly} = 1/\lambda_1$, where $\lambda_1$ is the largest Lyapunov exponent. For the Lorenz system, $\lambda_1 \approx 0.906 \text{ s}^{-1}$, giving $T_{Ly} \approx 1.10$ seconds or $\approx 44$ integration steps at $\Delta t = 0.025$. Valid prediction time expressed in Lyapunov times is a dimensionless, system-independent measure of prediction skill.

---

## Closed-Loop (Autonomous) Prediction

The prediction mode in which the model's own previous output is fed back as its next input, allowing it to generate extended forecasts without external driving. Both NVAR and ESN can run in closed-loop mode. For NVAR, the update is: compute features from $(\hat{\mathbf{u}}_{t-1}, \ldots, \hat{\mathbf{u}}_{t-k})$, then predict $\hat{\mathbf{u}}_t = W^{out} \mathbf{o}_t$. Small one-step errors amplify exponentially at the Lyapunov rate, so VPT is bounded by the amount of initial error and the Lyapunov exponent.
