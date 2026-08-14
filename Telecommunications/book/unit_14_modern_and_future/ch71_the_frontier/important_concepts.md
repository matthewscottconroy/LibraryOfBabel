# Chapter 71 — Important Concepts

A claim that failed once deserves more scepticism the second time, not less *(§71.1)* —
5G's URLLC, mMTC and slicing promises largely did not arrive (Chapter 46 §46.4), and 6G's
framework repeats several of them.

FSPL rises with the square of frequency, and terahertz adds absorption on top *(§71.1)* —
60 dB at 2.4 GHz, 92 dB at 100 GHz, 112 dB at 1 THz, over 10 m — plus water vapour peaks in
the hundreds of dB per kilometre. At 300 GHz a hand blocks the link — not attenuates,
blocks — and the wavelength is one millimetre, so a raindrop is an obstacle.

Terahertz will be deployed where the geometry is controlled *(§71.1)* — fixed links, data
centres, sensing — and mid-band will continue to carry mobile traffic. Which is exactly
what happened with mmWave, and the pattern is worth expecting.

A radio that transmits and receives is a radar *(§71.1)* — Integrated sensing is the
genuinely novel idea and the least discussed, and it works today at Wi-Fi frequencies. And a
network that can sense is a network that is sensing — presence, movement and activity within a
building, derivable by whoever operates it. The privacy framework does not exist and is the
harder problem.

A learned physical layer cannot exceed Shannon's limit *(§71.1)* — What learning can do is
approach it with less computation, adapt faster, or handle a régime where the classical model
fits badly. And two vendors' learned transceivers must interwork, which requires the learned
representation to be standardised, which removes much of the point.

Air interface latency is not end-to-end latency *(§71.1)* — A 1 ms radio and a 40 ms path
is a 41 ms system, and the fix is edge computing rather than radio. Deterministic wireless is
achievable in a controlled environment and not in general.

Optical capacity's four multipliers are exhausted or nearly so *(§71.2)* — The C-band is
full; the channel widths are flexible and the band is finite; bits per symbol are SNR-limited;
and only baud rate is still advancing. A single wavelength now carries more than an entire
DWDM system did in 2000, and it came from the terminals.

Above an optimal launch power, more power produces more noise and not more signal *(§71.2)*
— **the Kerr non-linearity** — which is the non-linear Shannon limit, and current systems are
within a small factor of it. The industry has moved from "how do we get more out of this
fibre?" to "how do we use more fibre?"

The C-band's convenience was a coincidence, and the bands beyond it do not have it *(§71.2)*
— Erbium's gain happens to coincide with silica's loss minimum (Chapter 50 §50.3). Outside
C and L there is no equivalent amplifier, which is the obstacle to using more spectrum.

Modern submarine cables run more pairs at lower modulation *(§71.2)* — Because power from
shore is fixed and power per bit is minimised at a lower modulation order. The optimum is
more pairs at QPSK rather than fewer at 16QAM, which inverts a decade of design instinct.

Space-division multiplexing requires new fibre, and there is unlit fibre available first
*(§71.2)* — which is why it is a research direction rather than a deployment.

As a system approaches its limit it becomes more sensitive to what margin used to absorb
*(§71.2)* — ageing, a dirty connector, an added span — so the pre-FEC error rate becomes a
more important operational signal (Chapter 50 §50.2).

The next optical generation's constraint is thermal *(§71.2)* — A 1.6T optic dissipates
25–30 W and 32 of them is a kilowatt in optics alone, which is why linear-drive and
co-packaged optics are being developed. A surprising statement fifteen years ago.

QKD distributes a key and nothing else *(§71.3)* — The data is still encrypted with AES.
It does not authenticate: it gives you a shared secret with somebody and does not say with
whom — **exactly Diffie–Hellman's limitation** (Chapter 58 §58.2) — so it requires an
authenticated classical channel, and the quantum guarantee sits on a classical foundation.

Single photons cannot be amplified, because amplification is a measurement *(§71.3)* —
20 dB of loss at 100 km, 40 dB at 200 km, and the PLOB bound gives 0.014 bits per channel use
at 100 km. Practical systems reach perhaps 100 km.

Each trusted node holds the key in the clear *(§71.3)* — So the end-to-end security is the
physical security of the intermediate nodes — a classical assumption, and exactly the one QKD
was meant to remove. China's 2,000 km backbone has 32 of them.

Multiple security agencies recommend post-quantum cryptography over QKD *(§71.3)* — citing
the authentication requirement, the trusted-node problem, the cost and the absence of a
compelling advantage. A strong signal, from organisations with the budget and motivation to
deploy QKD if it were better.

Entanglement does not transmit information *(§71.3)* — No-signalling is a theorem. The
classical channel is still required and still limited by Chapter 3 §3.1.

A guarantee is not a priority *(§71.4)* — QoS provides priority under contention;
determinism provides a bounded worst case. Not "usually under 10 ms" but "never more than
250 microseconds, and if it is, the machine stops."

A 1,500-byte frame occupies 1 Gb/s of wire for 12 µs *(§71.4)* — which is larger than an
entire motion control cycle, and it is why frame preemption exists. Eight queues' guard bands
consume 9.6% of a 1 ms cycle without it and 0.4% with it.

Time-aware shaping is time-division multiplexing reintroduced into a packet network
*(§71.4)* — During its window the critical queue's traffic is the only thing that can be
transmitted, so there is nothing to queue behind and the latency is computable. Another
instance of an industry that abandoned circuits wanting some of their properties.

Hardware timestamping is why 802.1AS achieves sub-microsecond where NTP achieves
milliseconds *(§71.4)* — The timestamp is applied by the MAC as the frame leaves the wire,
not by software when it is queued — which removes the operating system's scheduling jitter.

802.1CB removes reconvergence from the failure path *(§71.4)* — The second copy was already
in flight, so a link failure does not require detection and rerouting.

A TSN domain is as deterministic as its least capable switch *(§71.4)* — And time reserved
for a critical stream is unavailable to everything else whether it is used or not — Chapter 13
§13.1's circuit trade, unchanged.

Every mechanism that provides a guarantee reintroduces reservation and pays the same price
*(§71.4)* — ATM, MPLS-TE, IntServ, TSN, DetNet. The guarantee is bought where the
consequence of lateness justifies the cost, and best effort is used everywhere else.

AI on the network and AI in the network are different subjects *(§71.5)* — The second is
the largest single force on network design today and is discussed far less.

Large language models are confident when wrong *(§71.5)* — The same failure mode as
Chapter 58's cryptography warning: plausible output indistinguishable from correct output, and
it requires the same response. They know what networks are like, not what your network is —
a productivity tool for the text-handling parts of the job, and not an operator.

An all-reduce is a synchronous barrier, so the step time is the slowest path *(§71.5)* —
Which inverts every assumption in Chapter 52: a few enormous synchronised flows, statistical
multiplexing that does not work, 1:1 oversubscription, and a tail that stalls every accelerator
rather than affecting some users.

The accelerators cost more than the network by a large factor *(§71.5)* — so a network that
leaves them idle 20% of the time has wasted more money than the network cost, which justifies
engineering that would be absurd for any other workload.

PFC is usually wrong and is correct here *(§71.5)* — Because the traffic class is known,
the fabric is dedicated and the alternative is unacceptable — and its failure modes,
including deadlock in a fabric with cyclic buffer dependencies, are being learned expensively.

A training job's communication pattern is known in advance *(§71.5)* — so the topology can
be configured for it — which is a circuit-switched argument arriving where nobody expected
it, for the reason it arrives everywhere: the traffic is predictable and the guarantee is worth
more than the flexibility.

In-network aggregation is the strongest argument that programmable pipelines have a commercial
future *(§71.5)* — summing gradients in the switch shortens the critical path — even
though the general-purpose programmable switch did not survive (Chapter 68 §68.3).

The binding constraint on large AI deployments is power *(§71.5)* — 40–130 kW per rack
against a conventional 5–15, requiring liquid cooling and a site with grid capacity that may
not exist — so facilities are placed where the power is, which makes Chapter 50's long-haul
and Chapter 52's content delivery relevant to a workload that did not previously need them.
