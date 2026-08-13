# Chapter 46 — The People

**Douglas H. Ring (1907–2000) and W. Rae Young.** The **1947 Bell Labs memorandum** proposing
hexagonal cells with frequency reuse.

**The document is remarkable for how complete it is.** Cells, reuse, the capacity argument,
and the recognition that cells could be split as demand grew — **the entire architecture of
§46.1, thirty-two years before deployment.**

**And Ring identified the obstacle correctly:** the system required continuous measurement and
switching decisions **at a rate no manual process could achieve.** He did not have a name for
what was missing, because the computer that would supply it had barely been invented.

> **A design that is correct and unbuildable is not a failure. It is a specification waiting
> for a component**, and this book contains several — Babbage's engine, Ring's cells,
> Perlman's routed Layer 2 (Chapter 19).

**Richard H. Frenkiel (b. 1943) and Joel Engel (b. 1936).** Bell Labs, and **the people who
made Ring's idea into a system** in the late 1960s and 1970s.

**Their contribution was the operational design:** how handover would actually be decided, how
the switching centre would track subscribers, how channels would be allocated dynamically, and
**how the whole thing would be controlled by a computer.**

**AMPS, deployed in Chicago in 1983**, is theirs. **National Medal of Technology, 1994.**

**Martin Cooper (b. 1928).** Motorola, and **the first handheld cellular call, 3 April 1973**
— made from a New York street to Joel Engel at Bell Labs, which is a detail worth savouring.

**The significance is the word *handheld*.** Bell's vision was car telephones; **Cooper's
insistence that the device should be personal rather than vehicular** shaped everything after.
The DynaTAC weighed 1.1 kg and took ten hours to charge for thirty minutes of talk.

**Amos Joel Jr. (1918–2008).** **The handover patent** (1972) — the mechanism for transferring
a call between cells without interrupting it.

**Unglamorous and load-bearing:** without automatic handover, cellular is a system in which
calls drop whenever you move, **and the whole premise fails.**

**The GSM working group, and Thomas Haug (1927–2023).** GSM's standardisation through the
1980s, and it is **the most successful telecommunications standard ever produced** by adoption.

**The achievement was political as much as technical.** Getting European administrations to
agree one digital standard — against national champions each preferring their own — **created a
single market of a size that made handsets cheap**, and that economics is why GSM reached 80%
of the world while the US fragmented across incompatible systems.

**Friedhelm Hillebrand and Bernard Ghillebaert.** **SMS**, and **the 160-character limit** —
chosen by Hillebrand on the evidence that most postcards and telex messages fit in about that
much.

**A constraint derived from a survey of postcards** shaped a generation's writing.

**Irwin Jacobs (b. 1933) and Andrew Viterbi (b. 1935).** **Qualcomm**, and CDMA for commercial
cellular.

**Viterbi's algorithm** — for decoding convolutional codes — predates the company and is used
in essentially every digital communication system, including the ones that do not use CDMA.

**Their claim that CDMA would give many times TDMA's capacity was contested bitterly** through
the early 1990s, and **the truth was in between**: CDMA's advantages were real, its power-control
requirements were as demanding as critics said, and **its ideas — reuse of 1, soft handover,
and the treatment of interference as a shared budget — went on to shape 3G and beyond even as
the specific technology was superseded.**

**The 3GPP working groups.** LTE and 5G, and worth naming as an institution rather than
individuals.

**3GPP's model — releases rather than generations** — is why LTE improved continuously from
Release 8 (2008) through Release 15 without a rebranding, **and why "4G" and "5G" are marketing
labels applied to a continuum.**

**LTE Release 8's architectural decisions** — all-IP, flat architecture, no RNC, separated
control and user planes — **were made by committee and were right**, which is worth noting
because this book more often records committees getting things wrong.

**Erik Dahlman, Stefan Parkvall and Johan Sköld.** Ericsson, and **the authors of the standard
texts on LTE and NR.**

**Their books are how most engineers learn this material**, and they are unusually good at
explaining the *reasoning* behind the standard's choices — which the specifications themselves,
being normative documents, do not.

**The mmWave researchers — Ted Rappaport and the NYU Wireless group.** The measurement
campaigns that established what mmWave propagation actually does.

**Their contribution was empirical and it corrected an assumption.** mmWave had been thought
unusable for mobile because of its propagation; **their measurements showed that with
beamforming and reflection off buildings, urban mmWave was viable over a few hundred metres.**

**Which is exactly what was deployed**, and it is also why the coverage is what §46.4
describes — **the measurements were honest and the marketing was not.**

**Rappaport's textbook** is Chapter 42's reading list, which is a reasonable indication of
range.
