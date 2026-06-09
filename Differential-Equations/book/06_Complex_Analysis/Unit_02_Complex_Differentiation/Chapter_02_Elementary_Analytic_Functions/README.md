# Chapter 02: Elementary Analytic Functions

The classical transcendental functions — exponential, logarithm, trigonometric, hyperbolic, and power functions — extend from the real line to the complex plane in a natural and enlightening way. The complex extensions reveal algebraic relationships that are hidden on $\mathbb{R}$ (for instance, that $\sin z$ and $\cos z$ are closely related to the exponential via Euler's formula) and introduce new phenomena such as periodicity in the imaginary direction, branch points, and multivaluedness.

All of the functions studied in this chapter are built from the complex exponential $e^z$, which serves as the fundamental building block of complex analysis. Its analyticity, growth properties, and periodicity determine the behavior of every function derived from it.

## The Exponential Function

The complex exponential $e^z = e^{x+iy} = e^x(\cos y + i\sin y)$ is entire, satisfies $\frac{d}{dz}e^z = e^z$, and is periodic with period $2\pi i$: $e^{z + 2\pi i} = e^z$ for all $z$. It maps horizontal lines to rays from the origin and vertical lines to circles centered at the origin.

Unlike the real exponential, the complex exponential is not injective: $e^z = e^w$ if and only if $z - w = 2\pi ki$ for some integer $k$. This periodicity is the source of the multivaluedness of the logarithm.

## The Logarithm and Power Functions

The complex logarithm $\log z$ is the multivalued inverse of $e^z$. On any simply connected domain not containing the origin, a single-valued branch can be selected, yielding an analytic function with derivative $1/z$. The principal branch $\mathrm{Log}\, z = \ln|z| + i\,\mathrm{Arg}\, z$ is analytic on $\mathbb{C} \setminus (-\infty, 0]$.

Power functions $z^\alpha = e^{\alpha \log z}$ for general $\alpha \in \mathbb{C}$ are defined using the logarithm and are multivalued for non-integer $\alpha$. Their branches are analytic with derivatives given by the formal power rule $\frac{d}{dz}z^\alpha = \alpha z^{\alpha - 1}$.

## Trigonometric and Hyperbolic Functions

The complex sine and cosine are defined by $\sin z = (e^{iz} - e^{-iz})/(2i)$ and $\cos z = (e^{iz} + e^{-iz})/2$. These are entire functions that coincide with the real trigonometric functions on the real axis. They are no longer bounded on $\mathbb{C}$: for instance, $|\sin(iy)| = \sinh y \to \infty$ as $y \to \infty$. This unboundedness is consistent with Liouville's theorem.

The hyperbolic functions $\sinh z = (e^z - e^{-z})/2$ and $\cosh z = (e^z + e^{-z})/2$ are also entire and are related to the trigonometric functions by $\sin(iz) = i\sinh z$ and $\cos(iz) = \cosh z$.

## Inverse Functions

The inverse trigonometric and hyperbolic functions — $\arcsin z$, $\arctan z$, $\mathrm{arcsinh}\, z$, and so on — are multivalued functions defined via the logarithm. For example, $\arcsin z = -i\log(iz + \sqrt{1-z^2})$. They are analytic on appropriate domains with branch cuts, and their derivatives are given by the same formulas as in real calculus.

## Learning Objectives

After completing this chapter, a student should be able to:

- State the definition of $e^z$ via the series and the formula $e^{x+iy} = e^x(\cos y + i\sin y)$, and derive its key properties.
- State the definition of $\mathrm{Log}\, z$ and compute it for specific values.
- Define $\sin z$, $\cos z$, $\sinh z$, and $\cosh z$ via the exponential, and derive identities relating them.
- Compute with $z^\alpha$ for specific $\alpha \in \mathbb{C}$, selecting an appropriate branch.
- Find the domains of analyticity of the inverse trigonometric functions and identify their branch cuts.
