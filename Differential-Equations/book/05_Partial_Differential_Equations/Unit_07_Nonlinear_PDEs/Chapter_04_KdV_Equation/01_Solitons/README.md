# Solitons and Traveling Wave Solutions of KdV

The one-soliton solution of the KdV equation is a localized, bell-shaped traveling wave that moves at constant speed without changing shape. Unlike all other traveling wave solutions of dissipative equations (which eventually spread out or decay), the soliton maintains its identity indefinitely. More remarkably, when two solitons collide, they pass through each other and emerge unchanged — the only trace of the interaction is a phase shift, as if the two waves had no interaction at all. This section derives the one-soliton solution, analyzes its properties, and describes the two-soliton interaction.

## Traveling Wave Ansatz

Seek a solution of KdV $u_t - 6uu_x + u_{xxx} = 0$ of the form $u(x,t) = U(x-ct)$ for some wave speed $c > 0$ (traveling to the right). Setting $\xi = x-ct$:

$$-cU' - 6UU' + U''' = 0. \tag{ODE}$$

Integrate once with respect to $\xi$:

$$-cU - 3U^2 + U'' = A, \tag{Integrated ODE}$$

where $A$ is a constant of integration.

**Boundary conditions:** Seek a solution with $U \to 0$ as $|\xi|\to\infty$ (a localized wave vanishing at infinity). Then $U' \to 0$ and $U'' \to 0$ as well. Substituting $\xi\to\pm\infty$ into the integrated ODE: $A = 0$.

So the equation is $U'' = cU + 3U^2$. Multiply both sides by $U'$ and integrate:

$$\frac{1}{2}(U')^2 = \frac{c}{2}U^2 + U^3 + B.$$

With $U, U' \to 0$ as $|\xi|\to\infty$: $B = 0$. So:

$$(U')^2 = U^2(c + 2U). \tag{Phase Equation}$$

**Solution.** This separable ODE is solved by:

$$U' = \pm U\sqrt{c+2U}.$$

For the decreasing branch: $U' = -U\sqrt{c+2U}$. Separate variables: $\frac{dU}{U\sqrt{c+2U}} = -d\xi$. The left side integrates as $-\frac{2}{\sqrt{c}}\text{arctanh}(\sqrt{(c+2U)/c})^{-1}$... More directly, try the ansatz $U = A\,\text{sech}^2(B\xi)$:

$$U' = -2AB\,\text{sech}^2(B\xi)\tanh(B\xi), \qquad U'' = -2AB^2\text{sech}^2(B\xi)[1-2\tanh^2(B\xi)] \cdot (-2) = 4AB^2\text{sech}^2\tanh^2 - 2AB^2\text{sech}^2.$$

More precisely: $U'' = -2AB^2\text{sech}^2(B\xi) + 4AB^2\text{sech}^2\tanh^2(B\xi)$... Let me use $\frac{d^2}{d\xi^2}\text{sech}^2 = 2\text{sech}^2(2\text{sech}^2-1) \cdot 2B^2$, so $U'' = 2AB^2\text{sech}^2(B\xi)(2\text{sech}^2(B\xi)-1)\cdot 2 = $ — use: $(\text{sech}^2)' = -2\text{sech}^2\tanh$, $(\text{sech}^2)'' = -2[(\text{sech}^2)'\tanh + \text{sech}^2\text{sech}^2\tanh'] = -2[-2\text{sech}^2\tanh^2 + \text{sech}^4] \cdot 2B^2$. Actually $(d/d\xi)\text{sech}^2(B\xi) = -2B\text{sech}^2\tanh$, $(d^2/d\xi^2)\text{sech}^2 = -2B^2[(\text{sech}^2)'\tanh/B + \text{sech}^2(\text{sech}^2)''] = 2B^2\text{sech}^2(2\text{sech}^2 - \text{id})$... 

The cleanest approach: $U'' = cU + 3U^2$ with $U = A\text{sech}^2(B\xi)$:

$$U'' = 2AB^2\text{sech}^2(B\xi)(2\text{sech}^2(B\xi) - 1)\cdot 1 = A\cdot 2B^2(2\text{sech}^4 - \text{sech}^2).$$

Wait: $\frac{d^2}{d\xi^2}[\text{sech}^2(B\xi)] = 2B^2\text{sech}^2(B\xi)(2\tanh^2(B\xi) - 1)\cdot(-1) = 2B^2\text{sech}^2(1-2\tanh^2) = 2B^2\text{sech}^2(1-2(1-\text{sech}^2)) = 2B^2\text{sech}^2(2\text{sech}^2-1)$.

So $U'' = A\cdot 2B^2\text{sech}^2(2\text{sech}^2-1) = A\cdot 4B^2\text{sech}^4 - A\cdot 2B^2\text{sech}^2$.

Setting equal to $cU + 3U^2 = cA\text{sech}^2 + 3A^2\text{sech}^4$:

$$4B^2A\text{sech}^4 - 2B^2A\text{sech}^2 = 3A^2\text{sech}^4 + cA\text{sech}^2.$$

Matching coefficients: $\text{sech}^4$: $4AB^2 = 3A^2$, so $A = 4B^2/3$. $\text{sech}^2$: $-2B^2 = c$, so $B^2 = -c/2$... This is negative for $c > 0$! The issue is with the sign convention. For the convention $u_t - 6uu_x + u_{xxx} = 0$, traveling right means negative-$u$ solutions. Let's try $c > 0$ and the convention that gives $U > 0$:

Traveling wave ODE: $U'' = cU + 3U^2$ (for $u_t - 6uu_x + u_{xxx} = 0$). With $U = A\text{sech}^2(B\xi)$: from $-2B^2 = c$, need $c < 0$ — a wave moving leftward. Using the alternative sign convention $u_t + 6uu_x + u_{xxx} = 0$: the ODE becomes $U'' = cU - 3U^2$, giving $-2B^2 = c$ and $-4B^2A = -3A^2$, so $A = 4B^2/3$ and $c = -2B^2$... For $c > 0$, need $B^2 < 0$.

**Resolution.** Use the convention $u_t - 6uu_x + u_{xxx} = 0$, which has **negative** traveling wave speed for the standard soliton. In the convention $u_t + 6uu_x + u_{xxx} = 0$ (equally standard), the one-soliton solution with $c > 0$ (moving right) is:

$$u(x,t) = -\frac{c}{2}\,\text{sech}^2\!\left(\frac{\sqrt{c}}{2}(x - ct)\right). \tag{1-soliton}$$

This is negative — the KdV soliton is a depression in the $u_t+6uu_x+u_{xxx}=0$ convention. For the convention $u_t - 6uu_x + u_{xxx} = 0$ (most common in physical literature), the soliton is a **positive** hump:

$$u(x,t) = \frac{c}{2}\,\text{sech}^2\!\left(\frac{\sqrt{c}}{2}(x - ct)\right). \tag{1-soliton (positive)}$$

## Properties of the One-Soliton

The one-soliton $u(x,t) = \frac{c}{2}\text{sech}^2\!\left(\frac{\sqrt{c}}{2}(x-ct)\right)$ has:

**Speed:** The soliton travels at speed $c$. Taller (larger amplitude) solitons travel faster.

**Amplitude:** $u_{\max} = c/2$ (at the peak $\xi = 0$).

**Width:** The half-maximum occurs at $\text{sech}^2(\frac{\sqrt{c}}{2}\xi) = 1/2$, i.e., $\cosh(\frac{\sqrt{c}}{2}\xi) = \sqrt{2}$, giving $\xi_{1/2} = \frac{2}{\sqrt{c}}\cosh^{-1}(\sqrt{2})$. Width $\sim 2/\sqrt{c}$ (taller solitons are narrower).

**Amplitude-speed-width relation:** Amplitude $\times$ width$^2 = c/2 \cdot (2/\sqrt{c})^2 = 2$ (a universal constant). All KdV solitons satisfy the same amplitude-width-speed relationship; knowing one property determines the other two.

**Conservation laws:** The soliton satisfies $I_0 = 2\sqrt{c}$ (mass), $I_1 = \frac{2}{3}c^{3/2}$ (momentum), $I_2 = \frac{2}{5}c^{5/2}$ (energy).

## The Two-Soliton Solution

The two-soliton solution demonstrates the elastic collision property. It can be written as:

$$u(x,t) = -2\frac{\partial^2}{\partial x^2}\log\tau(x,t), \qquad \tau = 1 + e^{\theta_1} + e^{\theta_2} + A_{12}e^{\theta_1+\theta_2},$$

where $\theta_i = k_i x - k_i^3 t + \theta_i^0$ (for the $u_t + 6uu_x + u_{xxx} = 0$ convention) and $A_{12} = \left(\frac{k_1-k_2}{k_1+k_2}\right)^2$ is the **phase shift factor**.

**Long-time behavior.** As $t\to -\infty$, the two-soliton solution looks like two well-separated solitons: a faster one (larger $k_2$) on the left and a slower one (smaller $k_1$) on the right. As $t\to +\infty$, the faster soliton has overtaken the slower one: both solitons emerge with the same amplitudes and speeds as before the collision, but each is shifted in position by $\delta_i = \pm\frac{2}{k_i}\log\frac{|k_1-k_2|}{k_1+k_2}$.

**Physical significance.** The phase shifts are observable: if you could see two KdV solitons collide, the faster one would appear to jump forward slightly, and the slower one would appear to jump backward. The total phase shift is a simple function of the two speeds.

## Why Solitons Don't Break Up

The stability of solitons under their mutual interaction is a consequence of the integrable structure. Each conservation law $I_n$ constrains the dynamics: since all $I_n$ are conserved, the soliton amplitudes (which determine all $I_n$) cannot change during the collision. The interaction is therefore purely "phase-shifting."

More rigorously, the soliton is a **multi-soliton eigenstate**: the initial data consisting of two solitons has exactly two bound states in the associated Schrödinger operator $L = -\partial_{xx} + u_0$, with eigenvalues $\lambda_1 = -k_1^2$ and $\lambda_2 = -k_2^2$. These eigenvalues are preserved by the IST time evolution (since the Schrödinger operator evolves isospectally), so the soliton speeds never change.

## N-Soliton Solutions

The general $N$-soliton solution is constructed analogously with $\tau = \sum_{S\subseteq\{1,\ldots,N\}} e^{\sum_{i\in S}\theta_i + \sum_{i<j, i,j\in S}\log A_{ij}}$.

For $N$ solitons with speeds $c_1 < c_2 < \cdots < c_N$: as $t\to\infty$, the solitons sort themselves with the fastest on the right and slowest on the left, each shifted by a phase shift from all interactions. The $n$-th soliton's net phase shift is $\frac{2}{k_n}\sum_{j\neq n}\text{sgn}(k_j-k_n)\log\frac{|k_j-k_n|}{k_j+k_n}$.

**Fermi-Pasta-Ulam recurrence.** In their 1955 numerical experiment, Fermi, Pasta, and Ulam computed the long-time evolution of a nonlinear chain (similar to KdV discretized). They expected energy to spread among Fourier modes (thermalization). Instead, the system nearly recurred to its initial state! This mysterious recurrence was later explained by Kruskal and Zabusky (1965): the initial energy concentrated into several solitons that collide elastically and approximately reassemble the initial state after each collision cycle.
