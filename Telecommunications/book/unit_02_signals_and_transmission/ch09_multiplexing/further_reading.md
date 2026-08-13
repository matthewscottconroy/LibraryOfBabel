# Chapter 9 — Further Reading

## Primary sources

**Oliver, B. M., Pierce, J. R. & Shannon, C. E. (1948). "The Philosophy of PCM."
*Proceedings of the IRE* 36(11): 1324–1331.**
Eight pages, three authors of the first rank, and the case for digitising voice
made before anyone had built a system to do it. The regeneration argument that
Chapter 5 §5.1 develops is here in its original form, and §9.2's DS0 is its
descendant.

**Erlang, A. K. (1917). "Solution of Some Problems in the Theory of Probability of
Significance in Automatic Telephone Exchanges." *Post Office Electrical Engineers'
Journal* 10: 189–197.**
The blocking formula. Chapter 12 §12.4 applies it; it belongs here because the
underlying insight — that you need far fewer circuits than subscribers, and the
number is computable — is §9.3's argument in its original setting.

**Lamarr, H. & Antheil, G. (1942). US Patent 2,292,387, "Secret Communication
System."**
Readable in about twenty minutes, freely available, and worth reading precisely
because the story is so often garbled. The synchronisation mechanism using punched
rolls is on the second sheet of drawings.

**Mears, R. J., Reekie, L., Jauncey, I. M. & Payne, D. N. (1987). "Low-noise
erbium-doped fibre amplifier operating at 1.54 µm." *Electronics Letters* 23(19):
1026–1028.**
Two pages that reshaped the cost structure of global communications. Worth seeing
how short the announcement of a decisive result can be.

**ITU-T Recommendation G.694.1, *Spectral grids for WDM applications: DWDM
frequency grid*.**
Where §9.4's channel spacings are specified. Short, and it makes concrete that a
"channel" in DWDM is a defined frequency rather than a vague colour.

**ITU-T G.702 and G.703** (the plesiochronous hierarchy), and **ANSI T1.107** for
the North American side. The source of §9.2's rate tables, including the stuffed-bit
arithmetic that produces DS2's awkward 6.312 Mb/s.

## Books

**Bellamy, J. C. (2000). *Digital Telephony*, 3rd ed. Wiley.**
The standard reference for everything in §9.2. Chapters 4 and 7 cover PCM and the
digital hierarchies with a completeness no summary matches, including the
robbed-bit signalling detail and why it produces 56 kb/s. If you work anywhere near
carrier circuits, this is the book.

**Bertsekas, D. & Gallager, R. (1992). *Data Networks*, 2nd ed. Prentice Hall.**
Chapter 3 is the rigorous treatment of §9.3's statistical multiplexing, with the
queueing mathematics that Chapter 3 §3.2's ρ/(1−ρ) curve summarises. Demanding and
worth it.

**Kleinrock, L. (1975/1976). *Queueing Systems*, Volumes 1 and 2. Wiley.**
Where the results come from. Volume 2 applies them to computer networks
specifically. Not a first read.

**Viterbi, A. J. (1995). *CDMA: Principles of Spread Spectrum Communication.*
Addison-Wesley.**
By one of the people who made it commercial. Chapters 1–3 cover spreading,
processing gain and the near-far problem clearly; the power-control material is the
best available on why that is CDMA's defining operational burden.

**Ramaswami, R., Sivarajan, K. & Sasaki, G. (2009). *Optical Networks: A Practical
Perspective*, 3rd ed. Morgan Kaufmann.**
The standard WDM reference. Chapters 2 and 3 cover the components — amplifiers,
filters, ROADMs — and chapter 5 covers network design. Chapter 50 leans on it.

**Agrawal, G. P. (2012). *Fiber-Optic Communication Systems*, 4th ed. Wiley.**
Chapter 8 on multichannel systems covers WDM, including the four-wave mixing that
§9.4 identifies as dispersion-shifted fibre's undoing.

## Historical and popular

**Standage, T. (1998). *The Victorian Internet.* Walker.**
Good on why multiplexing mattered so much in the telegraph era, and on Bell's
harmonic telegraph as a commercial objective rather than a footnote.

**Rhodes, R. (2011). *Hedy's Folly: The Life and Breakthrough Inventions of Hedy
Lamarr.* Doubleday.**
A careful account of the 1942 patent that avoids both the dismissal and the
inflation the story usually receives. Good on Antheil, who is generally left out
entirely.

**Gertner, J. (2012). *The Idea Factory: Bell Labs and the Great Age of American
Innovation.* Penguin.**
Context for the PCM paper, the carrier hierarchies, and the institutional culture
that produced them. Useful for understanding why so many of this book's citations
share one employer.

## Tools

**`simnet.py statmux`** in this book's [tools/](../../../tools/) directory —
computes the reserved-versus-statistical capacity comparison and the overflow
probability for any population, and is the fastest way to see §9.3's gain grow with
scale. Run it at 10, 100, 1,000 and 10,000 users and watch the ratio move.

**`simnet.py queue`** — the ρ/(1−rho) curve, which is what determines the headroom
that a statistically multiplexed link needs.

**A spectrum analyser on a cable television drop**, if you can get access. Seeing
the FDM channel plan directly — television channels here, DOCSIS downstream there,
upstream in the low band — makes §9.1 concrete in a way diagrams do not.

## For the certification-minded

N10-009 covers T1/E1 rates and DS0 under the WAN objectives, and CWDM/DWDM under
the fibre and WAN objectives. It does not name statistical multiplexing, and three
of its consequences are examined throughout:

- **Oversubscription ratios** as a normal design parameter rather than a defect.
- **Why shared access media** (cable, PON) perform differently at peak than
  dedicated ones (DSL, fibre to the premises).
- **Why capacity planning targets peak rather than average utilisation.**

§9.3 is the mechanism behind all three, and the arithmetic makes them arguable
rather than merely assertable — which matters when a manager asks why you are
buying capacity that sits idle most of the day.
