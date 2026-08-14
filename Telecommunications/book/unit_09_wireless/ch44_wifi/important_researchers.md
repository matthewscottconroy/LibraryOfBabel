# Chapter 44 — The People

**Norman Abramson (1932–2020).** **ALOHAnet**, 1971 — Chapter 16's subject, and the direct
ancestor of everything in this chapter.

The random-access idea is his: transmit when you have something to send, detect failure
from a missing acknowledgement, back off randomly and retry. CSMA/CA is that, with
carrier sensing added and the backoff refined.

And the missing-acknowledgement failure detection of §44.2 is ALOHA's, unchanged — because
in a radio network there is still no other way to know.

**Vic Hayes (b. 1941).** Chapter 43's notes cover his role. The 802.11 working group's
achievement was interoperability, not invention: several incompatible wireless LAN products
existed, and the standard made them one market.

Bruce Tuch, Cees Links and the NCR Nieuwegein team. WaveLAN, from the late 1980s, and
much of what became 802.11's physical layer.

Links has written honestly about the commercial history — including that the wireless
LAN market was widely believed not to exist, and that the technology waited nearly a decade
for a use case that justified its cost.

**Phil Karn (b. 1956).** His second appearance — Chapter 37's RTT estimator is his — and here
for MACA (Multiple Access with Collision Avoidance), 1990, which introduced **RTS/CTS.**

**The insight is §44.2's:** carrier sense fails when stations cannot hear each other,
so replace "listen to the medium" with "ask the receiver". The receiver's clear-to-send is
heard by everyone in *its* neighbourhood, which is the neighbourhood that matters.

Vaduvur Bharghavan and colleagues refined it as **MACAW** (1994), adding the
acknowledgement and the backoff improvements, and 802.11's DCF is recognisably MACAW.

Karn's range across this book — packet radio, TCP timers, RTS/CTS — comes from being an
amateur radio operator who wrote TCP/IP for packet radio when nobody else had. The problems
he solved were problems he had.

**Gerard Foschini, Michael Gans and Emre Telatar.** The MIMO capacity result — Chapter 42's
notes. 802.11n is its commercial realisation, and the three orders of magnitude of
throughput growth in this chapter rest substantially on their theorem.

**Arogyaswami Paulraj (b. 1944).** Stanford, and the first patent on spatial multiplexing
using multiple antennas (1994) — before the capacity results were widely known.

His contribution was recognising it was implementable, and much of the early practical
MIMO work came from his group. **Marconi Prize, 2014.**

**Greg Raleigh.** Co-founded **Airgo Networks**, which shipped the first practical MIMO Wi-Fi
chips in 2003 — six years before 802.11n was ratified.

**Which is the pattern of this chapter:** the products preceded the standard, the standard
made them interoperable, and the working group's job was to choose among several working
implementations rather than to design from nothing.

The 802.11n task group, over **seven years** (2002–2009). The longest and most
contentious amendment, with two competing proposals (TGnSync and WWiSE) and a deadlock that
required a merged joint proposal.

**Worth knowing because it explains a pattern:** draft-n products shipped in 2006, three
years before ratification, and were largely interoperable because the vendors had agreed
in practice what the committee had not agreed formally. Chapter 23's rough consensus and
running code, in a body that normally works the other way.

The 802.11ax authors, and the change of goal. Chapter 44 §44.1's most interesting
decision: stop optimising the peak rate and start optimising dense-environment
efficiency.

**The motivation was measurement.** Stadium, campus and conference deployments demonstrated
that peak rate had stopped being the constraint — contention, overhead and airtime
fairness were — and the amendment addressed those instead.

OFDMA is borrowed from LTE (Chapter 46 §46.3), which had used it since 2009. 802.11
adopting a cellular technique is a notable convergence, and it comes with cellular's
scheduling model attached.

**Matthew Gast (b. 1974).** Not a standards author primarily — the person who explained
802.11 to the industry. *802.11 Wireless Networks: The Definitive Guide* and the free
*802.11ac: A Survival Guide* are how a generation of engineers learned this material.

The pedagogical contribution matters (Chapter 30's notes make the general point): a
specification tells you what a protocol does and rarely why, and the small number of people
who wrote down the why are the reason it can be learned rather than only absorbed.

**Mathy Vanhoef.** **KRACK** (2017) — the key reinstallation attack against WPA2's four-way
handshake — and **Dragonblood** (2019) against WPA3's SAE, and **FragAttacks** (2021).

KRACK's significance was that it attacked the handshake's *state machine* rather than its
cryptography. By replaying message 3, an attacker could force a client to reinstall an
already-used key, resetting the nonce and permitting decryption.

The protocol had been formally proved secure, and the proof was of the cryptography
rather than of the implementations' handling of retransmission. A demonstration that a
proof's scope matters as much as its validity.

His work has repeatedly found flaws in mechanisms everyone considered settled, and the
responsible-disclosure handling has been exemplary each time.

**The aircrack-ng and Wireshark developers.** The tooling that makes §44.3's analysis
possible.

And the tooling is dual-use, in the pattern of Chapter 18's ARP notes: `aircrack-ng`
demonstrates the deauthentication attack and the handshake capture in two commands, and the
existence of that demonstration is what forced 802.11w to be specified and eventually
deployed.

> **The attack tool is the argument.** It recurs throughout this book, and wireless is where
> it has been most consequential — WEP, WPA, and management-frame protection each moved
> because someone published working code rather than a paper.
