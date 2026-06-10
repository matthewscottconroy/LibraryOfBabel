# Chapter 10: Optical Interconnects

## The Bottleneck That Won't Move

Every generation of computer architecture has had a "memory wall": the observation that processor computation speed has grown faster than the speed at which data can be moved to and from memory. For the first 40 years of the transistor era, engineers found ways around this wall — caches, branch prediction, out-of-order execution, register files — but the fundamental problem never disappeared. It evolved.

In the AI era, the memory wall has metamorphosed into the **interconnect bottleneck**. A modern GPU cluster running large language model training has $10^{15}$ floating-point operations per second of compute throughput, but the bandwidth connecting GPUs to each other and to memory is orders of magnitude smaller than what would be needed to keep every compute unit busy. The result is that most compute hardware in AI systems is idle most of the time, waiting for data.

The numbers make this concrete. An NVIDIA H100 GPU has ~67 TB/s of on-chip bandwidth (within the GPU die) and ~3.35 TB/s of HBM memory bandwidth (to the DRAM stacked on the package). The GPU interconnect (NVLink 4.0) provides 900 GB/s between GPUs in a server. The inter-server interconnect (100 GbE) provides 12.5 GB/s between racks. At each level of the memory/networking hierarchy, the bandwidth drops by 3–10×.

This cascade of bandwidth cliffs means that as neural networks grow (GPT-3: 175B parameters; GPT-4: reportedly ~1.8T parameters), an ever-larger fraction of training time is spent moving weights around rather than multiplying them.

Optical interconnects — carrying information as photons rather than electrons — offer a potential solution: higher bandwidth per fiber, lower energy per bit, and freedom from the capacitive loading that limits electrical signaling at high frequencies. Whether this potential can be realized at each level of the hierarchy — from chip to chip, to board, to rack, to data center — is the question this chapter addresses.

## Chapter Structure

**Section 10.1 — The Interconnect Bottleneck**: Power scaling and bandwidth scaling in electrical interconnects; why copper is a harder and harder material to build with; co-packaged optics as the immediate response.

**Section 10.2 — Data Center Optical Networks**: The leaf-spine network topology, WDM optical links, and the emerging use of optical circuit switching for AI cluster interconnects.

**Section 10.3 — Photonic Network-on-Chip**: The long-range vision of optical connections within and between chips. The physics and engineering challenges — thermal management, laser integration, detector sensitivity — that stand between vision and practice.

---

## References

[1] Miller, D.A.B. (2017). "Attojoule optoelectronics for low-energy information processing and communications." *Journal of Lightwave Technology*, 35(3), 346–396. [The key reference for energy limits of optical interconnects; every relevant number is derived here.]

[2] Sun, C., Wade, M.T., Lee, Y., Orcutt, J.S., Alloatti, L., Georgas, M.S., ... & Ram, R.J. (2015). "Single-chip microprocessor that communicates directly using light." *Nature*, 528(7583), 534–538. [MIT/UC Berkeley demonstration of electronic-photonic co-integration at the chip level.]
