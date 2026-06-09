# Circular Membranes: The Drumhead Problem

A circular drumhead is one of the most beautiful applications of PDE theory: the vibration frequencies, the spatial patterns of the normal modes, and the relationship between the drum's geometry and its sound spectrum are all encoded in the eigenvalue problem for the Laplacian on a disk. The solutions involve Bessel functions, and the resulting vibration patterns (with their nodal curves) can be made visible experimentally using sand (Chladni figures).

## The Mathematical Problem

The transverse displacement $u(r,\theta,t)$ of a circular membrane of radius $R$, clamped at its boundary, satisfies:

$$u_{tt} = c^2\Delta u = c^2\!\left(u_{rr} + \frac{1}{r}u_r + \frac{1}{r^2}u_{\theta\theta}\right), \quad 0 < r < R,\; 0 \leq \theta < 2\pi,\; t > 0,$$
$$u(R,\theta,t) = 0, \qquad u \text{ bounded as } r\to 0.$$

## Separation of Variables

Seek $u = R(r)\Theta(\theta)T(t)$. Separating time: $T'' = -\omega^2 T$ gives $T = A\cos(\omega t) + B\sin(\omega t)$.

Separating the spatial equation $r^2 R''/R + rR'/R + r^2\omega^2/c^2 = -\Theta''/\Theta = m^2$ gives:

- **Angular equation:** $\Theta'' + m^2\Theta = 0$, periodic in $\theta$ with period $2\pi$, so $m = 0, 1, 2, \ldots$, with $\Theta_m(\theta) = A_m\cos(m\theta) + B_m\sin(m\theta)$.

- **Radial equation:** $R'' + \frac{1}{r}R' + \left(\frac{\omega^2}{c^2} - \frac{m^2}{r^2}\right)R = 0$ — Bessel's equation of order $m$.

Setting $k = \omega/c$ and $\rho = kr$, the radial equation is standard Bessel of order $m$: $R'' + R'/r + (k^2 - m^2/r^2)R = 0$. The bounded solution is $R(r) = J_m(kr)$.

## Eigenvalues and Mode Shapes

The boundary condition $R(R) = J_m(kR) = 0$ determines the allowed values of $k$: $kR = j_{mn}$ where $j_{mn}$ is the $n$-th positive zero of $J_m$.

The eigenvalues are $k_{mn} = j_{mn}/R$ and the natural frequencies are:

$$\omega_{mn} = \frac{c\,j_{mn}}{R}, \qquad f_{mn} = \frac{c\,j_{mn}}{2\pi R}.$$

The zeros $j_{mn}$ are tabulated:

| $m\backslash n$ | $n=1$ | $n=2$ | $n=3$ |
|---|---|---|---|
| $m=0$ | 2.4048 | 5.5201 | 8.6537 |
| $m=1$ | 3.8317 | 7.0156 | 10.1735 |
| $m=2$ | 5.1356 | 8.4172 | 11.6198 |

For comparison, a string of length $L$ has natural frequencies $f_n = nc/(2L)$, which are integer multiples (a harmonic series). The drum frequencies are $f_{mn} = cj_{mn}/(2\pi R)$, which are NOT in integer ratios — the drum is (generically) non-harmonic. This is why drums sound "thuddy" rather than pitched like strings.

## Mode Shapes and Nodal Patterns

The $(m,n)$-th mode has the spatial shape:

$$\Phi_{mn}(r,\theta) = J_m\!\left(\frac{j_{mn}r}{R}\right)\begin{cases}\cos(m\theta)\\\sin(m\theta)\end{cases}.$$

The nodal curves (where $\Phi_{mn} = 0$) are:
- **Nodal circles:** $J_m(j_{mn}r/R) = 0$, i.e., $r = Rj_{mk}/j_{mn}$ for $k = 1, \ldots, n-1$. There are $n-1$ nodal circles.
- **Nodal diameters:** $\cos(m\theta) = 0$ (for the cosine mode), giving $m$ nodal diameters at angles $\theta = \pi/(2m) + k\pi/m$. There are $m$ nodal diameters.

Total nodal lines: $(m,n)$-mode has $m$ diameters and $n-1$ circles.

**Chladni figures:** When a drumhead (or plate) is vibrated at a mode frequency and sand is sprinkled on it, the sand migrates to the nodal curves (where there is no motion) and accumulates there, tracing out the nodal pattern visually. These are Chladni figures, first described systematically by Ernst Chladni in 1787. They are one of the earliest examples of experimental mathematics.

## Degeneracy

For $m \geq 1$, the modes $J_m(j_{mn}r/R)\cos(m\theta)$ and $J_m(j_{mn}r/R)\sin(m\theta)$ have the same frequency $\omega_{mn}$. Any linear combination of these is also a normal mode, giving a two-dimensional space of modes at each non-radially-symmetric frequency. The orientation of the nodal diameters is not fixed by the boundary condition — it is determined by the initial conditions. This degeneracy means that the shape of the nodal pattern is not uniquely determined by the frequency for $m \geq 1$.

## Kac's Problem: Can One Hear the Shape of a Drum?

Mark Kac (1966) asked: if two drums have the same spectrum of natural frequencies $\{\omega_{mn}\}$, must they have the same shape? The answer is no: Gordon, Webb, and Wolpert (1992) constructed explicit pairs of non-congruent polygonal drumheads with identical frequency spectra. However, the spectrum does determine the area and perimeter of the drum (via the heat kernel expansion), and certain topological properties (number of holes).

This question connects directly to spectral geometry: the eigenvalues of the Laplacian on a domain carry geometric information about the domain, but in general do not uniquely determine its shape.
