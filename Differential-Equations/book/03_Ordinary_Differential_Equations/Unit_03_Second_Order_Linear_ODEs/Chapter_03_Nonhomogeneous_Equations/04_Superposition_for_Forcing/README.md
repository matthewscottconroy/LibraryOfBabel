# Superposition for Forcing Functions

When the forcing function $g(x)$ is a sum of simpler terms, the particular solution can be found by handling each term separately and summing the results. This **superposition of forcing** principle is a direct consequence of the linearity of the operator $L$.

## The Principle

**Theorem.** If $L[y_{p,k}] = g_k(x)$ for $k = 1, 2, \ldots, m$, then $y_p = \alpha_1 y_{p,1} + \cdots + \alpha_m y_{p,m}$ satisfies $L[y_p] = \alpha_1 g_1 + \cdots + \alpha_m g_m$.

**Proof.** $L[y_p] = \sum_{k} \alpha_k L[y_{p,k}] = \sum_k \alpha_k g_k$.

## Applications

**Example.** Solve $y'' - y = e^{2x} + 3\sin x$.

Homogeneous: $r^2 - 1 = 0$, roots $r = \pm 1$, so $y_h = c_1 e^x + c_2 e^{-x}$.

For $L[y_{p,1}] = e^{2x}$: try $y_{p,1} = Ae^{2x}$. Then $4Ae^{2x} - Ae^{2x} = 3Ae^{2x} = e^{2x}$, so $A = 1/3$.

For $L[y_{p,2}] = 3\sin x$: try $y_{p,2} = B\sin x + C\cos x$. Then $-B\sin x - C\cos x - B\sin x - C\cos x = -2B\sin x - 2C\cos x = 3\sin x$. So $B = -3/2$, $C = 0$.

$y_p = \frac{1}{3}e^{2x} - \frac{3}{2}\sin x$. General solution: $y = c_1 e^x + c_2 e^{-x} + \frac{1}{3}e^{2x} - \frac{3}{2}\sin x$.

## Fourier Decomposition and Steady-State Response

The superposition principle has a profound application in signal processing. Any periodic forcing $g(t)$ can be written as a Fourier series $g(t) = \sum_{n=0}^\infty (a_n\cos n\omega t + b_n\sin n\omega t)$. For a stable linear system $L[y] = g$, the steady-state particular solution is the sum of the responses to each Fourier component:

$$y_p = \sum_{n=0}^\infty (A_n\cos n\omega t + B_n\sin n\omega t),$$

where the $A_n, B_n$ are determined by undetermined coefficients applied to each Fourier mode. This decomposition of the response into frequency components is the mathematical foundation of frequency-domain analysis in engineering and the reason why linear systems can be completely characterized by their frequency response.

## Practical Value

In practice, superposition allows one to build up particular solutions for complicated forcing functions by tabulating responses to elementary forcing types (monomials, exponentials, sinusoids) and combining them. This is exploited systematically in the Laplace transform method (Unit 5), where convolution in the time domain corresponds to multiplication of transforms in the $s$-domain, and the response to any input can be read off from the system's transfer function.
