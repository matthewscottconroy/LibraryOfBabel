# The First Shifting Theorem

The first shifting theorem (also called the $s$-shifting theorem or frequency shifting) states:

$$\mathcal{L}\{e^{at}f(t)\} = F(s-a), \qquad \text{where } F(s) = \mathcal{L}\{f(t)\}.$$

**Proof.** Direct computation:

$$\mathcal{L}\{e^{at}f(t)\} = \int_0^\infty e^{-st}e^{at}f(t)\,dt = \int_0^\infty e^{-(s-a)t}f(t)\,dt = F(s-a).$$

The transform shifts from $s$ to $s - a$ because the exponential factor $e^{at}$ modifies the convergence condition.

## Applications

**Example 1.** $\mathcal{L}\{e^{-2t}\sin 3t\} = \mathcal{L}\{\sin 3t\}|_{s\to s+2} = \frac{3}{(s+2)^2 + 9}$.

**Example 2.** $\mathcal{L}\{e^t t^3\} = \mathcal{L}\{t^3\}|_{s\to s-1} = \frac{6}{(s-1)^4}$.

**Example 3.** $\mathcal{L}\{e^{2t}\cos(4t + \pi/3)\} = \text{Re}[\mathcal{L}\{e^{2t}e^{i(4t+\pi/3)}\}] = \text{Re}\!\left[e^{i\pi/3}\cdot\frac{1}{s-2-4i}\right]$: the phase shift $\pi/3$ is handled by the complex exponential.

## Inverse Transform Use

The theorem is equally valuable for inverting: if $F(s) = 1/(s^2 + 4s + 13)$, complete the square: $s^2 + 4s + 13 = (s+2)^2 + 9$, so $F(s) = 1/((s+2)^2 + 9)$. By the inverse shift with $a = -2$: $f(t) = e^{-2t}\mathcal{L}^{-1}\{1/(s^2+9)\} = e^{-2t}\sin(3t)/3$.

Completing the square to identify the shift is the standard technique for inverting transforms with quadratic denominators that don't factor over the rationals.
