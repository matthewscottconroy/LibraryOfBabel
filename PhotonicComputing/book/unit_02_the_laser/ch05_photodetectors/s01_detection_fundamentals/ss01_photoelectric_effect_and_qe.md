# 5.1.1 The Photoelectric Effect and Quantum Efficiency

## From Einstein to the Photodiode

Einstein's 1905 explanation of the photoelectric effect established that light transfers energy in discrete quanta $E = h\nu = \hbar\omega$ [1]. An electron is liberated from a metal surface if and only if the photon energy exceeds the work function $\phi$: no number of low-frequency photons can eject an electron, but a single photon with $h\nu > \phi$ can. This is not classical behavior; it is quantum mechanical.

The modern semiconductor photodiode generalizes this: when a photon with energy $\hbar\omega > E_g$ (the semiconductor bandgap) is absorbed, it creates an electron-hole pair. The electron and hole are separated by the electric field in the depletion region of a p-n junction and collected as a photocurrent.

## Quantum Efficiency

The **quantum efficiency (QE)** $\eta$ is the probability that an incident photon produces one electron-hole pair:

$$\eta = \frac{\text{number of electron-hole pairs collected}}{\text{number of incident photons}}$$

QE depends on:
1. **Reflection at the surface**: A fraction $R$ of photons is reflected before entering the device. Anti-reflection coatings reduce this below 1%.
2. **Absorption**: Photons that enter the device are absorbed with probability $1 - e^{-\alpha(\lambda)d}$ where $\alpha(\lambda)$ is the absorption coefficient and $d$ is the absorbing layer thickness.
3. **Collection**: Not all electron-hole pairs reach the contacts before recombining. The collection efficiency depends on minority carrier diffusion length and sweep-out velocity.

$$\eta = (1-R)(1 - e^{-\alpha d}) \eta_{coll}$$

For a well-designed p-i-n photodiode with AR coating: $\eta \approx 0.7$–0.95 at the design wavelength.

## Responsivity

The **responsivity** $\mathcal{R}$ relates photocurrent to optical power:

$$I_{ph} = \mathcal{R} P_{in}$$

$$\mathcal{R} = \frac{\eta e}{\hbar\omega} = \frac{\eta e \lambda}{hc} \quad \text{[A/W]}$$

For $\eta = 0.9$ at $\lambda = 1550$ nm:

$$\mathcal{R} = \frac{0.9 \times 1.6 \times 10^{-19}}{6.626 \times 10^{-34} \times (3\times10^8/1.55\times10^{-6})} = \frac{0.9 \times 1.6 \times 10^{-19}}{1.28\times10^{-19}} \approx 1.12 \text{ A/W}$$

Theoretical maximum responsivity ($\eta = 1$): $\mathcal{R}_{max} = e\lambda/(hc) = 1.25$ A/W at 1550 nm.

Practical values for Ge-on-Si photodetectors: $\mathcal{R} \approx 0.8$–1.0 A/W at 1550 nm.

## Wavelength Dependence and Cutoff

Responsivity goes to zero at two wavelengths:
1. **Long-wavelength cutoff** ($\lambda > \lambda_c = hc/E_g$): photon energy insufficient to create electron-hole pair. For Ge: $E_g = 0.67$ eV → $\lambda_c = 1850$ nm. For Si: $E_g = 1.12$ eV → $\lambda_c = 1100$ nm. Silicon is transparent at 1310 nm and 1550 nm; germanium absorbs strongly at both.
2. **Short-wavelength rolloff**: absorption becomes too strong, all photons absorbed near the surface before reaching the depletion region; surface recombination reduces collection efficiency.

This explains why silicon photonic integrated circuits use **germanium** for on-chip photodetectors: Si cannot absorb 1310 nm or 1550 nm photons, while Ge does efficiently.

## Reference

[1] Einstein, A. (1905). "Über einen die Erzeugung und Verwandlung des Lichtes betreffenden heuristischen Gesichtspunkt." *Annalen der Physik*, 322(6), 132–148.
