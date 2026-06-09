# Chapter 2: Operations on Distributions

The power of distribution theory lies in the operations it supports. Every operation that is classically defined for smooth functions—differentiation, multiplication by smooth functions, convolution, the Fourier transform, and pullback under smooth maps—extends naturally to distributions, with the same formal algebraic rules. This chapter develops these operations, paying particular attention to differentiation (the most important) and the Fourier transform (the most powerful tool for solving PDEs).

## The Principle of Duality

The key technique for extending operations to distributions is **duality**: define the operation on a distribution $T$ so that, when $T = T_f$ is a regular distribution, the result agrees with the classical operation on $f$. The definition is then forced by integration by parts.

For differentiation: $\int (Df)\phi = -\int f(D\phi)$ (integration by parts, boundary terms vanish since $\phi$ has compact support). This motivates $\langle DT, \phi \rangle = -\langle T, D\phi \rangle$.

For convolution with $g$: $\int (f*g)(x)\phi(x) \, dx = \int f(x)(\tilde{g}*\phi)(x) \, dx$ where $\tilde{g}(x) = g(-x)$. This motivates $\langle T*g, \phi \rangle = \langle T, \tilde{g}*\phi \rangle$.

The consistency of these definitions—that they agree with classical operations when restricted to smooth functions—is always the first thing to verify, and the proof is always integration by parts.

## Chapter Structure

**Section 1: Differentiation of Distributions.** Every distribution can be differentiated arbitrarily many times. The formula $\langle D^\alpha T, \phi \rangle = (-1)^{|\alpha|}\langle T, D^\alpha\phi \rangle$ defines a continuous linear map $D^\alpha: \mathcal{D}' \to \mathcal{D}'$. This makes $\mathcal{D}'$ a differential algebra (in a generalized sense) and is the central application to PDE theory.

**Section 2: Multiplication and Pullback.** Distributions can be multiplied by smooth functions: $\langle fT, \phi \rangle = \langle T, f\phi \rangle$. Pullback under smooth submersions is also defined. Notably, the product of two distributions cannot be defined in general—this is a fundamental limitation with consequences for nonlinear PDEs.

**Section 3: Fourier Transform of Distributions.** The Fourier transform extends from $L^1 \cap L^2$ to the space of tempered distributions $\mathcal{S}'$. Key facts: the transform is an isomorphism $\mathcal{S}' \to \mathcal{S}'$; it converts differentiation to multiplication by polynomials ($\widehat{D^\alpha T} = (i\xi)^\alpha \hat{T}$); and it converts convolution to multiplication. The Fourier transform is the primary tool for finding fundamental solutions of constant-coefficient PDEs.

## Distributions as a PDE Tool

The combination of differentiation and the Fourier transform within the distribution framework solves a fundamental problem: proving existence and uniqueness of solutions to PDEs with non-smooth data. For any constant-coefficient PDE $P(D)u = f$ with $f \in \mathcal{S}'$, one can apply the Fourier transform to get $P(i\xi)\hat{u} = \hat{f}$, divide by $P(i\xi)$ (when possible), and invert. The distribution framework handles all the technical issues—division by polynomials, inversion of singular transforms—that classical analysis cannot address.
