# Important Researchers

**Claude Shannon** (1916–2001) created this chapter's subject in a single 1948
paper, *A Mathematical Theory of Communication*, and it is one of the few genuine
cases of a field appearing at once with no real antecedents. Before it, nobody
thought "how much information is in a message" had a numerical answer; afterwards,
the answer was exact, the unit was named, and the limits of every communication
channel could be computed.

His master's thesis a decade earlier, written at twenty-one, showed that Boolean
algebra describes switching circuits — which is Chapter 8's entire foundation and
has been called the most important master's thesis ever written. He did both of
the things this book rests on, ten years apart, and then largely lost interest and
built juggling machines and a mechanical mouse that solved mazes.

Shannon also carried out the guessing experiment behind Section 33.1.2's estimate
that English is about one bit per character, using his wife Betty as a subject and
a text she had not read.

**David Huffman** (1925–1999) was a graduate student in 1951 when his professor,
Robert Fano, offered a choice between a final exam and a term paper on finding the
most efficient binary code. Huffman took the paper, worked at it for months,
nearly gave up, and then found the bottom-up greedy construction — which turned
out to be optimal, which neither Fano nor Shannon had managed with their own
top-down approaches. He said later he would never have attempted it had he known
they had tried and failed.

**Abraham Lempel** (1936–2023) and **Jacob Ziv** (1931–2023) published LZ77 and
LZ78 in 1977 and 1978, introducing dictionary compression: replace a repeated
sequence with a reference to its earlier occurrence. Their algorithms are the basis
of `gzip`, ZIP, PNG, and essentially every general-purpose compressor since, and
they are what allowed the English sample in Section 33.2.1 to beat its own
per-character entropy.

**Samuel Morse** (1791–1872) and **Alfred Vail** (1807–1859) built the first
variable-length code, and the method is the reason it worked: Vail visited a
printer's shop and counted how many pieces of each letter were in the type case,
using that as a frequency table. It is a century ahead of the theory that explains
why it was right, and a good illustration that engineering intuition can arrive
first.

**Robert Fano** (1917–2016) developed Shannon-Fano coding, the top-down
predecessor Huffman improved on, and taught the course that produced Huffman's
result. He later co-directed MIT's Project MAC, out of which came time-sharing and
much of what became modern computing.

**Andrey Kolmogorov** (1903–1987) appears here as a forward reference. His
alternative definition of information — the length of the shortest program that
produces a string — needs no probabilities and is a property of the string itself.
Chapter 34 takes it up, along with the reason it cannot be computed.
