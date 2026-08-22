# Summation of Series via Residues

The residue theorem provides an elegant method for computing infinite series $\sum_{n=-\infty}^\infty f(n)$ by converting the sum into a contour integral. The key ingredient is a meromorphic function with simple poles at every integer, each with residue $1$: the function $\pi\cot(\pi z)$ serves this purpose. Summing the residues of $\pi\cot(\pi z)f(z)$ over the non-integer poles gives the series $\sum f(n)$ via the residue theorem applied to a large square contour.

## The Cotangent Kernel

**Key fact.** The function $g(z) = \pi\cot(\pi z)$ has simple poles at every integer $n \in \mathbb{Z}$, and at each:
$$\mathrm{Res}(\pi\cot(\pi z); n) = 1.$$

**Proof.** Near $z = n$, write $\pi z = n\pi + \pi(z-n)$. Then $\sin(\pi z) = \sin(n\pi + \pi(z-n)) = (-1)^n\sin(\pi(z-n)) \approx (-1)^n\pi(z-n)$, and $\cos(\pi z) \approx (-1)^n$. So:
$$\pi\cot(\pi z) \approx \pi \cdot \frac{(-1)^n}{(-1)^n \pi(z-n)} = \frac{1}{z-n}.$$
Hence $\mathrm{Res}(\pi\cot(\pi z); n) = 1$. $\square$

## The Summation Method

**Theorem.** Let $f$ be a meromorphic function with poles at points $\{w_1, w_2, \ldots\} \subset \mathbb{C}$ (none of which are integers), satisfying $|f(z)| \to 0$ as $|z| \to \infty$ fast enough (specifically, $|zf(z)| \to 0$ uniformly on the squares $C_N$ below). Then:
$$\sum_{n=-\infty}^\infty f(n) = -\sum_k \mathrm{Res}(\pi\cot(\pi z)f(z); w_k).$$

**Proof.** Let $C_N$ be the square with vertices at $\pm(N + 1/2) \pm (N+1/2)i$. For large $N$, $|\pi\cot(\pi z)| \leq M$ on $C_N$ (a standard estimate: on vertical sides, $|\cot(\pi z)|$ is bounded; on horizontal sides with $|\mathrm{Im}(z)| = N + 1/2 \to \infty$, $|\cot(\pi z)| \to 1$). So $\left|\int_{C_N}\pi\cot(\pi z)f(z)\, dz\right| \leq M\cdot\max_{C_N}|f(z)|\cdot 8(N+1/2) \to 0$.

By the residue theorem:
$$\int_{C_N}\pi\cot(\pi z)f(z)\, dz = 2\pi i\left[\sum_{n=-N}^N f(n) + \sum_{k}\mathrm{Res}(\pi\cot(\pi z)f(z); w_k)\right].$$
As $N \to \infty$, the left side $\to 0$, giving the result. $\square$

## Worked Examples

**Example 1.** Evaluate $\displaystyle S = \sum_{n=1}^\infty \frac{1}{n^2}$.

Note $\sum_{n=-\infty}^\infty \frac{1}{n^2} = 2S +$ (the $n=0$ term, which doesn't exist). We modify: apply the method to $f(z) = 1/z^2$, which has a double pole at $z = 0$.

$$\mathrm{Res}\!\left(\frac{\pi\cot(\pi z)}{z^2}; 0\right).$$

Near $z = 0$: $\pi\cot(\pi z) = \frac{1}{z} - \frac{\pi^2 z}{3} - \frac{\pi^4 z^3}{45} - \cdots$.
So $\frac{\pi\cot(\pi z)}{z^2} = \frac{1}{z^3} - \frac{\pi^2}{3z} - \cdots$.
Residue $= -\pi^2/3$.

The theorem gives:
$$\sum_{n \neq 0} \frac{1}{n^2} = -\mathrm{Res}\!\left(\frac{\pi\cot(\pi z)}{z^2}; 0\right) = \frac{\pi^2}{3}.$$
Since $\sum_{n\neq 0}1/n^2 = 2\sum_{n=1}^\infty 1/n^2$, we get $\sum_{n=1}^\infty \frac{1}{n^2} = \frac{\pi^2}{6}$. $\square$

This is the celebrated Basel problem, solved by Euler in 1734 and here given a clean complex-analytic proof.

**Example 2.** Evaluate $\displaystyle S = \sum_{n=-\infty}^\infty \frac{1}{n^2 + a^2}$, $a > 0$, $a \notin \mathbb{Z}$.

$f(z) = 1/(z^2 + a^2)$ has poles at $z = \pm ia$.

$\mathrm{Res}$ at $z = ia$: $\frac{1}{2ia}$.
$\mathrm{Res}$ at $z = -ia$: $\frac{1}{-2ia} = \frac{-1}{2ia}$.

Residues of $\pi\cot(\pi z)/(z^2+a^2)$:
At $z = ia$: $\frac{\pi\cot(i\pi a)}{2ia} = \frac{\pi \cdot (-i)\coth(\pi a)}{2ia} = \frac{\pi\coth(\pi a)}{2a}$.
At $z = -ia$: $\frac{\pi\cot(-i\pi a)}{-2ia} = \frac{\pi \cdot i\coth(\pi a)}{-2ia} = \frac{\pi\coth(\pi a)}{2a}$.

Sum of residues $= \frac{\pi\coth(\pi a)}{a}$.

$$\sum_{n=-\infty}^\infty \frac{1}{n^2+a^2} = -\frac{\pi\coth(\pi a)}{a} \cdot (-1) = \frac{\pi\coth(\pi a)}{a}. $$

Wait: the theorem says the sum $= -$[sum of residues at non-integer poles] $= -\frac{\pi\coth(\pi a)}{a}$... That gives a negative answer, which is wrong. The sign: the theorem gives $\sum f(n) = -\sum \mathrm{Res}(\pi\cot f; w_k)$, and:

$-\left(\frac{\pi\coth(\pi a)}{2a} + \frac{\pi\coth(\pi a)}{2a}\right) = -\frac{\pi\coth(\pi a)}{a}$.

But $\sum \frac{1}{n^2+a^2} > 0$. There must be a sign error. Using $\cot(i\pi a) = i\coth(\pi a)/i...$ Let me recompute: $\cot(i\pi a) = \cos(i\pi a)/\sin(i\pi a) = \cosh(\pi a)/(i\sinh(\pi a)) = -i\coth(\pi a)$.

Residue at $ia$: $\pi\cot(i\pi a)/(2ia) = \pi(-i\coth(\pi a))/(2ia) = \pi\coth(\pi a)/(2a)$. (Good, positive.)

Residue at $-ia$: $\pi\cot(-i\pi a)/(-2ia) = \pi(i\coth(\pi a))/(-2ia) = \pi\coth(\pi a)/(2a)$. (Also positive.)

Sum = $\pi\coth(\pi a)/a$.

Formula: $\sum_{n=-\infty}^\infty f(n) = -\pi\coth(\pi a)/a$. But this is negative — contradiction. The issue is the sign in the theorem. Revisiting: the contour encloses all integer poles on the inside plus non-integer poles. The theorem as commonly stated is: the integral around $C_N \to 0$, and all residues (integer poles give $f(n)$ each, non-integer poles give residues of $\pi\cot f$ at $w_k$) sum to zero:
$$\sum_{n=-N}^{N} f(n) + \sum_k \mathrm{Res}(\pi\cot f; w_k) = 0,$$
so $\sum f(n) = -\sum_k \mathrm{Res}(\pi\cot f; w_k) = -\pi\coth(\pi a)/a$. Negative, but the series is positive. So the sum of the residues must be negative.

Recheck: $\cot(i\pi a) = \cos(i\pi a)/\sin(i\pi a)$. $\cos(it) = \cosh t$, $\sin(it) = i\sinh t$. So $\cot(i\pi a) = \cosh(\pi a)/(i\sinh(\pi a)) = -i\cosh(\pi a)/\sinh(\pi a) = -i\coth(\pi a)$. 

Residue at $ia$: $\pi(-i\coth(\pi a))/(2ia) = -i\pi\coth(\pi a)/(2ia) = \pi\coth(\pi a)/(2a) > 0$. 

Residue at $-ia$: $\pi\cot(-i\pi a)/(-2ia) = \pi(i\coth(\pi a))/(-2ia) = i\pi\coth(\pi a)/(-2ia) = \pi\coth(\pi a)/(2a) > 0$.

Sum of non-integer residues $= \pi\coth(\pi a)/a > 0$.

$\sum f(n) = -\pi\coth(\pi a)/a < 0$. This contradicts $f(n) = 1/(n^2+a^2) > 0$.

The issue is I'm computing residues of $\pi\cot(\pi z) \cdot f(z)$ at the poles of $f$, but I need to be more careful: the residue of $h(z) = \pi\cot(\pi z)/(z^2+a^2)$ at $z = ia$ is $[\pi\cot(\pi z) \cdot \frac{1}{z+ia}]|_{z=ia} \cdot \frac{1}{1}$... No: $h(z) = \pi\cot(\pi z)/(z^2+a^2)$, pole at $z=ia$ is simple with $\mathrm{Res} = \lim_{z \to ia}(z-ia)h(z) = \pi\cot(i\pi a)/(2ia) = -i\pi\coth(\pi a)/(2ia) = \pi\coth(\pi a)/(2a)$. OK.

So the formula gives $\sum f(n) = -\pi\coth(\pi a)/a < 0$, which is wrong. There must be a global sign error in how I stated the theorem. The correct statement: $\sum_{n=-\infty}^\infty f(n) = -\sum_k \mathrm{Res}(\pi\cot(\pi z)f(z); w_k)$, and here $-(\pi\coth(\pi a)/a) < 0$, which is wrong.

The actual result is $\sum_{n=-\infty}^\infty 1/(n^2+a^2) = \pi\coth(\pi a)/a$ (this is a standard result). The sum of non-integer residues must be $-\pi\coth(\pi a)/a$, so there is a sign error in the residues above.

Let me recompute carefully: $\cot(-i\pi a) = \cos(-i\pi a)/\sin(-i\pi a) = \cosh(\pi a)/(-i\sinh(\pi a)) = i\coth(\pi a)$.

Residue at $z = -ia$: $\pi \cdot i\coth(\pi a) / (-2ia) = i\pi\coth(\pi a)/(-2ia) = \pi\coth(\pi a)/(-2a) = -\pi\coth(\pi a)/(2a)$.

Total: $\pi\coth(\pi a)/(2a) + (-\pi\coth(\pi a)/(2a)) = 0$. That can't be right either.

I see the error: $\mathrm{Res}(h; -ia) = \pi\cot(-i\pi a) / (2 \cdot (-ia))$: the denominator factor from $z^2 + a^2 = (z-ia)(z+ia)$ gives at $z = -ia$: $\lim_{z\to -ia}(z+ia) \cdot \pi\cot(\pi z)/(z^2+a^2) = \pi\cot(-i\pi a)/(-ia - ia) = \pi\cot(-i\pi a)/(-2ia) = \pi(i\coth(\pi a))/(-2ia) = i\pi\coth(\pi a)/(-2ia) = \pi\coth(\pi a)/(-2a)$.

So residues are $+\pi\coth(\pi a)/(2a)$ and $-\pi\coth(\pi a)/(2a)$. Sum $= 0$. That gives $\sum f(n) = 0$, also wrong.

I believe the issue is that the formula $\sum f(n) = -\sum \mathrm{Res}(\pi\cot\cdot f; w_k)$ and the actual correct sum is $\pi\coth(\pi a)/a$, so the sum of residues at non-integer poles must be $-\pi\coth(\pi a)/a$. This requires $-\pi\coth(\pi a)/(2a) + (-\pi\coth(\pi a)/(2a)) = -\pi\coth(\pi a)/a$, which means both residues are $-\pi\coth(\pi a)/(2a)$.

Let me carefully use $\cot(-i\pi a) = i\coth(\pi a)$ and recompute at $z = ia$: $\pi\cot(i\pi a)/(2ia) = \pi(-i\coth(\pi a))/(2ia) = -i\pi\coth(\pi a)/(2ia) = -\pi\coth(\pi a)/(2a)$. Negative!

And at $z = -ia$: $\pi\cot(-i\pi a)/(-2ia) = \pi(i\coth(\pi a))/(-2ia) = i\pi\coth(\pi a)/(-2ia) = -\pi\coth(\pi a)/(2a)$. Also negative!

So the sum of non-integer residues $= -\pi\coth(\pi a)/a$, and $\sum f(n) = -(-\pi\coth(\pi a)/a) = \pi\coth(\pi a)/a$. Correct! My earlier computation of the residue at $ia$ was wrong.

$$\boxed{\sum_{n=-\infty}^\infty \frac{1}{n^2+a^2} = \frac{\pi\coth(\pi a)}{a}.}$$

**Example 3.** Euler's formula for $\zeta(2k)$: by applying the cotangent method to $f(z) = 1/z^{2k}$ (removing the pole at $0$), one recovers $\sum_{n=1}^\infty 1/n^{2k} = (-1)^{k+1}(2\pi)^{2k}B_{2k}/(2\cdot(2k)!)$ where $B_{2k}$ are Bernoulli numbers.

## The Cosecant Kernel

For alternating series $\sum_{n=-\infty}^\infty (-1)^n f(n)$, use $\pi/\sin(\pi z)$ instead: it has simple poles at integers with $\mathrm{Res} = (-1)^n$.
$$\sum_{n=-\infty}^\infty (-1)^n f(n) = -\sum_k \mathrm{Res}\!\left(\frac{\pi f(z)}{\sin(\pi z)}; w_k\right).$$
