# Chapter 71 — The People

**Charles Bennett (b. 1943) and Gilles Brassard (b. 1955).** **BB84 — quantum key distribution,
1984.**

**The paper's reception is worth knowing:** **it was presented at a conference in Bangalore, was
not published in a major journal, and was largely ignored for a decade.**

> **The idea rests on a physical fact that had been understood since the 1920s and that nobody
> had thought to use: measurement disturbs the state.** **Bennett and Brassard's contribution was
> to see that an unavoidable inconvenience of quantum mechanics is a security primitive.**

**Their earlier collaborator, Stephen Wiesner, had proposed "conjugate coding" in about 1970** —
**quantum money that cannot be counterfeited** — **and could not get it published for over a
decade.** **Bennett and Brassard's paper credits it, and Wiesner's manuscript eventually appeared
in 1983 in a newsletter.**

**Bennett's wider work is substantial** — **quantum teleportation (1993), reversible computation,
and the thermodynamics of information** — **and Brassard's on quantum cryptography's foundations
continues.**

> **The relevant lesson for this chapter is the timescale.** **BB84 is forty-one years old, the
> physics is not in doubt, and the deployment is a few point-to-point links and one national
> backbone.** **Which is a useful calibration for claims about how quickly quantum networking
> will arrive.**

**Artur Ekert (b. 1961).** **The entanglement-based protocol, 1991** — **E91.**

**Where BB84 uses single photons and a basis choice, Ekert's scheme uses entangled pairs and
Bell's inequality:**

> **If the correlations between the two parties' measurements violate Bell's inequality, no local
> hidden variable — including an eavesdropper's stored copy — can explain them.** **The security
> is derived from a testable physical property rather than from the protocol's construction.**

**Which is more elegant and harder to implement**, and it is the basis of **device-independent
QKD** — **security that does not depend on trusting the equipment**, which is the field's most
interesting current direction and is very far from practical.

**Peter Shor (b. 1959) and Lov Grover (b. 1961)**, for the threat rather than the remedy.

**Shor's algorithm (1994) factors integers and solves discrete logarithms in polynomial time on a
quantum computer** — **which breaks RSA and elliptic curve cryptography entirely** (Chapter 58
§58.4).

**Grover's algorithm (1996) searches an unstructured space in $\sqrt{N}$** — **which halves a
symmetric key's effective length and leaves AES-256 adequate.**

> **The asymmetry between the two results is the whole of the post-quantum transition.**
> **Asymmetric cryptography must be replaced; symmetric cryptography needs a larger key.** **And
> the reason the transition began before a machine exists is harvest-now-decrypt-later**
> (Chapter 58 §58.2).

**Shor's algorithm is also the reason quantum computing received sustained funding**, which is a
recurring pattern: **a cryptographic threat mobilises resources that a computational benefit does
not.**

**Stefano Pirandola and colleagues, and the bound.**

**The PLOB bound (Pirandola, Laurenza, Ottaviani, Banchi, 2017)** established **the maximum
key rate achievable over a lossy channel without repeaters** — **and it is a proven limit rather
than an engineering estimate.**

> **Which is the quantum equivalent of Shannon's result** (Chapter 4): **a bound that no
> implementation can exceed, that tells you what is worth attempting.** **§71.3's distance
> arithmetic is this bound.**

**Michael Johas Teener, and the IEEE 802.1 Audio/Video Bridging group.**

**TSN began as AVB, and its origin is unglamorous:** **synchronising audio across a stage.**

> **A professional audio installation needs many devices to play the same sample at the same
> instant, to microsecond accuracy, over a shared network.** **Which is a determinism requirement
> with no safety consequence and a very demanding tolerance**, and solving it produced the time
> synchronisation and the credit-based shaper that industrial and automotive TSN then adopted.

**The renaming from AVB to TSN in 2012 reflected the audience change** — **from audio to
industrial control and automotive** — **and the standards' shape still shows the origin.**

**And it is a good example of a general pattern:** **a demanding but low-stakes application
produces the mechanism that a high-stakes one then depends on**, which is also how Ethernet
(Chapter 16) and Wi-Fi (Chapter 44) reached the applications they now carry.

**Norm Finn and the 802.1 TSN authors** carried it into the safety-critical domains, **and the
work on frame preemption, replication and per-stream policing is theirs.**

**The Ultra Ethernet Consortium, and NVIDIA's InfiniBand lineage.**

**§71.5's AI fabric is a live standards contest**, and it is worth naming as such.

**InfiniBand** — **from the 1999 merger of two competing proposals, and it survived as a
high-performance computing interconnect when it failed as a general server fabric** — **has
credit-based flow control, lossless transport and a centralised subnet manager built in.**
**Which makes it well suited to the collective pattern, and it is a single-vendor ecosystem.**

**The Ultra Ethernet Consortium (2023)** — **AMD, Broadcom, Cisco, HPE, Intel, Meta, Microsoft and
others — is an attempt to build an Ethernet transport designed for the pattern** rather than
adapted to it: **multipath by default, out-of-order delivery to the application, and congestion
control designed for collectives.**

> **Which is Chapter 68's buyers'-consortium strategy again** (ONF, OpenConfig, OCP) — **large
> purchasers forcing an open alternative to a single-vendor product** — **and Chapter 69's
> observation applies: it succeeds when the participants benefit from commoditisation, and the
> hyperscalers demonstrably do.**

## What this chapter's history shows

**Three observations that generalise beyond any of its subjects.**

**Physical limits are discovered before they bind, and then they bind.** **Shannon (1948),
the non-linear Shannon limit, the PLOB bound** — **each established what was possible long before
engineering approached it**, and **optical networking is currently the clearest demonstration of
a field reaching one.**

**Demanding low-stakes applications produce the mechanisms that high-stakes ones adopt.**
**AVB's stage audio produced automotive TSN. Ethernet's office LAN produced the industrial
fieldbus's replacement. The pattern is consistent**, and it argues for taking apparently
frivolous requirements seriously.

**And the timescales are long.** **BB84 is forty-one years old with minimal deployment. TSN's
predecessor began in 2005 and is deploying now. 5G's promises are a decade old and partially
unmet.** **A frontier chapter's honest contribution is calibration**: **what is plausible, over
what period, and which claims are repeats.**

> **Which is why this chapter is written with more scepticism than enthusiasm.** **The
> enthusiasm is available everywhere else**, and **the discipline of asking "what does the
> physics permit, what did the last generation promise, and who benefits from this being
> believed?" is the transferable skill.**
