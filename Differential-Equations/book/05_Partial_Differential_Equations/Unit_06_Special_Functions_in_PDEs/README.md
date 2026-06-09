# Unit 6: Special Functions in PDEs

The separation of variables method converts a PDE into a system of ODEs, one for each coordinate. When the geometry is cylindrical or spherical rather than rectangular, these ODEs are not the simple harmonic oscillator equation but more exotic second-order linear ODEs with variable coefficients — **Bessel's equation** and **Legendre's equation** — whose solutions are the special functions of this unit. These functions are not curiosities: they are as fundamental to the analysis of waves and potentials in curved geometry as the trigonometric functions are to the analysis of problems on rectangles.

## What This Unit Covers

The two chapters of this unit provide a systematic treatment of the two families of special functions that appear most frequently in PDE applications.

**Chapter 1: Bessel Functions** treats $J_\nu(x)$, $Y_\nu(x)$, $I_\nu(x)$, and $K_\nu(x)$ — the solutions of Bessel's equation $x^2y'' + xy' + (x^2-\nu^2)y = 0$. Bessel functions arise whenever the Laplacian is separated in cylindrical coordinates, appearing in problems on disks, cylinders, wedges, and annuli. The chapter covers: the series definition of $J_\nu$ via the method of Frobenius; the second solution $Y_\nu$ (singular at the origin); the zeros $j_{\nu,n}$ of $J_\nu$ and their asymptotic behavior; the orthogonality relation on $[0,1]$ with weight $r$; Bessel-Fourier series for functions on a disk; and the modified Bessel functions $I_\nu$ and $K_\nu$ that arise in problems where the eigenvalue has the wrong sign (e.g., steady-state diffusion in a cylinder with heat source).

**Chapter 2: Legendre and Spherical Harmonics** revisits Legendre polynomials $P_\ell(t)$ and associated Legendre functions $P_\ell^m(t)$ from the standpoint of their role as building blocks of spherical harmonics $Y_\ell^m(\theta,\phi)$. While these objects appeared in Unit 5, this chapter develops their properties more systematically as a standalone reference: generating functions, recurrence relations, the Rodrigues formula, addition theorem, and the connection to group theory via representations of $SO(3)$.

## The Role of Special Functions

Special functions arise from the spectral theory of Sturm-Liouville operators. Bessel's equation on $[0,R]$ with appropriate boundary conditions is a Sturm-Liouville problem of the form $(xu')' + (\lambda x - \nu^2/x)u = 0$, and the Bessel functions $J_\nu(\sqrt{\lambda}\,x)$ are its eigenfunctions. The Sturm-Liouville theorem guarantees real eigenvalues, completeness of the eigenfunctions in $L^2([0,R]; x\, dx)$, and oscillation: $J_\nu$ has infinitely many zeros, and the $n$-th zero $j_{\nu,n}$ grows like $n\pi$ for large $n$ — just as the zeros of $\sin$ grow like $n\pi$.

Similarly, Legendre's equation on $[-1,1]$ is a Sturm-Liouville problem, and $P_\ell(t)$ are its polynomial eigenfunctions (degree-$\ell$ polynomials regular at $t = \pm 1$). The completeness of $\{P_\ell\}$ in $L^2([-1,1])$ is a consequence of Weierstrass approximation: every continuous function can be approximated by polynomials, and the Legendre polynomials are an orthogonal polynomial system for the weight $w=1$ on $[-1,1]$.

## Prerequisites and Connections

This unit assumes familiarity with the method of Frobenius for power series solutions of second-order ODEs, basic Fourier analysis (including the concept of an orthonormal basis for $L^2$), the Sturm-Liouville theory of eigenvalue problems, and the separation of variables as applied in Units 3, 4, and 5. The Bessel functions encountered in the heat equation chapter (cylindrical coordinates) and the wave equation chapter (circular membranes) are now treated fully. The spherical harmonics from Unit 5 are revisited with deeper algebraic tools.

The material here also connects forward to Unit 7 (nonlinear PDEs), where the linearization of equations like the KdV equation around special solutions leads to Schrödinger-type operators whose spectral theory involves special functions; and to Unit 8 (variational methods), where weak formulations of Bessel and Legendre equations are handled naturally by Sobolev space theory.

## Physical Significance

Bessel functions describe the vibration of circular membranes (drumheads), the eigenmodes of cylindrical waveguides, heat flow in cylindrical fins, quantum mechanical states in an infinite cylindrical well, and the far-field diffraction pattern of a circular aperture. The Fraunhofer diffraction pattern of a circular aperture is proportional to $J_1(ka\sin\theta)/(ka\sin\theta)$ — the first nonzero of $J_1$ at $j_{1,1} \approx 3.832$ corresponds to the dark ring of the Airy disk.

Spherical harmonics appear in atomic physics (quantum numbers $\ell$ and $m$ label angular momentum eigenstates), geophysics (the shape of the geoid), cosmology (CMB anisotropy spectrum $C_\ell$), computer graphics (environment maps and lighting), and signal processing on the sphere. In all these settings, the same mathematical structure — eigenfunctions of the Laplace-Beltrami operator on $S^2$ — provides the unifying framework.
