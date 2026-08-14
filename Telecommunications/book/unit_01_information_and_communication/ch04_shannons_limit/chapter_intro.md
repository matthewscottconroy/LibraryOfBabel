# Chapter 4 — Shannon's Limit

Most engineering disciplines do not get to know when they are finished.

A bridge designer can always find a stronger alloy. An engine designer can always
improve the combustion chamber. There is no theorem that says *this is the best
bridge that can ever exist*, and so the work continues indefinitely, and nobody
can tell in advance whether a proposed improvement is achievable or absurd.

Communications engineering is different, and the difference dates to 1948. In the
second half of his paper, Claude Shannon proved a theorem stating that every
channel has a number attached to it — a capacity, in bits per second — and that:

1. It is possible to transmit at any rate below that capacity with an error rate
   as small as you like, no matter how noisy the channel is.
2. It is not possible to transmit above it. Ever. By any means.

Both halves of that are astonishing, and the first half is the more astonishing
one. Before Shannon, the universal assumption was that noise degraded
communication *gradually*: that to halve your errors you would have to halve your
rate, or double your power, or repeat every message twice, and that perfectly
reliable communication over a noisy channel was simply impossible in the way that
perpetual motion is impossible. Shannon proved that this is false. Arbitrarily
reliable communication over an arbitrarily noisy channel is achievable, provided
only that you stay below the capacity, and provided you are willing to encode over
long enough blocks.

He did not, however, say how. The proof is non-constructive: it demonstrates that
good codes exist by showing that a *randomly chosen* code is almost certainly good,
without exhibiting one you could actually build. The subsequent seventy-eight years
of coding theory — Hamming codes, Reed–Solomon, convolutional codes, turbo codes,
LDPC, polar codes — have been an extended effort to find practical codes that get
close to a bound whose existence was proved before any of them were invented.

They got there. Modern LDPC codes in Wi-Fi 6 and 5G operate within a fraction of a
decibel of the Shannon limit. The gap that consumed a field for half a century has
essentially been closed, which is a rare and satisfying thing to be able to say
about an engineering problem.

## Why a networking student needs this

You might reasonably ask why a chapter of information theory belongs in a book
that is mostly about switches and subnets. Three reasons.

**It tells you what is possible.** When a vendor claims a new modulation scheme
will double throughput on your existing cable plant, you can check. When someone
proposes running 10 Gb/s over 200 metres of Cat5e, you can compute why it will not
work rather than merely suspecting it. Shannon's formula turns marketing into
arithmetic.

**It explains the shape of every technology in Units II and IX.** Why does Wi-Fi
drop to a lower data rate as you walk away from the access point? Because SNR
falls with distance and capacity falls with SNR, and the radio is tracking the
limit downward. Why did the 56 kb/s modem stop there and not at 64? Because of a
specific quantisation detail in the telephone network's digital core, sitting
right against a Shannon bound. Why does a fibre span need an amplifier every
80 km? Because the SNR budget runs out.

**It is the origin of the decibel discipline** that every physical-layer document
in the field is written in. Learning to think in dB is the most transferable
skill in this chapter, and §4.3 builds it from nothing.

## What this chapter does

§4.1 develops **entropy** — the average information per symbol — from Chapter 2's
notion of information as resolved choice, and shows what it implies about
compression.

§4.2 derives **Nyquist's** result: a channel of bandwidth *B* can carry at most
2*B* symbols per second without symbols smearing into one another. This is the
*bandwidth* half of the ceiling and it says nothing about noise.

§4.3 builds the **decibel** properly, defines signal-to-noise ratio, and computes
the thermal noise floor from physical constants — the number that determines what
every receiver on Earth can hear.

§4.4 assembles them into the **Shannon–Hartley theorem** and applies it to real
channels: a telephone line, a Wi-Fi channel, a fibre span, and a deep-space link.

## By the end you will be able to

- Compute the entropy of a source and use it to bound achievable compression.
- Apply Nyquist's limit to determine the maximum symbol rate of a given bandwidth,
  and combine it with bits-per-symbol to get a data rate.
- Convert fluently between linear ratios and decibels, and compute a noise floor
  from bandwidth and temperature.
- Compute the Shannon capacity of a channel from its bandwidth and SNR, and use it
  to judge whether a claimed data rate is plausible.
- Explain why raising transmit power gives diminishing returns, quantitatively —
  finishing the argument begun with Whitehouse in Chapter 1.

## Where this sits in the argument

This chapter closes Unit I. It takes the vague word *channel* from Chapter 1 and
attaches a number to it; that number is the ceiling under which every technology
in Unit II operates and every wireless decision in Unit IX is made.

It is the most mathematical chapter in the book. Nothing after it requires more
than the results, and §4.4's formula is the only thing you must carry forward.
