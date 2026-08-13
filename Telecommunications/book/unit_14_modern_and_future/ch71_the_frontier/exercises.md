# Chapter 71 — Exercises

## A. Recall

**A1.** Give the six 6G research themes and rank them by plausibility with a reason for each.

**A2.** Compute the FSPL at 10 m for 2.4 GHz, 28 GHz and 300 GHz, and state what a hand does at
the third.

**A3.** Name three atmospheric absorption features that constrain terahertz, and their
approximate frequencies.

**A4.** What is integrated sensing and communication, and what is the concern that accompanies
it?

**A5.** Why can a learned air interface not exceed Shannon's limit, and what can learning
actually contribute?

**A6.** Give the four multipliers that have produced optical capacity growth and state which are
exhausted.

**A7.** What is the non-linear Shannon limit, and why does raising launch power not raise
capacity indefinitely?

**A8.** Why does the C-band have no equivalent outside it? Which chapter established this?

**A9.** Why do modern submarine cables use more fibre pairs at lower modulation orders?

**A10.** Why is space-division multiplexing a research direction rather than a deployment?

**A11.** State what QKD does and the four things it does not do.

**A12.** Why does QKD require an authenticated classical channel, and what does that imply about
its security foundation?

**A13.** Why can single photons not be amplified, and what is the consequence for distance?

**A14.** What is a trusted node and why does it undermine the security argument?

**A15.** State the three sources of Ethernet's non-determinism and the TSN mechanism that
addresses each.

**A16.** Why does hardware timestamping give sub-microsecond accuracy where NTP gives
milliseconds?

**A17.** Compute the guard band with and without frame preemption at 1 Gb/s, and the cycle
overhead for eight queues in a 1 ms cycle.

**A18.** Distinguish AI *on* the network from AI *in* the network.

**A19.** Why does an all-reduce break statistical multiplexing?

**A20.** Why is PFC correct in an AI fabric and usually wrong elsewhere?

## B. Apply

**B1.** For each 6G claim, state the physics constraint that bounds it and a plausible
application:

(a) 1 Tb/s wireless
(b) Sub-millisecond end-to-end latency
(c) Ubiquitous terahertz coverage
(d) The network as a sensor
(e) Satellite service to ordinary handsets

**B2.** Compute the per-wavelength line rate for: 64 Gbaud DP-QPSK, 96 Gbaud DP-16QAM, and
130 Gbaud DP-64QAM.

(a) Give the three figures.
(b) A 130 Gbaud signal occupies roughly 150 GHz of spectrum. Using Shannon's expression, find the
OSNR at which a 150 GHz channel's capacity equals the third line rate.
(c) State what that figure implies about the reach of a DP-64QAM wavelength compared with a
DP-QPSK one, and relate it to Chapter 50 §50.3's reach–capacity trade.

**B3.** A submarine cable has a fixed power budget supporting either 8 fibre pairs at 16QAM or
18 pairs at QPSK.

(a) Compute the total capacity of each, assuming 80 channels per pair at 64 Gbaud.
(b) State which is chosen in practice and why.
(c) State what this says about the design philosophy that preceded it.

**B4.** A QKD link runs over 150 km of fibre at 0.22 dB/km.

(a) Compute the total loss and the transmittance.
(b) At a 1 GHz source rate, estimate the raw photon arrival rate.
(c) State three further factors that reduce this to a usable key rate.
(d) State what would be required to extend the link to 600 km and what it would cost in security
terms.

**B5.** For each requirement, state whether QKD or post-quantum cryptography is appropriate, with
a reason:

(a) Protecting data that must remain confidential for 50 years, between two buildings 8 km apart
(b) Protecting TLS sessions from millions of clients
(c) A government link between two capitals 900 km apart
(d) Signing software updates
(e) Protecting a submarine cable's traffic

**B6.** Design a TSN schedule for a 1 ms cycle carrying: motion control (200 µs, highest
priority), control traffic (150 µs), and best effort.

(a) Give the gate control list.
(b) Compute the guard bands with and without frame preemption at 1 Gb/s.
(c) State the usable best-effort capacity in each case.
(d) State what happens if an undeclared stream begins transmitting.

**B7.** An AI training cluster has 512 accelerators, each with two 400G ports.

(a) What is the aggregate accelerator bandwidth?
(b) At 1:1 oversubscription, what fabric capacity is required?
(c) At 3:1, what fraction of the accelerators' time would be spent waiting, assuming
communication is 30% of the step time and scales inversely with bandwidth?
(d) The accelerators cost 20 times the network. Assess the 3:1 design.

**B8.** For each, state whether it is AI on the network or in the network, and what it actually
requires:

(a) Anomaly detection on interface counters
(b) A 1:1 non-blocking fabric
(c) Generating a configuration template from a description
(d) In-network gradient aggregation
(e) Correlating four hundred alerts into six incidents
(f) Lossless transport for RDMA

## C. Analyse

**C1.** The chapter says a claim that failed once deserves more scepticism the second time.
Analyse the 5G URLLC and mMTC promises against the 6G framework, and state what would have to be
different for the second attempt to succeed.

**C2.** Analyse integrated sensing and communication as a privacy problem. What exactly can be
inferred, by whom, and what regulatory framework would be required?

**C3.** Optical capacity growth is approaching a fundamental limit. Analyse the consequences for
network economics over the next decade, and for the assumption that bandwidth becomes cheaper.

**C4.** Analyse the submarine cable design inversion — more pairs at lower modulation. What
changed to make it correct, and what general principle does it illustrate?

**C5.** Multiple security agencies recommend post-quantum cryptography over QKD. Analyse their
reasoning, construct the strongest counter-argument, and state your own position.

**C6.** Analyse QKD's authentication problem rigorously. Is there any configuration in which the
quantum guarantee is not resting on a classical foundation?

**C7.** TSN reintroduces reservation into a packet network. Analyse this against Chapter 13's
argument, and identify every other mechanism in this book that does the same. What do they have
in common?

**C8.** Analyse why an AI training fabric's requirements invert statistical multiplexing. Is this
a new class of workload or an old one at a new scale?

**C9.** Priority flow control is described as usually wrong and correct in an AI fabric. Analyse
the conditions that make the difference, and state what would have to change for it to be
correct generally.

**C10.** Analyse the claim that large language models are a productivity tool for the
text-handling parts of network engineering and not an operator. What would have to be true for
that to change, and how would you know?

## D. Design

**D1.** Write the one-page assessment you would give a CTO who has been asked to fund a 6G
research partnership. Cover what is plausible, what repeats a failed promise, and what the
organisation would actually gain.

**D2.** Design the optical capacity plan for a network whose traffic is growing 35% annually,
given that per-fibre capacity improvement has slowed to 10% per generation with a four-year
cadence. State when additional fibre is required and what it would cost to defer.

**D3.** An organisation proposes a QKD link between two data centres 40 km apart. Design the
evaluation: what you would ask, what you would test, what the alternatives are, and the
recommendation you would make with its reasoning.

**D4.** Design a TSN deployment for a manufacturing line with 40 motion controllers on a 500 µs
cycle, plus camera traffic and general IT. Specify the topology, the standards required, the
schedule, the admission control, and what happens when a switch fails.

**D5.** Design the network for a 2,000-accelerator AI training cluster: topology,
oversubscription, transport, congestion control, the front-end/back-end split, and the power and
cooling implications. State the three decisions you are least confident about and why.

## E. Troubleshoot

**E1.** A terahertz link works across a laboratory bench and fails across a corridor. Explain,
with numbers.

**E2.** An optical system's pre-FEC error rate has risen after a route was extended by 40 km.
The post-FEC rate is still zero. Explain and state what you would do.

**E3.** Raising the launch power on a DWDM system increases the error rate. Explain.

**E4.** A QKD system's key rate falls to zero when the classical channels on the same fibre are
brought into service. Diagnose.

**E5.** A TSN network meets its deadlines in testing and misses them in production. Give four
possible causes.

**E6.** A TSN domain loses time synchronisation and every guarantee fails simultaneously.
Explain the dependency.

**E7.** An AI training job's step time doubles when a single link degrades. Explain, and state
why this differs from a conventional application.

**E8.** A PFC-enabled fabric deadlocks. Explain the mechanism and the two design responses.

**E9.** An AI-generated Ansible playbook passes review and breaks fourteen switches. Analyse
where the process failed.

## F. Extend

**F1.** Compute the link budget for a 300 GHz link over 20 m with 30 dBi antennas at both ends,
including atmospheric absorption. Determine whether it closes with a plausible transmit power and
receiver sensitivity.

**F2.** Read a current 6G white paper from a standards body or a major vendor. For each claim,
identify the physics constraint that applies and whether the paper addresses it. Report the
proportion that do.

**F3.** Compute the Shannon capacity of a modern optical channel at several OSNR values and
compare with the capacity of a current commercial transponder. Report the ratio and what it
implies about remaining headroom.

**F4.** Read a national security agency's published guidance on QKD (NCSC, NSA, BSI or ANSSI).
Summarise its reasoning in one page and assess whether you find it persuasive.

**F5.** Configure `ptp4l` and `phc2sys` between two Linux hosts with hardware timestamping
support, and measure the achieved synchronisation accuracy. Compare with NTP on the same hosts.

**F6.** If you have access to TSN-capable hardware, configure a `taprio` schedule on Linux and
measure the latency of a priority stream with and without background traffic. If not, measure the
same thing with ordinary priority queuing and report the difference in worst-case latency.

**F7.** Model the collective communication pattern of a data-parallel training job: given $n$
accelerators, a model size, and a link rate, compute the all-reduce time for ring and for tree
algorithms. Determine at what scale the network becomes the bottleneck.

**F8.** Use a large language model to generate a network configuration or an automation template
for a requirement you specify. Review it as you would a colleague's work, and record every error
you find and every place where it was confidently wrong.
