# Chapter 03: Sine and Cosine Series

Many physical problems are posed on a half-interval rather than a full symmetric interval. A rod of length $L$ with temperature specified at one end or a vibrating string fixed at both ends naturally lives on $[0, L]$. The Fourier series on $[-L, L]$ requires specifying the function on the full symmetric interval, but the physical problem only determines $f$ on $[0, L]$. The resolution is to extend $f$ from $[0, L]$ to $[-L, L]$ in one of two canonical ways — as an even function (producing a cosine series) or as an odd function (producing a sine series) — and then apply the full Fourier theory.

## Chapter Overview

**Section 01: Even and Odd Extensions** develops the systematic procedure for extending a function $f : [0, L] \to \mathbb{R}$ to $[-L, L]$. The **even extension** is $f_e(x) = f(|x|)$, which satisfies $f_e(-x) = f_e(x)$. Since even functions have zero sine coefficients, the Fourier series of $f_e$ contains only cosine terms. The **odd extension** is $f_o(x) = \text{sgn}(x) \cdot f(|x|)$, which satisfies $f_o(-x) = -f_o(x)$. Since odd functions have zero cosine coefficients, the Fourier series of $f_o$ contains only sine terms.

Both extensions agree with the original $f$ on the half-interval $(0, L)$. At $x = 0$: the even extension gives $f_e(0) = f(0)$, while the odd extension gives $f_o(0) = 0$ regardless of $f(0)$. At $x = L$: the even extension has $f_e(L) = f(L)$, while the odd extension has a jump unless $f(L) = 0$.

**Section 02: Half-Range Expansions** presents the resulting series formulas and applies them to boundary value problems. The **half-range cosine series** of $f$ on $[0, L]$ is
$$f(x) = \frac{a_0}{2} + \sum_{n=1}^\infty a_n\cos\!\left(\frac{n\pi x}{L}\right), \quad a_n = \frac{2}{L}\int_0^L f(x)\cos\!\left(\frac{n\pi x}{L}\right)dx.$$
The **half-range sine series** is
$$f(x) = \sum_{n=1}^\infty b_n\sin\!\left(\frac{n\pi x}{L}\right), \quad b_n = \frac{2}{L}\int_0^L f(x)\sin\!\left(\frac{n\pi x}{L}\right)dx.$$
Both series represent $f(x)$ for $x \in (0, L)$ (at points of continuity), but they extend the function differently outside this interval.

## Connection to Boundary Value Problems

The choice between sine and cosine series is dictated by boundary conditions:

- **Dirichlet conditions** ($f(0) = f(L) = 0$): the sine series is appropriate. The functions $\sin(n\pi x/L)$ each satisfy the boundary conditions, and the series automatically satisfies them.
- **Neumann conditions** ($f'(0) = f'(L) = 0$): the cosine series is appropriate. The functions $\cos(n\pi x/L)$ have zero derivatives at the endpoints.
- **Mixed conditions** ($f(0) = 0$, $f'(L) = 0$): lead to series in $\sin((2n-1)\pi x/(2L))$.

This connection to boundary value problems is the main application of this chapter's material in the study of PDEs.

## Key Observations

The two expansions of the same function $f : [0,L] \to \mathbb{R}$ — the sine series and the cosine series — are generally different and represent different periodic functions on $\mathbb{R}$. The cosine series is even and $2L$-periodic; the sine series is odd and $2L$-periodic. Both converge to $f(x)$ for $x \in (0, L)$ at points of continuity, but they predict different values at $x = 0$ and $x = L$. Neither is "more correct" than the other — they answer different questions depending on the physical boundary conditions.
