# Section 8.2: Metasurfaces

A conventional lens works by refraction: light passes through a curved glass surface, accumulating different amounts of phase at different radial positions because the optical path length varies with thickness. The lens must be thick — millimeters to centimeters — to accumulate enough phase variation to focus or collimate a beam over a useful aperture.

A **metasurface** replaces the thick lens with a single ultrathin layer of sub-wavelength resonant structures. Each structure imparts a locally controlled phase shift $\phi(\mathbf{r})$ to the transmitted (or reflected) light, without requiring significant propagation distance. The entire thickness of the device is a few hundred nanometers — comparable to the wavelength of visible light — yet it can perform the same functions as a centimeter-thick lens, a grating, a hologram, or a beam steering device.

The key enabling idea is that sub-wavelength resonators can be designed to impart any phase shift between 0 and $2\pi$ by adjusting their geometry. A metallic nano-antenna, a dielectric pillar, a patterned hole — all of these, if designed correctly, act as local phase shifters. Arrange them in a 2D array with a spatially varying geometry, and you have a device that applies any desired spatial phase pattern to an incident optical beam.

This is not a mere engineering trick. It is a fundamental shift in how optical transformations are implemented: from accumulated phase via path length to controlled resonant coupling in a monolayer. The implications extend to any technology that requires precisely controlled optical wavefronts — imaging, communications, and computation.

This section develops metasurface physics and its relevance to optical computing in three subsections:

**Subsection 8.2.1 — The Metasurface Concept**: The generalized Snell's law, phase-gradient metasurfaces, and the physical mechanisms by which sub-wavelength structures achieve 0–$2\pi$ phase control.

**Subsection 8.2.2 — The Pancharatnam-Berry Phase**: A powerful and elegant mechanism for phase control based on the geometry of polarization rotations, entirely independent of resonance conditions. The geometric phase accumulates whenever a polarization state is transported around a closed loop on the Poincaré sphere.

**Subsection 8.2.3 — Metasurfaces for Optical Computing**: The D²NN (diffractive deep neural network) architecture, where light computes as it passes through multiple metasurface layers. The physical basis for this architecture, its current state of experimental realization, and its genuine and speculative capabilities.
