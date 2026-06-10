# Section 10.1: The Interconnect Bottleneck

Computing performance is not limited by logic gates alone. Moving a bit from one location to another — from DRAM to processor, from one GPU to another, from one server to the next rack — consumes energy and takes time. As processors have grown faster, the interconnects between them have grown relatively slower: the bandwidth-per-compute-operation ratio has declined for decades.

This section develops the physics and economics of the interconnect bottleneck:

**Subsection 10.1.1 — Power and Bandwidth Scaling**: The energy per bit in copper interconnects as a function of length, and why optical interconnects break this scaling.

**Subsection 10.1.2 — Co-Packaged Optics**: The near-term solution to the chip I/O bottleneck — integrating optical transceivers directly on the package with the compute die.
