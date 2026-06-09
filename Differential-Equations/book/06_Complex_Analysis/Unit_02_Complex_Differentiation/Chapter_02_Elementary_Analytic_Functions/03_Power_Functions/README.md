# Power Functions

For real positive $x$ and real $\alpha$, the power function $x^\alpha = e^{\alpha \ln x}$ is unambiguous. For complex $z$ and general $\alpha \in \mathbb{C}$, the analogous definition $z^\alpha = e^{\alpha \log z}$ immediately inherits the multivaluedness of the complex logarithm. Understanding this multivaluedness, selecting branches, and computing with them is essential for contour integration and conformal mapping. This section develops the theory of complex power functions systematically, from integer powers through irrational and complex exponents.

## Definition

**Definition.** For $z \neq 0$ and $\alpha \in \mathbb{C}$, the (multivalued) power function is
$$z^\alpha = e^{\alpha \log z} = e^{\alpha(\ln|z| + i\arg z)}.$$

Different choices of $\arg z$ — differing by multiples of $2\pi$ — yield different values of $z^\alpha$:
$$z^\alpha = e^{\alpha(\ln|z| + i(\theta_0 + 2\pi k))} = e^{\alpha\ln|z|} e^{i\alpha(\theta_0 + 2\pi k)}, \quad k \in \mathbb{Z}.$$

The set of values is $\{e^{\alpha\ln|z|} e^{i\alpha\theta_0} \cdot e^{2\pi i\alpha k} : k \in \mathbb{Z}\}$.

## How Many Values Does $z^\alpha$ Have?

- **$\alpha = n \in \mathbb{Z}$:** $e^{2\pi i n k} = 1$ for all $k$, so $z^n$ is single-valued (consistent with ordinary integer powers).
- **$\alpha = p/q \in \mathbb{Q}$ in lowest terms:** $e^{2\pi i (p/q) k}$ takes $q$ distinct values as $k$ ranges over $\{0, 1, \ldots, q-1\}$. So $z^{p/q}$ has exactly $q$ values (the $q$-th roots of $z^p$).
- **$\alpha \notin \mathbb{Q}$:** $e^{2\pi i\alpha k}$ is never repeated (since $\alpha k \neq \alpha j \pmod{1}$ for $k \neq j$ when $\alpha$ is irrational), so $z^\alpha$ has infinitely many values.
- **$\alpha \in \mathbb{C} \setminus \mathbb{R}$:** Again infinitely many values, as the exponentials $e^{2\pi i\alpha k}$ for $k \in \mathbb{Z}$ are all distinct.

## The Principal Branch

**Definition.** The principal value of $z^\alpha$ is
$$z^\alpha = e^{\alpha\,\mathrm{Log}\, z}, \qquad z \in \mathbb{C} \setminus (-\infty, 0].$$

This is a single-valued function analytic on $\mathbb{C} \setminus (-\infty, 0]$ (since $e^w$ is entire and $\mathrm{Log}\, z$ is analytic on that domain, and compositions of analytic functions are analytic).

**Derivative.** By the chain rule:
$$\frac{d}{dz} z^\alpha = \frac{d}{dz} e^{\alpha\,\mathrm{Log}\, z} = e^{\alpha\,\mathrm{Log}\, z} \cdot \frac{\alpha}{z} = \alpha z^{\alpha - 1}.$$
The formal power rule $\frac{d}{dz} z^\alpha = \alpha z^{\alpha-1}$ holds for any branch of $z^\alpha$ and $z^{\alpha-1}$ defined on the same domain.

## Worked Examples

**Example 1.** Compute all values of $(-1)^\alpha$ where $\alpha = 1/2$.

$(-1)^{1/2} = e^{(1/2)\log(-1)} = e^{(1/2)(\pi i + 2\pi ki)} = e^{i\pi(1/2 + k)}$ for $k \in \mathbb{Z}$.
- $k = 0$: $e^{i\pi/2} = i$.
- $k = 1$: $e^{i3\pi/2} = -i$.
- $k = 2$: $e^{i5\pi/2} = i$ (repeats).

So the two square roots of $-1$ are $i$ and $-i$. $\square$

**Example 2.** Compute the principal value of $(1 + i)^{1-i}$.

$\mathrm{Log}(1+i) = \ln\sqrt{2} + i\pi/4$.
$(1+i)^{1-i} = e^{(1-i)(\frac{1}{2}\ln 2 + i\pi/4)} = e^{\frac{1}{2}\ln 2 + i\pi/4 - i\frac{1}{2}\ln 2 + \pi/4}$.
$= e^{(\frac{1}{2}\ln 2 + \pi/4) + i(\pi/4 - \frac{1}{2}\ln 2)}$.
$= e^{(\ln\sqrt{2} + \pi/4)} \cdot e^{i(\pi/4 - \ln\sqrt{2})}$.

The modulus is $\sqrt{2}\, e^{\pi/4}$ and the argument is $\pi/4 - \frac{1}{2}\ln 2$. $\square$

**Example 3.** Compute all values of $i^i$.

$\log i = \frac{\pi}{2}i + 2\pi ki$, $k \in \mathbb{Z}$.
$i^i = e^{i \cdot i(\frac{\pi}{2} + 2\pi k)} = e^{-(\pi/2 + 2\pi k)}$, $k \in \mathbb{Z}$.

All values are real and positive: $\ldots, e^{3\pi/2}, e^{-\pi/2}, e^{-5\pi/2}, \ldots$. The principal value is $e^{-\pi/2} \approx 0.2079$.

## Branch Points

**Definition.** A point $z_0$ is a branch point of $z^\alpha$ if any loop around $z_0$ in the domain produces a change in the value of $z^\alpha$. Equivalently, it is a point where the multivalued function cannot be made continuous in any punctured neighborhood.

For $z^\alpha$ with non-integer $\alpha$:
- $z = 0$ is a branch point: traversing a circle $|z| = r$ once increases $\arg z$ by $2\pi$, changing $z^\alpha$ by the factor $e^{2\pi i\alpha} \neq 1$.
- $z = \infty$ is also a branch point (visible by substituting $w = 1/z$).

The branch cut connecting the two branch points $\{0, \infty\}$ is typically the negative real axis (for the principal branch).

## Analytic Continuation Between Branches

Different branches of $z^\alpha$ are related by analytic continuation. Starting with the principal branch on $\mathbb{C} \setminus (-\infty, 0]$ and continuing analytically across the branch cut from below, the argument of $z$ increases through $\pi$ to $\pi + \epsilon$ (instead of jumping to $-\pi$), and we enter the next branch of the logarithm, where $\arg z \in (\pi, 3\pi)$. This next branch of $z^\alpha$ is $e^{\alpha(\ln|z| + i\theta)}$ for $\theta \in (\pi, 3\pi)$, which differs from the principal branch by the factor $e^{2\pi i\alpha}$.

Continuing around the origin $n$ times multiplies the value of $z^\alpha$ by $e^{2\pi in\alpha}$. For rational $\alpha = p/q$, after $q$ full circuits we return to the original branch. For irrational $\alpha$, the function never returns to its original value: the Riemann surface has infinitely many sheets.

## Connection to $n$-th Roots

For $\alpha = 1/n$, the formula $z^{1/n} = e^{(1/n)\log z}$ yields the $n$-th root function. The $n$ values correspond to $k = 0, 1, \ldots, n-1$ in $\log z = \ln|z| + i(\theta + 2\pi k)$:
$$z^{1/n}_k = |z|^{1/n} e^{i(\theta + 2\pi k)/n}, \quad k = 0, 1, \ldots, n-1.$$
These are equally spaced on the circle of radius $|z|^{1/n}$, as established in Chapter 01.

## Use in Contour Integration

Power functions with non-integer exponents appear frequently as integrands in contour integration problems. Integrals of the form $\int_0^\infty x^\alpha f(x)\, dx$ for $-1 < \mathrm{Re}(\alpha) < 0$ are evaluated by completing a keyhole contour (a Pac-Man shaped region that avoids the branch cut on the positive real axis). The two contributions along the branch cut give a factor of $e^{2\pi i\alpha} - 1$, which cancels against residue contributions to yield the real integral. This technique is developed in Unit 04.
