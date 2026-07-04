# Chapter 22: Important Researchers

---

## Charles H. Bennett (1943–) and Gilles Brassard (1955–)

Bennett (IBM) and Brassard (Université de Montréal) invented quantum cryptography: the BB84 protocol (Section 22.1.1), the first proposal to turn the uncertainty principle into a key-distribution primitive, and the first working QKD demonstration over 32 cm of air in 1989. The same collaboration produced quantum teleportation (1993, the payload protocol of Section 22.2.3) and entanglement purification (BBPSSW, Section 22.2.3). Nearly every idea in this chapter descends from their work; BB84 remains the most-deployed quantum protocol in the world.

---

## Artur Ekert (1961–)

Ekert (Oxford; Singapore CQT) reinvented QKD from entanglement in 1991 (E91, Section 22.1.1): distribute Bell pairs and certify secrecy by violating a CHSH inequality, so that security rests on the *physics* of entanglement rather than on the sender's hardware. The idea is the seed of device-independent QKD (Stage 3 of the roadmap) and reframed eavesdropping as the degradation of entanglement monogamy — the conceptual backbone of the entanglement-based protocols Micius later ran from orbit. He went on to found and direct the Centre for Quantum Technologies in Singapore.

---

## Nicolas Gisin (1952–)

Gisin (Geneva) turned QKD from tabletop demonstration into deployed technology: long-distance fibre QKD, field tests across Lake Geneva, and the co-founding of ID Quantique (2001), the first QKD company. His group's breadth spans the chapter — the atomic-frequency-comb memory (Section 22.2.2), the standard repeater-architecture review, and the classic *Reviews of Modern Physics* survey of quantum cryptography — making him the field's central connective figure between physics and engineering. His foundational work on Bell nonlocality and quantum randomness also underwrites the device-independent security of Section 22.1.1.

---

## Hoi-Kwong Lo

Lo (Toronto; NUS) closed two of the largest gaps between BB84's proof and its practice. With Ma and Chen he made decoy-state QKD rigorous (2005, Section 22.1.2), restoring linear key-rate scaling against the photon-number-splitting attack; and with Curty and Qi he invented measurement-device-independent QKD (2012, Section 22.1.2), structurally immunizing QKD against all detector side channels. Both are now standard in deployed systems and underpin the Stage-2 untrusted-node networks of the roadmap. He also established, with Chau, the impossibility of unconditionally secure quantum bit commitment — correcting a widely held misconception about what quantum cryptography can deliver.

---

## Jian-Wei Pan (1970–)

Pan (USTC) built the experimental frontier of long-distance quantum communication. His group performed the first entanglement swapping (1998, Section 22.2.3) and, with the Micius satellite (Section 22.1.3), delivered the modern canon: satellite-to-ground QKD, a 1,203 km entanglement Bell test, ground-to-satellite teleportation, a 7,600 km intercontinental relay, and the 4,600 km integrated space-to-ground network. He also drives China's twin-field and fibre-backbone records — the most sustained quantum-communication program in the world.

---

## Andrew Shields and Zhiliang Yuan

Shields and Yuan (Toshiba, Cambridge) made QKD fast and industrial: gigahertz-clocked systems, self-differencing single-photon detectors, Mbit/s metropolitan key rates, and — most consequentially — the *twin-field QKD* protocol (Lucamarini, Yuan, Dynes & Shields, 2018, Section 22.1.2) that broke the PLOB rate–distance limit with $\sqrt{\eta}$ scaling. Their work defines the high-rate, long-distance edge of deployable fibre QKD and much of its commercial engineering.

---

## Stephanie Wehner (1977–)

Wehner (QuTech, Delft) supplied the quantum internet its conceptual architecture: the six-stage capability roadmap (Wehner, Elkouss & Hanson, 2018, Section 22.3.1) around which this section is built, and the layered protocol stack — including the first link-layer protocol for quantum networks (Section 22.3.2). Co-founder of the Quantum Internet Alliance, she is the leading architect of how entanglement becomes a *networked* resource rather than a laboratory curiosity.

---

## Ronald Hanson (1976–)

Hanson (QuTech, Delft) built the matter-qubit quantum network. His NV-centre experiments achieved heralded entanglement between nodes 1.3 km apart and the first loophole-free Bell test (2015, Section 22.2), then the first multi-node quantum network (2021) and qubit teleportation between non-neighbouring nodes across an entanglement swap (2022, Section 22.3.2) — the first working realization of the roadmap's Stage-4 quantum-memory network.

---

## Stefano Pirandola

Pirandola (York) established the fundamental limits the rest of the chapter measures itself against: the PLOB bound (Pirandola, Laurenza, Ottaviani & Banchi, 2017, Section 22.1.2) on the secret key of any repeaterless channel, and the end-to-end capacity theory of quantum networks (Section 22.2.3). A leading contributor to continuous-variable QKD and lead author of the authoritative *Advances in Quantum Cryptography* review, he supplies the field its rate–distance benchmarks. His secret-key-capacity results fixed the two-way-assisted capacity of the pure-loss channel exactly — the number every repeaterless system is judged against.
