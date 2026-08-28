# Entropy

The last lesson measured the surprise of one message. That is a start and it is not
yet useful, because you rarely send one message.

What you want is the average — how much a symbol from this source tells you,
typically — because that number turns out to be the exact size the data can be
compressed to. Not an estimate. The exact size, with a theorem behind it.

The surprise of one message is $-\log_2 p$. The interesting quantity is the
**average** surprise of a source — how much a symbol from it tells you, on
average.

That is **entropy**:

$$H = -\sum_i p_i \log_2 p_i \quad \text{bits per symbol}$$

Each outcome's surprise, weighted by how often it occurs.

## Reading the formula

Nothing in it is complicated once the pieces are named.

$p_i$ is the probability of outcome $i$. $-\log_2 p_i$ is its surprise. The sum
weights each surprise by its probability, which is what an average is.

The minus sign is there because $\log_2 p_i$ is negative for $p < 1$, and entropy
should be positive.

Two boundary cases:

**Complete certainty.** One outcome with $p = 1$: $H = -1 \times \log_2 1 = 0$.
Zero bits per symbol, because nothing is ever learned.

**Maximum uncertainty.** $n$ equally likely outcomes, each $p = 1/n$:

$$H = -\sum_{i=1}^{n} \frac{1}{n} \log_2 \frac{1}{n} = \log_2 n$$

So eight equally likely outcomes give exactly three bits, and this is the largest
$H$ can be for $n$ outcomes. **Entropy is maximized by uniformity**, which is
another way of saying that the hardest thing to predict is the thing with no
pattern.

## Measured

Character frequencies of four strings, with $H$ computed directly:

```
"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"       H = 0.0000 bits/char
"abababababababababababababababab"       H = 1.0000 bits/char
"abcdefghabcdefghabcdefghabcdefgh"       H = 3.0000 bits/char
"the quick brown fox jumps over th..."    H = 4.3855 bits/char
```

Each of the first three is exactly $\log_2 n$ for $n$ distinct equally frequent
symbols: $\log_2 1 = 0$, $\log_2 2 = 1$, $\log_2 8 = 3$.

The fourth is a real sentence, whose letters are unequally distributed, and 4.39
is below the $\log_2 26 \approx 4.70$ that uniform letters would give.

## The biased coin

Entropy against the bias of a coin:

```
p(heads) = 0.5    H = 1.0000 bits
p(heads) = 0.9    H = 0.4690 bits
p(heads) = 0.99   H = 0.0808 bits
p(heads) = 0.999  H = 0.0114 bits
p(heads) = 1.0    H = 0.0000 bits
```

A fair coin gives the full bit. A coin landing heads 90% of the time gives less
than half a bit per flip — because you can usually guess, and being right teaches
you nothing.

At $p = 0.999$, a thousand flips carry about 11 bits in total. You could transmit
them in two bytes, and Section 33.2.1 is about how.

The shape of that curve is the whole practical content of the chapter:
**predictable data is cheap to transmit, and unpredictable data is not.**

## The source coding theorem

Shannon's central result, and it is what makes entropy more than a definition.

> A source of entropy $H$ bits per symbol can be encoded in $H$ bits per symbol on
> average, and no encoding does better.

Two halves, both surprising.

**You cannot do better.** Any code averaging fewer than $H$ bits per symbol must
lose information. This is a lower bound of the kind Section 32.2.1 gave for
sorting, and it holds against every encoding that could ever be devised.

**You can get arbitrarily close.** Entropy is not merely a bound; it is
achievable. Section 33.2.1's Huffman coding reaches it exactly when the
probabilities are powers of two, and arithmetic coding reaches it in general.

So $H$ is not an estimate. **It is the exact size of the data**, and any file
larger than $H$ bits per symbol contains redundancy that some encoding could
remove.

## Entropy of English

Estimating it depends on how much context you use, and the progression is
instructive.

**Letters alone**, ignoring frequencies: $\log_2 26 \approx 4.70$ bits.

**With letter frequencies** — `e` common, `z` rare: about 4.1 bits.

**With pairs** — `q` is nearly always followed by `u`, `th` is common: about 3.5.

**With longer context**: about 2.

**Shannon's own estimate**, from experiments in which people guessed the next
letter of a text: **about 1 bit per character.**

That last number is worth sitting with. English prose contains roughly one bit of
information per character, and is normally stored at eight. Which predicts that
English text should compress to something like an eighth of its size, and
general-purpose compressors reach roughly that.

It also explains why you can read text with the vowels removed, why autocomplete
works, and why a language model can predict the next word. All three are exploiting
the same redundancy.

## Conditional entropy

The reason the estimates fell as context grew has a name.

$H(X)$ is the entropy of a symbol alone. $H(X \mid Y)$ is its entropy *given* that
you know $Y$ — the surprise remaining after the context is accounted for.

Knowing more never hurts:

$$H(X \mid Y) \le H(X)$$

with equality only when $Y$ tells you nothing about $X$.

That is why a compressor using context beats one that does not, and it is why the
per-character entropy of English falls from 4.7 to about 1 as the context
lengthens. The information was always about that low; the earlier estimates were
just not looking at enough.

This is also, in one line, the principle behind every modern predictive model. A
system that predicts the next symbol well has a low conditional entropy for that
symbol, and a system with low conditional entropy is a good compressor. Prediction
and compression are the same problem.

Next: doing something with all this.
