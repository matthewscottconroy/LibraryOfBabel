# 26.1.2 Lightelligence: The PACE Demonstration and the Optical Network-on-Chip

## Origins

Lightelligence was founded in 2017 in Boston by Yichen Shen — first author of the same 2017 *Nature Photonics* paper from Marin Soljačić's MIT group that seeded Lightmatter — together with collaborators from the MIT photonics community. The two companies are, in effect, sibling forks of one experiment, which is what makes their comparison so informative: similar founding physics, similar founding year, and convergent conclusions reached along different routes. Lightelligence maintained a dual footprint from early on, with research and engineering in the Boston area and substantial operations and market focus in Asia.

## PACE: A Real Photonic Computing System, Honestly Scoped

Lightelligence's signature technical achievement was **PACE** (Photonic Arithmetic Computing Engine), unveiled in 2021: an integrated photonic-electronic system combining a silicon photonic matrix engine of roughly 64×64 scale — thousands of photonic devices, among the largest coherent photonic circuits assembled for computing at the time — with a co-packaged electronic die performing drive, readout, and recurrence at GHz-class clock rates.

Two features of PACE deserve attention from a student of this book:

1. **The workload was chosen to fit the physics.** PACE was demonstrated not on general neural-network inference but on recurrent Ising-type optimization heuristics (the problem class of Section 27.1) — workloads where a fixed matrix is applied repeatedly at very high rate, so the optical engine's strengths (single-pass matrix multiplication, latency of nanoseconds) dominate and its weaknesses (reprogramming cost, limited precision) are minimized. The company reported latency advantages of orders of magnitude relative to a contemporary GPU *on that specific recurrent workload*. This is exactly the "algorithm-hardware fit" reasoning of Chapter 25: the honest form of a photonic speedup claim is workload-specific.

2. **It was a system demonstration, not a component demonstration.** PACE closed the loop — optics, converters, electronics, memory — and therefore its numbers, whatever their marketing framing, belong to the meaningful category of end-to-end measurements.

## The Same Pivot, by Another Route

After PACE, Lightelligence's public roadmap shifted — as Lightmatter's did — toward interconnect. Its **optical network-on-chip (oNOC)** product line, announced in 2023 under the name Hummingbird, places a photonic die beneath electronic compute dies to provide low-latency, all-to-all broadcast interconnect between cores — a topology (recall the broadcast-and-weight discussions of Unit VI) that is expensive in copper and natural in waveguides. The company subsequently emphasized optical interconnect products (including photonic links for memory and accelerator clusters) in the co-packaged optics ecosystem.

That both siblings of the 2017 paper independently migrated from selling optical *arithmetic* to selling optical *bandwidth* is about as close to a controlled economic experiment as this industry provides. The physics of the MZI mesh did not change between 2017 and 2024. What changed was the market's revealed preference: the AI build-out made bandwidth between digital chips the scarcest resource, and photonics' most defensible advantage — communication — found its buyer first.

## Reported Scale

Lightelligence's disclosed venture funding totaled over $200M within its first several years — smaller than Lightmatter's later AI-cycle rounds, but sufficient to carry two generations of full-system silicon. As with every company in this chapter, treat specific figures as a dated snapshot and the trajectory as the durable fact.

## How to Read Lightelligence

PACE remains one of the most instructive artifacts in photonic computing: a peer-visible, full-stack demonstration whose claimed advantage was explicitly conditioned on workload structure. When you encounter any photonic accelerator claim, the PACE template gives you the right questions: What is the workload? Is the matrix static or reprogrammed per step? What precision did the task require? What was on the other side of the comparison, and was it similarly optimized? (Chapter 25 provides the full checklist; Section 27.1 revisits the Ising workload itself.)
