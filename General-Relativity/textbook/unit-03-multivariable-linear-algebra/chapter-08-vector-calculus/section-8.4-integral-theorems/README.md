# Section 8.4: The Integral Theorems

---

## Section Introduction

The fundamental theorem of calculus in one dimension says: to compute $\int_a^b f'(x)\,dx$, you need only evaluate $f$ at the boundary $\{a, b\}$. The **integral theorems** of vector calculus generalize this: to compute a derivative-type integral over a region, you need only know the function on the boundary of that region. These theorems — Green's theorem, Stokes' theorem, and the divergence theorem — are all instances of a single deep principle.

**Green's theorem** (planar Stokes): For a region $D$ in the plane with boundary $\partial D$,
$$\oint_{\partial D}(P\,dx + Q\,dy) = \iint_D\left(\frac{\partial Q}{\partial x} - \frac{\partial P}{\partial y}\right)dA$$
This relates a line integral around a closed curve to a double integral over the enclosed region.

**Stokes' theorem**: For a surface $S$ in $\mathbb{R}^3$ with boundary curve $\partial S$,
$$\oint_{\partial S}\mathbf{F}\cdot d\mathbf{r} = \iint_S (\nabla\times\mathbf{F})\cdot d\mathbf{A}$$
The circulation of $\mathbf{F}$ around the boundary equals the flux of its curl through the surface.

**The divergence theorem** (Gauss): For a region $V$ with boundary surface $\partial V$,
$$\oiint_{\partial V}\mathbf{F}\cdot d\mathbf{A} = \iiint_V\nabla\cdot\mathbf{F}\,dV$$
The flux through the boundary equals the total divergence inside.

All three are unified in the language of differential forms as $\int_{\partial M}\omega = \int_M d\omega$ — **Stokes' theorem in the general sense**. This is one of the most important theorems in all of mathematics, and it is the key to understanding conservation laws in physics: a locally conserved quantity (zero divergence) has no flux through closed surfaces, meaning none of it is "created or destroyed" in any closed region.

---

## Subsections

- [8.4.1: Green's Theorem](8.4.1-greens.md)
- [8.4.2: Stokes' Theorem](8.4.2-stokes.md)
- [8.4.3: The Divergence Theorem](8.4.3-divergence.md)
- [8.4.4: Unified Formulation via Differential Forms](8.4.4-unified.md)
- [8.4.5: Applications to Conservation Laws](8.4.5-conservation.md)
