# Chapter 5 — Further Reading

## Primary sources

**Fourier, J. (1822). *Théorie analytique de la chaleur*.**
The book that finally got the series into print, fifteen years after the rejected
memoir. Available in English as *The Analytical Theory of Heat* (Dover). The
introductory discourse is readable without the mathematics and shows a scientist
arguing for a method he knows to be more general than the problem he applied it to.

**Cooley, J. W. & Tukey, J. W. (1965). "An Algorithm for the Machine Calculation
of Complex Fourier Series." *Mathematics of Computation* 19(90): 297–301.**
Four pages. Worth reading for how little space a transformative algorithm needs,
and for the closing acknowledgement that Garwin suggested the problem.

**Heideman, M. T., Johnson, D. H. & Burrus, C. S. (1984). "Gauss and the History of
the Fast Fourier Transform." *IEEE ASSP Magazine* 1(4): 14–21.**
The archaeology: Gauss in 1805, and the half-dozen rediscoveries between. A good
corrective to the idea that important algorithms are found once.

**Nyquist, H. (1928). "Certain Topics in Telegraph Transmission Theory."
*Transactions of the AIEE* 47: 617–644.**
The zero-ISI pulse-shaping criterion, which connects this chapter's frequency
view to Chapter 7's practice.

## Books

**Pierce, J. R. (1980). *An Introduction to Information Theory: Symbols, Signals
and Noise*, 2nd ed. Dover.**
The single best accessible treatment of signals, spectra and information, by
someone who did the engineering. Chapters 2–4 cover this chapter's ground with
almost no mathematics and no loss of honesty. It costs very little and is the
recommendation if you read one supplementary book for Unit II.

**Smith, S. W. (1997). *The Scientist and Engineer's Guide to Digital Signal
Processing.* California Technical Publishing.**
**Free online in full** at dspguide.com. Chapters 8–12 build the Fourier transform
from scratch for people who want to *use* it rather than prove things about it,
with the clearest explanation of the time/frequency duality available anywhere.
If §5.2 felt fast, start here.

**Bracewell, R. N. (2000). *The Fourier Transform and Its Applications*, 3rd ed.
McGraw-Hill.**
The standard engineering reference. Rigorous, and unusually good on the pictorial
intuition — the "pictorial dictionary" of transform pairs in chapter 3 is worth
photocopying and pinning up.

**Oppenheim, A. V. & Willsky, A. S. (1996). *Signals and Systems*, 2nd ed.
Prentice Hall.**
The standard undergraduate text if you want the full treatment. Chapters 3 and 4
cover Fourier series and transforms; chapter 6 covers filtering. Demanding and
thorough.

**Horowitz, P. & Hill, W. (2015). *The Art of Electronics*, 3rd ed. Cambridge.**
Its section 1.7 on impedance and reactance, and its chapter 6 on filters, explain why
a cable behaves as a low-pass filter in terms of components rather than
abstractions. The book that makes the physical layer feel like something you could
build.

## Video and interactive

**3Blue1Brown, "But what is the Fourier Transform? A visual introduction."**
Twenty minutes, free on YouTube. The best available visual intuition for what the
transform actually does, and it will make §5.2 click for a substantial fraction of
readers who found the algebra opaque.

**Jez Swanson, "An Interactive Introduction to Fourier Transforms."**
Free at jezzamon.com. Drag things and watch the spectrum change. Ten minutes,
and it makes the harmonics-are-the-corners point far better than prose can.

## Standards and reference

**ITU-T Recommendation G.712, *Transmission performance characteristics of pulse
code modulation channels*.**
Where the 300–3,400 Hz voice band and the quantisation specifications are
actually written down, rather than quoted second-hand.

**TIA-568 series, *Commercial Building Telecommunications Cabling Standard*.**
The source of the category bandwidth ratings discussed in §5.3. Note what is
specified: bandwidth in MHz and a set of transmission parameters, **not** a data
rate. That distinction is §5.3's point and reading the actual standard makes it
unmissable.

## Tools

**`perfcalc.py shannon`** in this book's [tools/](../../../tools/) directory —
plot capacity against SNR for a given bandwidth, and confirm §5.3's claim that a
bandwidth supports a *range* of rates.

**Audacity**, or any audio editor with a spectrum view. Record yourself saying a
vowel and look at the spectrum; record a whistle and compare. Ten minutes of this
does more for intuition about spectra than an hour of reading, and it demonstrates
directly why speech survives band-limiting to 3.4 kHz.

**GNU Radio**, if you have an SDR dongle. Watching real spectrum in real time is
the single best way to make §5.2 concrete, and a receive-only dongle costs very
little.

## For the certification-minded

N10-009 does not examine Fourier analysis. It does examine, under objectives 1.5
and 2.4, the distinction between baseband and broadband transmission, cable
category bandwidth ratings, and the reasons a medium has a maximum distance. All
three are consequences of this chapter, and knowing the mechanism makes them
impossible to confuse — particularly the point in §5.3 that a category rating is a
bandwidth specification and not a data rate.
