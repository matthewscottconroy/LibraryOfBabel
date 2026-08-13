# Chapter 5 — The People

**Joseph Fourier (1768–1830).** French mathematician and physicist, and — a detail
that explains the shape of his career — Napoleon's governor of Lower Egypt from
1798 to 1801. He developed his series while studying heat conduction, submitting
the work to the Institut de France in 1807, where a committee including Lagrange,
Laplace and Legendre rejected it. Lagrange objected that a sum of continuous
sinusoids could not represent a function with a discontinuity, and on the narrow
technical point he was substantially correct: the series converges to the midpoint
at a jump, and near one it overshoots persistently — an effect rediscovered by
Michelson in 1898 and named for Josiah Gibbs. Fourier rewrote the work as
*Théorie analytique de la chaleur* (1822) and it eventually became one of the most
generally applied results in mathematics. He also proposed, in 1824, that the
atmosphere warms the Earth's surface by trapping outgoing radiation, which makes
him the originator of the greenhouse effect as well.

**Joseph-Louis Lagrange (1736–1813).** Italian-French mathematician, and included
here not as an obstacle but as a caution. His objection to Fourier was rigorous,
made in good faith, and technically defensible; it was also wrong about what
mattered. The episode is worth remembering whenever a technically correct
objection is raised against a practically transformative idea — a pattern that
recurs in this book with AT&T's response to Baran (Chapter 13) and the industry's
response to Ethernet (Chapter 16).

**James W. Cooley (1926–2016) and John Tukey (1915–2000).** Their 1965 paper *An
Algorithm for the Machine Calculation of Complex Fourier Series* reduced the
transform from *N*² to *N* log *N* operations. Cooley was at IBM Research; Tukey
was at Princeton and Bell Labs and had already named the bit (Chapter 2) and
coined "software". The FFT is generally counted among the most important
algorithms of the twentieth century, and it is running, right now, in the device
you used to connect to whatever network you are on.

The algorithm has an unusual history: it was discovered by **Carl Friedrich Gauss**
in about 1805 — before Fourier published the series it transforms — in the course
of interpolating asteroid orbits. Gauss wrote it in a notebook in an idiosyncratic
Latin and never published it. It was rediscovered at least half a dozen times over
the following century and a half before Cooley and Tukey's version, which arrived
at a moment when computers existed to run it and a specific application — detecting
Soviet nuclear tests from seismic data — was pressing.

**Harry Nyquist (1889–1976).** His 1928 pulse-shaping criterion is the bridge
between this chapter's frequency-domain view and Chapter 7's line codes: it
specifies the pulse shapes for which successive symbols do not interfere, and the
raised-cosine filters implementing it are in every modem and radio in this book.
See Chapters 1 and 4 for the fuller biography.

**John R. Pierce (1910–2002).** Bell Labs engineer and executive who directed the
work leading to the transistor (and named it), championed the first
communications satellites, and wrote several of the best accessible books on
information theory and signals. He is included here because his *An Introduction
to Information Theory* is the recommendation in this chapter's further reading and
because he is a rare example of someone who did first-rate engineering and
first-rate exposition of it.

**Josiah Willard Gibbs (1839–1903).** American physicist, the first American to
earn a doctorate in engineering, and the man whose name attaches to the overshoot
near a discontinuity in a truncated Fourier series. He described it in
correspondence to *Nature* in 1899 after Michelson's mechanical harmonic analyser
produced the effect and Michelson assumed his machine was faulty. It was not; the
mathematics does that. The episode is a small illustration of the chapter's theme:
the frequency-domain view predicts things the time-domain view finds surprising.

**Ralph Hartley (1888–1970).** Beyond the information measure of Chapter 4, Hartley
invented the oscillator circuit bearing his name and did substantial work on the
mathematics of modulation — the operation Chapter 8 develops and which §5.4
identifies as the thing that makes broadband signalling and frequency-division
multiplexing possible.
