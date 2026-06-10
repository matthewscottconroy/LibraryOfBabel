# Chapter 17: Photonic Reservoir Computing

---

> *"Light is the fastest messenger. The question is whether it can also think."*

---

## Chapter Introduction

Of all physical substrates for reservoir computing, photonics has received the most experimental attention and achieved the most dramatic performance results. The reason is straightforward: light can be processed at frequencies exceeding $10^{14}$ Hz, with interconnect speeds limited by the speed of light itself rather than by the transit time of electrons. A photonic reservoir computer that processes information at the bandwidth of its optical carrier could in principle operate at $10^{12}$–$10^{14}$ operations per second — speeds that no electronic system can approach.

The practical reality is more modest but still impressive. Optoelectronic systems (which mix optical propagation with electronic nonlinearities) operate at $10^8$–$10^9$ symbols per second, already $10^2$–$10^3$ times faster than well-optimized GPU-based ESN simulations. All-optical systems using passive silicon photonic chips have demonstrated inference at hundreds of gigahertz on small tasks. The thermodynamic advantage is equally striking: a photonic chip can perform the same computation as a GPU while consuming power at nanowatt levels, compared to hundreds of watts for the GPU.

This chapter examines two landmark implementations. The first, by Appeltant et al. [Appeltant2011], is the optoelectronic delay-feedback reservoir: a single semiconductor laser with external feedback, implementing the time-multiplexed virtual node architecture of Chapter 16. The second, by Vandoorne et al. [Vandoorne2014], is an all-optical passive chip reservoir: 16 coupled micro-ring resonators on a silicon photonic chip, with no active driving required after fabrication.

Understanding these implementations requires understanding the physical systems they exploit — the Mackey-Glass oscillator, the Ikeda ring, and coupled-mode theory for micro-ring resonators — and we develop these in sufficient detail to appreciate both the power and the limitations of each approach.

---

## What You Will Learn

- The optoelectronic delay-feedback reservoir: architecture, the Ikeda ring nonlinearity, Mackey-Glass oscillator hardware
- Appeltant et al. 2011: the first experimental demonstration, architecture details, NARMA-10 results
- Vandoorne et al. 2014: on-chip silicon photonic reservoir, micro-ring resonators, coupled mode equations
- Speed advantages of photonic over electronic reservoirs: theoretical limits and experimental results
- The tradeoffs: energy, programmability, and the challenge of training physical readouts

---

## Prerequisites

This chapter requires Chapter 16 (physical RC principles and the virtual node construction). No specialist knowledge of photonics is required; all physical concepts are introduced from scratch.
