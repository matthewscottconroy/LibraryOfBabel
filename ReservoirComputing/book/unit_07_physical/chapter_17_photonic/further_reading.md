# Chapter 17 — Further Reading and References

---

## Essential References

### [Appeltant2011]

**Appeltant, L., Soriano, M.C., Van der Sande, G., Danckaert, J., Massar, S., Dambre, J., Schrauwen, B., Mirasso, C.R., & Fischer, I. (2011). Information processing using a single dynamical node as complex systems. *Nature Communications*, 2, 468.**

The founding paper of experimental physical reservoir computing. Short, clear, and contains the full mathematical treatment of the virtual node construction. Required reading.

### [Vandoorne2014]

**Vandoorne, K., Mechet, P., Van Vaerenbergh, T., Fiers, M., Morthier, G., Verstraeten, D., Schrauwen, B., Dambre, J., & Bienstman, P. (2014). Experimental demonstration of reservoir computing on a silicon photonic chip. *Nature Communications*, 5, 3541.**

The on-chip photonic reservoir paper. Demonstrates that passive optical physics can implement reservoir computation. Contains the coupled mode theory derivation, the TPA model, and experimental verification.

### [Brunner2013]

**Brunner, D., Soriano, M.C., Mirasso, C.R., & Fischer, I. (2013). Parallel photonic information processing at gigabyte per second data rates using transient states. *Nature Communications*, 4, 1364.**

Spatial (parallel) photonic reservoir at Gb/s rates. The architectural complement to the temporal-multiplexing approach: many nodes processed simultaneously rather than sequentially.

---

## Optoelectronic Delay Systems

### [Larger2012]

**Larger, L. et al. (2012). Photonic information processing beyond Turing: An optoelectronic implementation of reservoir computing. *Optics Express*, 20(3), 3241–3249.**

Full characterization of the optoelectronic delay reservoir, including spoken digit recognition results.

### [Paquot2012]

**Paquot, Y., Duport, F., Smerieri, A., Dambre, J., Schrauwen, B., Haelterman, M., & Massar, S. (2012). Optoelectronic reservoir computing. *Scientific Reports*, 2, 287.**

Independent demonstration of the optoelectronic reservoir by the Brussels group. Provides complementary experimental data and analysis.

### [Martinenghi2012]

**Martinenghi, R., Rybalko, S., Jacquot, M., Chembo, Y.K., & Larger, L. (2012). Photonic nonlinear transient computing with multiple-delay wavelength dynamics. *Physical Review Letters*, 108(24), 244101.**

Extends the delay-feedback reservoir to wavelength-multiplexed implementations, effectively creating multiple parallel reservoirs from a single physical loop.

---

## Silicon Photonic Networks

### [Shen2017]

**Shen, Y. et al. (2017). Deep learning with coherent nanophotonic circuits. *Nature Photonics*, 11(7), 441–446.**

Uses silicon photonic mesh networks to implement trained optical neural networks (not RC: the weights are physically set). Demonstrates optical matrix-vector multiplication at speed-of-light latency. Conceptual background for photonic computing beyond reservoir computing.

### [Shastri2021]

**Shastri, B.J. et al. (2021). Photonics for artificial intelligence and neuromorphic computing. *Nature Photonics*, 15(2), 102–114.**

A comprehensive review of photonic neural networks including photonic RC, covering optoelectronic, integrated, and free-space implementations.

---

## Physical Background

### [Ikeda1979]

**Ikeda, K. (1979). Multiple-valued stationary state and its instability of the transmitted light by a ring cavity system. *Optics Communications*, 30(2), 257–261.**

The paper that introduced what became known as the Ikeda map/Ikeda chaos. The first description of optical bistability and chaos in a ring cavity with nonlinear refractive index.

### [Mackey1977]

**Mackey, M.C. & Glass, L. (1977). Oscillation and chaos in physiological control systems. *Science*, 197(4300), 287–289.**

The original Mackey-Glass oscillator paper. The DDE structure introduced here is structurally analogous to the optoelectronic delay reservoir.

### [Nakajima2021, Chapters 3–5]

**Nakajima, K. & Fischer, I. (eds.) (2021). *Reservoir Computing: Theory, Physical Implementations, and Applications*. Springer.**

Chapters 3–5 cover photonic and optoelectronic implementations in depth, with complete mathematical treatments. Essential supplementary reading for this chapter.
