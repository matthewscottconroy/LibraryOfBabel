# Chapter 7 — Further Reading

## Primary sources

**Widmer, A. X. & Franaszek, P. A. (1983). "A DC-Balanced, Partitioned-Block,
8B/10B Transmission Code." *IBM Journal of Research and Development* 27(5):
440–451.**
Twelve pages, and the code is still in the machine you are reading this on. §II
states the requirements — bounded run length, bounded disparity, comma
detection — more crisply than any textbook, and the partitioning argument in §III
is a small masterclass in making a good idea implementable.

**Ungerboeck, G. (1982). "Channel Coding with Multilevel/Phase Signals."
*IEEE Transactions on Information Theory* 28(1): 55–67.**
Trellis-coded modulation. Demanding, and the introduction alone is worth reading
for the statement of why treating coding and modulation separately leaves several
decibels on the table.

**Ungerboeck, G. (1987). "Trellis-Coded Modulation with Redundant Signal Sets,
Parts I and II." *IEEE Communications Magazine* 25(2): 5–21.**
The accessible version, written five years later for practitioners. Start here
rather than with the 1982 paper.

**IEEE 802.3, Clause 36 (1000BASE-X, 8B/10B), Clause 40 (1000BASE-T, PAM-5),
Clause 49 (10GBASE-R, 64B/66B), Clause 55 (10GBASE-T, PAM-16), Clause 126
(2.5G/5GBASE-T), Clause 119 (200G/400G, 256B/257B).**
The specifications themselves, and freely available six months after publication
through the IEEE GET program. Clause 36's 8B/10B tables and Clause 49's scrambler
polynomial are the authoritative statements of §7.3's content, and reading a
clause once is a different experience from reading summaries of it.

## Books

**Sklar, B. & Harris, F. J. (2020). *Digital Communications: Fundamentals and
Applications*, 3rd ed. Pearson.**
Chapter 2 covers baseband formatting and line codes with unusually clear diagrams
of every waveform in §7.2. The treatment of the tradeoffs — bandwidth against
timing against DC balance — is the best available at an accessible level.

**Proakis, J. G. & Salehi, M. (2007). *Digital Communications*, 5th ed.
McGraw-Hill.**
Chapter 9 on digital communication through band-limited channels covers the
intersymbol-interference constraint that all of this exists to manage, and
Chapter 8 covers trellis-coded modulation rigorously.

**Lin, S. & Costello, D. J. (2004). *Error Control Coding*, 2nd ed. Prentice Hall.**
The standard reference for the coding side, including the trellis and LDPC material
that §7.4 depends on. Not a first read, and the place to go when the summary here
is insufficient.

**Johnson, H. & Graham, M. (1993). *High-Speed Digital Design.* Prentice Hall.**
Chapter 2's treatment of why AC coupling exists and what baseline wander does to a
real signal is the practical complement to §7.1's theory. It explains transformer
coupling from the point of view of someone who has to make it work.

**Petzold, C. (2022). *Code*, 2nd ed. Microsoft Press.**
The early chapters on Morse and Braille build the idea of encoding from nothing,
and make the point — which §7.3's control symbols depend on — that a code with
spare capacity can carry information about itself.

## Applied and reference

**Xilinx / AMD, Altera / Intel FPGA transceiver user guides.**
Freely available, and unusually good on the practical realities of 8B/10B and
64B/66B: comma alignment, elastic buffers, clock correction, and what actually
goes wrong. Written for people implementing it rather than teaching it, which
makes them honest about the failure modes.

**Cisco, "Gigabit Ethernet Auto-Negotiation" and the 1000BASE-T technology notes.**
The clearest vendor-neutral explanation of why 1000BASE-T uses four pairs
bidirectionally and what echo cancellation is doing.

**The 802.3bz (2.5G/5GBASE-T) task force archives, ieee802.org/3/bz/.**
Public presentations from the standardisation process, including the channel
measurements that determined what Cat5e could actually support. Reading a standards
body's working documents is instructive about how a specification comes to say what
it says.

## Tools

**A logic analyser or a fast oscilloscope, if available.** Capturing real
Manchester and real NRZ and comparing them takes fifteen minutes and settles §7.2
permanently. Many university labs have suitable equipment.

**`gnuradio` or a Python script.** Implementing a 4B/5B encoder and a scrambler is
an afternoon's work and it makes the run-length guarantee concrete: generate
adversarial input, run it through both, and count the longest run each produces.
exercise C3 of Chapter 7 is more interesting done this way than on paper.

**`perfcalc.py db`** in this book's [tools/](../../../tools/) directory — for
converting the 20 log₁₀(*M*−1) penalties of §7.4 in both directions.

## For the certification-minded

Line codes are not on N10-009. Three of this chapter's consequences effectively
are:

- **Cable category requirements per Ethernet standard** (objective 1.5): Cat3 for
  10BASE-T, Cat5 for 100BASE-TX, Cat5e for 1000BASE-T and 2.5GBASE-T, Cat6a for
  10GBASE-T. §7.2 and §7.4 explain why those and not others.
- **Wire signalling rate exceeding data rate**, which contributes to the gap
  between advertised and delivered throughput (objectives 3.1, 5.4).
- **Why a link can negotiate a rate it cannot sustain** (objective 5.2) — §7.4's
  diagnostic scenario, which appears in practice at every equipment refresh.
