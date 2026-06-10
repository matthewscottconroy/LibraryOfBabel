# Chapter 10: Important Researchers

## David A.B. Miller (Stanford University)

David Miller is the central theorist of optical interconnects. His 2009 paper "Device requirements for optical interconnects to silicon chips" established the energy target (~1 fJ/bit fundamental minimum), the bandwidth density argument, and the scaling analysis that frames the entire field's engineering goals. The "Miller limit" is named after this work.

Miller's research group at Stanford (the Ginzton Laboratory) has demonstrated electroabsorption modulators using the quantum-confined Stark effect in GaAs/AlGaAs quantum wells, achieving ~10 fJ/bit modulation energy, and has developed the theoretical framework for understanding why optics has advantages over electronics specifically for interconnects (not for logic). His 1990 paper with Ozguz on "Optical interconnects to electronic chips" and subsequent work spanning three decades defined the field.

Miller was also an early leader in demonstrating optically interconnected VLSI circuits (the "smart pixel" concept), showing that free-space optical interconnects could provide massive parallelism in 2D optical data transfer at a time when fiber-optic interconnects were limited to serial connections.

---

## Amin Vahdat (Google)

Amin Vahdat is VP of Network Engineering at Google and the architect of Google's data center network — including the Jupiter network (deployed since ~2012, described publicly in the 2015 SIGCOMM paper) and the OCS deployment described in public keynotes beginning in 2022. Vahdat is responsible for the most sophisticated large-scale deployment of optical data center networking in the world.

At UCSD before joining Google, Vahdat's group produced the Helios paper (2010) — the first demonstration of hybrid electrical/optical switching in a data center context, and the foundational architecture that all subsequent OCS data center designs follow. He was also a co-author of the VL2 paper (2009) that proposed randomized routing as a practical approach to achieving fat-tree bandwidth in real data centers.

Vahdat's public talks (OFC keynotes, SIGCOMM) are the most reliable sources of data on hyperscale optical network deployment at scales that no academic research group can access.

---

## Nick McKeown (Stanford University / Barefoot Networks)

Nick McKeown is a co-inventor of OpenFlow (software-defined networking) and the P4 network programming language. His work on programmable network data planes is foundational for the OCS scheduling systems described in this chapter: the ability to classify flows and trigger OCS reconfigurations in hardware requires exactly the kind of programmable packet processing that P4 enables.

McKeown co-founded Barefoot Networks (acquired by Intel in 2019), which produced the Tofino series of programmable switch ASICs — now the dominant platform for in-network computing and OCS scheduling in hyperscale environments.

---

## Vladimir Stojanović (UC Berkeley) and Rajeev Ram (MIT)

Stojanović and Ram led the effort that produced the 2015 *Nature* paper "Single-chip microprocessor that communicates directly using light" — the most significant experimental milestone in photonic network-on-chip research. The paper demonstrated a functional RISC-V processor with integrated silicon photonic waveguides, ring modulators, and germanium photodetectors, fabricated in a commercial CMOS process (GlobalFoundries 45 nm SOI). The chip communicated at 2.5 Gbps/channel using on-chip photonics.

This work demonstrated that PNoC is physically achievable in a manufacturable CMOS process — the key proof-of-concept that has driven subsequent research. Stojanović's group (now at UC Berkeley) continues to develop CMOS-photonics integration; Ram's group at MIT focuses on the photonic device physics.

---

## John Bowers (UC Santa Barbara)

John Bowers is the pioneer of heterogeneous III-V/Si laser integration. His group at UCSB demonstrated the first electrically pumped hybrid silicon laser in 2006 (with HP Labs), and subsequently developed the wafer-bonding technique used by Intel in their silicon photonic transceivers. Bowers' group also produced the first high-quality quantum dot laser on silicon (2016), the breakthrough that made monolithic photonic-electronic integration credible.

Bowers has also made foundational contributions to high-speed photodetectors, semiconductor amplifiers, and microwave photonics. His lab has been the dominant force in heterogeneous photonic integration for more than 15 years.

---

## Keren Bergman (Columbia University)

Keren Bergman's group at Columbia has been one of the most sustained contributors to photonic network-on-chip architecture. Her research focuses on the full system stack: from device-level components through network topology and protocol design to application-level performance. The group produced early demonstrations of multi-chip photonic interconnects and comprehensive energy models for PNoC architectures.

Bergman was also a co-PI on the DARPA-funded POEM (Photonics on Electronics and Memory) program, one of the largest photonic integration research efforts in US academic history, which demonstrated BEOL photonics (SiN waveguides above CMOS transistors) as a path to full monolithic integration.

---

## Mohammad Alizadeh (MIT) / Hari Balakrishnan (MIT)

Mohammad Alizadeh and Hari Balakrishnan are leading researchers in data center networking. Alizadeh's DCTCP work (2010) — which demonstrated that ECN-based congestion control can dramatically reduce queue buildup in data center networks — is foundational for understanding why data center traffic behaves differently from WAN traffic, and why OCS scheduling is feasible. Balakrishnan's group at MIT has produced fundamental work on network congestion control (TCP, QUIC) and AI/ML network optimization that informs the scheduling algorithms used in OCS systems.
