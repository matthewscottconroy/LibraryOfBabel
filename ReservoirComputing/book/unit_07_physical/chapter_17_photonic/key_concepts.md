# Chapter 17 — Key Concepts

---

## Optoelectronic Delay-Feedback Reservoir

A physical reservoir consisting of a semiconductor laser, electro-optic modulator (nonlinear element), optical fiber delay loop, photodetector, and feedback amplifier. Governed by the delay-differential equation:
$$\tau_R \frac{dx}{dt} + x(t) = f(\beta x(t-\tau) + \gamma u(t))$$
The time-multiplexed virtual nodes are extracted by sampling the photodetector output at $N$ equally spaced times per clock cycle. First demonstrated by Appeltant et al. [Appeltant2011].

---

## Ikeda Nonlinearity

The sinusoidal transfer function of a Mach-Zehnder interferometer:
$$f(v) = \sin^2\!\left(\frac{\pi v}{2V_\pi} + \phi_0\right)$$
Named for K. Ikeda, who first analyzed the chaotic dynamics of delay-feedback optical systems with this nonlinearity [Ikeda1979]. Provides smooth, bounded nonlinearity essential for stable reservoir operation. At quadrature ($\phi_0 = \pi/4$), the response is most linear near zero input, which is often the optimal operating point.

---

## Mackey-Glass Oscillator

A first-order delay-differential equation:
$$\frac{dx}{dt} = \frac{ax(t-\tau)}{1 + x(t-\tau)^{10}} - bx(t)$$
with chaotic dynamics for $\tau > 16.8$ (standard parameters). Structurally similar to the Ikeda DDE; both are scalar DDEs with delayed nonlinear feedback. Originally proposed as a model of physiological hematopoiesis [Mackey1977]. Used as both a hardware architecture guide and a benchmark time series throughout the reservoir computing literature.

---

## Delay-Differential Equation (DDE)

An ordinary differential equation in which the derivative depends on the current state and on the state at one or more earlier times (the "delays"). DDEs are formally infinite-dimensional: the state at time $t$ is the function segment $x(\cdot)|_{[t-\tau, t]}$. Physical delay-feedback reservoirs implement DDEs in hardware, leveraging the infinite-dimensional phase space to provide rich dynamics without explicit construction of many physical degrees of freedom.

---

## Micro-Ring Resonator (MRR)

A small silicon waveguide loop (radius $\sim 5$ µm) that supports resonant optical modes. Light at resonant wavelengths is trapped in the ring and amplified; off-resonant light passes through. The coupled mode equations describe how the ring field $a$ responds to an input field $s_{in}$. In the Vandoorne reservoir, TPA and free-carrier effects in silicon make the ring's response nonlinear and history-dependent — the essential reservoir dynamics.

---

## Two-Photon Absorption (TPA)

A nonlinear optical effect in silicon: simultaneous absorption of two photons creates one conduction-band electron-hole pair. TPA provides the primary nonlinearity in the Vandoorne silicon photonic reservoir. It is proportional to the square of the optical intensity, making it a quadratic (Kerr-like) nonlinear effect. The generated free carriers also modify the refractive index (free-carrier dispersion) and absorb additional light (free-carrier absorption), creating complex, intensity-dependent dynamics.

---

## Free-Carrier Dispersion (FCD)

The change in refractive index of silicon due to photo-generated free carriers (electrons and holes). FCD creates a dynamic phase shift in the micro-ring resonator that depends on the history of optical intensity (since carriers take time to recombine). This time-dependent phase shift is the source of fading memory in the passive silicon reservoir. The carrier lifetime ($\sim 0.5$ ns) sets the effective memory time constant.

---

## Silicon Photonics

The technology of fabricating optical waveguides and components on silicon chips using standard CMOS-compatible processes. Enables integration of photonic components at the sub-micron scale, with potential for millions of optical elements per chip. The Vandoorne reservoir [Vandoorne2014] demonstrated the first on-chip passive optical reservoir using this platform.

---

## Processing Speed Hierarchy

From slowest to fastest for reservoir computing implementations:
1. GPU-based digital ESN: $\sim 10^6$ symbols/second
2. Optoelectronic time-multiplexed (Appeltant): $\sim 10^7$ sym/s
3. Silicon photonic with carrier limit (Vandoorne): $\sim 10^9$ sym/s
4. Theoretical all-optical (photon-lifetime limited): $\sim 10^{11}$ sym/s

The speedup from digital to photonic is $10^3$–$10^5$, with power consumption reduced by $10^3$–$10^6$.

---

## Passive vs. Active Physical Reservoir

**Passive:** Uses only passive optical elements (waveguides, resonators) whose nonlinear dynamics arise from optical physics (TPA, FCD). No electronic driving required after input injection. Lower power, potentially higher speed. Harder to tune and reprogram. (Vandoorne 2014.)

**Active:** Uses active electronic-optic modulation (MZI driven by feedback amplifier). More easily controlled and tuned. Requires electronic components and power for the feedback chain. (Appeltant 2011, Brunner 2013.)
