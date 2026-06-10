# 3.3.1 — The Nonlinear Polarization Expansion

## Beyond Linear Response

In the linear regime, the polarization $\mathbf{P}$ is proportional to $\mathbf{E}$:

$$\mathbf{P}^{(1)} = \varepsilon_0 \chi^{(1)} \mathbf{E}$$

When the field becomes large enough that the electron's displacement is no longer small compared to the atomic dimensions, higher powers of $\mathbf{E}$ contribute. The Taylor expansion of $\mathbf{P}$ in powers of $\mathbf{E}$:

$$\mathbf{P} = \varepsilon_0(\chi^{(1)}\mathbf{E} + \chi^{(2)}\mathbf{E}^2 + \chi^{(3)}\mathbf{E}^3 + \cdots)$$

More precisely, since $\mathbf{P}$ and $\mathbf{E}$ are vectors and $\chi^{(n)}$ is a rank-$(n+1)$ tensor:

$$P_i = \varepsilon_0\left[\chi^{(1)}_{ij}E_j + \chi^{(2)}_{ijk}E_jE_k + \chi^{(3)}_{ijkl}E_jE_kE_l + \cdots\right]$$

(with implicit summation over repeated indices). The terms $\mathbf{P}^{(2)} = \varepsilon_0\chi^{(2)}\mathbf{E}\mathbf{E}$ and $\mathbf{P}^{(3)} = \varepsilon_0\chi^{(3)}\mathbf{E}\mathbf{E}\mathbf{E}$ are the nonlinear polarizations responsible for $\chi^{(2)}$ and $\chi^{(3)}$ effects.

## Physical Estimate of Magnitudes

The nonlinear susceptibilities decrease rapidly with order. The ratio $\chi^{(2)}/\chi^{(1)} \sim e/(m\omega_0^2 x_\text{sat})$ where $x_\text{sat} \sim a_0$ (atomic displacement where anharmonicity becomes important). Numerically:

$$\chi^{(2)} \sim \chi^{(1)} / E_\text{atom} \approx 10/5 \times 10^{11} \approx 2 \times 10^{-11} \text{ m/V} = 20 \text{ pm/V}$$

$$\chi^{(3)} \sim \chi^{(1)} / E_\text{atom}^2 \approx 10^{-21} \text{ m}^2/\text{V}^2$$

These are rough estimates. Typical values for real materials:
- LiNbO₃: $\chi^{(2)} \approx 2d_{33} \approx 2 \times 27 = 54$ pm/V (where $d_{33}$ is the nonlinear coefficient)
- BBO: $\chi^{(2)} \approx 2 \times 2 = 4$ pm/V
- Silicon: $\chi^{(2)} = 0$ (by symmetry — see below); $\chi^{(3)} \approx 6 \times 10^{-19}$ m²/V²
- Silica fiber: $\chi^{(3)} \approx 2.5 \times 10^{-22}$ m²/V² (very weak but accumulated over km)

## Symmetry Constraints on $\chi^{(2)}$

The second-order susceptibility $\chi^{(2)}$ vanishes in centrosymmetric materials (materials with inversion symmetry). This is a group-theory result: if the crystal has inversion symmetry $\mathbf{r} \to -\mathbf{r}$, then $\mathbf{P}^{(2)}(-\mathbf{E}) = -\mathbf{P}^{(2)}(\mathbf{E})$ (the polarization reverses with the field, as required by linearity). But $\mathbf{P}^{(2)} = \varepsilon_0\chi^{(2)}\mathbf{E}^2$ transforms as $\mathbf{P}^{(2)}(-\mathbf{E}) = \varepsilon_0\chi^{(2)}(-\mathbf{E})^2 = +\mathbf{P}^{(2)}(\mathbf{E})$ — which means $\mathbf{P}^{(2)} = 0$ for a centrosymmetric material.

**Silicon ($O_h$ point group: centrosymmetric)**: $\chi^{(2)}_\text{bulk} = 0$. This is why silicon has no electro-optic (Pockels) effect in bulk. The plasma dispersion effect (Soref-Bennett) is the *only* practical electro-optic effect in silicon, and it is a free-carrier effect, not a true $\chi^{(2)}$ effect.

**LiNbO₃ (non-centrosymmetric)**: $\chi^{(2)} \neq 0$, giving the Pockels effect (Section 2.4.4). This is why LiNbO₃ is the dominant electro-optic modulator material.

**At surfaces and interfaces**: inversion symmetry is always broken. Therefore, surfaces of centrosymmetric materials like Si have a non-zero $\chi^{(2)}$ (surface SHG). This surface SHG is used to probe surface conditions and strain in silicon photonic devices.

**Strained silicon**: If silicon is strained (by applying mechanical stress or by depositing a strained film), the inversion symmetry is broken and a small bulk $\chi^{(2)}$ can be induced. This approach has been explored for Pockels-effect modulation in silicon [1].

## The Nonlinear Refractive Index

The $\chi^{(3)}$ term contributes an intensity-dependent correction to the refractive index. For a field $E = E_0\cos\omega t$:

$$P^{(3)} = \varepsilon_0\chi^{(3)}E^3 = \varepsilon_0\chi^{(3)}E_0^3\cos^3\omega t = \varepsilon_0\chi^{(3)}E_0^3\left[\frac{3}{4}\cos\omega t + \frac{1}{4}\cos 3\omega t\right]$$

The term at frequency $\omega$ (the same frequency as the input) contributes a correction to the linear susceptibility:

$$\chi_\text{eff}(\omega) = \chi^{(1)} + \frac{3}{4}\chi^{(3)}|E_0|^2$$

The refractive index correction:

$$n = n_0 + \Delta n, \quad \Delta n = \frac{3\chi^{(3)}}{8n_0}|E_0|^2 = n_2 I$$

where $n_2 = 3\chi^{(3)}/(4\varepsilon_0 cn_0^2)$ is the *nonlinear refractive index* (units: m²/W). This is the **Kerr effect**: the refractive index depends linearly on the optical intensity $I$.

The term at frequency $3\omega$ (the *third-harmonic generation* term) drives polarization at three times the input frequency — producing light at $3\omega$ if phase-matched.

**Values of $n_2$**:
| Material | $n_2$ (m²/W) |
|---------|-------------|
| Silica fiber | $2.6 \times 10^{-20}$ |
| Silicon (1550 nm) | $6 \times 10^{-18}$ (230× larger than silica!) |
| Silicon nitride | $2.4 \times 10^{-19}$ |
| LiNbO₃ | $1.8 \times 10^{-19}$ |
| Chalcogenide glass | $10^{-16}$ to $10^{-14}$ (very large) |

Silicon has a large $n_2$ (∼100× silica) due to its small bandgap. Silicon nanowire waveguides with their small mode areas ($A_\text{eff} \sim 0.1$ μm²) have an effective nonlinear parameter $\gamma = n_2\omega/(cA_\text{eff}) \approx 250$ W⁻¹m⁻¹ at 1550 nm — five orders of magnitude larger than standard single-mode fiber ($\gamma \approx 10^{-3}$ W⁻¹m⁻¹).

## Summary

- Nonlinear polarization: $P = \varepsilon_0(\chi^{(1)}E + \chi^{(2)}E^2 + \chi^{(3)}E^3 + \cdots)$.
- $\chi^{(2)} = 0$ in centrosymmetric materials (silicon bulk). Non-zero in LiNbO₃, BBO, KTP.
- $\chi^{(3)}$ always nonzero; gives intensity-dependent $n = n_0 + n_2 I$ (Kerr effect).
- Silicon $n_2 \approx 6 \times 10^{-18}$ m²/W, with large $\gamma$ in nanowire waveguides due to small mode area.

---

*References*

[1] Jacobsen, R.S. et al. (2006). Strained silicon as a new electro-optic material. *Nature*, 441(7090), 199–202. [DOI: 10.1038/nature04706] [Demonstrates $\chi^{(2)}$ in strained silicon by breaking inversion symmetry.]
