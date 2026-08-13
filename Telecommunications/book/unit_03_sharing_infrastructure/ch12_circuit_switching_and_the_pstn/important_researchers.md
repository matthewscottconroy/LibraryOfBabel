# Chapter 12 — The People

**Almon Brown Strowger (1839–1902).** Kansas City undertaker, Civil War veteran,
and — improbably — the inventor of automatic telephone switching. Convinced that the
local operator was diverting his calls to a competitor, he built a working model
reportedly using a collar box and pins, patented it in 1891, and founded a company
to manufacture it. The first exchange opened in La Porte, Indiana, in 1892 with
about eighty subscribers.

He sold his patents for a modest sum in 1896 and died before the technology's
significance was apparent. The Automatic Electric Company he founded became a major
manufacturer; the switch bearing his name ran telephone networks for a century. It
is the best example in this book of consequential engineering produced by someone
with no relevant training and a specific grievance.

**Agner Krarup Erlang (1878–1929).** Danish mathematician who joined the Copenhagen
Telephone Company in 1908 and spent the rest of his career there. Asked how many
circuits an exchange required, he found that no mathematics existed to answer it and
built the mathematics — publishing *The Theory of Probabilities and Telephone
Conversations* in 1909 and the blocking formula in 1917.

He founded queueing theory as a working engineer answering a working question,
which is a pattern this book returns to. He was reportedly a solitary and modest
man, worked alone, published little, and gave his results away without patent. The
unit of offered traffic bears his name, as do two formulas and a programming
language.

**Harry Nyquist (1889–1976).** The sampling theorem that determines §12.2's 8 kHz.
See Chapters 1, 4 and 5.

**Bernard Oliver (1916–1995), John Pierce (1910–2002) and Claude Shannon
(1916–2001).** Their 1948 paper *The Philosophy of PCM* made the case for digital
voice transmission — sampling, quantising, and the regeneration argument — before
any system existed to do it. §12.2's T1 is its implementation fourteen years later.

**Alec Reeves (1902–1971).** British engineer who patented **pulse code modulation**
in 1938, while working for International Telephone and Telegraph in Paris — ten
years before the Bell Labs paper and thirty years before it was practical. The
electronics of 1938 could not implement it economically; vacuum tubes made a PCM
encoder enormous and unreliable. He is a clear case of an idea arriving before the
technology to execute it, and he lived long enough to see it become the basis of the
world's telephone network. He also worked on radio navigation systems during the
Second World War and, later and more eccentrically, on parapsychology.

**John Draper (b. 1943), "Captain Crunch".** Discovered that a toy whistle
distributed in Cap'n Crunch cereal produced a clean 2,600 Hz tone. He did not
originate phreaking — blind teenagers with perfect pitch had found the tone earlier,
and Joybubbles (Josef Engressia) is usually credited first — but he popularised it
and was prosecuted for it. His subsequent career included writing EasyWriter, an
early word processor for the IBM PC, reportedly while under a work-release
arrangement.

**Steve Wozniak (b. 1950) and Steve Jobs (b. 1955–2011).** Built and sold blue boxes
before founding Apple, an episode both discussed publicly and unapologetically.
Wozniak has said the experience convinced him that two people in a bedroom could
build something that affected large systems — which is a reasonable summary of what
they did next.

**The Bell Labs SS7 architects (1970s).** SS7's separation of signalling from media
was a security response to phreaking and an efficiency response to trunk holding
during setup. The architecture they produced — a separate packet network for control,
with databases consulted during call setup — is the one SIP reimplemented twenty
years later, and it is the direct ancestor of the "intelligent network" and of
everything that requires a lookup while a call is being established.

**Karsten Nohl (b. 1981).** German security researcher whose public demonstrations
from 2014 onward showed SS7 interception of calls and SMS in production networks,
including a live demonstration for a US congressman's phone. His work is the reason
SS7's security assumption became widely understood outside the industry, and a
substantial part of why SMS-based two-factor authentication is now regarded as weak.
He also did the significant early work on GSM encryption weaknesses.

**Tobias Engel.** Presented SS7 location-tracking attacks at the Chaos Communication
Congress in 2008 and again in 2014, some years before the industry treated the
problem as urgent. The gap between demonstration and response is itself instructive
about how infrastructure security actually improves.
