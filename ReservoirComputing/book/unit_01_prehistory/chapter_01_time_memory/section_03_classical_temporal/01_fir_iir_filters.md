# 1.3.1–1.3.2 FIR and IIR Filters

## Finite Impulse Response Filters

A **Finite Impulse Response (FIR) filter** of order $K$ computes the output as a finite weighted sum of the current and past inputs:

$$y_t = \sum_{k=0}^{K} h_k u_{t-k} = h_0 u_t + h_1 u_{t-1} + \cdots + h_K u_{t-K}$$

The coefficients $h_0, h_1, \ldots, h_K$ are the **filter taps**. The filter's name comes from its impulse response — the output when the input is an impulse ($u_0 = 1$, $u_t = 0$ for $t \neq 0$) is exactly the sequence $(h_0, h_1, \ldots, h_K, 0, 0, \ldots)$, which is finite: it dies to zero in $K$ steps.

FIR filters are exactly the feedforward sliding-window architecture discussed in Section 1.1.2, specialized to the case of a linear (single-layer, no nonlinearity) network. They are:
- **Always stable**: the output is a finite sum, so no runaway behavior is possible.
- **Linear phase**: under certain symmetry conditions on the taps, FIR filters introduce no phase distortion — an important property for audio and communications.
- **Finite memory**: the filter "remembers" exactly $K$ past inputs and nothing before.

**The design problem** for FIR filters is: given a desired frequency response $H(e^{j\omega})$, find the $K+1$ taps $h_k$ that best approximate it. This is a linear algebra problem, solved by methods such as the Parks-McClellan algorithm or least-squares design [Parks1972].

**The fundamental limitation** is the same as for all sliding-window systems: the memory is exactly $K$ steps, no more. For tasks requiring long memory, $K$ must be large, and the number of parameters scales linearly with $K$.

## The Z-Transform and the Frequency Domain

FIR filters are most cleanly analyzed in the **Z-transform** domain. The Z-transform of a sequence $\{h_k\}$ is:

$$H(z) = \sum_{k=0}^{K} h_k z^{-k}$$

This is a polynomial in $z^{-1}$. The filter's frequency response is $H(e^{j\omega})$, obtained by evaluating $H(z)$ on the unit circle $z = e^{j\omega}$.

The FIR filter computes $Y(z) = H(z) \cdot U(z)$ in the Z-transform domain — multiplication of polynomials, which is convolution in the time domain. This convolution structure is what limits the filter: it implements only a finite polynomial relationship between input and output.

## Infinite Impulse Response Filters

An **Infinite Impulse Response (IIR) filter** introduces a fundamental extension: the output depends not only on past inputs but on past outputs as well:

$$y_t = \sum_{k=0}^{M} b_k u_{t-k} - \sum_{k=1}^{N} a_k y_{t-k}$$

The coefficients $b_k$ are **feedforward (numerator) coefficients** and $a_k$ are **feedback (denominator) coefficients**. The subtraction of past outputs is called **feedback** or **auto-regression**.

The impulse response of an IIR filter is, in general, infinite — it decays exponentially but never reaches exactly zero. This is the source of the name. In the Z-transform domain:

$$Y(z) = \frac{B(z)}{A(z)} U(z) = \frac{\sum_{k=0}^M b_k z^{-k}}{1 + \sum_{k=1}^N a_k z^{-k}} U(z)$$

This is a **rational function** of $z^{-1}$, not a polynomial. Rational functions can represent a much richer class of frequency responses than polynomials — in particular, they can implement sharp resonances and deep notches with far fewer parameters than an equivalent FIR filter.

**The key insight:** the denominator polynomial $A(z)$ has roots — the **poles** of the filter — which determine the decay rate of the impulse response. If all poles are inside the unit circle ($|z_p| < 1$ for every pole $z_p$), the filter is **stable** and the impulse response decays exponentially with rate $\max_p |z_p|$. If any pole is outside the unit circle, the filter is unstable and the output grows without bound.

**The echo state property** in reservoir computing is precisely the requirement that the reservoir's "poles" are inside the unit circle — that is, the spectral radius condition $\rho(W^{rec}) < 1$ for linear reservoirs. This is not a coincidence. An ESN is, in essence, a high-dimensional, nonlinear IIR filter, and the stability conditions inherited from IIR filter theory.

## Comparing FIR and IIR

| Property | FIR | IIR |
|----------|-----|-----|
| Memory | Exactly $K$ steps | Infinite (exponential decay) |
| Stability | Always stable | Stable iff poles inside unit circle |
| Phase | Linear (if symmetric taps) | Nonlinear |
| Parameters | $K+1$ for $K$-step memory | $M + N + 1$ for equivalent frequency response |
| Nonlinearity | Can use nonlinear taps in extension | Can use nonlinear autoregression |
| Design | Convex optimization | Pole placement or gradient descent |

The IIR filter's ability to achieve long effective memory with few parameters — through the recursion structure — is precisely what makes recurrent systems (of which IIR filters are the linear, finite-order special case) so powerful for temporal computation. A reservoir can be seen as a high-dimensional, nonlinear IIR filter where the denominator structure (the recurrent weight matrix) is fixed and the numerator structure (the readout) is trained.

## Infinite IIR Memory and the ARMA Model

The connection between IIR filters and the ARMA (Autoregressive Moving-Average) models of time series analysis [Box2015] is direct:

$$u_t = \sum_{i=1}^p \phi_i u_{t-i} + \sum_{j=0}^q \theta_j \varepsilon_{t-j}$$

The AR part (autoregressive) corresponds to the IIR feedback; the MA part (moving average) to the FIR feedforward. An ARMA($p$, $q$) model has finite-order parameters but represents an infinite impulse response through the autoregressive recursion.

What ARMA cannot represent: nonlinear dependencies between past inputs and current output. The next step — nonlinear autoregression — requires either a nonlinear state-space model (the Volterra series, or a recurrent neural network), or the simplest nonlinear IIR structure: the reservoir.

---

## References

- [Parks1972] Parks, T.W. & McClellan, J.H. (1972). Chebyshev approximation for nonrecursive digital filters with linear phase. *IEEE Transactions on Circuit Theory*, 19(2), 189–194.
- [Oppenheim1999] Oppenheim, A.V. & Schafer, R.W. (1999). *Discrete-Time Signal Processing*, 2nd ed. Prentice Hall. **[The standard reference for filter theory.]**
- [Box2015] Box, G.E.P., Jenkins, G.M., Reinsel, G.C., & Ljung, G.M. (2015). *Time Series Analysis: Forecasting and Control*, 5th ed. Wiley.
