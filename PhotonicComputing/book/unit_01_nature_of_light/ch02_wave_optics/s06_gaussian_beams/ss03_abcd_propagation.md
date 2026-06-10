# 2.6.3 — ABCD Propagation of Gaussian Beams

## The $q$-Parameter Transformation

The most powerful result in Gaussian beam optics: a Gaussian beam characterized by complex beam parameter $q$ at the input of any paraxial optical system with ABCD matrix $\begin{pmatrix} A & B \\ C & D \end{pmatrix}$ has a $q$-parameter at the output given by the *ABCD law* (or *Möbius transformation*):

$$q_\text{out} = \frac{A q_\text{in} + B}{C q_\text{in} + D}$$

This is a linear fractional (Möbius) transformation of the complex $q$-parameter. The same matrix that propagates geometric rays (Section 2.1.3) propagates Gaussian beams — with a complex "ray" $q$ replacing the real ray vector $(y, \theta)$.

**Derivation**: The $q$-transformation is derived from the paraxial wave equation by substituting the Gaussian ansatz and imposing consistency. Alternatively, it follows from the fact that the Gaussian beam is characterized by its complex radius of curvature $q$, and any paraxial system maps complex wavefronts according to the same laws as geometric wavefronts (with complex radii of curvature). The proof is given in Siegman (1986) [1], Chapter 20.

## Deriving the Output Beam Parameters

Given $q_\text{out} = (Aq_\text{in} + B)/(Cq_\text{in} + D)$, we extract the physical parameters using $1/q = 1/R - i\lambda/(\pi w^2)$:

$$\frac{1}{q_\text{out}} = \frac{C q_\text{in} + D}{A q_\text{in} + B}$$

Taking real and imaginary parts of $1/q_\text{out}$ gives the output wavefront curvature $R_\text{out}$ and beam size $w_\text{out}$. For $q_\text{in} = iz_R$ (input beam at its waist, $R_\text{in} \to \infty$):

$$q_\text{out} = \frac{Aiz_R + B}{Ciz_R + D} = \frac{B + iAz_R}{D + iCz_R}$$

Multiplying numerator and denominator by the complex conjugate of the denominator:

$$q_\text{out} = \frac{(B + iAz_R)(D - iCz_R)}{D^2 + C^2z_R^2} = \frac{BD + ACz_R^2 + i(ADz_R - BCz_R)}{D^2 + C^2z_R^2}$$

Since $AD - BC = 1$ (determinant condition for a system in the same medium):

$$q_\text{out} = \frac{BD + ACz_R^2}{D^2 + C^2 z_R^2} + i\frac{z_R}{D^2 + C^2 z_R^2}$$

The imaginary part: $\text{Im}(q_\text{out}) = z_R/(D^2 + C^2z_R^2)$. Since $\text{Im}(q) = z_{R,\text{out}} = \pi w_0'^2/\lambda$, the output waist:

$$w_0'^2 = \frac{\lambda}{\pi} \cdot \text{Im}(q_\text{out}) = w_0^2 \frac{1}{D^2 + C^2z_R^2}$$

$$\boxed{w_0' = \frac{w_0}{\sqrt{D^2 + (Cw_0^2\pi/\lambda)^2}} = \frac{w_0}{\sqrt{D^2 + C^2z_R^2}}}$$

The real part gives the position of the new waist: the waist is located at $z' = \text{Re}(q_\text{out})$ from the output plane.

## Key Examples

### Free Propagation (Distance $d$)

$M = \begin{pmatrix} 1 & d \\ 0 & 1 \end{pmatrix}$: $A = 1$, $B = d$, $C = 0$, $D = 1$.

$$q_\text{out} = q_\text{in} + d$$

Since $q_\text{in} = z_\text{in} - iz_R$: $q_\text{out} = (z_\text{in} + d) - iz_R$. This is just the beam parameter at position $z_\text{in} + d$ — propagation simply advances $z$. The waist size $w_0$ and Rayleigh range $z_R$ are unchanged.

### Thin Lens (Focal Length $f$)

$M = \begin{pmatrix} 1 & 0 \\ -1/f & 1 \end{pmatrix}$: $A = 1$, $B = 0$, $C = -1/f$, $D = 1$.

$$q_\text{out} = \frac{q_\text{in}}{-q_\text{in}/f + 1} = \frac{fq_\text{in}}{f - q_\text{in}}$$

For a beam at its waist ($q_\text{in} = iz_R$) at the lens:

$$q_\text{out} = \frac{fiz_R}{f - iz_R} = \frac{fiz_R(f + iz_R)}{f^2 + z_R^2} = \frac{-fz_R^2 + if^2z_R}{f^2 + z_R^2}$$

New waist: $w_0'^2 = \lambda/(pi) \cdot f^2 z_R/(f^2 + z_R^2)$:

$$w_0' = w_0 \cdot \frac{f}{\sqrt{f^2 + z_R^2}} = \frac{w_0}{\sqrt{1 + (z_R/f)^2}}$$

For $f \gg z_R$ (focal length much larger than Rayleigh range — the usual regime for loose focusing): $w_0' \approx w_0$. For $f \ll z_R$ (tight focusing): $w_0' \approx fw_0/z_R = f\lambda/(\pi w_0)$.

The position of the new waist (output focal plane):

$$z' = \text{Re}(q_\text{out}) = \frac{-fz_R^2}{f^2+z_R^2} = \frac{-f}{1 + (f/z_R)^2}$$

For $f \gg z_R$: $z' \approx -f$ — the new waist is at the back focal plane of the lens (as expected from geometric optics). For $f \approx z_R$: the waist shifts toward the lens.

**The minimum achievable focused spot size**: $w_0' = f\lambda/(\pi w_\text{in})$ (in the tight-focusing limit, where $z_R < f$). To focus to $w_0' = 1$ μm at $\lambda = 1550$ nm using a lens with $f = 1$ mm, the input beam must have $w_\text{in} = f\lambda/(\pi w_0') = 10^{-3} \times 1.55 \times 10^{-6}/(\pi \times 10^{-6}) \approx 0.49$ mm radius at the lens. The numerical aperture of the focus is NA $= w_\text{in}/f = 0.49$ — a moderately high NA lens.

## Resonator Modes

A key application of the ABCD law: finding the self-consistent mode of an optical resonator. A resonator mode must reproduce itself after one round trip, so $q_\text{out} = q_\text{in} = q$ (the fixed point of the Möbius transformation):

$$q = \frac{Aq + B}{Cq + D} \implies Cq^2 + (D-A)q - B = 0$$

$$q = \frac{(A-D) \pm \sqrt{(A-D)^2 + 4BC}}{2C} = \frac{(A-D) \pm \sqrt{(A+D)^2 - 4}}{2C}$$

(using $AD - BC = 1$). For a stable resonator ($|(A+D)/2| < 1$), the discriminant $(A+D)^2 - 4 < 0$, and the fixed point $q$ is complex (has nonzero imaginary part), corresponding to a Gaussian beam mode. The beam parameters are determined by the resonator geometry.

## Summary

- ABCD law for Gaussian beams: $q_\text{out} = (Aq_\text{in} + B)/(Cq_\text{in} + D)$ — same matrix as for geometric rays.
- Free propagation: $q_\text{out} = q_\text{in} + d$ (advances position, preserves waist and Rayleigh range).
- Thin lens: focuses beam to new waist $w_0' = w_0 f/\sqrt{f^2+z_R^2}$ at position $z' = -f/(1+(f/z_R)^2)$.
- Resonator mode: self-consistent $q$ (fixed point of round-trip ABCD transformation) — real for unstable, complex for stable resonators.

---

*References*

[1] Siegman, A.E. (1986). *Lasers*. University Science Books. Chapter 20. [The definitive reference for ABCD Gaussian beam optics and its applications to laser design.]
