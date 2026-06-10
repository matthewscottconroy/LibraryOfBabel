# 7.2.2 Directional Couplers

## Coupled Mode Theory

A directional coupler (DC) consists of two closely spaced parallel waveguides. The evanescent fields of the two modes overlap, causing power to transfer periodically between the waveguides. The coupled mode equations are (for equal waveguides):

$$\frac{dA_1}{dz} = i\beta A_1 + i\kappa A_2$$
$$\frac{dA_2}{dz} = i\kappa A_1 + i\beta A_2$$

where $\kappa$ is the coupling coefficient (cm$^{-1}$, real for lossless coupling). The solutions with initial condition $A_1(0) = 1$, $A_2(0) = 0$:

$$A_1(z) = \cos(\kappa z)e^{i\beta z}$$
$$A_2(z) = i\sin(\kappa z)e^{i\beta z}$$

Power transfer:
$$P_1(z) = \cos^2(\kappa z), \quad P_2(z) = \sin^2(\kappa z)$$

The coupling length for complete power transfer ($P_1 = 0$, $P_2 = 1$) is $L_c = \pi/(2\kappa)$.

## Coupling Coefficient Calculation

The coupling coefficient depends on the gap between waveguides and the waveguide geometry:

$$\kappa = \frac{\omega}{4} \frac{\int_{\text{guide 2}} (n^2 - n_{clad}^2) E_1 \cdot E_2^* dA}{\int_{\text{all}} (E_1 \times H_1^*) \cdot \hat{z} dA}$$

For 450 × 220 nm Si strip waveguides with SiO₂ cladding, typical values:
- 100 nm gap: $\kappa \approx 5000$ m$^{-1}$, $L_c \approx 3$ μm
- 200 nm gap: $\kappa \approx 1500$ m$^{-1}$, $L_c \approx 10$ μm
- 300 nm gap: $\kappa \approx 300$ m$^{-1}$, $L_c \approx 50$ μm

**Sensitivity to fabrication variations**: The coupling coefficient is exponentially sensitive to gap width (since it involves the evanescent overlap). A ±10 nm variation in gap width changes $\kappa$ by ~10–20%, changing the splitting ratio significantly. This sensitivity is why directional couplers must be carefully designed with simulation and calibrated on-chip.

## 50:50 Splitter Design

A 50:50 beam splitter requires $\kappa L = \pi/4$, i.e., $L = L_c/2$. For a 200 nm gap DC: $L_{50:50} \approx 5$ μm coupling section. The total device is typically ~20 μm long including S-bend access waveguides.

**Wavelength dependence**: Unlike an MMI coupler, a directional coupler has a wavelength-dependent splitting ratio (because $\kappa$ and hence $L_c$ depend on wavelength). The splitting ratio variation is approximately 1–2% per nm over the C-band for a well-designed DC. This is acceptable for most photonic computing applications but must be accounted for in WDM systems.

## Directional Coupler as the MZI Building Block

The MZI matrix of Chapter 2 (Section 2.2.4) is built from beam splitters that are implemented as directional couplers on-chip. The 2×2 unitary transfer matrix:

$$U_{DC} = \frac{1}{\sqrt{2}}\begin{pmatrix}1 & i \\ i & 1\end{pmatrix}$$

corresponds to a 50:50 DC with the $i = e^{i\pi/2}$ phase arising from the coupling matrix (the phase on the coupled port is $\pi/2$ ahead of the transmitted port). This is the same matrix derived from symmetry in Chapter 2.
