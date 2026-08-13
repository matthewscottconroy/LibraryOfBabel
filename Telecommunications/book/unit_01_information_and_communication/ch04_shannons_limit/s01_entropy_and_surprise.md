# 4.1 Entropy and Surprise

Chapter 2 established that identifying one of *n* equally likely possibilities
costs log₂ *n* bits, and that a single outcome of probability *p* carries
−log₂ *p* bits. Now we take the average, and the average has a name.

## The definition

For a source emitting symbols from an alphabet where symbol *i* has probability
*pᵢ*, the **entropy** is

$$H = -\sum_i p_i \log_2 p_i \quad \text{bits per symbol}$$

That is: the average, over all symbols, of each symbol's self-information,
weighted by how often it occurs. Nothing more mysterious than a weighted mean.

Shannon named it entropy on the advice of John von Neumann, who — according to an
account Shannon gave years later — told him: *"You should call it entropy, for two
reasons. In the first place your uncertainty function has been used in statistical
mechanics under that name, so it already has a name. In the second place, and
more important, no one knows what entropy really is, so in a debate you will
always have the advantage."* The anecdote is probably improved in the telling. The
mathematical parallel with Boltzmann's thermodynamic entropy is genuine and deep.

## Working the arithmetic

**A fair coin.** Two outcomes, each *p* = 0.5.

$$H = -(0.5 \log_2 0.5 + 0.5 \log_2 0.5) = -(0.5 \times -1 + 0.5 \times -1) = 1 \ \text{bit}$$

One bit per flip. Exactly what intuition demands, and the reason the unit is
calibrated this way.

**A biased coin, 90/10.**

$$H = -(0.9 \log_2 0.9 + 0.1 \log_2 0.1) = -(0.9 \times -0.152 + 0.1 \times -3.322) = 0.469 \ \text{bits}$$

Less than half a bit. You already mostly know what it will say, so observing it
tells you little. Note the shape of this: entropy is *maximised* when outcomes are
equally likely and falls as the distribution becomes skewed.

**A completely biased coin, 100/0.** *H* = 0. A source that always says the same
thing conveys nothing. This is not a paradox; it is the definition working
correctly.

**The sensor from Chapter 2** with probabilities 0.90, 0.06, 0.02, 0.015, 0.005:

$$H = -(0.9 \log_2 0.9 + 0.06 \log_2 0.06 + 0.02 \log_2 0.02 + 0.015 \log_2 0.015 + 0.005 \log_2 0.005)$$

$$= 0.137 + 0.244 + 0.113 + 0.091 + 0.038 = 0.623 \ \text{bits per report}$$

A fixed-length code needs ⌈log₂ 5⌉ = 3 bits per report. The entropy is 0.623. We
are using nearly five times the necessary capacity.

## The source coding theorem

Shannon's first main theorem states the consequence precisely:

> A source of entropy *H* bits per symbol can be encoded, without loss, using an
> average of *H* bits per symbol — and cannot be encoded in fewer.

Both directions matter. The first is an existence promise: however wasteful your
current encoding, a better one exists, and its target is *H*. The second is a
prohibition: below *H*, lossless compression is impossible, and any product
claiming otherwise is either lying or not lossless.

This is the theorem that makes the recurring claim "our algorithm compresses any
file by 50%" provably false. Apply it twice and you compress any file to a single
bit, which cannot represent two distinguishable files. The counting argument is
elementary and the confidence with which such claims are still made periodically
is a permanent feature of the industry.

**Huffman coding** (David Huffman, 1952, as a term-paper alternative to a final
exam) constructs an optimal prefix-free code and gets within one bit of *H*.
**Arithmetic coding** essentially reaches *H*. Everything in the DEFLATE, gzip,
Brotli and Zstandard family is a descendant.

## Where the real gains hide: context

The naive calculation above treats each symbol as independent. Real sources are
not.

English text has a per-character entropy of about 4.1 bits if you treat characters
as independent with their observed frequencies. Shannon estimated the *actual*
entropy of English, accounting for context, at roughly **0.6 to 1.3 bits per
character** — a figure he obtained in a 1951 paper by an ingenious experiment:
asking human subjects to guess the next letter of a text and recording how many
guesses they needed.

The gap is enormous and it comes entirely from **conditional structure**. After
`q`, the next letter is `u` with probability near 1, so it carries almost no
information. After `th`, `e` is overwhelmingly likely. Modern compressors exploit
exactly this by modelling context, which is why a text file compresses to about
25% of its size while a file of random bytes does not compress at all — the random
file already has maximum entropy, and there is nothing to remove.

This has a direct networking consequence worth flagging: **already-compressed data
cannot be compressed again.** Enabling compression on a WAN accelerator for traffic
that is already JPEG, MP4, or TLS-encrypted spends CPU and gains nothing. Encrypted
data in particular is designed to be statistically indistinguishable from random,
which means maximum entropy, which means incompressible — and it is why compression
must always be applied *before* encryption, never after. (That ordering has its own
hazard: the CRIME and BREACH attacks of 2012–2013 exploited compression-before-
encryption to extract secrets by observing ciphertext length. Chapter 58 returns
to it.)

## Entropy in the rest of the book

You will meet this quantity again in four places:

- **Chapter 7**, where line codes deliberately *add* redundancy — reducing
  information per symbol below the maximum — to buy clock recovery and DC balance.
- **Chapter 15**, where the CRC adds 32 bits of pure redundancy to detect errors.
- **Chapter 58**, where cryptographic key material must have *full* entropy, and
  where the failure of random number generators to provide it has broken real
  systems (notably the 2008 Debian OpenSSL bug, which reduced the key space to
  32,767 possibilities).
- **Chapter 54**, where entropy-based measures are used to detect anomalies in
  traffic flows.

Redundancy is not waste. It is what you spend to buy reliability, and the whole of
coding theory is the study of spending it efficiently.

> **Network+ note.** Entropy is not on N10-009. The practical consequences are:
> that compression is not universally beneficial, that encrypted traffic does not
> compress, and that "lossless compression ratios" quoted by WAN optimisation
> vendors depend entirely on the traffic mix. Those three points are worth more in
> a procurement meeting than the formula.
