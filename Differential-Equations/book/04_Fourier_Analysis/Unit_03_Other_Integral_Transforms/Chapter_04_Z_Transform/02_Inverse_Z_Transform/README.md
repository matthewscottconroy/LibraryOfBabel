# The Inverse Z-Transform

Given the Z-transform $X(z)$ of an unknown sequence, recovering the sequence $(x_n)$ is the inverse Z-transform problem. Three methods are standard: partial fraction decomposition with table lookup, power series expansion, and the contour integral formula (a direct analog of the Bromwich integral for the Laplace transform).

## The Contour Integral Formula

The Z-transform is $X(z) = \sum_{n=0}^\infty x_n z^{-n}$, which is a Laurent series in $z^{-1}$. By Cauchy's coefficient theorem, $x_n$ is the coefficient of $z^{-n}$ in the Laurent expansion:
$$x_n = \frac{1}{2\pi i}\oint_C X(z)\,z^{n-1}\,dz,$$
where $C$ is a closed contour (traversed counterclockwise) in the ROC of $X(z)$. By the residue theorem, this equals the sum of residues of $X(z)z^{n-1}$ at poles inside $C$.

**Proof.** $\oint_C X(z)z^{n-1}\,dz = \sum_{k=0}^\infty x_k \oint_C z^{n-1-k}\,dz = x_n \cdot 2\pi i$, since $\oint z^m\,dz = 2\pi i\delta_{m,-1}$ for any simple closed contour encircling the origin.

## Method 1: Partial Fractions

For rational $X(z) = B(z)/A(z)$, factor the denominator $A(z) = \prod_k (z - p_k)^{m_k}$ (where $p_k$ are the poles of $X$). Write:
$$\frac{X(z)}{z} = \sum_k \sum_{j=1}^{m_k} \frac{A_{kj}}{(z-p_k)^j}.$$
Then multiply through by $z$ and use the table of inverse Z-transforms for each term.

**Example.** $X(z) = \frac{z}{(z-1)(z-2)}$ for $|z| > 2$.

$$\frac{X(z)}{z} = \frac{1}{(z-1)(z-2)} = \frac{A}{z-1} + \frac{B}{z-2}.$$
$A = \frac{1}{z-2}\big|_{z=1} = -1$, $B = \frac{1}{z-1}\big|_{z=2} = 1$.

So $X(z) = \frac{-z}{z-1} + \frac{z}{z-2}$, giving $x_n = -1^n + 2^n = 2^n - 1$ for $n \geq 0$.

**Verification:** $X(z) = \sum_{n=0}^\infty (2^n - 1)z^{-n} = \frac{z}{z-2} - \frac{z}{z-1}$. The partial fractions check out.

## Method 2: Power Series Expansion (Long Division)

Expand $X(z)$ as a power series in $z^{-1}$: the coefficient of $z^{-n}$ is $x_n$.

**Example.** $X(z) = \frac{z}{z - 0.5} = \frac{1}{1 - 0.5z^{-1}}$ for $|z| > 0.5$.

Geometric series: $X(z) = \sum_{n=0}^\infty (0.5)^n z^{-n}$, so $x_n = (0.5)^n = (1/2)^n$.

**More complex example.** $X(z) = \frac{1 + z^{-1}}{1 - z^{-1} + 0.5z^{-2}}$.

Perform polynomial long division of $1 + z^{-1}$ by $1 - z^{-1} + 0.5z^{-2}$:

$1 + z^{-1} = (1)(1 - z^{-1} + 0.5z^{-2}) + \text{remainder} = 1 - z^{-1} + 0.5z^{-2} + \text{rem}$...

Subtract: remainder $= (1 + z^{-1}) - (1 - z^{-1} + 0.5z^{-2}) = 2z^{-1} - 0.5z^{-2}$.

So $X(z) = 1 + \frac{2z^{-1} - 0.5z^{-2}}{1 - z^{-1} + 0.5z^{-2}}$.

Continue dividing: $2z^{-1} - 0.5z^{-2} = 2z^{-1}(1 - z^{-1} + 0.5z^{-2}) + \text{rem}$, remainder $= (2z^{-1} - 0.5z^{-2}) - (2z^{-1} - 2z^{-2} + z^{-3}) = 1.5z^{-2} - z^{-3}$.

So far: $X(z) = 1 + 2z^{-1} + \frac{1.5z^{-2} - z^{-3}}{1 - z^{-1} + 0.5z^{-2}}$, giving $x_0 = 1, x_1 = 2, \ldots$

## Method 3: Residue Computation

**Example.** $X(z) = \frac{z^2}{(z-1)(z-0.5)}$, ROC: $|z| > 1$.

The formula is $x_n = \sum_k \text{Res}_{z=p_k}\left[X(z)z^{n-1}\right]$ for poles $p_k$ inside $C$.

For the ROC $|z| > 1$, take $C$ to be a circle of radius $R > 1$. Both poles $z = 1$ and $z = 0.5$ are inside $C$ for $n \geq 0$.

$\text{Res}_{z=1}\left[\frac{z^2\cdot z^{n-1}}{(z-1)(z-0.5)}\right] = \frac{1^{n+1}}{1-0.5} = 2$.

$\text{Res}_{z=0.5}\left[\frac{z^{n+1}}{(z-1)(z-0.5)}\right] = \frac{(0.5)^{n+1}}{0.5-1} = \frac{(0.5)^{n+1}}{-0.5} = -(0.5)^n$.

So $x_n = 2 - (0.5)^n$ for $n \geq 0$.

## Multiple Poles (Repeated Roots)

When $X(z)$ has a pole of order $m$ at $z = p$:
$$\text{Res}_{z=p}\left[X(z)z^{n-1}\right] = \frac{1}{(m-1)!}\lim_{z\to p}\frac{d^{m-1}}{dz^{m-1}}\left[(z-p)^m X(z)z^{n-1}\right].$$

**Example.** $X(z) = \frac{z}{(z-a)^2}$.

$X(z)z^{n-1} = \frac{z^n}{(z-a)^2}$.
$\text{Res}_{z=a} = \frac{d}{dz}[z^n]\big|_{z=a} = na^{n-1}$.
So $x_n = na^{n-1}$ for $n \geq 0$.

This corresponds to $\mathcal{Z}[na^{n-1}](z) = z/(z-a)^2$, consistent with the table entry for $na^n$ (shifted by one in $n$).

## Stability and the Inverse Transform

The behavior of $x_n$ for large $n$ is determined by the pole of $X(z)$ with the largest modulus inside the ROC boundary:
- All poles inside $|z| < 1$: $x_n \to 0$ as $n \to \infty$ (stable, decaying).
- A pole at $z = 1$: $x_n$ approaches a nonzero constant.
- A pole at $|z| > 1$: $x_n$ grows exponentially (unstable).
- Complex poles at $z = re^{\pm i\omega}$: $x_n \sim r^n\cos(\omega n + \phi)$ (oscillating with exponential envelope).

The inverse Z-transform via partial fractions makes these contributions explicit, allowing engineering intuition about system behavior.
