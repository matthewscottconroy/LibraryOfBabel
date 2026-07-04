# Section 14.2: Optical Implementations

## What This Section Is About

The diffractive framework of Section 14.1 is platform-agnostic. It asks only for a stack of thin, spatially patterned phase screens separated by free-space propagation, with the transmission of each trainable pixel written as $t^l(x,y) = e^{i\phi^l(x,y)}$. Nothing in the Rayleigh–Sommerfeld forward model or the differentiable-diffraction training that optimizes it fixes a wavelength, a material, or a fabrication process. Three device families realize the same mathematics across the spectrum, and they divide along two axes that organize this section: **fixed versus reconfigurable**, and **free-space versus on-chip**.

**14.2.1 — 3D-printed phase plates** encode the phase mask as surface height in a dielectric slab. Cheap, entirely passive, and fixed once printed, they are the natural home of the terahertz and millimetre-wave demonstrations that launched the field, where the wavelength is large enough that a printer's voxel is a small fraction of it.

**14.2.2 — Spatial light modulators** replace the printed slab with an electrically addressed liquid-crystal or micromirror array at visible and near-infrared wavelengths. Their pixels are reprogrammable, turning a static optical network into a reconfigurable "diffractive processing unit" whose weights can be updated between inferences — the enabling step for hardware-in-the-loop training.

**14.2.3 — Metasurfaces** shrink each pixel below the wavelength using dielectric meta-atoms, producing ultrathin, CMOS-compatible layers with enormous space-bandwidth product, and — in their on-chip incarnation — bringing diffractive computing into an integrated photonic waveguide.

The organizing point is that the platform is not a mere implementation detail. It sets the operating wavelength, the pixel pitch (and hence the space-bandwidth product and the required inter-layer spacing), and, decisively, whether the network's weights can change at all after fabrication.
