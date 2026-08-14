# Chapter 7 — The People

**G. E. Thomas.** Proposed what became Manchester encoding in 1949, at the
University of Manchester, for storing data on the magnetic drum of the Manchester
Mark 1 — one of the first stored-program computers. The scheme was invented for
storage rather than transmission, which is worth noting: a magnetic drum has the
same clock-recovery problem as a wire, because the head must know where one bit
ends and the next begins with no external timing reference. It transferred to
Ethernet thirty-four years later essentially unchanged. Thomas's original polarity
convention is the opposite of IEEE 802.3's, which is why standards documents always
specify which they mean.

**Albert Widmer and Peter Franaszek.** IBM researchers whose
1983 paper *A DC-Balanced, Partitioned-Block, 8B/10B Transmission Code* is one of
the most quietly consequential pieces of coding engineering ever published. The
partitioning into 5B/6B and 3B/4B was the practical insight — it turned one
256-entry table into two small ones, which mattered enormously in 1983 silicon —
and the running disparity mechanism gave a construction-guaranteed DC balance that
nobody had achieved so cheaply. The code went into Fibre Channel, then Gigabit
Ethernet, then PCI Express, SATA, DisplayPort and InfiniBand. It is difficult to
find a computer built after 1998 that does not contain an implementation.

**Gottfried Ungerboeck (b. 1940).** Austrian engineer at IBM Zurich whose 1982
paper *Channel Coding with Multilevel/Phase Signals* introduced trellis-coded
modulation. The result was startling because the field had treated modulation and
error correction as separate problems solved in sequence — pick a constellation,
then bolt on a code. Ungerboeck showed that designing them jointly, so that the
code constrains which *sequences* of constellation points are legal, gained 3–6 dB
for free. It went immediately into the V.32 and V.34 modem standards, which is a
substantial part of why dial-up modems reached 33.6 kb/s rather than stalling near
14.4, and it is in every multilevel system since — including the PAM-5 of
1000BASE-T, where the fifth level exists to carry the trellis code's redundancy.

**Andrew Viterbi (b. 1935).** His 1967 algorithm is what makes trellis decoding
computationally feasible: it finds the most likely sequence through a trellis in
time linear in the sequence length rather than exponential. Every trellis-coded
system, every convolutional decoder, and — unchanged — a great deal of speech
recognition and bioinformatics runs it. He co-founded Qualcomm and his work on
CDMA underlies Chapter 46. See Chapter 4's notes for more.

**Robert Gallager (b. 1931).** His low-density parity-check codes, invented in his
1960 doctoral thesis and impractical for thirty-five years, provide the coding gain
that makes 10GBASE-T's PAM-16 viable despite its 23.5 dB penalty. The story is in
Chapter 4's notes; it belongs here because §7.4's arithmetic only works with a
strong code behind it.

**Claude Shannon (1916–2001).** The capacity theorem is what tells you whether a
proposed multilevel scheme is possible at all, and §7.4's tradeoff between bits per
symbol and required SNR is the practical face of *C* = *B* log₂(1 + SNR). Every
code in this chapter is an attempt to approach a bound he established before any of
them existed.

**The IEEE 802.3 working group.** Not a person, and worth naming anyway, because
the efficiency ladder in §7.3 is a record of collective engineering judgement
exercised repeatedly over thirty-five years. Each step — Manchester to 4B/5B to
8B/10B to 64B/66B to 256B/257B — was a decision by a committee of competing vendors
about how much overhead was tolerable and how much risk was acceptable, made with
the constraint of an installed cable plant that nobody could replace. The
consistent thread is that the *interface* — the Ethernet frame — never changed,
while the mechanism beneath it was rebuilt completely four times. Chapter 16 §16.3
draws the general lesson.
