# 8.1.1 The Bragg Reflector: Photonic Crystals in 1D

## A Stack of Mirrors

The simplest photonic crystal is a periodic stack of alternating dielectric layers: material A with refractive index $n_A$ and thickness $d_A$, followed by material B with index $n_B$ and thickness $d_B$, repeated $N$ times. This is the **Bragg reflector**, used as a mirror in VCSELs (Section 4.4.2), optical coatings, and laser cavities since the 1960s. Yet the Bragg reflector contains, in miniature, all of the essential physics of photonic bandgaps.

Consider light at normal incidence (wavevector $k = 0$ for the transverse component; only the $z$-direction matters). The wave equation in each layer is:

$$\frac{d^2 E}{dz^2} + k_i^2 E = 0, \quad k_i = \frac{n_i \omega}{c}$$

with $i = A$ or $B$. The solutions in each layer are forward and backward propagating plane waves. At each interface, the standard boundary conditions (continuity of $E_y$ and $H_x$ for TE polarization) connect the fields. The transfer matrix method tracks the field amplitudes through the entire stack.

## Transfer Matrix Method

For TE polarization (electric field perpendicular to the plane of incidence, parallel to the layer interfaces), the transfer matrix for a single interface from medium $i$ to medium $j$ is:

$$M_{ij} = \frac{1}{t_{ij}}\begin{pmatrix} 1 & r_{ij} \\ r_{ij} & 1 \end{pmatrix}$$

where $t_{ij} = 2n_i/(n_i + n_j)$ and $r_{ij} = (n_i - n_j)/(n_i + n_j)$ are the Fresnel coefficients. The propagation matrix through a layer of thickness $d_i$ and wavevector $k_i$ is:

$$P_i = \begin{pmatrix} e^{ik_i d_i} & 0 \\ 0 & e^{-ik_i d_i} \end{pmatrix}$$

The total transfer matrix for one unit cell (A followed by B) is:

$$M_{\text{cell}} = M_{AB} P_B M_{BA} P_A = M_{AB} P_B M_{AB}^{-1} P_A$$

(noting $M_{BA} = M_{AB}^{-1}$). The $N$-period stack has total matrix $M_{\text{cell}}^N$.

For large $N$, the reflectance $R = |r|^2$ where $r$ is extracted from $M_{\text{cell}}^N$ using:

$$\begin{pmatrix} E^+ \\ E^- \end{pmatrix}_{\text{output}} = M_{\text{cell}}^N \begin{pmatrix} E^+ \\ E^- \end{pmatrix}_{\text{input}}$$

and the reflectance from the input side is $r = -(M_{11})^{-1} M_{12}$ (more precisely: for input $E^+ = 1$ and no backward input $E^-_{\text{output}} = 0$).

## The Photonic Bandgap

The crucial insight comes from analyzing the eigenvalues of the unit cell matrix $M_{\text{cell}}$.

For a lossless periodic system, $M_{\text{cell}}$ is a unimodular matrix ($\det M_{\text{cell}} = 1$) with eigenvalues $e^{\pm iKa}$ where $K$ is the Bloch wavevector and $a = d_A + d_B$ is the period. The eigenvalue equation gives:

$$\cos(Ka) = \cos(k_A d_A)\cos(k_B d_B) - \frac{1}{2}\left(\frac{n_A}{n_B} + \frac{n_B}{n_A}\right)\sin(k_A d_A)\sin(k_B d_B)$$

This is the **Bragg dispersion relation** for the 1D photonic crystal.

Now consider the right-hand side as a function of frequency $\omega$ (since $k_i = n_i\omega/c$). When the right-hand side has magnitude $\leq 1$, $K$ is real and waves propagate — these are the allowed *bands*. When the magnitude $> 1$, $\cos(Ka)$ would exceed 1, which requires $K$ to be complex. The solution is $K = \pi/a + i\kappa$ with $\kappa > 0$ — the wave is evanescent, decaying as $e^{-\kappa z}$. This is the **photonic bandgap**.

The bandgap is centered at the **Bragg condition**:

$$k_A d_A = k_B d_B = \frac{\pi}{2}$$

which gives:

$$\frac{n_A \omega_{\text{Bragg}} d_A}{c} = \frac{\pi}{2} \implies d_A = \frac{\lambda_{\text{Bragg}}}{4n_A}$$

This is the **quarter-wave condition**: each layer is one quarter wavelength thick in the medium. The Bragg reflector is the optical analog of the BCC/FCC crystal diffraction condition in X-ray crystallography (Bragg's law).

The bandgap width scales as:

$$\frac{\Delta\omega_{\text{gap}}}{\omega_{\text{center}}} = \frac{4}{\pi}\arcsin\left(\frac{n_A - n_B}{n_A + n_B}\right) \approx \frac{4}{\pi}\frac{\Delta n}{n_{\text{avg}}}$$

for small index contrast $\Delta n = |n_A - n_B| \ll n_{\text{avg}}$. For Si/SiO₂ with $n_A = 3.478$, $n_B = 1.444$:

$$\frac{\Delta\omega_{\text{gap}}}{\omega_{\text{center}}} \approx \frac{4}{\pi} \times \frac{2.034}{2.461} \approx 1.05$$

This is 105% — larger than 1! In practice this means the bandgap extends from essentially zero to twice the Bragg frequency, an impossibly large bandgap. The formula breaks down for large index contrast; the full transfer matrix calculation gives a finite (though very large) bandgap.

For more realistic materials like TiO₂/SiO₂ ($n_A = 2.35$, $n_B = 1.46$):

$$\frac{\Delta\omega_{\text{gap}}}{\omega_{\text{center}}} \approx \frac{4}{\pi} \times \frac{0.89}{1.905} \approx 0.59$$

Meaning a bandgap of about 59% of the center frequency — still enormous compared to typical electronic bandgaps.

## Reflectance of Finite Bragg Reflectors

For a finite stack of $N$ pairs, the peak reflectance at the Bragg frequency approaches 1 exponentially with $N$:

$$R_N = \left(\frac{1 - (n_A/n_B)^{2N}}{1 + (n_A/n_B)^{2N}}\right)^2$$

(for equal-impedance surrounding media). For Si/SiO₂ with $n_A/n_B = 2.41$:
- $N = 5$: $R \approx 0.9986$
- $N = 10$: $R \approx 0.9999997$
- $N = 20$: $(n_A/n_B)^{20} \approx 10^{12}$, essentially perfect reflectance

This is why VCSEL top and bottom mirrors need only 20–30 pairs to achieve >99.5% reflectance — each pair contributes the same multiplicative factor.

## Analogy with Quantum Mechanics

The Bragg dispersion relation is mathematically identical to the Kronig-Penney model in quantum mechanics: the propagation of electrons through a periodic potential. In the quantum case, the periodic potential opens energy gaps at the Brillouin zone boundary $k = \pi/a$; in the photonic case, the periodic dielectric opens frequency gaps at the same wavevector boundary.

This analogy is not superficial — both arise from the same mathematics of Bloch's theorem, which we develop in the next subsection for the full 3D case. The analogy is productive: intuitions developed for electronic band structure (effective mass, density of states, impurity levels, topological invariants) all have photonic counterparts, and many phenomena first discovered electronically have been subsequently found optically.

---

## Key Results

- **Bragg condition**: Quarter-wave stack ($d_i = \lambda/(4n_i)$) maximizes reflectance and opens the bandgap.
- **Bandgap width**: $\Delta\omega/\omega \approx (4/\pi)\Delta n/n_{\text{avg}}$ for small contrast.
- **Reflectance**: Approaches 1 exponentially with number of pairs $N$.
- **Transfer matrix method**: Provides an exact solution for arbitrary multilayer stacks; the foundation for all 1D photonic crystal calculations.

---

## References

[1] Born, M. & Wolf, E. (1980). *Principles of Optics*, 6th ed. Pergamon Press. [Chapter 1 covers the transfer matrix method for multilayer optical systems — the definitive reference.]

[2] Joannopoulos, J.D., Johnson, S.G., Winn, J.N., & Meade, R.D. (2008). *Photonic Crystals: Molding the Flow of Light*, 2nd ed. Princeton University Press. [The standard textbook for photonic crystals; available free online at ab-initio.mit.edu/book. Chapter 4 covers 1D photonic crystals.]

[3] Yeh, P. (1988). *Optical Waves in Layered Media*. Wiley. [The comprehensive treatment of multilayer optics and 1D photonic structures.]
