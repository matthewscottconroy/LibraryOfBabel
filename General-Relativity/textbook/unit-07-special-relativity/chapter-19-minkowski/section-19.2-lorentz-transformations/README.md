# Section 19.2: Lorentz Transformations

---

## Section Introduction

A Lorentz transformation is a linear map from one inertial frame to another that preserves the Minkowski interval $ds^2$. These transformations encode time dilation, length contraction, and the relativity of simultaneity — the "paradoxes" of special relativity that turn out, on careful analysis, to be perfectly consistent consequences of spacetime geometry.

The key insight: the Lorentz transformation is the analog of a rotation in spacetime — but a rotation that mixes time and space, with the hyperbolic functions $\cosh$ and $\sinh$ playing the role of $\cos$ and $\sin$. The "angle" of this rotation is the **rapidity** $\phi = \tanh^{-1}(v/c)$, which (unlike velocity) adds linearly when boosts are combined.

---

## 19.2.1 Derivation of the Lorentz Transformation

Consider two inertial frames $S$ and $S'$, with $S'$ moving at velocity $v$ in the $x$-direction relative to $S$, with origins coinciding at $t = t' = 0$.

**Requirement**: The transformation $x^\mu \to x'^\mu$ must be linear (inertial motion maps to inertial motion), must agree with Galilean transformations for $v \ll c$, and must satisfy $ds^2 = ds'^2$ (invariance of the interval).

For motion in the $x$-direction, $y' = y$, $z' = z$, and:

$$t' = \gamma\left(t - \frac{vx}{c^2}\right), \qquad x' = \gamma(x - vt)$$

where $\gamma = 1/\sqrt{1 - v^2/c^2}$ is the **Lorentz factor**.

*Verification*:
$$ds'^2 = -c^2 dt'^2 + dx'^2 = -c^2\gamma^2\left(dt - \frac{v}{c^2}dx\right)^2 + \gamma^2(dx - v\,dt)^2$$
$$= \gamma^2\left[(-c^2 + v^2)dt^2 + 2v\left(1 - 1\right)dt\,dx + \left(1 - \frac{v^2}{c^2}\right)dx^2\right]$$
$$= \gamma^2(v^2 - c^2)dt^2 + \gamma^2\left(1 - \frac{v^2}{c^2}\right)dx^2 = -c^2\,dt^2 + dx^2 = ds^2 \qquad \checkmark$$

**In matrix form**: The Lorentz boost with velocity $v$ in the $x$-direction is:

$$\Lambda^\mu_{\ \nu} = \begin{pmatrix} \gamma & -\beta\gamma & 0 & 0 \\ -\beta\gamma & \gamma & 0 & 0 \\ 0 & 0 & 1 & 0 \\ 0 & 0 & 0 & 1 \end{pmatrix}, \qquad \beta = \frac{v}{c}$$

**Rapidity**: Define rapidity $\phi$ by $\tanh\phi = \beta$, so $\gamma = \cosh\phi$, $\beta\gamma = \sinh\phi$. The boost becomes:

$$\Lambda = \begin{pmatrix} \cosh\phi & -\sinh\phi & 0 & 0 \\ -\sinh\phi & \cosh\phi & 0 & 0 \\ 0 & 0 & 1 & 0 \\ 0 & 0 & 0 & 1 \end{pmatrix}$$

This is a "hyperbolic rotation" in the $(t, x)$ plane — the spacetime analog of a rotation in the $(x, y)$ plane. The analogy is exact: rotations use $\cos$, $\sin$; Lorentz boosts use $\cosh$, $\sinh$.

---

## 19.2.2 Time Dilation

A clock at rest in $S'$ (at position $x' = 0$) ticks at its proper time rate. Two ticks at $\Delta x' = 0$, $\Delta t' = T_0$ (the proper time period). In frame $S$:

$$\Delta t = \gamma\left(\Delta t' + \frac{v\Delta x'}{c^2}\right) = \gamma T_0$$

The coordinate time between the ticks is $\Delta t = \gamma T_0 > T_0$ — the moving clock runs slow by factor $\gamma$. This is **time dilation**.

**Physical consequence**: A muon created in the upper atmosphere ($\sim 15$ km altitude) by cosmic rays has a proper lifetime $\tau_0 = 2.2\,\mu$s and travels at $v \approx 0.998c$. Its coordinate lifetime in Earth's frame is $\gamma\tau_0 \approx 15\tau_0 \approx 33\,\mu$s. In this time, it travels $ct\gamma = 0.998c \times 33\,\mu\text{s} \approx 10$ km — reaching sea level and being detected. Without time dilation, it would travel only $c\tau_0 \approx 660$ m and stop far above the ground. The observed muon flux at sea level is direct experimental confirmation of time dilation. [Frisch and Smith, *American Journal of Physics*, 31 (1963), 342.]

---

## 19.2.3 Length Contraction

A rod of proper length $L_0$ at rest in $S'$, aligned along $x'$. Its endpoints are at $x'_1 = 0$ and $x'_2 = L_0$.

To measure its length in $S$: observe both endpoints **simultaneously** (at the same $t$). From the Lorentz transformation:

$$L = x_2 - x_1 = \frac{1}{\gamma}(x'_2 - x'_1) = \frac{L_0}{\gamma}$$

(using $\Delta t = 0$ and $\Delta x = \gamma(\Delta x' + v\Delta t') = \gamma\Delta x'$ in reverse).

The rod appears shorter in the frame in which it moves: $L = L_0/\gamma \leq L_0$. This is **length contraction** (Lorentz-FitzGerald contraction).

**Important**: Length contraction is not a physical compression of the rod. The rod is perfectly intact in its rest frame; its Lorentz-contracted length is a consequence of the measurement procedure (simultaneity is relative).

**The ladder "paradox"**: A ladder of length $L_0 > d$ (the width of a barn) cannot fit in the barn in its rest frame. But moving at relativistic speed, the ladder contracts to $L = L_0/\gamma < d$ — it fits! Paradox: in the ladder's frame, it's the barn that contracts. Resolution: "simultaneously inside the barn" is a frame-dependent statement. In the barn's frame, both ends are inside the barn at the same time. In the ladder's frame, they are never both inside at the same time.

---

## 19.2.4 Relativistic Velocity Addition

If an object moves at velocity $v'$ (in the $x$-direction) relative to frame $S'$, and $S'$ moves at velocity $V$ relative to $S$, the object's velocity in $S$ is:

$$v = \frac{v' + V}{1 + v'V/c^2}$$

(not $v = v' + V$ as in Galilean relativity).

*Derivation*: From the Lorentz transformation of $dx$ and $dt$:
$$v = \frac{dx}{dt} = \frac{\gamma(dx' + V\,dt')}{\gamma(dt' + V\,dx'/c^2)} = \frac{dx'/dt' + V}{1 + V(dx'/dt')/c^2} = \frac{v' + V}{1 + v'V/c^2}$$

**Check**: If $v' = c$: $v = (c + V)/(1 + V/c) = c$. ✓ Light travels at $c$ in all frames.

**Rapidity addition**: Using the rapidity parametrization, the velocity addition formula becomes $\phi = \phi_1 + \phi_2$ — rapidities add linearly. This is why rapidity is the natural parametrization of Lorentz boosts.

---

## 19.2.5 The Lorentz Group

The **Lorentz group** $O(3,1)$ consists of all $4\times 4$ matrices $\Lambda$ satisfying $\Lambda^T\eta\Lambda = \eta$ (where $\eta = \text{diag}(-1,+1,+1,+1)$). It has four connected components, classified by $\det\Lambda = \pm 1$ and $\Lambda^0_{\ 0} \gtrless 0$.

The **proper orthochronous Lorentz group** $SO^+(3,1) = L^\uparrow_+$ is the component containing the identity, with $\det\Lambda = +1$ and $\Lambda^0_{\ 0} \geq 1$. It is generated by:
- **Rotations** $J_i$: infinitesimal generators $(J_1, J_2, J_3)$ of rotations in $(y,z)$, $(z,x)$, $(x,y)$ planes
- **Boosts** $K_i$: infinitesimal generators $(K_1, K_2, K_3)$ of Lorentz boosts in $(t,x)$, $(t,y)$, $(t,z)$ planes

The Lie algebra is:

$$[J_i, J_j] = i\varepsilon_{ijk}J_k, \quad [K_i, K_j] = -i\varepsilon_{ijk}J_k, \quad [J_i, K_j] = i\varepsilon_{ijk}K_k$$

(The minus sign in $[K_i, K_j]$ reflects the Lorentzian signature.)

**Double cover**: The Lorentz group is not simply connected: $\pi_1(SO(3)) = \mathbb{Z}_2$. The double cover is $SL(2, \mathbb{C})$ — 2×2 complex matrices with determinant 1. This is the group underlying spinors (half-integer spin representations), which is why fermions (electrons, quarks) don't return to their original state after a $2\pi$ rotation — they require $4\pi$.

**GR connection**: In GR, the structure group of the frame bundle is the local Lorentz group. Spinor fields in curved spacetime require the spin connection (an $\mathfrak{sl}(2,\mathbb{C})$-valued connection) rather than the Christoffel symbols. The Dirac equation in curved spacetime uses the tetrad (vierbein) formalism.

---

## References

- Einstein, A. (1905). "Zur Elektrodynamik bewegter Körper." *Annalen der Physik*, 17, 891–921. [Derives the Lorentz transformation from the two postulates; derives time dilation; addresses the "light clock" thought experiment.]
- Lorentz, H.A. (1904). "Electromagnetic phenomena in a system moving with any velocity less than that of light." *Proceedings of the Royal Netherlands Academy of Arts and Sciences*, 6, 809–831. [Derived the Lorentz transformation as a mathematical artifact of electromagnetic theory — not recognizing its kinematic significance. Einstein's 1905 paper gives the physical interpretation.]
- Frisch, D.H. and Smith, J.H. (1963). "Measurement of the relativistic time dilation using μ-mesons." *American Journal of Physics*, 31, 342–355. [Classic laboratory experiment: measures the time dilation of muons traveling from mountain top to sea level. The muon flux at sea level is far higher than expected without time dilation.]
- Weinberg, S. (1995). *The Quantum Theory of Fields*, Vol. I. Cambridge University Press. [Chapter 2: the Lorentz and Poincaré groups, their representations, and the construction of relativistic quantum mechanics. The most thorough treatment of the Lorentz group from the physics perspective.]
- Penrose, R. and Rindler, W. (1984). *Spinors and Space-Time*, Vol. 1. Cambridge University Press. [Chapter 1: the Lorentz group, spinors, and the 2-component spinor formalism. Essential for the spinor approach to GR.]
