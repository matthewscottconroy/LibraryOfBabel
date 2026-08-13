# Unit II — Making Bits Travel

Chapter 4 ended with a formula and a promise. The formula says a channel of
bandwidth *B* at signal-to-noise ratio SNR can carry *B* log₂(1+SNR) bits per
second. The promise is that this rate is achievable. Neither the formula nor the
promise tells you how to actually get a one from one end of a copper pair to the
other, and that is the entire business of this unit.

The gap between "a channel has capacity" and "here is a working link" is where
telecommunications stops being mathematics and becomes engineering, and it is
populated by a set of problems that are not obvious until you try.

Here is the first one. You want to send `1`. You put five volts on the wire. At
the far end, three hundred metres away, you measure... four point one volts,
smeared over a longer interval than you sent it, riding on top of a sixty-hertz
hum picked up from the fluorescent lighting, with a spike in it from the lift
motor starting. Is that a `1`? Probably. What about the next symbol, which reads
two point three volts? What about the one after that, which reads four point one
volts again but which arrives eleven nanoseconds late because the receiver's clock
has drifted, so you are not entirely sure whether you sampled the symbol you think
you sampled?

Every question in that paragraph has a chapter behind it.

**Chapter 5** establishes the two ways of looking at a signal — as a shape in time
and as a spectrum in frequency — and argues that the second view, which is less
intuitive, is the one that explains everything. A channel's bandwidth is a
statement about frequency; a distortion is a statement about frequency; the
difference between baseband and broadband is a statement about frequency.

**Chapter 6** catalogues what the world does to your signal on the way: it makes
it weaker (attenuation), adds to it (noise), reshapes it (distortion and
dispersion), and lets the neighbours into it (crosstalk and interference). Each of
these is a distinct physical mechanism with a distinct signature, and each will
reappear in Chapter 65 as a distinct fault to be diagnosed.

**Chapter 7** solves the clock problem. If the receiver has to sample at the right
instant, and the receiver has no wire carrying the sender's clock, then the clock
must be recoverable from the data itself — and a long run of identical bits
carries no timing information at all. Line codes are the answer, and the story of
Ethernet's line codes from Manchester through 8B/10B to 64B/66B is a story of
engineers repeatedly renegotiating how much of the wire's capacity to spend on
keeping time.

**Chapter 8** takes the signal off baseband and puts it on a carrier, which is what
you must do to use radio at all, and which turns out to be the technique that
extracts multiple bits from every symbol. Quadrature amplitude modulation and its
constellation diagrams are the direct practical expression of Chapter 4's
bits-per-symbol arithmetic.

**Chapter 9** asks how several conversations share one physical channel, and finds
four answers — by frequency, by time, by code, and by wavelength — plus a fifth,
statistical multiplexing, which is not a physical technique at all but an economic
argument, and which is the single idea that made packet networks beat circuit
networks.

**Chapter 10** is where all of it becomes purchasable. Twisted pair, coax, fibre
and free space, with the numbers: what each costs, how far each reaches, how much
each carries, and what defeats each. The chapter ends with a decision procedure,
because "which cable" is a question you will be asked in your first week of work
and the answer is never "the fastest one."

---

A word about how to read this unit.

It is the most physics-heavy part of the book, and it is the part that students
preparing for a certification are most tempted to skim, because the exam asks
"which connector goes with single-mode fibre" rather than "why does modal
dispersion limit multimode reach." Skimming is a mistake, and not for the usual
moralising reason.

It is a mistake because the physical layer generates a disproportionate share of
real faults, and because physical faults are the ones that produce the most
misleading symptoms. A marginal cable does not fail cleanly; it produces
intermittent packet loss that looks exactly like congestion, that comes and goes
with temperature, and that will consume a week of someone's life if they do not
know to look at the interface error counters. A fibre run with a dirty connector
loses 3 dB and works perfectly at 1 Gb/s and not at all at 10 Gb/s. A Cat5e run
that passes a continuity test fails at 2.5 Gb/s because continuity is not the
property that matters.

You cannot diagnose what you cannot imagine. This unit is where you acquire the
imagination.
