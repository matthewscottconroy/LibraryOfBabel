# 1.5.2 Radiation Pressure and Optical Forces

## Light Carries Momentum

The fact that light carries momentum is a conceptual surprise — photons have no mass. But momentum is not the exclusive property of massive objects; it is associated with all physical fields. Maxwell showed in 1873 that the momentum density of the electromagnetic field is [1]:

$$\mathbf{g} = \frac{\mathbf{S}}{c^2} = \varepsilon_0(\mathbf{E}\times\mathbf{B})$$

For a plane wave, the time-averaged momentum density is $\langle g\rangle = I/c^2$.

## Radiation Pressure on a Surface

When an electromagnetic wave is absorbed by a surface, its momentum is transferred to the surface, exerting a force. The pressure (force per unit area) exerted by a wave with intensity $I$ [W/m²]:

- **On an absorbing surface** (all light absorbed): $P = I/c$
- **On a perfectly reflecting surface** (all light reflected): $P = 2I/c$ (momentum is reversed, so the change is twice as large)

**Magnitude estimate**: For $I = 10^7\ \text{W/m}^2$ on a reflecting surface:
$$P = 2I/c = 2\times10^7/(3\times10^8) \approx 0.067\ \text{Pa}$$

This is small — about $6.7\times10^{-7}$ atm — but on micron-scale objects, it becomes significant.

## Optical Tweezers: Trapping with Light

The gradient force exerted by a strongly focused laser beam on a small dielectric particle is the basis of **optical tweezers**, for which Arthur Ashkin received the Nobel Prize in Physics in 2018 [2].

A dielectric particle in an inhomogeneous light field experiences a gradient force pulling it toward the intensity maximum (for particles with refractive index higher than the surrounding medium). By focusing a laser to a tight spot, the particle is trapped at the focus and can be manipulated in three dimensions.

This principle is relevant to photonic computing in an unexpected way: as photonic chips become denser, the optical forces between adjacent waveguides can become significant. The field enhancement in high-Q resonators can also exert measurable mechanical forces on the resonator structure, coupling optical and mechanical degrees of freedom (optomechanics) [3].

## Optomechanics and Photonic Computing

The coupling between optical fields and mechanical motion in microresonators — optomechanics — creates a regime in which:
- Optical fields drive mechanical oscillations (radiation pressure)
- Mechanical motion modulates optical fields (via the photoelastic effect and moving boundaries)

This bi-directional coupling has been exploited for sensing (displacement at the $10^{-19}$ m level), signal transduction, and microwave-to-optical frequency conversion [4]. It is also a potential noise source in photonic computing chips: thermal fluctuations can cause mechanical vibrations that perturb the optical path length in waveguides, causing phase noise. Understanding radiation pressure is therefore relevant not just as abstract physics but as a practical engineering consideration.

---

## References

[1] Maxwell, J.C. (1873). *A Treatise on Electricity and Magnetism*. Oxford: Clarendon Press. Vol. 2, §792.

[2] Ashkin, A., Dziedzic, J.M., Bjorkholm, J.E., & Chu, S. (1986). "Observation of a single-beam gradient force optical trap for dielectric particles." *Optics Letters*, 11(5), 288–290. [The paper introducing the single-beam optical trap; Ashkin received the Nobel Prize in Physics 2018 for this work.]

[3] Kippenberg, T.J., & Vahala, K.J. (2008). "Cavity optomechanics: back-action at the mesoscale." *Science*, 321(5893), 1172–1176.

[4] Aspelmeyer, M., Kippenberg, T.J., & Marquardt, F. (2014). "Cavity optomechanics." *Reviews of Modern Physics*, 86(4), 1391.
