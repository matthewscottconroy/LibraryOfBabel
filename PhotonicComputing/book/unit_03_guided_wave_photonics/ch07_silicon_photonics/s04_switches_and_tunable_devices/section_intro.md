# Section 7.4: Switches and Tunable Devices

The previous section addressed modulators: devices that encode *data* into the optical domain at high speed. This section addresses a related but distinct class of device: switches and tunable elements that control the routing and configuration of optical signals.

The distinction matters. A modulator operates continuously, encoding a stream of bits or analog values at gigahertz rates. A switch operates episodically: it sets the path of an optical signal from one configuration to another, then holds that configuration for some period ranging from microseconds (in a rapidly reconfigurable network) to weeks (in a set-and-forget optical fiber network cross-connect). The specifications for switches are therefore quite different from those for modulators: switching speed matters, but holding-state power consumption and long-term stability often matter more.

In photonic computing, the distinction blurs. A programmable MZI mesh — the hardware substrate for photonic matrix multiplication — is neither a pure modulator nor a pure switch. Its phase elements are *programmed* with matrix weights at some reconfiguration rate (perhaps every millisecond in an adaptive system), and then *hold* those settings while the computation runs. The physics of how the phase elements work determines whether the system is energy-efficient in the hold state, and what limits its reconfiguration speed.

This section covers the three main physical mechanisms for optical switching and tuning in silicon photonic systems:

**Subsection 7.4.1 — Thermo-optic switching**: Using the silicon thermo-optic coefficient ($dn/dT = 1.87 \times 10^{-4}$ K⁻¹) to produce phase shifts via local heating. Slow (microsecond to millisecond response), power-hungry in the hold state, but extremely reliable, CMOS-compatible, and capable of large phase shifts with simple electrode designs. The workhorse of reconfigurable silicon photonics.

**Subsection 7.4.2 — MEMS optical switches**: Using microelectromechanical systems to physically reconfigure waveguide structures. Extremely low hold-state power (electrostatic hold, essentially zero current), large optical switching contrast (mechanical coupling gap changes from touching to separated), and compatibility with silicon foundry processes. Limited to microsecond-to-millisecond switching speeds by mechanical resonance, but ideal for weight banks that update infrequently.

**Subsection 7.4.3 — Phase-change material switches**: Using materials like Ge₂Sb₂Te₅ (GST) or Ge₂Sb₂Se₄Te₁ (GSST) that can be switched between amorphous and crystalline phases with dramatically different optical properties. Non-volatile: once switched, the state is maintained with zero static power. The potential to store optical weights without any holding power makes PCM devices uniquely attractive for in-memory photonic computing.

The section concludes with a comparison of the three mechanisms against the requirements of different photonic computing architectures: fast reconfiguration, energy-efficient static operation, and precision analog weight storage.

---

## The Power Tradeoff in Reconfigurable Photonics

Any reconfigurable photonic system must address a fundamental tension: the same mechanisms that make phase elements responsive also make them susceptible to noise and power dissipation.

For a phase element maintaining a phase shift $\Delta\phi_0$ over time $T$:
- **Electro-optic** (carrier plasma, Pockels): requires sustained voltage, but essentially zero current in the ideal case (capacitive load). Static power = voltage leakage through junction or dielectric. Can be very low (<1 μW) but typically 1–100 μW per element in practice.
- **Thermo-optic**: requires sustained heating power. Static power for $\pi$-phase shift: $P_\pi \approx 10$–40 mW per element in silicon. This is the Achilles heel of large thermo-optic meshes.
- **MEMS**: requires sustained electrostatic force (voltage across a gap, essentially zero current for ideal dielectric). Static power essentially zero, but position noise from thermal fluctuations can cause phase drift.
- **Phase-change**: zero static power once switched. But write energy per phase change event is 1–100 pJ, limiting how often weights can be updated.

For a photonic neural network with $N^2$ weights:
- At 10 mW per thermo-optic element: $N = 8$ → 640 mW static power; $N = 16$ → 2.56 W.
- At 0 W per MEMS element: $N = 64$ → 0 W static; write energy 10 pJ per weight update.
- At 0 W per PCM element: $N = 64$ → 0 W static; write energy 10 pJ per weight update.

This scaling analysis is why MEMS and PCM approaches are attracting increasing attention as matrix sizes grow.
