# 26.2.2 Xanadu: Squeezed Light, Gaussian Boson Sampling, and the CV Route

## The Bet

Xanadu (Toronto, founded 2016 by Christian Weedbrook) is the standard-bearer for the continuous-variable (CV) route of Chapter 21: encode quantum information in the quadratures of squeezed light, manipulate it with Gaussian operations (beamsplitters, phase shifts, homodyne detection), and obtain fault tolerance eventually via bosonic codes — Gottesman-Kitaev-Preskill (GKP) states — as detailed in the company's architectural blueprint [Bourassa et al., *Quantum*, 2021]. The CV bet trades PsiQuantum's problem (making and not losing single photons) for a different one: making sufficiently squeezed, sufficiently pure resource states, with GKP state generation as the acknowledged hardest step.

## The Demonstrations

Xanadu's experimental record is anchored by three peer-reviewed milestones:

- **X8 (2021)**: an eight-mode programmable nanophotonic chip generating squeezed states and executing Gaussian boson sampling (GBS) and small algorithms via cloud access [Arrazola et al., *Nature*, 2021] — a genuine full-stack integration exercise, from chip to cloud API.
- **Borealis (2022)**: a time-multiplexed GBS machine with 216 squeezed modes and programmable three-loop delay interferometry, claiming quantum computational advantage on the GBS sampling task [Madsen et al., *Nature*, 2022]. As with all sampling-advantage claims (recall Jiuzhang, Chapter 22), the claim is a moving target: subsequent classical-simulation advances, exploiting experimental noise and photon loss, narrowed the gap for instances of this class. The honest summary circa 2025: GBS advantage claims survive in weakened, instance-dependent form, and the episode is a case study in why "advantage" must always be indexed to the best *current* classical algorithm.
- **Aurora (2025)**: a modular, rack-scale prototype — dozens of photonic chips, tens of squeezers, fiber-networked into a small number of logical-photonic units — demonstrating end-to-end the *architecture* (sources, multiplexing, real-time feedforward, networking between modules) at small scale [Aghaee Rad et al., *Nature*, 2025]. Aurora's significance is architectural completeness, not computational power: it is the CV analog of PsiQuantum's manufacturability paper — evidence that the company's path is an engineering pipeline rather than a sequence of hero experiments.

## PennyLane: The Software Flank

Xanadu's most widely used product is not hardware. **PennyLane**, its open-source quantum machine learning and differentiable quantum programming framework (with the earlier Strawberry Fields and The Walrus libraries for CV and GBS work), became one of the most popular quantum software toolchains of the era, hardware-agnostic by design. The strategic logic mirrors classical platform plays: the framework builds community, talent flow, and mindshare independent of hardware milestones — and hedges the architectural bet, since PennyLane runs happily on competitors' qubits.

## Capital

Xanadu raised on the order of \$250M in venture funding through its 2021 Series C (at a valuation crossing \$1B), placing it second to PsiQuantum among pure photonic quantum ventures in the period covered here.

## How to Read Xanadu

Score the CV route on its own critical path, which differs from the DV route's: (1) squeezing level and purity per source, (2) loss between squeezer and detector (loss degrades squeezing exactly as Chapter 21 quantified), and (3) above all, progress toward high-fidelity, high-rate GKP state generation — the step the 2021 blueprint itself identifies as decisive. GBS applications marketed for the interim (molecular vibronic spectra, graph similarity, point processes) should be evaluated with the Chapter 25 discipline: against the best classical method for the same task, at matched cost.
