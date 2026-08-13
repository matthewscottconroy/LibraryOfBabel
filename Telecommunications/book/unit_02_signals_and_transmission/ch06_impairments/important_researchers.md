# Chapter 6 — The People

**William Thomson, Lord Kelvin (1824–1907).** His 1855 "law of squares" was the
first quantitative treatment of what §6.3 calls delay distortion: on a capacitive
submarine cable, signalling speed falls as the square of length. He was arguing
against a practitioner who believed the problem was insufficient power, and the
episode established a pattern this book returns to repeatedly. His mirror
galvanometer and later siphon recorder were receivers sensitive enough to read the
smeared, attenuated signals that Whitehouse's brute force could not force through.
See Chapter 1 for the fuller account.

**Oliver Heaviside (1850–1925).** English self-taught engineer and mathematician,
and the person who worked out what to actually *do* about Kelvin's problem. His
1887 analysis showed that a transmission line's distortion could be eliminated by
adding inductance — the **loading coil** — and that a line satisfying a particular
relationship between its resistance, inductance, capacitance and conductance would
transmit all frequencies at the same speed and with the same attenuation. He also
reformulated Maxwell's twenty equations into the four vector equations everyone now
uses, introduced the operational calculus that engineers use to analyse circuits,
and coined the terms impedance, inductance, conductance and permeability. He worked
alone, in poverty, was frequently at odds with the scientific establishment, and
was largely vindicated. The Heaviside layer in the ionosphere is also his
prediction.

**Michael Pupin (1858–1935) and George Campbell (1870–1954).** Both independently
turned Heaviside's loading-coil result into a practical technology around 1900 —
Pupin at Columbia, Campbell at AT&T. The resulting patent dispute was substantial;
AT&T bought Pupin's patent and Campbell's employer was AT&T, which resolved it
commercially rather than intellectually. Loading coils extended the reach of
telephone lines enormously and are the reason long-distance telephony became
practical before amplification existed. They also, in a nice historical irony,
block the high frequencies that DSL needs, which is why loading coils have to be
removed from a loop before it can carry DSL (Chapter 49 §49.1).

**John B. Johnson (1887–1970) and Harry Nyquist (1889–1976).** The measurement and
the theory of thermal noise, published back to back in *Physical Review* in 1928.
Johnson measured a fluctuating voltage across every resistor he tried and showed it
depended on temperature and resistance and not on the material; Nyquist derived it
from thermodynamics. Together they established the floor beneath every receiver in
this book. See Chapter 4 for more.

**Harald Friis (1893–1976).** Danish-American radio engineer at Bell Labs. Two
results bear his name and both appear in this book: the **noise figure formula** of
§6.2, showing that a cascade's noise is dominated by its first stage, and the
**free-space path loss** transmission equation that Chapter 42 §42.3 uses for link
budgets. The first is why a low-noise amplifier belongs at the antenna; the second
is why doubling distance costs 6 dB. He also did foundational work on antenna
design and on the horn-reflector antennas that later, incidentally, detected the
cosmic microwave background.

**Sidney Darlington (1906–1997).** Bell Labs engineer, best known for the
transistor pair, who also did significant work on network synthesis and
equalisation — the theory of designing a filter whose response inverts a channel's
distortion, which §6.3 identifies as the reason Cat5e now carries 2.5 Gb/s.

**Robert Lucky (b. 1936).** Bell Labs engineer who in 1965 invented the **adaptive
equaliser**, a filter that measures the channel from the received signal itself and
adjusts continuously. Before Lucky, equalisation was fixed and set by hand for a
known line; after him, a modem could be plugged into an arbitrary telephone line
and work it out. Essentially every high-speed link in this book — every DSL modem,
every 10GBASE-T transceiver, every coherent optical receiver — contains a
descendant of his algorithm. He later ran Bell Labs' research division and wrote
*Silicon Dreams*, a good popular book on information and human interfaces.

**Charles Kao (1933–2018).** His argument that fibre's attenuation was a purity
problem rather than a property of glass is the reason §6.1's loss figures are what
they are. Chapter 10's notes cover him fully; he belongs here too, because the
1966 paper is fundamentally a paper about attenuation and what causes it.
