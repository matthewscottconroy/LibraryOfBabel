# 26.2.3 QuiX, Quandela, ORCA, and the Component Specialists

Not every quantum photonics company bets the firm on a full fault-tolerant machine. A European cluster has built viable businesses on the *components and subsystems* of Unit VII — and in doing so has created the supply chain that any future photonic quantum computer, including the giants', will draw on.

## QuiX Quantum: The Processor Specialist

QuiX Quantum (Enschede, Netherlands, founded 2019) builds reconfigurable linear-optical processors — Clements-style meshes (Chapter 11) — on the ultra-low-loss Si₃N₄ (TriPleX) platform of the Twente/LioniX ecosystem. Its processors progressed from 12 modes [Taballione et al., 2021] to a 20-mode universal interferometer with insertion losses low enough for multi-photon experiments [Taballione et al., *Quantum*, 2023], sold as laboratory instruments to quantum optics groups and as the photonic core for boson-sampling-class machines, including contracts to deliver photonic quantum computers to national laboratories (notably the German Aerospace Center, DLR). The business model is instructive: the programmable interferometer is the one subsystem *every* discrete-variable architecture needs, so QuiX profits from the field's growth without wagering on a single architecture.

## Quandela: The Source Specialist That Became a Full Stack

Quandela (Paris region, founded 2017 by Pascale Senellart, Valérian Giesz, and Niccolo Somaschi out of CNRS/C2N) commercialized the semiconductor quantum-dot-in-micropillar single-photon source — the deterministic, cavity-enhanced source technology of Chapter 18, whose near-optimal brightness and indistinguishability were established in the founding team's research [Somaschi et al., *Nature Photonics*, 2016]. From that anchor component the company built upward: fiber-coupled source products, the open-source photonic computing framework **Perceval**, and cloud-accessible few-photon quantum processors (the Ascella line and successors), with the full platform described in the peer-reviewed literature [Maring et al., *Nature Photonics*, 2024]. Quandela demonstrates the "component wedge" strategy: own the hardest single component (the source), then integrate.

## ORCA Computing: Memory, Multiplexing, and Pragmatism

ORCA Computing (London, founded 2019, with roots in the Oxford quantum memory work of Ian Walmsley's and Josh Nunn's groups) attacks the probabilistic-source problem of Chapter 22 with *time*: fiber-loop architectures and quantum memories that synchronize and multiplex heralded photons, converting probabilistic sources into quasi-deterministic streams. Its rack-mounted PT-series systems — deliberately modest, fiber-based, room-temperature-friendly machines — were sold to early government and HPC customers (including the UK Ministry of Defence and the Polish supercomputing center PSNC) as testbeds for hybrid quantum-classical machine learning experiments. ORCA's positioning is the pragmatic inverse of PsiQuantum's: ship small, imperfect, useful-for-research hardware now, and grow toward fault tolerance through multiplexing.

## The Rest of the Supply Chain

The specialist pattern extends across the ecosystem, and a student should know the map:

| Niche | Representative companies |
|---|---|
| Single-photon sources | Quandela, Sparrow Quantum (Copenhagen; photonic-crystal waveguide QD sources from the Lodahl group), Aegiq |
| Programmable processors | QuiX Quantum, iPronics (classical programmable meshes usable in quantum labs) |
| Single-photon detectors | Single Quantum, ID Quantique, Photon Spot; waveguide-integrated SNSPDs (Pixel Photonics) |
| Squeezed light / CV | Xanadu (in-house), academic spin-offs |
| Full-stack DV machines | PsiQuantum, Photonic Inc. (spin-photon hybrid, Canada), TuringQ (China) |

Two structural observations close the section. First, Europe dominates the component layer — a direct legacy of its academic strength in quantum optics (Section 26.3.2) and of coordinated public funding (the EU Quantum Flagship, national programs, PhotonDelta in the Netherlands). Second, the component specialists are collectively *architecture-neutral infrastructure*: whichever bet from Sections 26.2.1–26.2.2 wins, it will buy — or copy — what these companies learned to make.
