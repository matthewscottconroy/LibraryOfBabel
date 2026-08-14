# Chapter 42 — The People

**James Clerk Maxwell (1831–1879).** **The four equations**, published in 1865, which
unified electricity and magnetism and predicted electromagnetic waves travelling at the
speed of light — from which he inferred that light *is* an electromagnetic wave.

The prediction preceded any observation by more than twenty years. Nobody had generated
a radio wave, nobody had detected one, and the mathematics said they must exist and how fast
they must travel.

> Everything in Unit IX is a consequence of four equations written before anyone had seen
> the phenomenon they describe.

**Feynman's assessment** — that from a long view of history, the most significant event of
the nineteenth century will be judged Maxwell's discovery of the laws of electrodynamics —
is not obviously an exaggeration.

**Heinrich Hertz (1857–1894).** Generated and detected radio waves in 1887, confirming
Maxwell experimentally with a spark gap and a loop of wire with a gap in it.

He measured their speed, their reflection, their refraction and their polarisation — the
whole of §42.4's behaviours, established in a laboratory in a few years.

And asked what use they were, he said: "It's of no use whatsoever." He believed he had
confirmed a theory, which he had. He died at 36, before Marconi's first transmissions.

The unit of frequency is his, which is the appropriate memorial.

**Guglielmo Marconi (1874–1937).** Took Hertz's laboratory demonstration and made it a
communication system — and the transatlantic transmission of December 1901 was the moment
radio became a technology rather than a curiosity.

**The transmission should not have worked.** Marconi's engineers expected line-of-sight
limits, and 3,500 km is far beyond the horizon. It worked because of the ionosphere,
whose existence was not established until Heaviside and Kennelly proposed it independently
in 1902 — so the most famous radio experiment in history succeeded through a mechanism
nobody yet knew about.

**Oliver Heaviside (1850–1925) and Arthur Kennelly (1861–1939).** The ionosphere, proposed
independently to explain Marconi's result.

**Heaviside is underrated in this book's territory.** He reformulated Maxwell's twenty
equations into the four vector equations everyone now uses, invented the terms impedance,
inductance, admittance and conductance, and developed the transmission-line theory that
Chapter 8's cable behaviour rests on.

He was self-taught, worked alone, was frequently in dispute with the scientific
establishment, and much of what an engineer calls "Maxwell's equations" is actually
Heaviside's formulation of them.

**Edward Appleton (1892–1965).** Proved the ionosphere existed and measured its height,
in 1924, by an ingenious method: vary the transmitted frequency and observe the
interference between the ground wave and the sky wave, from which the path difference and
hence the reflecting layer's height follow.

**Nobel Prize, 1947**, and the technique — inferring a physical structure from interference
between two paths of the same signal — is recognisably the ancestor of §42.4's multipath
analysis.

**Augustin-Jean Fresnel (1788–1827).** The Fresnel zones of §42.3, from his work on the
wave theory of light in the 1810s and 1820s.

They are an optics result, applied to radio a century later — because both are
electromagnetic waves and the diffraction mathematics is identical. The 60% clearance rule
that determines mast heights on a microwave link comes from work on the interference of
light, done before anyone knew light was electromagnetic.

**Harald Friis (1893–1976).** The Friis transmission equation — §42.3's free-space path
loss, in its original form — at Bell Labs in 1946.

He also gave us the noise figure and much of the systematic treatment of noise in
receivers, which is what makes §42.1's SNR discussion quantitative rather than
qualitative.

**Hedy Lamarr (1914–2000) and George Antheil (1900–1959).** **Frequency-hopping spread
spectrum**, patented in 1942.

The idea was for radio-controlled torpedoes — hop the frequency in a pattern known to
both ends, so the signal cannot be jammed without jamming the whole band. Antheil's
contribution was the synchronisation mechanism, using a piano-roll design borrowed from his
work with player pianos.

**The US Navy ignored it.** The patent expired unused, and frequency hopping became
fundamental to Bluetooth (Chapter 47 §47.1), to early 802.11, and to military communications
generally.

Lamarr was a film actress, which is how the story is usually told, and the invention
is a serious one whose principle underlies a substantial fraction of modern wireless.

**Norman Abramson (1932–2020).** ALOHAnet — Chapter 16's subject — and the first wireless
packet network. The direct ancestor of everything in Chapter 44.

Gerard Foschini and Michael Gans, and separately **Emre Telatar.** The MIMO capacity
result, in the mid-1990s at Bell Labs.

**Their finding is genuinely surprising:** in a rich scattering environment, capacity
scales linearly with the number of antennas — so doubling the antennas at both ends
doubles the capacity, without more bandwidth or more power.

> This was not an incremental improvement. It said that multipath, which everyone had
> treated as an impairment to be mitigated, was an unexploited resource.

**Foschini's BLAST architecture** was the first practical scheme, and 802.11n (Chapter 44
§44.1) is its commercial descendant. The result is why Wi-Fi throughput has risen by three
orders of magnitude while the spectrum allocation has risen by one.

**Claude Shannon (1916–2001).** Chapter 4's subject, and he belongs here because §42.1's
SNR discussion is his capacity theorem applied to a radio channel. The reason raising the
noise floor destroys throughput exactly as attenuating the signal does is that capacity
depends on their ratio, which he proved in 1948.
