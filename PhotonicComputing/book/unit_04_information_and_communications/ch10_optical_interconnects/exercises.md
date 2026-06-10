# Chapter 10: Exercises

## Mathematical Exercises

**10.1** (Interconnect energy scaling) A 10 mm on-chip electrical wire uses 3 nm CMOS technology with $V_{DD} = 0.65$ V and wire capacitance 0.15 fF/μm with repeaters every 0.8 mm consuming 15 fJ/bit each.

(a) Calculate the total wire capacitance and intrinsic charging energy $E = \frac{1}{2}CV^2$.

(b) Add the repeater energy and calculate the total energy per bit.

(c) An optical alternative on the same link uses a ring modulator (5 fJ/bit drive), Ge photodetector + TIA (150 fJ/bit), and a shared laser source feeding 32 WDM channels with 10% wall-plug efficiency and 0.5 mW optical power per channel. Calculate the total optical energy per bit.

(d) At what laser wall-plug efficiency does the optical link break even with the electrical link?

**10.2** (Fat-tree bandwidth) For a $k = 32$ fat-tree network:

(a) Calculate the total number of servers, edge switches, aggregation switches, and core switches.

(b) If all links are 400 Gbps, calculate the bisection bandwidth.

(c) If the oversubscription ratio at the edge is 2:1, what is the effective per-server bandwidth?

(d) An AI training run with 8,192 servers and a 70B parameter model needs 280 GB all-reduced every 10 seconds. What fraction of the bisection bandwidth is consumed by the all-reduce operation?

**10.3** (Ring resonator thermal analysis) A silicon ring resonator with radius $R = 5$ μm, $Q = 8000$, operating at 1550 nm, must maintain resonance to within $\pm 20\%$ of its linewidth.

(a) Calculate the resonance linewidth in nm and the tolerable temperature variation.

(b) The chip temperature varies by 15°C under workload changes. Calculate the heater power needed to compensate, given that the heater resistance is 400 Ω and is embedded in the SiO₂ cladding 500 nm above the ring with thermal resistance 500 K/W.

(c) Express the thermal stabilization energy as fJ/bit for a 100 Gbps link.

(d) Compare this to the intrinsic ring modulator energy of 8 fJ/bit. What fraction of the total energy is thermal control?

**10.4** (Optical circuit switch scheduling) An OCS fabric connects 128 servers, each with 400 Gbps uplinks. The fabric has MEMS switches with 20 ms reconfiguration time, and the demand matrix is updated every 100 ms.

(a) During a reconfiguration, no traffic can flow. What fraction of bandwidth is lost to reconfiguration overhead?

(b) For AI training all-reduce with 1 TB flows (time to transmit at 400 Gbps: 20 s), what fraction of the flow duration is the reconfiguration overhead?

(c) An alternative LCoS switch reconfigures in 8 ms. How does this change the overhead?

(d) If the OCS handles only flows > $F_{\text{min}}$ bytes, and flows smaller than $F_{\text{min}}$ go to the electrical fabric, what is $F_{\text{min}}$ for the reconfiguration overhead to be < 1% of flow duration?

**10.5** (WDM PNoC bandwidth density) Compare the bandwidth density of a photonic WDM bus and an electrical bus.

(a) A silicon photonic waveguide carries 64 WDM channels at 100 Gbps each. The waveguide is 500 nm × 220 nm; practical pitch (center-to-center) is 3 μm. Calculate the bandwidth density in Tbps/μm.

(b) A differential electrical bus at 32 Gbps per pair with 2 μm pitch. Calculate the electrical bandwidth density.

(c) For a 1 cm-wide chip I/O block, how many Tbps can be routed electrically vs. optically?

(d) How does this comparison change if the optical waveguide pitch can be reduced to 1.5 μm using deep-trench isolation?

**10.6** (Photonic butterfly network) Design a photonic butterfly network for $N = 32$ compute nodes.

(a) How many stages are needed? How many switch elements per stage? What is the total switch count?

(b) If each switch element is a $2 \times 2$ MZI switch (50 fJ/bit dynamic, 5 mW static power), and average utilization is 60%, calculate the total static power and the dynamic energy per bit.

(c) For a ring-resonator-based switch alternative (0 static power, 10 fJ/bit), how do the totals compare?

(d) At what bit rate per link does the ring-resonator switch become more energy-efficient than the MZI switch (considering static power amortized per bit at the given utilization)?

---

## Conceptual Exercises

**10.7** Miller's minimum energy argument (Section 10.1.1) gives ~1 fJ/bit as the theoretical minimum for optical interconnects. Current systems achieve 500–1500 fJ/bit. Identify the three largest contributors to the gap between theory and practice, explain the physics of each, and describe what would be required to close each gap.

**10.8** The co-packaged optics (CPO) architecture places optical transceivers in the same package as the switch ASIC. Explain, using the RC energy formula, why this reduces energy consumption compared to pluggable transceivers in the faceplate. What physical properties of the chip-to-transceiver electrical path improve, and by how much?

**10.9** Consider the claim: "Optical circuit switching is superior to electronic packet switching for AI training workloads." Evaluate this claim. What properties of AI training traffic make OCS favorable? What would need to change in the traffic pattern (or in the OCS technology) to make OCS unfavorable? Give specific quantitative thresholds.

**10.10** The broadcast-and-select PNoC architecture requires that every receiver monitors every wavelength on the bus. For a 64-node network, this means 64 drop filters per node (to monitor all senders). Explain why this is not the actual implementation — in a BAS network, what determines how many filters each receiver needs, and why?

**10.11** Silicon rings have $dn/dT = 1.87 \times 10^{-4}$ K$^{-1}$, while Si₃N₄ rings have $dn/dT = 2.5 \times 10^{-5}$ K$^{-1}$ (7.5× smaller). Explain why this does not make Si₃N₄ strictly superior to Si for PNoC ring-resonator applications. What does Si₃N₄ lose in the comparison?

**10.12** The leaf-spine topology achieves 1:1 oversubscription (full bisection bandwidth) when the number of uplinks per leaf switch equals the number of server ports divided by the oversubscription ratio. Explain why a large AI training cluster might deliberately choose a 2:1 oversubscribed leaf-spine network rather than a 1:1 network, despite the bandwidth reduction.

---

## Lab / Experimental Exercises

**10.13** (Simulation: fat-tree vs. torus) Using a network simulation tool (e.g., ns-3 or the simpler netbench):

(a) Build a 64-node fat-tree and 64-node 8×8 torus network, both with 100 Gbps links.
(b) Simulate all-reduce traffic (each node sends to all others simultaneously) and measure total completion time and link utilization.
(c) Simulate uniform random traffic and compare.
(d) Which topology is better for AI training? For general-purpose computing? Explain the result in terms of the topologies' bandwidth and path-length properties.

**10.14** (Energy model: PNoC) Implement the energy model for a BAS photonic NoC in Python:

(a) Write a function `pnoc_energy(N_nodes, B_per_channel, N_wavelengths, ring_thermal_mW, laser_wpe, laser_power_mW, link_loss_dB, det_energy_fJ)` that returns total energy per bit.
(b) Sweep ring_thermal_mW from 0.01 to 5 mW and plot the total energy vs. ring thermal power.
(c) Find the ring thermal stabilization power threshold below which PNoC beats the electrical alternative (640 fJ/bit).
(d) Is this threshold achievable with current MEMS tuning technology?

**10.15** (Ring resonator characterization) If you have access to a photonic test setup (or a simulation tool like Lumerical MODE/FDTD or MIT Meep):

(a) Simulate or measure the transmission spectrum of a silicon ring resonator at T = 25°C, 35°C, 45°C, and 55°C.
(b) Measure the thermal shift rate in pm/K and compare to the theoretical value of 69 pm/K.
(c) Implement a simple PID controller simulation that adjusts heater power to maintain resonance at the T=25°C position as the background temperature varies sinusoidally between 25°C and 55°C at 1 Hz.
(d) Measure the average heater power consumed by the PID controller and express it as fJ/bit for 100 Gbps operation.

**10.16** (OCS scheduling algorithm) Implement a simple OCS scheduler in Python:

(a) Given a demand matrix $D$ (where $D_{ij}$ = requested bandwidth from node $i$ to node $j$), compute the minimum number of OCS configurations (matchings) needed to route all demand, using the Hungarian algorithm or random rounding.
(b) Test on an all-reduce demand matrix for 16 nodes.
(c) Test on a random demand matrix.
(d) Compute the average number of reconfigurations per second needed to track demand changes in an AI training scenario (assume the demand matrix changes every 10 seconds).
