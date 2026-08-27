# Important Concepts

**Information is the reduction of uncertainty** — a message carries information in
proportion to how surprising it is. So it is a property of a message *and what the
receiver already believed*, not of the message alone.

**The measure** — an event of probability $p$ carries $-\log_2 p$ bits. Certainty
is 0 bits, a fair coin is 1, a fair die is 2.58, a one-in-a-thousand event is
about 10.

**Why the logarithm** — information from independent events must add, and their
probabilities multiply, so the measure must turn multiplication into addition. Base
2 makes the unit a bit.

**Yes-or-no questions** — the information in a message is the number of well-chosen
binary questions needed to determine it. Which makes binary search's $\log_2 n$
steps an information bound as well as a complexity bound.

**Unequal probabilities** — `e` carries about 3.06 bits and `z` about 10.5, so
spending eight bits on each is waste. Chapter 1's claim about fixed-width
encodings, quantified.

**Shannon information is not meaning** — a million random characters carry more
than a million characters of Shakespeare, because they are less predictable. The
definition is about what a channel must carry.

**Entropy** — $H = -\sum p_i \log_2 p_i$, the average surprise per symbol. Zero for
a certain source; maximized at $\log_2 n$ when $n$ outcomes are equally likely.

**Measured entropies** — 0.0000 for a repeated character, 1.0000 for two
alternating, 3.0000 for eight equally frequent, 4.3855 for an English sentence.
Each of the first three is exactly $\log_2 n$.

**The biased coin** — 1.0000 bits at $p = 0.5$, 0.4690 at $p = 0.9$, 0.0114 at
$p = 0.999$. Predictable data is cheap; unpredictable data is not.

**The source coding theorem** — a source of entropy $H$ can be encoded in $H$ bits
per symbol and no better. Both halves matter: it is a hard lower bound *and* it is
achievable, so $H$ is the exact size of the data.

**Entropy of English** — about 4.7 bits per letter ignoring frequency, 4.1 with
frequencies, 3.5 with pairs, and about 1 with real context, which was Shannon's own
estimate from guessing experiments. Stored at 8.

**Conditional entropy** — $H(X \mid Y) \le H(X)$; knowing more never hurts. Why
context-using compressors win, and why prediction and compression are the same
problem.

**Variable-length codes** — short codes for frequent symbols. Morse did it in 1838
by counting type in a printer's case.

**The prefix property** — no code word is a prefix of another, so a decoder needs
no lookahead and no separators. Equivalent to putting symbols at the leaves of a
binary tree. UTF-8 is a prefix code, which is why it resynchronizes after damage.

**Huffman's algorithm** — repeatedly join the two least frequent nodes under a new
parent. Optimal among integer-length codes, and four lines long.

**Huffman hits the entropy exactly when probabilities are powers of two** —
verified at 1.7500 bits per character both ways, for a string of 8 `a`, 4 `b`, 2
`c`, 2 `d`. Otherwise it is within one bit per symbol, and arithmetic coding closes
the gap.

**Real compressors add structure** — run-length encoding for repetition, and
dictionary compression (LZ77) for repeated sequences. `gzip` is LZ77 followed by
Huffman.

**Measured compression** — 9,000 bytes to 44 for a repeated character, to 108 for
repeated English, to 5,636 for random letters. The last is close to the $4.70/8$
its entropy predicts; the middle beats its per-character entropy because LZ77 sees
repetition that per-character entropy cannot.

**Lossy compression** discards information deliberately and is not bound by the
source coding theorem, because it sends a different, simpler message. Which is why
re-encoding a JPEG repeatedly degrades it.

**No compressor shrinks every input** — $2^n$ inputs cannot map one-to-one into the
$2^n - 1$ strings of length under $n$. So for every lossless compressor, some
inputs must grow. Verified: 200 of 200 random inputs grew.

**Every real compressor has a fallback** — store the block uncompressed with a flag
saying so. That flag is the unavoidable growth, minimized.

**Compressing twice is worse than once** — verified at 9,000 to 108 to 109 to 130.
The first pass removed the structure and the later passes only add headers.

**An impossibility result lets you dismiss a class of claim without inspecting
it** — you need not find the flaw in a universal-compression scheme; you know one
is there.

**The counting technique** — count what exists, count what the mechanism can
distinguish, observe the second is smaller. The sorting lower bound, the
compression bound, and Chapter 34's results are all this argument.

**Almost every string is incompressible** — fewer than one $n$-bit string in 512
can be shrunk by ten bits. Compression works only because the files we care about
occupy a vanishingly small, highly structured corner of the space.
