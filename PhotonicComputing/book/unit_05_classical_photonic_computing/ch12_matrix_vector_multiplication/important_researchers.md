# Chapter 12: Important Researchers

## David A. B. Miller (Stanford University)

Miller appears in this book three times — optical logic (Chapter 11), interconnects (Chapter 10), and here — because he has repeatedly identified what light is and is not good for. For programmable linear optics his 2013 "self-configuring universal linear optical component" showed that an interferometer mesh can align *itself* using only local detectors and progressive feedback, needing neither precise fabrication nor global computation. This result underlies modern self-configuring meshes and much of the practical hope for scaling coherent processors beyond the calibration wall. His broader analyses of the fundamental limits of optical computing — energetics, communication, and the proper division of labor between photons and electrons — remain the field's intellectual compass.

## Dirk Englund (MIT)

Englund's group built the programmable nanophotonic processor on which the Shen et al. 2017 experiment ran, and has since produced much of the rigorous engineering that the 2017 paper made necessary: hardware error correction for meshes (Bandyopadhyay et al. 2021), self-configuration of rectangular interferometers (with Hamerly), large-scale ONN architectures based on photoelectric multiplication, and single-chip photonic deep neural networks. His laboratory spans classical and quantum photonic information processing, and the dual-use character of programmable meshes — the same chip simulating quantum transport and classifying vowels — is largely his group's demonstration.

## Marin Soljačić (MIT)

A theorist of photonic crystals and electromagnetic phenomena, Soljačić co-led (with Englund) the deep-learning-with-nanophotonics program at MIT and co-authored the 2017 paper. His group's earlier work on strong light confinement and novel electromagnetic modes provided several of the enabling ideas, and his students and postdocs — Shen, Harris, Hamerly, Bandyopadhyay among them — populate both the academic and commercial leadership of the field.

## Yichen Shen (Lightelligence; formerly MIT)

Shen was the first author of "Deep learning with coherent nanophotonic circuits" (2017), the experiment reconstructed in Section 12.3.2, and subsequently founded Lightelligence to commercialize photonic matrix processing. Under his leadership the company demonstrated PACE (2021), an integrated photonic engine executing recurrent Ising-machine workloads that exploit the picosecond latency of optical matrix multiplication. His trajectory — from benchmark-setting academic demonstration to venture-scale engineering — is representative of the field's rapid 2017–2022 industrialization.

## Nicholas Harris (Lightmatter; formerly MIT)

Harris designed the 56-MZI programmable nanophotonic processor as a graduate student (using it for quantum transport simulation as well as the 2017 deep learning experiment) and founded Lightmatter in 2017. The company's Mars device (Hot Chips 2020) introduced 3D-stacked electronic-photonic integration for AI inference, and its Passage product line redirected the same photonic platform toward chip-to-chip interconnect — a pivot that has proven commercially decisive and diagnostically interesting for the whole field.

## Paul Prucnal (Princeton University)

Prucnal leads the school of photonic computing that this chapter calls incoherent: broadcast-and-weight networking, microring weight banks, modulator neurons, and photonic spike processing. His group (with Tait, Shastri, Nahmias, de Lima, Huang) built the first silicon neuromorphic photonic networks, established the weight-precision records via feedback-controlled microrings, and produced the fiber-nonlinearity-compensation demonstration that remains the most convincing application of photonic neural processing to a problem electronics cannot reach. He co-authored the standard text *Neuromorphic Photonics* (2017) and the field-defining *Nature Photonics* review (2021).

## Alexander Tait (Queen's University; formerly Princeton, NIST)

Tait, as Prucnal's student, invented the broadcast-and-weight protocol (2014) and the microring weight bank, and carried out the first silicon implementations of both. His careful papers on multi-channel weight control turned the ring bank from a concept into a calibrated instrument, and his continued work bridges neuromorphic photonics and superconducting optoelectronics. Much of Section 12.4 is built directly on results from his thesis and subsequent publications.

## Ryan Hamerly (MIT / NTT Research)

Hamerly's analyses define the quantitative boundaries of the field: the photoelectric-multiplication ONN architecture showing sub-attojoule optical MACs at $N \sim 10^6$ (PRX 2019), the error-scaling theory of MZI meshes, and (with Bandyopadhyay and Englund) the self-configuration algorithms that recover mesh fidelity without external calibration. If you want to know whether a proposed photonic accelerator's energy claim survives scrutiny, his papers are where you check.
