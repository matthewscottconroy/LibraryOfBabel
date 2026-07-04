# 26.2.1 PsiQuantum: Fault Tolerance or Nothing, at Foundry Scale

## The Bet

PsiQuantum (Palo Alto, founded 2016 by Jeremy O'Brien, Terry Rudolph, Pete Shadbolt, and Mark Thompson — the Bristol and Imperial linear-optical quantum computing lineage of Unit VII) is defined by two refusals. It refuses to build NISQ-era machines: the company's stated position is that only a fault-tolerant, million-physical-qubit-class machine is commercially interesting, so intermediate devices are a distraction. And it refuses to build in a laboratory: every component must come off a commercial 300 mm semiconductor line or it does not count.

The architecture is the fusion-based quantum computation (FBQC) scheme developed in Chapter 22: heralded single-photon sources feed the generation of small entangled *resource states*; fusion measurements (Bell-type measurements implemented with linear optics and single-photon detectors) stitch these into the fault-tolerant fabric; photon loss — the dominant error — is handled as erasure, which enjoys far higher correction thresholds (of order 10% per photon in FBQC constructions [Bartolucci et al., *Nature Communications*, 2023]) than general gate errors (~1%). The machine is therefore, physically, a photon factory: sources, switches, delays, and superconducting nanowire single-photon detectors (SNSPDs), replicated by the million, plus cryoplant for the detectors (~4 K, far warmer and cheaper per qubit than millikelvin dilution refrigeration).

## Manufacturing as the Strategy

PsiQuantum's distinctive move was to partner with GlobalFoundries (announced 2021) to fabricate its photonic and electronic chips in a standard 300 mm production fab, arguing that the transition from thousands to millions of components is a semiconductor-manufacturing problem, not a physics problem, and must be solved in a real fab or not at all. In 2025 the company published a peer-reviewed account of this platform [PsiQuantum team (Alexander, K., et al.), "A manufacturable platform for photonic quantum computing," *Nature*, 2025; arXiv:2404.17570], reporting foundry-fabricated source and interferometer performance, integrated SNSPDs, and — significantly for the loss budget — high-speed optical switching based on barium titanate (BaTiO₃) electro-optics rather than lossy or slow thermo-optic tuning.

Read that paper the way Unit VII taught you to: find the per-component loss figures and multiply them along the path a photon must survive. As of the mid-2020s the published component losses remained above the ultimate FBQC budget, with a gap of roughly 2–3× to close across the whole stack simultaneously — better sources, lower-loss switches and fiber attach, higher-efficiency detectors. That multiplicative, whole-stack character is why the company's manufacturing-first strategy is coherent: a hero-device improvement in one laboratory component does not compound; a process improvement in a fab does.

## Capital and Commitments

PsiQuantum's financing is the largest in photonic quantum computing: roughly $665M in private capital by 2021, followed in 2024 by two government partnerships — with Australia (federal and Queensland commitments of order AU$940M to site a utility-scale machine in Brisbane) and with the State of Illinois (a Chicago-area quantum campus) — each attached to late-2020s delivery ambitions. Treat the announced timelines as what they are: milestones on a plan whose critical path (the loss budget) is publicly quantifiable, and against which the company can be honestly scored.

## How to Read PsiQuantum

The company is a single, falsifiable proposition: *that photon loss can be beaten by manufacturing scale before capital runs out*. Its published numbers give the student everything needed to track the proposition — which is more than can be said for most quantum ventures, and is itself a consequence of the all-or-nothing bet: with no NISQ product to sell, the only credible interim deliverables are components, processes, and papers.
