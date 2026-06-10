# 2.1.3 — Ray Transfer Matrices (ABCD Matrices)

## The Paraxial Approximation

A *paraxial ray* is one that makes a small angle $\theta$ with the optical axis, so that $\sin\theta \approx \tan\theta \approx \theta$ (in radians). This is the small-angle or paraxial approximation. It sounds crude, but it is valid for most practical laser and imaging systems, where beam divergence angles are small. In fiber optics and integrated photonics, guided modes are nearly paraxial.

Under the paraxial approximation, ray optics becomes *linear*: the output ray is a linear function of the input ray. This means the propagation of a ray through any sequence of optical elements can be described by matrix multiplication — a powerful and compact representation.

## State Vector and Transfer Matrix

A paraxial ray at any point along the optical axis is fully described by two quantities:
- $y$: the height (transverse position) of the ray from the optical axis
- $\theta$: the angle the ray makes with the optical axis (positive for upward slope)

We represent this as a column vector (the *ray vector*):

$$\begin{pmatrix} y \\ \theta \end{pmatrix}$$

Each optical element transforms the input ray vector into an output ray vector via a $2 \times 2$ matrix:

$$\begin{pmatrix} y_\text{out} \\ \theta_\text{out} \end{pmatrix} = \begin{pmatrix} A & B \\ C & D \end{pmatrix} \begin{pmatrix} y_\text{in} \\ \theta_\text{in} \end{pmatrix}$$

This is the *ray transfer matrix* or *ABCD matrix*. The power of the formalism: for a sequence of $N$ optical elements with matrices $M_1, M_2, \ldots, M_N$ (in order of traversal), the total system matrix is the product $M_\text{total} = M_N \cdots M_2 M_1$ (note the reversed order: $M_1$ acts first but appears rightmost in the product).

## Fundamental ABCD Matrices

### Free Propagation (Distance $d$ in medium $n$)

A ray travels distance $d$ in a medium of refractive index $n$. Its height changes by $d\theta$; its angle is unchanged:

$$y_\text{out} = y_\text{in} + d\theta_\text{in}, \qquad \theta_\text{out} = \theta_\text{in}$$

$$M_\text{free} = \begin{pmatrix} 1 & d \\ 0 & 1 \end{pmatrix}$$

(Note: some formulations use the reduced angle $n\theta$ to make the matrices symplectic; we use $\theta$ directly for simplicity in the paraxial limit.)

### Thin Lens (Focal Length $f$)

A thin lens with focal length $f$ changes the ray angle by $-y/f$ without changing its height:

$$y_\text{out} = y_\text{in}, \qquad \theta_\text{out} = \theta_\text{in} - \frac{y_\text{in}}{f}$$

$$M_\text{lens} = \begin{pmatrix} 1 & 0 \\ -1/f & 1 \end{pmatrix}$$

Converging (positive) lens: $f > 0$. Diverging (negative) lens: $f < 0$.

### Planar Interface (Snell's Law, Paraxial)

At a flat interface from medium $n_1$ to $n_2$, Snell's law in the paraxial limit gives $n_1\theta_1 = n_2\theta_2$:

$$M_\text{interface} = \begin{pmatrix} 1 & 0 \\ 0 & n_1/n_2 \end{pmatrix}$$

### Spherical Mirror (Radius of Curvature $R$)

$$M_\text{mirror} = \begin{pmatrix} 1 & 0 \\ -2/R & 1 \end{pmatrix}$$

Concave mirror: $R > 0$ (by convention, center of curvature on the incoming side). The focal length is $f = R/2$.

## The Determinant: A Conservation Law

For propagation through lossless media, the determinant of the ABCD matrix equals the ratio of input to output refractive index: $\det(M) = n_1/n_2$. For a complete system in the same medium, $\det(M) = AD - BC = 1$.

This is not a coincidence — it is a consequence of Liouville's theorem in phase space (the conservation of phase space volume for Hamiltonian systems). The ray vector $(y, n\theta)$ lives in a phase space where the symplectic area is conserved. For photonic computing, this has a practical consequence: you cannot focus light arbitrarily tightly without it diverging rapidly (conservation of étendue), which limits the density of information that can be packed into a free-space optical system.

## Worked Example: Two-Lens System (Imaging and Fourier)

**Imaging**: A thin lens of focal length $f$ at distance $d_o$ from an object creates an image at distance $d_i$ where $1/d_o + 1/d_i = 1/f$. The ABCD matrix from object plane to image plane:

$$M = M_\text{free}(d_i) \cdot M_\text{lens}(f) \cdot M_\text{free}(d_o) = \begin{pmatrix} 1 & d_i \\ 0 & 1 \end{pmatrix}\begin{pmatrix} 1 & 0 \\ -1/f & 1 \end{pmatrix}\begin{pmatrix} 1 & d_o \\ 0 & 1 \end{pmatrix}$$

Computing:

$$= \begin{pmatrix} 1 - d_i/f & d_o + d_i - d_o d_i/f \\ -1/f & 1 - d_o/f \end{pmatrix}$$

For an image, $B = 0$ (rays from a single object point converge to a single image point regardless of angle):

$$B = d_o + d_i - \frac{d_o d_i}{f} = 0 \implies \frac{1}{d_o} + \frac{1}{d_i} = \frac{1}{f}$$

The thin lens equation follows from requiring $B = 0$.

**4f System (Fourier)**: Two lenses each of focal length $f$, separated by $2f$, with input at distance $f$ before the first lens and output at distance $f$ after the second. This is the *4f system*, fundamental to Fourier optics (Section 2.3.3):

$$M = M_\text{free}(f) \cdot M_\text{lens}(f) \cdot M_\text{free}(2f) \cdot M_\text{lens}(f) \cdot M_\text{free}(f)$$

Computing this product gives $M = \begin{pmatrix} -1 & 0 \\ 0 & -1 \end{pmatrix}$, which means $y_\text{out} = -y_\text{in}$ and $\theta_\text{out} = -\theta_\text{in}$ — the image is inverted, but the system maps angle to position in the Fourier plane (the middle plane, at distance $f$ after the first lens).

The Fourier plane is where a ray entering at angle $\theta$ arrives at height $y = f\theta$ — converting angular information (spatial frequency) to position. A mask placed in this plane multiplies the Fourier transform of the input, implementing a spatial filter. This is the physical principle behind free-space optical processing (Section 2.3.3 and Unit V).

## Stability of Optical Resonators

The ABCD matrix formalism is used to determine whether an optical resonator (two mirrors, possibly with lenses inside) is stable — whether rays bounce back and forth indefinitely or eventually escape.

For a resonator with round-trip ABCD matrix $M = \begin{pmatrix} A & B \\ C & D \end{pmatrix}$, the resonator is stable if and only if:

$$\left|\frac{A+D}{2}\right| \leq 1$$

This stability condition is the same as the condition for a bounded orbit in the corresponding discrete map (a ray bouncing back and forth). The condition $(A+D)/2 = \cos\phi$ gives the round-trip Gouy phase $\phi$, which determines the resonant frequencies of the Gaussian modes.

For a Fabry-Pérot resonator with two flat mirrors separated by $L$:
- Round-trip matrix: $M = M_\text{free}(L) \cdot M_\text{mirror} \cdot M_\text{free}(L) \cdot M_\text{mirror}$

With flat mirrors ($R = \infty$, so no focusing), $M = \begin{pmatrix} 1 & 2L \\ 0 & 1 \end{pmatrix}$, giving $(A+D)/2 = 1$: marginally stable (rays walk off after many bounces). Adding curved mirrors brings the system to stable operation — which is why practical laser cavities use curved mirrors.

## Connection to Gaussian Beams

The ABCD matrix formalism extends exactly to Gaussian beam propagation. A Gaussian beam is characterized by a single complex parameter $q$ (the complex beam parameter, introduced in Section 2.6), and the transformation of $q$ through any ABCD system is:

$$q_\text{out} = \frac{A q_\text{in} + B}{C q_\text{in} + D}$$

This is a Möbius transformation — a linear fractional transformation of the complex $q$-plane. The same matrices that describe ray propagation describe Gaussian beam propagation, with a complex ray replacing the real ray vector. This is one of the most elegant results in paraxial optics: the wave-optics description of a laser beam maps exactly onto the geometric-optics description of a complex ray.

## Summary

- In the paraxial approximation ($\theta \ll 1$), ray propagation is linear and described by $2 \times 2$ ABCD matrices.
- The fundamental matrices: free propagation $\begin{pmatrix} 1 & d \\ 0 & 1 \end{pmatrix}$, thin lens $\begin{pmatrix} 1 & 0 \\ -1/f & 1 \end{pmatrix}$.
- System matrix = product of element matrices (rightmost acts first).
- $\det(M) = 1$ for a round trip in the same medium: this is conservation of phase space area (étendue).
- Resonator stability: $|(A+D)/2| \leq 1$.
- The same ABCD matrices describe Gaussian beam propagation via the $q$-parameter transformation.

---

*References*

[1] Saleh, B.E.A. & Teich, M.C. (2019). *Fundamentals of Photonics*, 3rd ed. Chapter 1. Wiley. [Comprehensive treatment of ray transfer matrices and their applications.]

[2] Siegman, A.E. (1986). *Lasers*. University Science Books. [The authoritative reference on ABCD matrices and Gaussian beam optics; Chapters 15–20.]
