# Chapter 9 — The People

**Émile Baudot (1845–1903).** French telegraph engineer whose 1874 system
time-division multiplexed several operators onto one telegraph line — the first
practical TDM, and the direct ancestor of everything in §9.2. His apparatus used a
rotating distributor that gave each operator the line in turn, with a mechanical
timing arrangement that had to keep both ends in step; the synchronisation problem
that §9.2 identifies as TDM's defining requirement was his to solve first. He
worked for the French postal telegraph service for most of his career and received
very little of the money his patents generated. His five-bit code appears in
Chapter 2 and his name in the baud of Chapter 4.

**Alexander Graham Bell (1847–1922).** Included here for what he was *trying* to
build. Western Union was offering serious money for a **harmonic telegraph** — a
frequency-division multiplexer using tuned reeds, so that several telegraph
messages could share one wire — because the cost of telegraphy was overwhelmingly
the cost of the wire. Bell's harmonic telegraph worked poorly. In the course of
building it, on 2 June 1875, he and Thomas Watson noticed that the apparatus was
transmitting not merely the presence of a tone but its *shape*, and within a year
that observation had become the telephone. The multiplexing problem he abandoned
was solved by others; the accident he found on the way to it reshaped the world.

**Agner Krarup Erlang (1878–1929).** Danish mathematician at the Copenhagen
Telephone Company, who between 1909 and 1917 founded queueing theory in order to
answer his employer's actual question: how many circuits does an exchange need so
that calls are rarely blocked? His answer — that the number is far fewer than one
per subscriber, and computable — is the statistical multiplexing argument of §9.3
made seventy years before packet switching, for a different resource. Chapter 12
§12.4 uses his blocking formula directly. The unit of offered traffic bears his
name.

**Bernard Oliver (1916–1995), John Pierce (1910–2002) and Claude Shannon
(1916–2001).** Their 1948 paper *The Philosophy of PCM* laid out the case for
digital transmission of voice — sampling, quantising, and the regeneration argument
of Chapter 5 §5.1 — which is what made §9.2's DS0 and the whole digital hierarchy
possible. Pierce also named the transistor and championed communications satellites;
Oliver founded HP Labs and later ran NASA's SETI programme.

**Hedy Lamarr (1914–2000) and George Antheil (1900–1959).** US Patent 2,292,387,
granted 1942, for a frequency-hopping "Secret Communication System" intended to
prevent radio-controlled torpedoes being jammed. Lamarr — an Austrian-born film
actress who had absorbed a great deal about armaments from her first marriage —
supplied the communications concept; Antheil, an avant-garde composer who had
synchronised sixteen player pianos for his *Ballet Mécanique*, supplied a
synchronisation mechanism using punched paper rolls on exactly that principle.

The Navy declined it. The patent expired unused. Lamarr received essentially no
recognition until the 1990s, and was inducted into the National Inventors Hall of
Fame in 2014, fourteen years after her death.

The story is often told with the claim that she "invented Wi-Fi", which is not
accurate: frequency hopping is one spread-spectrum technique and direct-sequence
CDMA is another, and the patent was one of several contemporaneous ideas. What is
genuinely remarkable, and sufficient, is that two people entirely outside the field
produced a sound, well-specified design that the professionals ignored for fifty
years.

**Andrew Viterbi (b. 1935) and Irwin Jacobs (b. 1933).** Co-founders of Qualcomm,
who took CDMA from a military technique to a commercial cellular standard against
substantial industry scepticism — the prevailing view in the late 1980s being that
CDMA's capacity claims were physically impossible. IS-95 proved otherwise and
became the basis of 3G. Viterbi's decoding algorithm appears in Chapters 4 and 7;
his and Jacobs's work on power control is what made §9.4's near-far problem
tractable.

**David Payne (b. 1944).** British physicist at the University of Southampton whose
group demonstrated the **erbium-doped fibre amplifier** in 1987, independently of
parallel work at Bell Labs. Without optical amplification, DWDM would require one
regenerator per wavelength per span, and the economics of §9.4 would not exist. It
is difficult to overstate how much of the modern Internet's cost structure rests on
this one device, and it is a rare case of a university group producing a technology
that immediately reshaped an industry.

**Emmanuel Desurvire (b. 1955).** Led the parallel EDFA work at Bell Labs and did
much of the theoretical characterisation — gain spectra, noise figure, saturation
behaviour — that turned the demonstration into an engineerable component. The gain
transients that §9.4's "what breaks here" mentions are described in his work.

**Charles Kao (1933–2018).** The fibre itself, without which none of §9.4 applies.
See Chapters 6 and 10.
