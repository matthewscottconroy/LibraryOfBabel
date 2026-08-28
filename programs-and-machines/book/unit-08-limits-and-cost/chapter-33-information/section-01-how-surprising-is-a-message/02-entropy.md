# Entropy

You can now measure the surprise of a single message. That is a start, and on its
own it is not much use, because you almost never send one message.

What you actually want is the average — how much a symbol from some source tells
you, typically. And that number turns out to be worth far more than an average
usually is, because it is *the exact size the data can be compressed to*. Not a
rule of thumb. Not a good estimate. The exact size, with a theorem standing behind
it.

The surprise of one message was $-\log_2 p$. Weight each outcome's surprise by how
often that outcome happens, add them up, and you have **entropy**:

$$H = -\sum_i p_i \log_2 p_i \quad \text{bits per symbol}$$

## Taking the formula apart

Nothing in there is difficult once the pieces have names.

$p_i$ is the probability of outcome $i$. $-\log_2 p_i$ is that outcome's surprise.
The sum weights each surprise by how often it occurs, which is exactly what taking
an average means.

The minus sign is bookkeeping. $\log_2 p_i$ is negative whenever $p < 1$, and we
would like entropy to come out positive.

Now try the two extreme cases yourself before reading them, because they are the
quickest way to feel whether the definition is the right one.

**Complete certainty.** One outcome, $p = 1$. Then $H = -1 \times \log_2 1 = 0$.
Zero bits per symbol. A source that always says the same thing tells you nothing,
ever, and the formula agrees.

**Maximum uncertainty.** $n$ outcomes, all equally likely, each with $p = 1/n$:

$$H = -\sum_{i=1}^{n} \frac{1}{n} \log_2 \frac{1}{n} = \log_2 n$$

Eight equally likely outcomes give you exactly three bits. And this is the largest
$H$ can possibly be for $n$ outcomes — **entropy is maximized by uniformity**,
which is a formal way of saying that the hardest thing to predict is the thing with
no pattern in it.

## Measured

Here are the character frequencies of four strings with $H$ computed straight from
the definition. Look at the first three and see whether you can predict the numbers
before you read them.

```
"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"       H = 0.0000 bits/char
"abababababababababababababababab"       H = 1.0000 bits/char
"abcdefghabcdefghabcdefghabcdefgh"       H = 3.0000 bits/char
"the quick brown fox jumps over th..."    H = 4.3855 bits/char
```

Each of the first three is exactly $\log_2 n$ for $n$ equally frequent symbols:
$\log_2 1 = 0$, $\log_2 2 = 1$, $\log_2 8 = 3$. Clean, and reassuring.

The fourth is a real sentence. Its letters are unevenly distributed, and 4.39 comes
in below the $\log_2 26 \approx 4.70$ that perfectly uniform letters would have
given. English is already leaking its predictability into the measurement, and we
have not even started looking for patterns yet.

## Watch a coin get less interesting

Entropy plotted against the bias of a coin:

```
p(heads) = 0.5    H = 1.0000 bits
p(heads) = 0.9    H = 0.4690 bits
p(heads) = 0.99   H = 0.0808 bits
p(heads) = 0.999  H = 0.0114 bits
p(heads) = 1.0    H = 0.0000 bits
```

A fair coin hands over the full bit, every flip. A coin that comes up heads nine
times in ten gives you *less than half a bit* — because you can usually guess it,
and being right teaches you nothing at all.

Go down to $p = 0.999$ and a thousand flips carry about eleven bits between them.
A thousand flips. Eleven bits. You could put the whole sequence in two bytes and
have room to spare, and the next section is about how.

That curve is the entire practical content of this chapter, in one shape:
**predictable data is cheap to send, and unpredictable data is not.**

## The theorem that makes this more than a definition

Here is Shannon's central result. It is short, and both halves of it are
surprising.

> A source of entropy $H$ bits per symbol can be encoded in $H$ bits per symbol on
> average, and no encoding does better.

**You cannot do better.** Any code averaging fewer than $H$ bits per symbol must
be losing information — not might be, must be. This is a lower bound of the same
species as the sorting bound in Section 32.2.1, and it holds against every encoding
scheme that has ever been invented or ever will be. Somebody arriving with a
brilliant new compressor is not exempt.

**You can get arbitrarily close.** Entropy is not only a wall; it is a wall you can
lean on. Huffman coding hits it exactly when the probabilities happen to be powers
of two, and arithmetic coding gets there in general.

Put those together and $H$ stops being an estimate of anything. **It is the size of
the data.** Any file sitting at more than $H$ bits per symbol is carrying
redundancy that some encoding could take away.

## How much information is in English?

The answer depends on how much context you allow yourself, and the progression is
the interesting part. Watch the number fall.

**Letters alone**, pretending all 26 are equally likely: $\log_2 26 \approx 4.70$
bits.

**Letter frequencies included** — `e` is everywhere, `z` is rare: about 4.1 bits.

**Pairs of letters** — `q` is essentially always followed by `u`, `th` turns up
constantly: about 3.5.

**Longer stretches of context**: about 2.

**Shannon's own estimate**, which he got by sitting people down and having them
guess the next letter of a text one character at a time: **about 1 bit per
character.**

Stop at that last number for a moment.

English prose carries roughly one bit of information per character. We store it at
eight. Which is a prediction, and a testable one — English text ought to compress
to something like an eighth of its size — and general-purpose compressors land
right about there.

The same number explains a few things you already knew without knowing why. It is
why you can rd ths sntnc wth th vwls tkn t. It is why autocomplete works. It is
why a language model can guess your next word. All three are living off the same
redundancy, and Shannon measured it in 1951 by asking people to play a guessing
game.

## Why the number kept falling

That progression from 4.7 down to 1 has a name attached to it.

$H(X)$ is the entropy of a symbol on its own. $H(X \mid Y)$ is its entropy *given
that you already know* $Y$ — how much surprise survives once the context has been
accounted for.

And knowing things never hurts:

$$H(X \mid Y) \le H(X)$$

with equality only in the case where $Y$ tells you nothing whatsoever about $X$.

So a compressor that uses context beats one that does not, necessarily, as a matter
of arithmetic. And the per-character entropy of English falls as the context grows
because the information was always down around one bit. The early estimates were
not wrong about English. They were just not looking at enough of it.

There is a closing thought here that reaches a long way past this chapter. A system
that predicts the next symbol well is a system with low conditional entropy for
that symbol. And a system with low conditional entropy is, by everything above, a
good compressor.

Prediction and compression are the same problem wearing two hats.

Next: doing something with all of this.
