# Section 2.2: Interference

Interference is the phenomenon at the heart of wave optics — and at the heart of most photonic computing architectures. When two or more electromagnetic waves are present simultaneously at a point, the total field is their vector sum (superposition). Because intensity is proportional to the square of the field amplitude, the intensity of the combined wave is generally *not* the sum of the individual intensities. The cross terms — the interference terms — can constructively add intensity (where the waves are in phase) or destructively cancel it (where they are out of phase). The spatial and temporal pattern of these additions and cancellations encodes information about the phases of the contributing waves, which is why interference is both a measurement tool and a computational mechanism.

To see the fundamental significance: the Mach-Zehnder interferometer (MZI), the building block of virtually all photonic neural network processors, is an interference device. The weight applied to an input signal is determined by the phase difference between two optical paths, which is set by an electro-optic phase modulator. The computation happens because of interference — the two paths combine constructively or destructively in a controllable way. Without wave optics, without interference, there is no MZI, and without MZIs, the current generation of photonic AI accelerators does not exist.

This section develops the physics of interference from its simplest form (two-beam superposition) through the two devices most important for photonic computing.

## Subsections

- **2.2.1 — Superposition and Visibility**: The fundamental interference condition; fringe visibility; conditions for interference.
- **2.2.2 — Young's Double Slit**: The canonical interference experiment; wavelength measurement; spatial coherence.
- **2.2.3 — The Fabry-Pérot Cavity**: Resonance from multiple-beam interference; the Airy function; finesse; Q factor; free spectral range. The physical basis of ring resonators and laser cavities.
- **2.2.4 — The Mach-Zehnder Interferometer**: Two-path interference; the MZI transfer function; how phase difference controls intensity splitting; the MZI as a unitary 2×2 operation; MZI meshes.
