# 6.1.3 The Eigenvalue Equation and Mode Cutoff

## Normalized Variables

The eigenvalue equations from Section 6.1.2 can be written in terms of normalized frequency $V$ and normalized propagation constant $b$:

$$V = k_0 d \sqrt{n_1^2 - n_2^2} = \frac{2\pi d}{\lambda}\text{NA}$$

$$b = \frac{\beta^2/k_0^2 - n_2^2}{n_1^2 - n_2^2} = \frac{n_{eff}^2 - n_2^2}{n_1^2 - n_2^2}$$

where $n_{eff} = \beta/k_0$ is the effective index ($n_2 < n_{eff} < n_1$ for a guided mode).

In terms of $V$ and $b$:
- $\kappa d/2 = V\sqrt{1-b}/2$
- $\gamma d/2 = V\sqrt{b}/2$

The eigenvalue equations become:

**TE even**: $\tan(V\sqrt{1-b}/2) = \sqrt{b/(1-b)}$

**TE odd**: $-\cot(V\sqrt{1-b}/2) = \sqrt{b/(1-b)}$

These are transcendental equations that must be solved numerically or graphically. The solution $b(V)$ gives the normalized propagation constant as a function of normalized frequency — the dispersion curve of the waveguide.

## Cutoff Condition

A mode reaches cutoff when $b \to 0$, i.e., $\gamma \to 0$: the evanescent field extends to infinity in the cladding, the mode is no longer confined. At cutoff:

**TE/TM even (m=0)**: Cutoff at $V_c = 0$. The fundamental mode has NO cutoff — it is guided for any waveguide, no matter how thin. (Though as $d \to 0$, the mode becomes very weakly confined and spreads into the cladding.)

**TE/TM odd (m=1)**: Cutoff at $V_c = \pi/2$ (TE) or $V_c = \pi n_1/(2n_2)$ (TM, approximately).

**General m-th order mode**: Cutoff at $V_c = m\pi/2$ for TE modes.

## Single-Mode Condition

The waveguide supports only the fundamental mode when $V < \pi/2$ (first-order mode is at cutoff). Using $V = 2\pi d \text{NA}/\lambda$:

$$d < \frac{\lambda}{4\text{NA}} \equiv d_{sm}$$

**For a Si/SiO₂ slab waveguide** at 1550 nm: NA = 3.17, $d_{sm} = 1550/(4 \times 3.17) = 122$ nm. But this is for a slab (1D confinement). For a 2D waveguide (strip or ridge), the full 2D eigenvalue problem must be solved numerically (e.g., by finite difference mode solving).

**For silicon strip waveguides at 1550 nm**: The empirical single-mode condition is approximately width $< 450$–500 nm, height $= 220$ nm for TE, and width $< 400$ nm for TM. This is why the standard silicon photonic waveguide dimension is 450 × 220 nm — it is the widest single-mode strip waveguide, maximizing mode area (and therefore minimizing nonlinearity and roughness-induced loss) while remaining single-mode.

## Effective Index Method

For 2D (strip) waveguides, the effective index method provides an analytical approximation: solve the 1D eigenvalue problem in the height direction first (getting an effective index $n_{eff,h}$ for the core slab), then use this as the "core index" in a horizontal slab waveguide to find the horizontal mode structure. The method is accurate to within a few percent for weakly-guiding waveguides and useful for rapid design estimation.

## Group Velocity and Dispersion

The propagation constant $\beta(\omega)$ determines the phase velocity $v_{ph} = \omega/\beta$ and the group velocity:

$$v_g = \frac{d\omega}{d\beta} = \frac{c}{n_g}$$

where $n_g = n_{eff} + \omega \frac{dn_{eff}}{d\omega}$ is the group index. The group velocity dispersion (GVD) is:

$$\beta_2 = \frac{d^2\beta}{d\omega^2} = \frac{1}{v_g^2}\frac{dv_g}{d\omega}$$

In silicon strip waveguides, the waveguide contribution to GVD ($\beta_{2,wg}$) is large and typically anomalous at 1550 nm — opposing the normal material dispersion of silicon — allowing dispersion engineering by adjusting waveguide dimensions. This is the basis of dispersion-engineered silicon waveguides for parametric amplification and frequency comb generation.
