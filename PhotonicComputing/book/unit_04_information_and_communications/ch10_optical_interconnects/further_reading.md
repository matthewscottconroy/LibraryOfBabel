# Chapter 10: Further Reading and References

## Foundational Papers

**Miller, D.A.B. (2009).** "Device requirements for optical interconnects to silicon chips." *Proceedings of the IEEE*, 97(7), 1166–1185.
*The canonical paper for optical interconnect energy analysis. Every person working on optical interconnects should read this. Miller derives the fundamental limits, establishes the ~1 fJ/bit target, and explains precisely why optical interconnects are advantageous for chips — and what they need to achieve to realize that advantage.*

**Miller, D.A.B. (2017).** "Attojoule optoelectronics for low-energy information processing and communications." *Journal of Lightwave Technology*, 35(3), 346–396.
*An update to the 2009 paper, incorporating a decade of progress. Reviews achieved energies for modulators (sub-100 aJ is possible), detectors, and lasers. The most comprehensive analysis of where the energy goes in optical links.*

**Al-Fares, M., Loukissas, A., & Vahdat, A. (2008).** "A scalable, commodity data center network architecture." *Proceedings of ACM SIGCOMM 2008*, 63–74.
*The fat-tree paper. Defines the topology now used in virtually every hyperscale data center. Essential background for understanding why the all-reduce bandwidth problem exists and what the network architecture constraints are.*

**Farrington, N., et al. (2010).** "Helios: A hybrid electrical/optical switch architecture for modular data centers." *ACM SIGCOMM 2010*, 339–350.
*The foundational paper for OCS in data center networks. Defines the hybrid architecture, the elephant-flow argument, and the bandwidth improvement achievable.*

**Sun, C., et al. (2015).** "Single-chip microprocessor that communicates directly using light." *Nature*, 528, 534–538.
*The Nature paper demonstrating a functional RISC-V processor with integrated silicon photonic network. The most compelling experimental demonstration of PNoC and the benchmark against which all subsequent integration proposals are measured.*

---

## Data Center Network Architecture

**Singh, A., et al. (2015).** "Jupiter rising: A decade of Clos topologies and centralized control in Google's datacenter network." *ACM SIGCOMM 2015*, 183–197.
*Google's authoritative account of their data center network evolution. Provides unprecedented public data on traffic patterns, failure modes, and the engineering decisions that drove hyperscale network design.*

**Vahdat, A. (2020, 2022).** OFC Conference Keynotes.
*Vahdat's OFC keynotes are the best public source of data on Google's optical network infrastructure. The 2022 keynote described the OCS deployment in detail.*

**Mellette, W.M., et al. (2017).** "RotorNet: A scalable, low-complexity, optical datacenter network." *ACM SIGCOMM 2017*, 267–280.
*Describes a rotating-mirror OCS architecture that uses periodic, pre-programmed topologies — eliminating the need for real-time scheduling. Simpler and more robust than demand-adaptive OCS; well-suited for AI training where communication patterns are known in advance.*

**Narayanan, D., et al. (2021).** "Efficient large-scale language model training on GPU clusters using Megatron-LM." *SC '21: Proceedings of the International Conference for High Performance Computing, Networking, Storage and Analysis*.
*Megatron-LM defines the tensor/pipeline/data parallelism framework used for LLM training. Quantifies the bandwidth requirements that motivate AI-cluster-specific OCS design.*

---

## Co-Packaged Optics

**Thraskias, C.A., et al. (2018).** "Survey of photonic and plasmonic interconnect technologies for intra-datacenter and high-performance computing communications." *IEEE Communications Surveys & Tutorials*, 20(4), 2758–2783.
*Comprehensive survey of optical interconnect technologies. Good technical depth on CPO, WDM, and on-chip photonic options.*

**Broadcom. (2023).** *Tomahawk 5 Product Brief*. Broadcom Inc.
*The Tomahawk 5 (51.2 Tbps) is the switch ASIC at the heart of 2024 hyperscale spine switches. Its CPO port configuration is described in Broadcom's public materials.*

**Intel. (2023).** *Intel Silicon Photonics Transceiver Technology*. Intel Corporation.
*Intel's public documentation on their silicon photonic transceivers, covering CWDM4, LR4, and the ODA (optical disaggregated architecture) co-packaging technology.*

---

## Photonic Network-on-Chip

**Vantrease, D., et al. (2008).** "Corona: System implications of emerging nanophotonic technology." *ACM ISCA 2008*, 153–164.
*The Corona paper, one of the first comprehensive photonic NoC architecture analyses with realistic energy models. Proposed using optical resonators for all-optical interconnect in a 1000-core processor. Highly cited; provided the architecture template that subsequent PNoC proposals refined.*

**Kurian, G., et al. (2010).** "ATAC: A 1000-core cache-coherent processor with online dynamic management and a scalable on-chip optical network." *ACM PACT 2010*.
*Photonic torus network for 1000-core processor. Identifies the ring-resonator thermal control problem as the dominant energy overhead.*

**Stojanović, V., et al. (2018).** "Monolithic silicon-photonic platforms in state-of-the-art CMOS SOI processes." *Optics Express*, 26(10), 13106–13121.
*MIT CMOS-compatible silicon photonics process. Describes the process and energy numbers for the components used in the 2015 Nature chip.*

**Beamer, S., et al. (2010).** "Re-architecting DRAM memory systems with monolithically integrated silicon photonics." *ACM ISCA 2010*.
*Argues that the dominant application for PNoC is processor-to-DRAM (not core-to-core). Sets the energy targets for the chiplet-to-HBM use case.*

---

## Laser Integration

**Fang, A.W., et al. (2006).** "Electrically pumped hybrid AlGaInAs-silicon evanescent laser." *Optics Express*, 14(20), 9203–9210.
*The foundational heterogeneous III-V/Si laser paper. Demonstrated CW lasing from a wafer-bonded InP structure on a silicon waveguide substrate.*

**Liu, A.Y., et al. (2016).** "High performance continuous wave 1.3 μm quantum dot lasers on silicon." *Applied Physics Letters*, 108, 221107.
*The QD laser on silicon breakthrough paper. Demonstrated > 100,000 hour MTTF for monolithically grown quantum dot lasers on silicon — the key result enabling monolithic photonic-electronic integration.*

**Bowers, J.E., et al. (2024).** "Recent advances in heterogeneous III-V on silicon photonic integrated circuits." *Journal of Lightwave Technology* (review article).
*Up-to-date review of heterogeneous integration from Bowers' group. Covers wafer bonding, QD epitaxy, yield, and the roadmap toward production.*

---

## Optical Switches

**Seok, T.J., et al. (2019).** "Large-scale broadband digital silicon photonic switches with vertical adiabatic couplers." *Optica*, 6(4), 389–395.
*Silicon photonic $32 \times 32$ switch matrix with 0.5 dB insertion loss per stage. One of the largest on-chip silicon photonic switch demonstrations.*

**Edinger, P., et al. (2021).** "Silicon photonic microelectromechanical phase shifters for fast and low-power switching." *Optics Letters*, 46(22), 5671–5674.
*MEMS phase shifter with 2.25 V actuation and < 1 μW static power; the record-low-energy silicon photonic switch element.*

**Calient Technologies. (2023).** *S-Series MEMS Optical Circuit Switch*. Calient product documentation.
*Data sheets for 160× and 320× 3D MEMS optical cross-connects. Performance specifications quoted in this chapter.*

---

## Review Articles

**Thraskias (2018)** (cited above) — comprehensive survey of interconnect technologies.

**Siew, S.Y., et al. (2021).** "Review of silicon photonics technology and platform development." *Journal of Lightwave Technology*, 39(13), 4374–4389.
*Broad review of silicon photonics platforms including CMOS integration; covers all major foundry processes.*

**Zhou, P., et al. (2022).** "A review of optical neural networks." *IEEE Access*, 10, 23938–23968.
*For readers who want to connect the interconnect material of this chapter to the photonic computing applications of Units V and VI.*
