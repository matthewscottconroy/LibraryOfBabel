# Chapter 2: Sturm-Liouville Theory

Sturm-Liouville theory is the spectral theory of second-order self-adjoint differential operators. It provides the theoretical foundation for eigenfunction expansions — series representations of functions in terms of the eigenfunctions of a differential operator — which are the cornerstone of the separation of variables method for partial differential equations. The theory is also the natural framework for understanding the properties of classical special functions (Legendre polynomials, Bessel functions, Fourier sine and cosine series) as eigenfunctions of specific Sturm-Liouville operators.

A **regular Sturm-Liouville problem** has the form:

$$\frac{d}{dx}\left[p(x)y'\right] + [\lambda w(x) - q(x)]y = 0, \qquad a < x < b,$$

with separated boundary conditions $\alpha_0 y(a) + \alpha_1 y'(a) = 0$ and $\beta_0 y(b) + \beta_1 y'(b) = 0$, and with the regularity conditions $p, p', q, w$ continuous on $[a,b]$ and $p(x) > 0$, $w(x) > 0$ on $[a,b]$.

The SL operator $L = -\frac{1}{w(x)}\left[\frac{d}{dx}\left(p(x)\frac{d}{dx}\right) - q(x)\right]$ is self-adjoint on the Hilbert space $L^2([a,b], w\,dx)$ with inner product $\langle f,g\rangle = \int_a^b f(x)g(x)w(x)\,dx$. The SL problem becomes the eigenvalue problem $Ly = \lambda y$.

The spectral theorem for $L$ is the ODE analogue of the spectral theorem for real symmetric matrices: the eigenvalues are real, the eigenfunctions are orthogonal, and they form a complete orthonormal basis. The consequences are profound: any function in $L^2([a,b], w\,dx)$ can be expanded as a generalized Fourier series in the eigenfunctions, with convergence in $L^2$ and (under smoothness conditions) pointwise convergence.

This chapter develops the theory in four sections. The regular Sturm-Liouville problem is set up and the self-adjoint framework is established. The eigenvalue properties (real, simple, countable, unbounded from above) are proved. Orthogonality of eigenfunctions is derived from self-adjointness. Finally, completeness and eigenfunction expansions are discussed — the central application of SL theory to PDEs.
