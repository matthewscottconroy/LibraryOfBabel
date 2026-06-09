# Trigonometric and Hyperbolic Functions

The complex trigonometric and hyperbolic functions are defined through the exponential, and this definition immediately uncovers a network of algebraic identities connecting sine, cosine, sinh, and cosh that is invisible on the real line. In the complex domain, these functions are entire, satisfy the same recurrence and addition formulas as their real counterparts, and exhibit behavior — such as growing without bound — that contradicts real intuition. This section defines and analyzes each function systematically.

## The Complex Exponential Revisited

Euler's formula $e^{iy} = \cos y + i\sin y$ motivates the definitions below. For complex $z$, we have both $e^{iz} = \cos z + i\sin z$ and $e^{-iz} = \cos z - i\sin z$ (formally). Adding and subtracting:
$$\cos z = \frac{e^{iz} + e^{-iz}}{2}, \qquad \sin z = \frac{e^{iz} - e^{-iz}}{2i}.$$
These are taken as definitions for complex $z$.

## Complex Sine and Cosine

**Definition.** For $z \in \mathbb{C}$:
$$\sin z = \frac{e^{iz} - e^{-iz}}{2i}, \qquad \cos z = \frac{e^{iz} + e^{-iz}}{2}.$$

Both functions are entire (as combinations of entire functions). Their derivatives are:
$$\frac{d}{dz}\sin z = \cos z, \qquad \frac{d}{dz}\cos z = -\sin z,$$
which follow immediately from $(e^{iz})' = ie^{iz}$ and $(e^{-iz})' = -ie^{-iz}$.

Writing $z = x + iy$ and using $e^{i(x+iy)} = e^{-y+ix} = e^{-y}(\cos x + i\sin x)$:
$$\sin(x + iy) = \sin x \cosh y + i\cos x \sinh y,$$
$$\cos(x + iy) = \cos x \cosh y - i\sin x \sinh y.$$

**Moduli:**
$$|\sin z|^2 = \sin^2 x + \sinh^2 y, \qquad |\cos z|^2 = \cos^2 x + \sinh^2 y.$$
Since $\sinh y \to \infty$ as $|y| \to \infty$, both $\sin z$ and $\cos z$ are unbounded on $\mathbb{C}$. This is consistent with Liouville's theorem: no nonconstant entire function can be bounded.

**Zeros.** $\sin z = 0 \iff e^{iz} = e^{-iz} \iff e^{2iz} = 1 \iff 2iz \in 2\pi i\mathbb{Z} \iff z = n\pi$, $n \in \mathbb{Z}$. The zeros of $\sin z$ on $\mathbb{C}$ are exactly the real zeros $n\pi$.

Similarly, $\cos z = 0 \iff z = \pi/2 + n\pi$, $n \in \mathbb{Z}$.

## Standard Identities

All classical identities extend to $\mathbb{C}$:
$$\sin^2 z + \cos^2 z = 1,$$
$$\sin(z + w) = \sin z\cos w + \cos z\sin w,$$
$$\cos(z + w) = \cos z\cos w - \sin z\sin w,$$
$$\sin(z + 2\pi) = \sin z, \qquad \cos(z + 2\pi) = \cos z.$$

These follow from the definitions via straightforward algebra with exponentials.

**Worked example.** Compute $\sin(1 + i)$.

$$\sin(1 + i) = \sin 1 \cosh 1 + i\cos 1 \sinh 1.$$
Numerically: $\sin 1 \approx 0.8415$, $\cosh 1 \approx 1.5431$, $\cos 1 \approx 0.5403$, $\sinh 1 \approx 1.1752$.
$$\sin(1+i) \approx 0.8415 \times 1.5431 + i \times 0.5403 \times 1.1752 \approx 1.2985 + 0.6350i.$$

## The Other Trigonometric Functions

$$\tan z = \frac{\sin z}{\cos z}, \quad \cot z = \frac{\cos z}{\sin z}, \quad \sec z = \frac{1}{\cos z}, \quad \csc z = \frac{1}{\sin z}.$$

$\tan z$ and $\sec z$ are analytic wherever $\cos z \neq 0$, i.e., on $\mathbb{C} \setminus \{\pi/2 + n\pi : n \in \mathbb{Z}\}$. The function $\pi\cot(\pi z)$ has simple poles at every integer with residue $1$, making it the key tool for summing infinite series via residues (Unit 04).

## Complex Hyperbolic Functions

**Definition.** For $z \in \mathbb{C}$:
$$\sinh z = \frac{e^z - e^{-z}}{2}, \qquad \cosh z = \frac{e^z + e^{-z}}{2}.$$

Both are entire, with $\sinh'(z) = \cosh z$ and $\cosh'(z) = \sinh z$, and $\cosh^2 z - \sinh^2 z = 1$.

**Relations to trigonometric functions:**
$$\sinh(iz) = i\sin z, \qquad \cosh(iz) = \cos z.$$
$$\sin(iz) = i\sinh z, \qquad \cos(iz) = \cosh z.$$

These relations show that the real and imaginary axes exchange the roles of the trigonometric and hyperbolic functions. In the complex plane, sine and sinh are not really different functions — they are the same function restricted to different axes.

Writing $z = x + iy$:
$$\sinh(x + iy) = \sinh x\cos y + i\cosh x\sin y,$$
$$\cosh(x + iy) = \cosh x\cos y + i\sinh x\sin y.$$

## Periodicity

$\sin z$ and $\cos z$ are periodic with period $2\pi$: $\sin(z + 2\pi) = \sin z$.
$\sinh z$ and $\cosh z$ are periodic with period $2\pi i$: $\sinh(z + 2\pi i) = \sinh z$.

This is a direct consequence of $e^z$ having period $2\pi i$: for $\sinh$, $\sinh(z + 2\pi i) = (e^{z+2\pi i} - e^{-z-2\pi i})/2 = (e^z - e^{-z})/2 = \sinh z$.

## Worked Example: Solving Trigonometric Equations in $\mathbb{C}$

**Example.** Solve $\sin z = 2$.

Using the definition: $(e^{iz} - e^{-iz})/(2i) = 2$, so $e^{iz} - e^{-iz} = 4i$. Let $w = e^{iz}$: $w - 1/w = 4i$, i.e., $w^2 - 4iw - 1 = 0$. Quadratic formula: $w = (4i \pm \sqrt{-16 + 4})/2 = (4i \pm \sqrt{-12})/2 = 2i \pm i\sqrt{3}$.

So $e^{iz} = i(2 \pm \sqrt{3})$. Taking logarithms: $iz = \log(i(2 \pm \sqrt{3})) = \log(2 \pm \sqrt{3}) + i\pi/2 + 2\pi ki$.

Therefore: $z = \pi/2 - i\log(2 \pm \sqrt{3}) + 2\pi k$, $k \in \mathbb{Z}$.

Check: Since $2 + \sqrt{3} = 1/(2 - \sqrt{3})$, the two families of solutions are $z = \pi/2 - i\log(2 + \sqrt{3}) + 2\pi k$ and $z = \pi/2 + i\log(2 + \sqrt{3}) + 2\pi k$. These are all real only if $\log(2 \pm \sqrt{3}) = 0$, i.e., $2 \pm \sqrt{3} = 1$, which is false. So $\sin z = 2$ has no real solutions (as expected) but infinitely many complex ones. $\square$

## Connection to Physics and Applications

The hyperbolic functions arise naturally in the solutions to the Laplace equation in rectangular coordinates: the general solution of $\Delta \phi = 0$ that is periodic in $y$ involves $\cosh(ny)$ and $\sinh(ny)$ with factors $\cos(nx)$ and $\sin(nx)$. In waveguide theory, the complex trigonometric and hyperbolic functions describe the propagation of electromagnetic waves in conductors, where the wavenumber is complex (the imaginary part encoding attenuation).
