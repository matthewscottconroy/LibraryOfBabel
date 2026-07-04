# 28.2.1 Is There a Photonic Transistor?

## The Oldest Open Problem

Optical computing's founding dream, dating to the 1960s, was to replace the electronic transistor with an optical one: a device in which one beam of light controls another, so that logic gates — and eventually whole processors — could be built from photons that never slow to electronic speeds. The dream has never been realized to specification, and the reason is instructive enough that David Miller devoted a widely cited editorial to stating precisely what "to specification" means [Miller, *Nature Photonics*, 2010]. His answer is the cleanest test the field has, and every proposed "optical transistor" should be held against it.

## Miller's Criteria

Miller's insight is that "a device that switches light with light" is far too weak a requirement — a saturable absorber does that much. A logic *technology* must support arbitrarily large circuits, and that imposes a specific list. Paraphrasing [Miller, *Nature Photonics*, 2010], a usable optical logic device must provide:

1. **Gain and fan-out** — one output must be able to drive at least two inputs of identical downstream devices, so the signal survives branching;
2. **Cascadability** — the output must be the same *kind* of physical quantity as the input (same wavelength, spatial mode, polarization, and format), so devices chain without an intervening translator;
3. **Logic-level restoration** — a slightly degraded input must yield a cleaner output, so noise and error do not accumulate down a long chain; this requires a nonlinear, saturating transfer function;
4. **Input-output isolation** — the output must not feed back to corrupt the input; the device must have a definite direction, as a transistor's gate is isolated from its drain;
5. **A single controlling quantity that sets the state** and, above all, a **low switching energy** competitive with the aJ–fJ that a well-designed electronic gate already achieves [Miller, *Journal of Lightwave Technology*, 2017].

The verdict Miller draws, and that the intervening years have upheld, is that *most optical switching schemes fail one or more of these criteria outright* — typically fan-out and restoration, and almost always energy. An electronic transistor satisfies all of them cheaply and at scale; that, and not any deficiency of imagination, is why it remains the substrate of computing.

## Why It Is Hard: Photons Do Not Interact

The difficulty is fundamental, not incidental. Photons do not interact in vacuum: Maxwell's equations in free space are linear, so two beams pass through each other unchanged. To make one beam gate another you must route both through matter and exploit an *optical nonlinearity*, in which the material's response depends on optical intensity. The available nonlinearities are all, in their own way, weak:

- **$\chi^{(2)}$** (second-order, in non-centrosymmetric crystals such as lithium niobate) enables three-wave mixing but requires phase matching and sizeable interaction lengths;
- **$\chi^{(3)}$** (the Kerr and four-wave-mixing response present in all materials, including silicon) is weaker still, and in silicon at telecom wavelengths is accompanied by two-photon absorption that dumps energy and generates free carriers;
- **resonant and excitonic nonlinearities** (in quantum wells, quantum dots, 2D materials, and polaritons) are far stronger per photon but narrowband, often slow, and temperature- or saturation-limited.

Because the intrinsic effect is small, every candidate device buys strength by one of three expensive means: **high optical power** (which wrecks the energy budget), **a high-$Q$ resonant cavity** (which recycles photons through the material many times, trading away bandwidth and adding thermal and stability problems), or **a long interaction length** (which costs area and loss). The photonic transistor problem is, at bottom, that the one knob you must turn — nonlinearity — is bolted down.

## The Candidates, Honestly

Following the discipline of this book — DEMONSTRATED physics, OPEN engineering — the serious candidates each demonstrate real switching while failing at least one of Miller's criteria.

**Photonic-crystal nanocavities.** The strongest classical result is all-optical switching at sub-femtojoule switching energies in a high-$Q$, wavelength-scale silicon photonic-crystal cavity [Nozaki et al., *Nature Photonics*, 2010]. This is genuinely impressive physics that approaches electronic switching energies — but it inherits the cavity's narrow bandwidth and finite recovery time, and, critically, a passive Kerr or free-carrier switch supplies no gain and does not restore logic levels. It switches; it is not a transistor.

**Cavity-QED and slow-light (EIT) single-photon switches.** Using electromagnetically induced transparency, or a single atom or quantum dot strongly coupled to a high-finesse cavity, a small number of control photons — in the best cases approaching one — can gate the transmission of a signal beam. As quantum optics this is a landmark, and it underpins the photon-photon gates of linear-optical quantum computing (Unit VII). As *classical logic* it fails on the practical axes: the switching rates, fan-out, and operating conditions (cryogenic, ultra-high vacuum, or warm atomic-vapor cells) are nowhere near what a room-temperature processor running at gigahertz rates requires, and few-photon control does not yet arrive with gain.

**Polariton and 2D-material switches.** Exciton-polariton condensates in microcavities, and heterostructures built from transition-metal dichalcogenides (Chapter 27), offer strong and fast nonlinearities and have demonstrated switching and even transistor-like gain — but under conditions (optical cavities, low temperature, small-signal operation) that do not yet satisfy cascadability and manufacturability together.

**Phase-change all-optical logic and memory.** A different strategy abandons the requirement that the device be volatile. Chalcogenide phase-change materials (PCMs) integrated onto waveguides switch between amorphous and crystalline states with light, implementing all-optical arithmetic, non-volatile memory, and even synaptic and spiking primitives [Feldmann et al., *Nature*, 2019; Wuttig et al., *Nature Photonics*, 2017]. This is one of the most useful "optical logic" lineages in the literature — but the state change is a *material reconfiguration*: slow, energetically costly per write, and endurance-limited (Section 28.2.3). PCM is therefore better read as programmable optics and memory than as a fast logic transistor.

The through-line is stark: **no demonstrated device meets cascadability, gain and restoration, low energy, and manufacturability all at once.** Each candidate is honest physics missing at least one column of Miller's table.

## What the Field Did Instead

The absence of a photonic transistor is not a footnote; it is the load-bearing fact that shaped the modern field. Because optics cannot cheaply make decisions, the field executed two strategic retreats that this book has traced throughout.

1. **Linear optics for what optics does well.** Rather than build nonlinear logic, use light for the operations it performs natively and for free — the weighted sums and matrix-vector products of a passive interferometer mesh (Chapter 11), and, above all, communication (Chapter 26) — and hand the nonlinear, decision-making steps back to electronics. Every leading photonic AI accelerator is this hybrid: optical multiply, electronic nonlinearity and control.
2. **Analog, not digital.** Give up on restoring logic levels at every gate — the very thing a transistor is for — and instead compute in a single analog shot, accepting finite precision (Chapter 25) as the price of not needing gain and restoration.

This is also why the transistor question reappears, in modern dress, as the *nonlinear-computing* frontier of Chapter 27: the live research question is no longer "can we build an optical NAND gate" but "which optical nonlinearity is cascadable, low-energy, and manufacturable enough to serve as a neural-network *activation function*" — a weaker and far more useful target. Whether any nonlinearity ever meets the full transistor specification remains, sixty years on, genuinely open. The prudent reading (Chapter 25's system view) is that it need not: a technology can be transformative without replacing the transistor, and photonics' near-term value lies precisely in the roles that *do not* require one.
