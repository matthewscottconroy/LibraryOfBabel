# Chapter 13: Important Researchers

## Paul Prucnal (Princeton University)

Prucnal founded the school of neuromorphic photonics that supplies this chapter's incoherent architectures and its most-cited activation mechanism: the modulator neuron, whose electro-optic transfer function doubles as a nonlinearity and whose O/E/O stage restores signal levels between layers. With his students and collaborators he built the first silicon photonic spike processors and weight banks, and he co-authored both the book *Neuromorphic Photonics* (2017) and the field-defining *Nature Photonics* survey (Shastri et al. 2021). His work is the practical answer to Section 13.2's activation problem — nonlinearity purchased in the electrical domain — and the origin of the reservoir- and spike-adjacent architectures of Section 13.4.

## Bhavin Shastri (Queen's University)

Shastri, formerly of Princeton, is lead author of the standard survey "Photonics for artificial intelligence and neuromorphic computing" (*Nature Photonics*, 2021) that orients the whole chapter, and co-author of the careful photonic multiply-accumulate energy accounting (Nahmias et al. 2020) behind Section 13.5's fJ/MAC figures. His research on photonic neurons, weight-bank calibration, and the systems-level energy case for optical AI connects the device physics of Chapter 12 to the network-level claims examined here.

## Gordon Wetzstein (Stanford University)

Wetzstein works at the intersection of optics and machine learning, treating the optical system itself as a trainable front-end for inference. He is lead author of the cross-community perspective "Inference in artificial intelligence with deep optics and photonics" (*Nature*, 2020), which frames the inference-not-training thesis of Section 13.1 and unifies integrated, free-space, and computational-imaging approaches. His emphasis — that optics is most valuable where it does computation *before* detection — informs the chapter's reading frame of letting the physics be itself.

## Dirk Englund (MIT)

Englund's laboratory built the programmable nanophotonic processor behind the field-launching Shen et al. (2017) experiment and has since produced much of the hardware that training and scaling arguments depend on: the photoelectric-multiplication architecture (Hamerly et al. 2019) that is Section 13.5's natural fit for activation-by-activation attention, and single-chip photonic deep networks with on-chip training (Bandyopadhyay et al. 2024). His group's dual-use meshes — quantum transport one day, vowel classification the next — exemplify the generality of programmable linear optics.

## Shanhui Fan (Stanford University)

Fan is the theorist behind in-situ backpropagation. The 2018 result (Hughes, Minkov, Shi, Fan) that the loss gradient with respect to every phase shifter can be measured physically — as an interference term between a forward field and an adjoint field injected backward, with reciprocity supplying the transpose — is the intellectual centerpiece of Section 13.3.2. His broader program in nanophotonics, adjoint-based inverse design, and non-reciprocal photonics provides the electromagnetic foundations on which optical training rests.

## Tyler Hughes (Stanford University; photonic design-automation industry)

Hughes, as Fan's student, was first author of the in-situ backpropagation paper (2018) that showed backpropagation can run in the optics itself, and contributed the differentiable-photonics tooling that made adjoint-based training and inverse design practical. He subsequently carried that expertise into the photonic simulation and design-automation industry. His work marks the transition of optical training from a theoretical possibility to an engineering discipline.

## Peter McMahon (Cornell University)

McMahon, formerly a Stanford postdoc on coherent Ising machines, leads the Cornell program on physical neural networks and optical transformers. He is senior author of "Deep physical neural networks trained with backpropagation" (Wright et al. 2022), which introduced physics-aware training (Section 13.3.4); of "Optical transformers" (Anderson et al. 2024), the energy-scaling analysis central to Section 13.5.2; and author of the skeptical review "The physics of optical computing" (2023) that serves as the chapter's capstone. His stance — quantify the advantage honestly, and let the physics do the work it is good at — pervades this chapter.

## Logan Wright (NTT Research; formerly Cornell)

Wright, first author of the physics-aware-training paper (*Nature*, 2022), showed that backpropagation can train arbitrary physical systems — optical, mechanical, electronic — by pairing a hardware forward pass with a differentiable digital model, closing the sim-to-real gap without a full in-situ apparatus. His work at Cornell and NTT Research's Physics and Informatics Laboratories generalizes the training methods of Section 13.3 beyond photonic meshes to any controllable nonlinear medium.

## Daniel Brunner (FEMTO-ST Institute, CNRS, Besançon)

Brunner is a central figure in photonic reservoir computing. As first author of the gigabyte-per-second transient-state processing demonstration (2013) and co-author of the field's standard review (Van der Sande, Brunner, Soriano 2017) and of the founding single-node paper (Appeltant et al. 2011), he established both the speed and the conceptual framework of delay-based photonic reservoirs treated in Section 13.4. His more recent work extends reservoir ideas to large spatial networks and on-chip learning.

## Laurent Larger (FEMTO-ST Institute, Université de Franche-Comté, Besançon)

Larger pioneered high-speed optoelectronic delay-dynamics systems and used them to build the reservoir that classifies spoken digits at roughly one million words per second (*Physical Review X*, 2017) — the headline throughput result of Section 13.4.2. His decades of work on nonlinear delay oscillators supplied the physical substrate (a single nonlinear node plus a long feedback loop) that time-delay reservoir computing exploits.

## Peter Bienstman (Ghent University / imec)

Bienstman leads the integrated-photonics wing of reservoir computing. His group's "Experimental demonstration of reservoir computing on a silicon photonics chip" (Vandoorne et al. 2014) replaced the delay line with a passive network of silicon waveguides, showing that a fabricated, energy-passive photonic circuit can serve as the reservoir — the integrated counterpoint of Section 13.4.3 and a template for on-chip neuromorphic photonics.

## Ingo Fischer (IFISC, CSIC-UIB, Palma de Mallorca)

Fischer is an authority on the nonlinear dynamics of delay-coupled and semiconductor-laser systems, the physics that makes photonic reservoirs work. He co-authored the founding single-node time-delay paper (Appeltant et al. 2011) and the Gb/s demonstration (Brunner et al. 2013), and his broader work on delay dynamics, consistency, and transient computation underpins the echo-state behavior and virtual-node picture of Section 13.4.2.
