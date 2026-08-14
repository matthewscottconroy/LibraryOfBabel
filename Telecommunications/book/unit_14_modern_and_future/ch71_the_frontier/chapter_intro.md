# Chapter 71 — The Frontier

Predicting the future of networking has a poor record, and it is worth looking at the
failures before making any predictions of our own, because the pattern in them is
instructive.

ATM was going to carry everything, from the desktop to the backbone; Ethernet ate it
from below. IPv6 was going to be deployed by 2005; it is at roughly 45% adoption in
2026. The videophone was imminent in 1964, 1974, 1994 and 2004, and became ubiquitous
in 2020 for reasons that had nothing to do with the technology. WAP was going to be the
mobile Internet. Everything was going to be peer-to-peer, and then everything was going
to be centralised in the cloud, and the truth has been messier than either.

The pattern in the failures: technologists consistently overestimate how fast
infrastructure changes and underestimate how much economics and installed base
determine outcomes. A technically superior system that requires everyone to change
simultaneously loses to an inferior one that can be adopted incrementally. This is the
lesson of Chapter 16's Ethernet, Chapter 22's OSI, Chapter 28's IPv6, and Chapter 68's
OpenFlow, and it is the lens this chapter applies.

So the sections below are organised by **confidence**, and say which is which.

## Near certainties

Optics keep improving, and the margin keeps shrinking. §71.2 covers coherent
detection, higher-order modulation, and the approach to the Shannon limit (Chapter 4
§4.4) on installed fibre. 800G is deployed; 1.6T is standardised. The interesting
question is not whether capacity rises but what happens as the gap to the theoretical
limit closes — because the answers are spatial (more fibres, multi-core fibre) rather
than spectral, and that changes the economics from "upgrade the electronics" back to
"install more glass," which is Chapter 10's expensive term.

Wireless keeps getting faster in ways that require you to stand closer. §71.1
covers 6G research, and the honest assessment is that the physics of Chapter 42
constrains it tightly: higher frequencies mean more path loss and more absorption, so
higher rates mean smaller cells, which means more sites, which is a civil engineering
and planning problem rather than a radio one. Terahertz communication research is real
and its practical range is measured in metres.

More things get networked, with the security consequences of Chapter 47's IoT
discussion, and the segmentation response of Chapter 60.

## Genuinely uncertain

**Quantum networking**, §71.3, and this section is written carefully because the
subject attracts more nonsense than any other in the field.

**Quantum key distribution** is real physics and works. It distributes encryption keys
with security guaranteed by quantum mechanics rather than by computational hardness —
an eavesdropper necessarily disturbs the quantum states and is detected. It has been
demonstrated over hundreds of kilometres of fibre and via satellite (China's Micius,
2017).

What it does *not* do, contrary to a great deal of writing: it does not transmit data,
it does not enable faster-than-light anything, and it does not solve authentication —
QKD without an authenticated classical channel is defeated by an on-path attacker
exactly as Diffie–Hellman is (Chapter 58 §58.2). It also requires special hardware,
does not survive optical amplification (so it cannot cross the amplified spans of
Chapter 50), and is limited to a few hundred kilometres without quantum repeaters,
which remain a research problem.

The honest assessment: QKD is a real technology with narrow applicability, and
**post-quantum cryptography** — classical algorithms believed resistant to quantum
attack, standardised by NIST in 2024 — is the answer to the quantum threat that will
actually be deployed, because it requires no new hardware and can be rolled out as a
software update.

Deterministic networking and TSN, §71.4, which is Chapter 13's argument returning
for the fourth time in this book. IEEE 802.1 Time-Sensitive Networking adds bounded
latency and zero congestion loss to Ethernet — scheduled traffic, time synchronisation,
frame preemption — for industrial control, automotive networks and professional audio,
where a late packet is a failed control loop rather than a stutter.

Whether this remains specialised or becomes general is genuinely open. The pattern in
this book suggests general-purpose absorbs specialised eventually; the counterargument
is that hard real-time guarantees may be one of the things that cannot be absorbed
without becoming the specialised thing.

## Overhyped, with a real core

Machine learning in and on the network, §71.5, split into the two directions
because they are different problems.

*ML for networking* — using learning to operate networks — is Chapter 70 §70.4's AIOps
assessment: anomaly detection and correlation work, autonomous remediation of novel
faults does not.

*Networking for ML* — building networks for training clusters — is where the genuinely
interesting current work is, and it is less discussed. Training a large model involves
thousands of accelerators exchanging gradients in a tightly synchronised pattern, where
the slowest link determines the speed of the entire job and a single congested path
stalls everything. This has driven RDMA over Converged Ethernet, lossless fabrics with
priority flow control, and in-network aggregation on programmable switches
(Chapter 68 §68.3). It is a workload with requirements unlike anything in the previous
thirteen units, and it is reviving the deterministic-fabric arguments in a new context.

## The pattern worth carrying

§71.5 closes with the thing to take from the chapter, which is not any specific
prediction.

**Watch the economics, not the specifications.** When evaluating any claim about the
future of networking, ask: what does adoption cost, who bears that cost, who receives
the benefit, and can it be adopted incrementally by one party at a time? A technology
that answers those questions well beats a technically superior one that does not.

IPv6 answers them badly and has taken thirty years. Ethernet answered them well at
every step and is everywhere. That is the most reliable predictor this field offers.

## By the end you will be able to

- Explain why infrastructure predictions fail, and apply the incremental-adoption test.
- State what QKD provides, what it does not, and why post-quantum cryptography is the
  more consequential development.
- Explain what TSN adds to Ethernet and connect it to Chapter 13's argument.
- Distinguish ML *for* networking from networking *for* ML.
- Evaluate a claim about a future technology on economic rather than technical grounds.
