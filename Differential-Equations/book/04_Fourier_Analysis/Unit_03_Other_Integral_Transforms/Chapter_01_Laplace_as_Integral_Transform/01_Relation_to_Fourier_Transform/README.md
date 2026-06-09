# Relation of the Laplace Transform to the Fourier Transform

The Fourier transform and the Laplace transform are not independent inventions; they are two aspects of the same underlying idea, differing in the domain of the transform variable and the class of functions they handle. Understanding their relationship illuminates both tools and shows why the Laplace transform is the right tool for initial value problems while the Fourier transform is natural for stationary signals.

## The Fourier Transform on the Imaginary Axis

Recall the Fourier transform:
$$\hat{f}(\xi) = \int_{-\infty}^\infty f(t)\,e^{-2\pi i\xi t}\,dt.$$
In the angular-frequency convention (which matches the Laplace convention better), this is
$$F(i\omega) = \int_{-\infty}^\infty f(t)\,e^{-i\omega t}\,dt, \quad \omega \in \mathbb{R}.$$

The bilateral Laplace transform is
$$\mathcal{B}[f](s) = \int_{-\infty}^\infty f(t)\,e^{-st}\,dt, \quad s \in \mathbb{C}.$$

Formally, $\mathcal{B}[f](s) = F(-s)$ in the angular notation, or equivalently, $\mathcal{B}[f](i\omega) = F(i\omega)$: the bilateral Laplace transform evaluated on the imaginary axis $s = i\omega$ is the Fourier transform $F(i\omega)$.

So the bilateral Laplace transform extends the Fourier transform from the imaginary axis $s = i\omega$ to the complex $s$-plane. The extension is analytic: if $\mathcal{B}[f]$ converges absolutely for some $s_0$, it defines an analytic function in a strip or half-plane containing $s_0$.

## Exponential Damping and the Region of Convergence

The bilateral Laplace transform $\int_{-\infty}^\infty f(t)e^{-st}\,dt$ converges absolutely when $|f(t)e^{-st}| = |f(t)|e^{-\text{Re}(s) t}$ is integrable. For a causal function $f(t) = 0$ for $t < 0$, the integral is $\int_0^\infty |f(t)|e^{-\sigma t}\,dt$ where $\sigma = \text{Re}(s)$. This converges if $f(t)$ does not grow faster than $e^{\sigma_0 t}$ for some $\sigma_0$: then the transform converges for $\sigma > \sigma_0$.

The **region of convergence (ROC)** of $\mathcal{B}[f]$ is a vertical strip $\{\sigma_1 < \text{Re}(s) < \sigma_2\}$ (or a half-plane for causal/anti-causal $f$). The Fourier transform corresponds to $\text{Re}(s) = 0$; it is defined when $0$ lies in the ROC, i.e., when $\sigma_1 < 0 < \sigma_2$.

**One-sided (unilateral) Laplace transform.** The standard Laplace transform used for ODEs is the one-sided version:
$$\mathcal{L}[f](s) = \int_0^\infty f(t)\,e^{-st}\,dt.$$
This is $\mathcal{B}[f \cdot \mathbf{1}_{[0,\infty)}](s)$: the bilateral transform of the causal version of $f$.

## Relationship for Causal Functions

Suppose $f : [0,\infty) \to \mathbb{R}$ grows at most exponentially: $|f(t)| \leq Ce^{\sigma_0 t}$ for some $C, \sigma_0 > 0$. Define $g(t) = f(t)e^{-\sigma t}$ for $\sigma > \sigma_0$, extended by zero to $(-\infty, 0)$. Then $g \in L^1(\mathbb{R})$ and
$$\hat{g}(\xi) = \int_0^\infty f(t)e^{-\sigma t}e^{-2\pi i\xi t}\,dt = \mathcal{L}[f](\sigma + 2\pi i\xi).$$

This says: the Laplace transform of $f$ at $s = \sigma + 2\pi i\xi$ equals the Fourier transform of $g(t) = f(t)e^{-\sigma t}$ at frequency $\xi$.

In the angular convention ($s = \sigma + i\omega$):
$$\mathcal{L}[f](\sigma + i\omega) = \mathcal{F}_\omega[f(t)e^{-\sigma t}\mathbf{1}_{[0,\infty)}](t).$$

## Why the Laplace Transform Handles Growing Functions

The Fourier transform of a function growing like $e^{at}$ does not exist classically (the integral $\int e^{at}e^{-i\omega t}\,dt$ diverges). But $\mathcal{L}[e^{at}](s) = 1/(s-a)$ for $\text{Re}(s) > a$, because multiplying by $e^{-st}$ with $\text{Re}(s) > a$ creates exponential decay. The Laplace transform handles growth by working in a shifted complex plane where the imaginary axis is moved to $\text{Re}(s) = \sigma > a$.

This is why the Laplace transform, not the Fourier transform, is the natural tool for:
- **Causal functions** (defined on $[0,\infty)$, possibly growing).
- **Initial value problems** (initial conditions at $t = 0$, future evolution for $t > 0$).
- **Stability analysis** (poles of $\mathcal{L}$ in the complex plane indicate growth/decay rates).

## Inversion via the Bromwich Integral

Given $F(s) = \mathcal{L}[f](s)$, the inverse Laplace transform is
$$f(t) = \frac{1}{2\pi i}\int_{\sigma - i\infty}^{\sigma + i\infty} F(s)\,e^{st}\,ds,$$
where $\sigma$ is any real number to the right of all singularities of $F$. This is the **Bromwich integral** (or Bromwich-Hankel integral). The vertical contour $\{\text{Re}(s) = \sigma\}$ corresponds to the Fourier inversion at imaginary frequency:
$$f(t) = \frac{1}{2\pi}\int_{-\infty}^\infty F(\sigma + i\omega)e^{(\sigma + i\omega)t}\,d\omega = e^{\sigma t}\mathcal{F}^{-1}[F(\sigma + i\cdot)](t).$$

For rational $F(s) = P(s)/Q(s)$ (the case arising from linear ODEs with constant coefficients), the Bromwich integral is evaluated by the residue theorem: close the contour to the left (where $e^{st} \to 0$ as $\text{Re}(s) \to -\infty$ for $t > 0$), and sum the residues at the poles. This gives the partial-fraction decomposition approach to inverse Laplace transforms learned earlier in the course.

## Worked Example: $F(s) = 1/(s^2 + \omega_0^2)$

From the table or by factoring: $F(s) = \frac{1}{(s + i\omega_0)(s - i\omega_0)}$. The poles are at $s = \pm i\omega_0$ on the imaginary axis. For the Bromwich integral with $\sigma > 0$, close to the left and pick up both poles:
$$f(t) = \text{Res}_{s=i\omega_0}\left[\frac{e^{st}}{s^2+\omega_0^2}\right] + \text{Res}_{s=-i\omega_0}\left[\frac{e^{st}}{s^2+\omega_0^2}\right] = \frac{e^{i\omega_0 t}}{2i\omega_0} + \frac{e^{-i\omega_0 t}}{-2i\omega_0} = \frac{\sin(\omega_0 t)}{\omega_0}.$$
This confirms $\mathcal{L}[\sin(\omega_0 t)/\omega_0](s) = 1/(s^2 + \omega_0^2)$.
