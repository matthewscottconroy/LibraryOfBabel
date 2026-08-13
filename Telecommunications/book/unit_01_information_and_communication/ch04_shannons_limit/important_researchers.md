# Chapter 4 — The People

**Claude Shannon (1916–2001).** The capacity theorem is the second half of the 1948
paper. Its proof is non-constructive — it shows that good codes exist by
demonstrating that a randomly chosen code is almost certainly good — which meant
that for fifty years engineers knew a target existed without knowing how to reach
it. Shannon also wrote the 1949 paper *Communication Theory of Secrecy Systems*,
which did for cryptography what the 1948 paper did for communication, and which
proved that the one-time pad is unbreakable and that nothing shorter can be. See
Chapters 1 and 2 for the fuller biography.

**Harry Nyquist (1889–1976).** Emigrated from Sweden to the United States at
eighteen, worked at AT&T and then Bell Labs from 1917 to 1954. Three results in
this chapter are his: the 1924 bandwidth–symbol-rate limit, the 1928 sampling
theorem, and the 1928 theoretical explanation of Johnson's measured thermal noise.
He also gave control theory the Nyquist stability criterion. He held 138 patents
and, by the account of colleagues, said very little in meetings.

**John B. Johnson (1887–1970).** Swedish-born physicist at Bell Labs who in 1926
measured the tiny fluctuating voltage present across any resistor and showed it was
proportional to temperature and resistance and independent of the material. Nyquist
explained it theoretically the following year. The phenomenon is called
Johnson–Nyquist noise, and it is the floor beneath every receiver in this book.

**Ralph Hartley (1888–1970).** The "Hartley" in Shannon–Hartley. His 1928
logarithmic measure of information and his analysis of the bandwidth–information
relationship are the direct ancestors of the capacity formula. He also invented the
Hartley oscillator, which is in a great deal of radio equipment.

**David A. Huffman (1925–1999).** As an MIT graduate student in 1951, offered the
choice between a final exam and a term paper on finding the most efficient binary
code, he chose the paper, and — after nearly giving up — found the optimal
algorithm. Huffman coding is in JPEG, MP3, DEFLATE, and therefore in essentially
every compressed file in existence. He later founded the computer science
department at UC Santa Cruz. He was also a serious mathematical origami
practitioner.

**Richard Hamming (1915–1998).** Bell Labs mathematician, frustrated by weekend
batch jobs failing on a single-bit error with no way to correct it, who in 1950
published the first error-*correcting* codes — the first constructive step toward
Shannon's promise. His 1986 talk "You and Your Research" is widely read and worth
the twenty minutes. Chapter 15's error detection is the simpler cousin of his work.

**Claude Berrou (b. 1951) and Alain Glavieux (1949–2004).** French engineers at
ENST Bretagne whose 1993 paper introduced **turbo codes**, achieving performance
within 0.5 dB of the Shannon limit — a result so far ahead of the field's
expectations that the initial reaction from reviewers was that it must be wrong.
It was not. Turbo codes went into 3G and deep-space communication and ended a
fifty-year gap.

**Robert Gallager (b. 1931).** MIT professor who invented **low-density
parity-check (LDPC) codes** in his 1960 doctoral thesis, at which point the
computation they required was entirely impractical. They were forgotten for
thirty-five years, rediscovered by David MacKay and Radford Neal in the mid-1990s
when hardware had caught up, and are now in Wi-Fi 6, 5G, DVB-S2, and 10GBASE-T.
It is one of the field's better stories about the value of publishing work whose
time has not yet come.

**Erdal Arıkan (b. 1958).** Turkish engineer at Bilkent University whose 2008 work
on **polar codes** produced the first family of codes with a mathematical proof of
achieving capacity, with practical encoding and decoding complexity. Adopted for
5G control channels in 2016, eight years after publication — a very short interval
by this field's standards.

**Andrew Viterbi (b. 1935).** Italian-American engineer, co-founder of Qualcomm,
whose 1967 algorithm for decoding convolutional codes is used in essentially every
digital communication system built since, and whose work on CDMA underlies
Chapter 46. The Viterbi algorithm also turns up, unchanged, in speech recognition
and bioinformatics.
