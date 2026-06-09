# Computing Residues

The residue of a meromorphic function at an isolated singularity is the coefficient of $(z - z_0)^{-1}$ in its Laurent series. It is a single complex number that encodes the "singular behavior" of the function at $z_0$, and it is the quantity that determines the contribution of the singularity to a contour integral via the residue theorem. This section develops all the practical formulas for computing residues, organized by the type and order of the singularity.

## Definition

**Definition.** Let $f$ have an isolated singularity at $z_0$, with Laurent expansion $f(z) = \sum_{n=-\infty}^\infty a_n(z-z_0)^n$ valid in $0 < |z-z_0| < R$. The residue of $f$ at $z_0$ is:
$$\mathrm{Res}(f; z_0) = a_{-1}.$$

By the Laurent coefficient formula:
$$\mathrm{Res}(f; z_0) = \frac{1}{2\pi i}\oint_{|z-z_0|=r} f(z)\, dz$$
for any $r \in (0, R)$.

## Simple Poles ($m = 1$)

If $f$ has a simple pole at $z_0$, then $f(z) = a_{-1}/(z-z_0) + a_0 + a_1(z-z_0) + \cdots$, so $(z-z_0)f(z) \to a_{-1}$ as $z \to z_0$.

**Formula:**
$$\mathrm{Res}(f; z_0) = \lim_{z \to z_0}(z - z_0)f(z).$$

**Quotient formula.** If $f = p/q$ where $p$ and $q$ are analytic at $z_0$, $p(z_0) \neq 0$, and $q$ has a simple zero at $z_0$ (i.e., $q(z_0) = 0$, $q'(z_0) \neq 0$):
$$\mathrm{Res}(f; z_0) = \frac{p(z_0)}{q'(z_0)}.$$

**Proof.** $(z-z_0)f(z) = (z-z_0)p(z)/q(z) = p(z)/\frac{q(z)-q(z_0)}{z-z_0} \to p(z_0)/q'(z_0)$. $\square$

**Worked examples.**

$\mathrm{Res}\!\left(\frac{e^z}{z}, 0\right) = e^0 = 1$.

$\mathrm{Res}\!\left(\frac{1}{z^2 + 1}, i\right) = \frac{1}{(z^2+1)'|_{z=i}} = \frac{1}{2i}$.

$\mathrm{Res}\!\left(\frac{z^2}{z^2 + z - 2}, 1\right)$: $z^2 + z - 2 = (z-1)(z+2)$. At $z = 1$: $p(1) = 1$, $q'(z) = 2z+1$, $q'(1) = 3$. Residue $= 1/3$.

## Poles of Order $m$

If $f$ has a pole of order $m$ at $z_0$:

**Formula:**
$$\mathrm{Res}(f; z_0) = \frac{1}{(m-1)!}\lim_{z \to z_0}\frac{d^{m-1}}{dz^{m-1}}\left[(z-z_0)^m f(z)\right].$$

**Derivation.** Write $f(z) = \sum_{n=-m}^\infty a_n(z-z_0)^n$. Then $(z-z_0)^m f(z) = \sum_{n=0}^\infty a_{n-m}(z-z_0)^n = a_{-m} + a_{-m+1}(z-z_0) + \cdots + a_{-1}(z-z_0)^{m-1} + \cdots$. Differentiating $m-1$ times and evaluating at $z_0$ picks out $(m-1)! a_{-1}$.

**Worked examples.**

$\mathrm{Res}\!\left(\frac{e^z}{z^2}, 0\right)$: pole of order $2$. $(z-0)^2 \cdot \frac{e^z}{z^2} = e^z$. $\frac{d}{dz}e^z|_{z=0} = 1$. Residue $= 1/1! \cdot 1 = 1$.

$\mathrm{Res}\!\left(\frac{\cos z}{z^3}, 0\right)$: pole of order $3$. $(z-0)^3 \cdot \frac{\cos z}{z^3} = \cos z$. $\frac{d^2}{dz^2}\cos z|_{z=0} = -\cos(0) = -1$. Residue $= \frac{1}{2!}(-1) = -\frac{1}{2}$.

$\mathrm{Res}\!\left(\frac{z}{(z-1)^2(z+2)}, 1\right)$: pole of order $2$ at $z = 1$. $(z-1)^2 f(z) = z/(z+2)$. $\frac{d}{dz}\frac{z}{z+2}\big|_{z=1} = \frac{(z+2) - z}{(z+2)^2}\big|_{z=1} = \frac{2}{9}$. Residue $= \frac{1}{1!}\cdot\frac{2}{9} = \frac{2}{9}$.

## Residues at Essential Singularities

For essential singularities, none of the above formulas apply. One must expand the Laurent series directly.

**Worked example.** $\mathrm{Res}(e^{1/z}; 0)$.

$e^{1/z} = 1 + \frac{1}{z} + \frac{1}{2!z^2} + \cdots$. The coefficient of $z^{-1}$ is $1$. So $\mathrm{Res}(e^{1/z}; 0) = 1$.

**Worked example.** $\mathrm{Res}(z^2 e^{1/z}; 0)$.

$z^2 e^{1/z} = z^2\left(1 + \frac{1}{z} + \frac{1}{2z^2} + \frac{1}{6z^3} + \cdots\right) = z^2 + z + \frac{1}{2} + \frac{1}{6z} + \cdots$. Residue $= 1/6$.

## Residues via Series Manipulation

Often the quickest method is to extract the $z^{-1}$ coefficient by multiplying or substituting series.

**Worked example.** $\mathrm{Res}\!\left(\frac{\sin z}{z^4}, 0\right)$.

$\frac{\sin z}{z^4} = \frac{1}{z^4}\left(z - \frac{z^3}{6} + \frac{z^5}{120} - \cdots\right) = \frac{1}{z^3} - \frac{1}{6z} + \frac{z}{120} - \cdots$. Residue $= -1/6$.

**Worked example.** $\mathrm{Res}\!\left(\frac{1}{z^2\sin z}, 0\right)$.

$\sin z = z - z^3/6 + z^5/120 - \cdots = z(1 - z^2/6 + z^4/120 - \cdots)$.
$\frac{1}{\sin z} = \frac{1}{z}\cdot\frac{1}{1 - z^2/6 + \cdots} = \frac{1}{z}(1 + z^2/6 + \cdots)$ (using $1/(1-u) \approx 1 + u$ for small $u = z^2/6 - \cdots$).
$\frac{1}{z^2\sin z} = \frac{1}{z^3}(1 + z^2/6 + \cdots) = \frac{1}{z^3} + \frac{1}{6z} + \cdots$. Residue $= 1/6$.

## Summary Table of Formulas

| Singularity type | Formula |
|---|---|
| Simple pole | $\lim_{z \to z_0}(z-z_0)f(z)$ |
| Simple pole of $p/q$ | $p(z_0)/q'(z_0)$ |
| Pole of order $m$ | $\frac{1}{(m-1)!}\lim \frac{d^{m-1}}{dz^{m-1}}[(z-z_0)^m f(z)]$ |
| Essential singularity | Read off $a_{-1}$ from Laurent series |

## Residues and Antiderivatives

The residue at a singularity measures the failure of a local antiderivative to exist globally. If $\mathrm{Res}(f; z_0) = 0$, then $f$ has a single-valued antiderivative in a punctured neighborhood of $z_0$ (the pole or essential singularity is "integrable away"). If $\mathrm{Res}(f; z_0) \neq 0$, the antiderivative acquires a logarithmic branch cut: the integral $\int f\, dz$ around any small loop enclosing $z_0$ is $2\pi i \cdot \mathrm{Res}(f; z_0) \neq 0$, so no single-valued antiderivative can exist near $z_0$.

For example, $1/z$ has residue $1$ at $0$, and its antiderivative is $\log z$, which is multivalued with a $2\pi i$ monodromy around $0$. The function $1/z^2$ has residue $0$ at $0$, and its antiderivative $-1/z$ is single-valued on $\mathbb{C} \setminus \{0\}$.
