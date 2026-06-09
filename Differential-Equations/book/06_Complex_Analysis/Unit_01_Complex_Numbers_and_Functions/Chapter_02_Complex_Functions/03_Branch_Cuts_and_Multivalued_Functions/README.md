# Branch Cuts and Multivalued Functions

Consider the equation $e^w = z$ for a given nonzero $z \in \mathbb{C}$. Writing $z = re^{i\theta}$ and $w = u + iv$, this requires $e^u = r$ and $e^{iv} = e^{i\theta}$, so $u = \ln r$ and $v = \theta + 2\pi k$ for any integer $k$. The equation has not one solution but infinitely many, differing by multiples of $2\pi i$. This is the phenomenon of multivaluedness, and the function "inverse to $e^z$" — the complex logarithm — is the canonical example of a multivalued function. This section develops the theory of branch cuts and branches, which is the machinery for converting multivalued expressions into honest single-valued functions on suitably restricted domains.

## The Complex Logarithm

**Definition.** For $z \neq 0$, the (multivalued) complex logarithm is
$$\log z = \ln|z| + i\arg z,$$
where $\arg z$ ranges over all arguments of $z$: any two logarithms of $z$ differ by an integer multiple of $2\pi i$.

More precisely, the set of all logarithms of $z$ is $\{\ln|z| + i(\theta_0 + 2\pi k) : k \in \mathbb{Z}\}$ where $\theta_0$ is any particular argument. For example:
$$\log(-1) = \ln 1 + i(\pi + 2\pi k) = i(2k+1)\pi, \quad k \in \mathbb{Z}: \quad \ldots, -3\pi i, -\pi i, \pi i, 3\pi i, \ldots$$
$$\log(i) = \ln 1 + i(\pi/2 + 2\pi k) = i(\pi/2 + 2\pi k), \quad k \in \mathbb{Z}: \quad \ldots, -3\pi i/2, \pi i/2, 5\pi i/2, \ldots$$

## Branches and the Principal Branch

To define a genuine function from the multivalued expression $\log z$, we fix a continuous, single-valued determination of $\arg z$ on some domain. Such a determination is called a branch of $\arg z$, and the corresponding single-valued logarithm is called a branch of $\log z$.

**Definition.** The principal branch of the logarithm is
$$\mathrm{Log}\, z = \ln|z| + i\,\mathrm{Arg}\, z, \qquad z \neq 0,$$
where $\mathrm{Arg}\, z \in (-\pi, \pi]$ is the principal argument.

The principal branch is defined and continuous on $\mathbb{C} \setminus (-\infty, 0]$: the domain is the complex plane with the nonpositive real axis removed. This removed ray is the branch cut of the principal logarithm.

**Why the cut is necessary.** On the circle $|z| = 1$, as $z$ moves counterclockwise from a point just below the negative real axis to a point just above it, $\mathrm{Arg}\, z$ jumps from $-\pi$ to $\pi$. No continuous determination of the argument can be defined at points on this cut, so it must be excluded from the domain.

## Branch Cuts and Topology

The need for a branch cut reflects a topological obstruction. The argument function $\arg z$ cannot be made continuous on all of $\mathbb{C} \setminus \{0\}$ because the fundamental group $\pi_1(\mathbb{C} \setminus \{0\}) \cong \mathbb{Z}$ is nontrivial: loops around the origin cannot be contracted to a point. Traversing a loop around the origin once increases $\arg z$ by $2\pi$, so no continuous global inverse of $e^{i\theta} \mapsto \theta$ can exist on the punctured plane.

A branch cut is any curve from $0$ to $\infty$ whose removal makes the remaining domain simply connected. On a simply connected domain, a continuous branch of $\arg z$ can always be chosen (this is a consequence of the monodromy theorem). The negative real axis is the conventional choice, but any ray from the origin would work equally well.

**Example of an alternative branch.** Take the branch cut along the positive real axis. Then we can define a branch of $\arg z$ on $\mathbb{C} \setminus [0, \infty)$ by $\arg z \in (0, 2\pi)$. The corresponding branch of the logarithm takes the value $\log(-1) = \pi i$ under the principal branch, but $\log(-1) = \pi i$ remains the same here (since $-1$ is not on the positive real axis). However, $\log(1)$ is now $2\pi i$ along the upper side of the cut, not $0$.

## The Multivalued Square Root

The function $z^{1/2}$ is also multivalued: every nonzero $z$ has exactly two square roots, $\pm \sqrt{|z|}\,e^{i\arg z/2}$. Writing $z = re^{i\theta}$:
$$z^{1/2} = r^{1/2} e^{i(\theta + 2\pi k)/2}, \quad k = 0, 1.$$
For $k = 0$: $r^{1/2} e^{i\theta/2}$. For $k = 1$: $r^{1/2} e^{i(\theta/2 + \pi)} = -r^{1/2}e^{i\theta/2}$.

The principal branch of the square root is $z^{1/2} = \sqrt{|z|}\,e^{i\,\mathrm{Arg}(z)/2}$, defined on $\mathbb{C} \setminus (-\infty, 0]$ with branch cut along the nonpositive real axis.

**Worked example.** Compute the principal value of $(-4)^{1/2}$.

$\mathrm{Arg}(-4) = \pi$, $|-4| = 4$, so $(-4)^{1/2} = 2 e^{i\pi/2} = 2i$. $\square$

This is consistent with the familiar fact that $\sqrt{-4} = 2i$ when we take the positive imaginary root.

## Analyticity of Branches

Once a branch is selected, the resulting single-valued function is analytic on its domain. Specifically:

**Theorem.** The principal branch $\mathrm{Log}\, z$ is analytic on $\mathbb{C} \setminus (-\infty, 0]$, with derivative
$$\frac{d}{dz}\mathrm{Log}\, z = \frac{1}{z}.$$

**Proof sketch.** For $z$ not on the branch cut, write $\mathrm{Log}\, z = \ln|z| + i\,\mathrm{Arg}\, z$ and verify the Cauchy-Riemann equations for $u = \ln\sqrt{x^2+y^2}$ and $v = \arctan(y/x)$ (in the appropriate quadrant). Computing partial derivatives and checking the equations yields $u_x = v_y$ and $u_y = -v_x$, and the common value of $u_x - i u_y$ is $x/(x^2+y^2) - iy/(x^2+y^2) = 1/(x+iy) = 1/z$. $\square$

More generally, any branch of $\log z$ is analytic with derivative $1/z$ on its domain of definition.

## General Power Functions

For $\alpha \in \mathbb{C}$ and $z \neq 0$, the multivalued power function is defined by
$$z^\alpha = e^{\alpha \log z}.$$
Because $\log z$ is multivalued, so is $z^\alpha$ in general. The principal value is
$$z^\alpha = e^{\alpha\, \mathrm{Log}\, z}, \qquad z \in \mathbb{C} \setminus (-\infty, 0].$$

**Special cases:**
- If $\alpha = n \in \mathbb{Z}$: $z^n = e^{n\log z}$ is single-valued (all branches give the same value).
- If $\alpha = 1/n$: $z^{1/n}$ has $n$ branches, corresponding to the $n$ choices of $\arg z$.
- If $\alpha$ is irrational: $z^\alpha$ has infinitely many branches.

**Worked example.** Compute all values of $i^i$.

$$i^i = e^{i \log i} = e^{i(\pi/2 + 2\pi k)i} = e^{i^2(\pi/2 + 2\pi k)} = e^{-(\pi/2 + 2\pi k)}, \quad k \in \mathbb{Z}.$$
All values are real and positive! The principal value (using $k = 0$) is $e^{-\pi/2} \approx 0.2079$. $\square$

## Worked Example: Choosing a Branch for a Computation

**Example.** Evaluate $\int_C \frac{dz}{\sqrt{z}}$ along the arc $C$ from $1$ to $-1$ traversing the upper half of the unit circle $|z| = 1$, using the principal branch of $z^{1/2}$.

Parametrize $C$ by $z = e^{i\theta}$, $\theta \in [0, \pi]$. Then $dz = ie^{i\theta}\,d\theta$ and
$$(e^{i\theta})^{1/2} = e^{i\theta/2}$$
(principal branch, since $\theta/2 \in [0, \pi/2] \subset (-\pi, \pi)$). The integral becomes
$$\int_0^{\pi} \frac{1}{e^{i\theta/2}} \cdot ie^{i\theta}\, d\theta = i\int_0^{\pi} e^{i\theta/2}\, d\theta = i\left[\frac{2}{i} e^{i\theta/2}\right]_0^{\pi} = 2(e^{i\pi/2} - 1) = 2(i - 1).$$

This computation depended critically on the choice of branch: a different branch would give a different value of $\sqrt{z}$ at each point, and the computation would change. $\square$

## Summary

Multivalued functions arise naturally in complex analysis whenever a function is defined as an inverse of a many-to-one map. The fundamental examples are the logarithm, the power functions $z^{1/n}$, and the inverse trigonometric functions. In each case, the remedy is to introduce a branch cut — a curve in the domain whose removal makes the remaining domain simply connected — and to choose a single-valued determination of the function on this restricted domain. The resulting branch is then analytic, and its derivative is computed by the chain rule from the known derivative of the original function.
