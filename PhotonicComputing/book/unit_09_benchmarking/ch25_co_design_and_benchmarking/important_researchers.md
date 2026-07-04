# Chapter 25: Important Researchers

This chapter's intellectual lineage runs through two communities that rarely share a conference: the photonic-device physicists who build the optical cores, and the computer architects and circuit designers who insist on measuring whole systems. The names below are the ones whose work makes honest photonic benchmarking possible.

---

## David A. B. Miller

David Miller (Stanford University) is the field's conscience on energy. His 2017 review *"Attojoule Optoelectronics for Low-Energy Information Processing and Communications"* is the canonical analysis of where optoelectronic energy actually goes — showing that interface capacitance, not the optical process itself, dominates, and setting the attojoule-scale targets that any competitive photonic system must approach. He also developed the theory of self-configuring and self-aligning optical devices, which underlies the calibration and error-correction machinery that large programmable meshes require. Miller's work is why this chapter insists that the boundary — what is inside the watts — is where the argument is won or lost.

---

## Ryan Hamerly

Ryan Hamerly (NTT Research and MIT) produced the architecture-level energy analysis that frames the optimistic case for photonics honestly. His 2019 *Physical Review X* paper on large-scale optical neural networks based on photoelectric multiplication carried the standard-quantum-limit accounting through to a per-MAC photon budget, showing why optical energy per MAC *improves* with array size (the $2^{2b}E_{\text{ph}}/N$ scaling) — the single most important pro-photonics scaling argument in the field. With Englund and Bandyopadhyay he also quantified how component errors accumulate in MZI meshes and how to correct them.

---

## Bhavin J. Shastri, Paul R. Prucnal, and Alexander N. Tait

The Princeton-lineage neuromorphic-photonics group (now distributed across Queen's University and NIST) built much of the experimental and conceptual foundation this chapter benchmarks. **Prucnal** pioneered photonic spike processing and, with **Shastri**, wrote *Neuromorphic Photonics* (2017), the field's first textbook. **Tait** demonstrated silicon photonic weight banks and the broadcast-and-weight WDM architecture (*Scientific Reports*, 2017) that turns a laser comb into parallel neurons. Their joint reviews — notably *"Photonics for artificial intelligence and neuromorphic computing"* (*Nature Photonics*, 2021) — are the standard surveys of noise, precision, and training for analog photonic hardware.

---

## Mitchell A. Nahmias

Mitchell Nahmias (Princeton PhD; co-founder of Luminous Computing) is lead author of *"Photonic Multiply-Accumulate Operations for Neural Networks"* (*IEEE JSTQE*, 2019/2020), the paper that gave the photonic MAC a formal definition and worked out its energy scaling and the $O(N)$-conversion / $O(N^2)$-computation asymmetry that this chapter's amortization arguments rest on. His move from academia to a photonic-computing startup mirrors the field's central question — whether the device-level physics survives translation into a shipping system.

---

## Dirk Englund and Marin Soljačić

Dirk Englund and Marin Soljačić (both MIT) led the demonstrations that launched the modern era of coherent photonic computing. With Yichen Shen and Nicholas Harris they published *"Deep learning with coherent nanophotonic circuits"* (*Nature Photonics*, 2017) — the 56-MZI programmable mesh whose *honest* reporting of hardware accuracy (76.7%) against the digital baseline (91.7%) this chapter holds up as a model of good practice. Englund's group later produced hardware error correction for programmable photonics and the *delocalized* photonic deep-learning scheme (Sludds et al., *Science*, 2022), streaming weights over fiber to receiver-only edge clients. Both are also startup founders (Soljačić: Lightelligence; the Shen/Harris line: Lightmatter), placing them on both sides of the research-to-product boundary.

---

## Nicholas C. Harris

Nicholas Harris (co-founder and CEO of Lightmatter; MIT PhD in Englund's group) bridges the coherent-mesh demonstrations and the commercial photonic accelerators that this chapter's auditor's checklist is designed to interrogate. Company disclosures of photonic tensor cores with on-package SRAM and digital control — the Hot-Chips-class systems that actually draw a system boundary — are the most benchmarkable evidence the field has produced, and the reason the chapter treats vendor claims as a distinct, and instructive, category.

---

## Bahram Jalali

Bahram Jalali (UCLA) is the originator of the photonic time-stretch technique and time-stretch analog-to-digital conversion (Coppinger, Bhushan & Jalali, 1999). His work is the cleanest example of the chapter's co-design thesis in the input domain: rather than compete with the electronic ADC, photonics *rescues* it, dilating a wideband transient in time so that a modest converter can capture signals otherwise beyond the jitter ceiling. He also demonstrated the silicon Raman laser, connecting this work to the device foundations of earlier units.

---

## Boris Murmann

Boris Murmann (formerly Stanford, now University of Hawaiʻi) maintains the long-running ADC performance survey that defines the empirical energy envelope — the 10–100 fJ per conversion-step figure at multi-GS/s rates — that every analog accelerator inherits at its output. Because the ADC is usually the single most expensive element in the conversion chain, Murmann's dataset is, indirectly, one of the most important constraints on photonic-accelerator system efficiency, and the source of the hard numbers behind Section 25.1.1.

---

## Mohammed A. Al-Qadasi, Sudip Shekhar, and Lukas Chrostowski

The University of British Columbia group produced *"Scaling up silicon photonic-based accelerators: challenges and opportunities"* (*APL Photonics*, 2022), the independent full-system model that itemizes DAC, ADC, TIA, laser, and thermal-tuning power and reaches the same conclusion as this chapter's worked example: conversion and laser overheads dominate, and array size and precision set the crossover with electronics. **Chrostowski** is also widely known for silicon-photonics design education and open foundry access (SiEPIC), and **Shekhar** for high-speed link and circuit design — exactly the electronic-side expertise co-design demands.

---

## Norman P. Jouppi

Norman Jouppi (Google) led the Tensor Processing Unit and authored *"In-datacenter performance analysis of a tensor processing unit"* (ISCA, 2017), the exemplary study of *delivered* versus *peak* accelerator performance via roofline analysis — the discipline this chapter demands of photonic claims. His group's TPU v4 work (ISCA, 2023) also deployed optical circuit switching in production AI infrastructure, making it the control case for "photonics already ships, as interconnect."

---

## Albert Reuther

Albert Reuther (MIT Lincoln Laboratory) leads the annual *"Survey and Benchmarking of Machine Learning Accelerators"* (IEEE HPEC), whose peak-performance-versus-power scatter plots are the shared coordinate system on which any accelerator — electronic or photonic — can be located. When photonic projections are re-plotted at the wall-plug boundary onto Reuther's axes, most land on or near the electronic frontier rather than an order of magnitude above it, which is the empirical heart of this chapter's honest summary.

---

## Mark Horowitz

Mark Horowitz (Stanford University) supplied the numbers that anchor every energy budget in this chapter. His 2014 ISSCC paper *"Computing's Energy Problem (and what we can do about it)"* tabulated the energy of digital arithmetic and, crucially, memory access — the picojoule SRAM and nanojoule DRAM figures that determine whether a tiled or streamed accelerator, photonic or otherwise, is dominated by moving data rather than computing on it.

---

## Harish Bhaskaran and Wolfram Pernice

Harish Bhaskaran (Oxford) and Wolfram Pernice (Heidelberg/Münster) built the non-volatile in-memory photonic devices that, in this chapter's accounting, delete the two worst line items of the energy budget — static weight-holding power and weight-fetch traffic. Their integrated photonic tensor core with phase-change weights (Feldmann et al., *Nature*, 2021) is the existence proof that a photonic matrix can be held at zero static power directly in the optical path, turning the difference between 2.3 and 13 TOPS/W in the worked example.
