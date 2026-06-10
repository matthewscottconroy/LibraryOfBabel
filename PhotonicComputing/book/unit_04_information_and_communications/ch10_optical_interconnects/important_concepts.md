# Chapter 10: Important Concepts

## The Interconnect Bottleneck

**Electrical wire energy scales with distance, voltage, and frequency.** The charging energy $E = \frac{1}{2}CV^2$ per bit grows linearly with wire length (due to capacitance) and quadratically with voltage. At 10 nm CMOS, a 10 mm on-chip electrical wire consumes ~1 pJ/bit including repeaters. Reducing $V_{DD}$ helps (energy ~ $V^2$) but worsens noise margin; optics escapes this trade-off.

**Skin effect degrades electrical bandwidth at high frequency.** Conductor resistance scales as $R \propto \sqrt{f}$, increasing attenuation at high frequency. By 100 GHz, copper traces have ~40 dB/m loss, requiring repeaters or equalization at significant energy cost. Optical fiber loss is frequency-independent across the entire modulation bandwidth.

**Miller's limit: ~1 fJ/bit fundamental minimum for optical interconnects.** Derived from the energy to flip a semiconductor junction by one photon (~$\hbar\omega \sim$ attojoules), multiplied by practical receiver sensitivities and component efficiencies. Current systems are 500–1500 fJ/bit; the gap is mostly due to laser efficiency and ring thermal control.

**Co-packaged optics (CPO) reduces energy by shortening the chip-to-transceiver electrical path.** Moving the transceiver from a faceplate connector (100 mm traces, 1 pJ/bit SerDes) to the switch package (1 mm traces, 100 fJ/bit) cuts the dominant electrical component by 10×. Current CPO: 500–1500 fJ/bit total. Target: ~100 fJ/bit.

---

## Data Center Networks

**Fat-tree / Clos topology provides non-blocking bandwidth for $k^3/4$ servers using $k$-port switches.** Every server can communicate at full bandwidth simultaneously; there are no bottleneck switches. The bisection bandwidth equals $(k/2)^2 \times B_{\text{link}}$.

**Leaf-spine is the practical implementation of fat-tree.** Two-layer variant; leaf switches connect servers, spine switches connect leaves. Oversubscription = (server ports × server bandwidth) / (uplinks × uplink bandwidth).

**AI training all-reduce consumes bandwidth proportional to parameter count.** For a 70B parameter model, each all-reduce step moves ~280 GB across the network. At 400 Gbps per node, this requires 224 Gbps per node to not be bandwidth-limited — approaching the capacity of the fastest pluggable transceivers.

**Optical circuit switching (OCS) is competitive with electronic packet switching for elephant flows.** Elephant flows (large, long-duration transfers) make up > 50% of data center bytes but < 0.1% of flows. OCS can route these flows without OEO conversion at each hop, saving ~10× power vs. electronic switching for the same capacity. MEMS switches (5–25 ms reconfiguration) are sufficient for AI training flows (thousands of seconds long).

**OCS cannot buffer photons.** Optical fiber delay lines require ~200 m of fiber to buffer 10 × 100 ns packets — impractical. OCS works only for traffic that is predictable enough to schedule circuits before data arrives. AI training traffic satisfies this; general-purpose web traffic does not.

**Google demonstrated OCS at production scale in 2022.** Deployed free-space MEMS OCS in spine layer; achieved 91.7% demand satisfaction vs. 91.4% for best electronic alternative, while consuming ~1/10 the switching power. First large-scale production validation of OCS for data center networking.

---

## Photonic Network-on-Chip

**The PNoC motivation: WDM bandwidth density is ~100× higher than electrical.** A 500 nm × 220 nm Si waveguide carrying 64 WDM × 100 Gbps achieves ~2 Tbps/μm bandwidth density; a differential electrical pair achieves ~20 Gbps/μm. The photonic density advantage is real and robust.

**The PNoC energy bottleneck: ring thermal stabilization.** Silicon rings drift at 69 pm/K; a $Q = 10^4$ ring requires ±0.22 K stability. Active stabilization consumes 0.5–2 mW per ring — dominating all other optical energy at 100 Gbps. This is the central practical challenge for ring-resonator-based PNoC.

**Three on-chip laser integration approaches: external fiber coupling, flip-chip bonding, heterogeneous wafer bonding.** Flip-chip and wafer bonding achieve 70–85% coupling efficiency; QD lasers grown on Si are maturing toward >100,000 hr MTTF. Laser wall-plug efficiency (10–30%) determines the per-bit laser energy: at 10% WPE and 1 mW optical power, $E_{\text{laser}} = 100$ fJ/bit at 100 Gbps.

**Photonic-CMOS integration has a fundamental process incompatibility.** Silicon photonics needs 220 nm SOI with thick BOX; advanced CMOS needs 3–8 nm transistor bodies with multi-gate structures. Solutions: BEOL photonics (SiN waveguides above CMOS transistors), 3D hybrid bonding (separate chips bonded face-to-face), or purpose-built CMOS-photonics processes (GF 45SPCLO). No approach is fully satisfying yet.

**The most promising near-term PNoC application is chiplet-to-chiplet, not core-to-core.** Replacing the electrical HBM memory interface (~10 mm) with optics is the application where physics strongly favors optics, integration complexity is manageable (3D bonding), and the application pull from AI accelerators is strongest. On-chip core-to-core optical routing faces severe thermal and integration challenges that make it a longer-term target.

**Post-fabrication ring trimming is essential for PNoC yield.** Lithographic variation causes ±2 nm resonance scatter. Laser annealing or ion implantation trims to ±0.1 nm. With $N = 128$ rings, per-ring yield must be > 99.9% to achieve acceptable chip yield; redundancy (spare rings) can relax this to 99%.
